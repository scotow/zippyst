use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str;

#[cfg(feature = "fetch")]
use hyper::body::HttpBody;
#[cfg(feature = "fetch")]
use hyper::client::Client;
#[cfg(feature = "fetch")]
use hyper::{header, Body, Response};
#[cfg(feature = "fetch")]
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use percent_encoding::percent_decode_str;
use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

use crate::error::Error;

lazy_static! {
    static ref VARIABLE_REGEX: Regex = Regex::new(r#"var\s*(\w+)\s*=\s*([\d+\-*/%]+);?"#).expect("cannot build variable matching regex");
    static ref LINK_GENERATOR_REGEX: Regex = Regex::new(r#"document\.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s*\+\s*([\d\w\s+\-*/%()]+)\s*\+"/([/\w%.-]+)";?"#).expect("cannot build link generator regex");
}

#[derive(Clone, Debug)]
pub struct File {
    pub domain: String,
    pub id: String,
    pub key: i64,
    pub name: String,
    pub encoded_name: String,
}

impl File {
    #[cfg(feature = "fetch")]
    pub async fn fetch_and_parse(url: &str) -> Result<Self, Error> {
        async fn fetch(url: &str) -> Result<Response<Body>, Error> {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let response = client
                .get(
                    url.parse()
                        .map_err(|err| Error::InvalidUrl { source: err })?,
                )
                .await
                .map_err(|err| Error::ContentFetchingFailure { source: err })?;

            if !(response.status().is_success() || response.status().is_redirection()) {
                return Err(Error::InvalidStatusCode {
                    code: response.status(),
                });
            }
            Ok(response)
        }

        let mut response = fetch(url).await?;
        // Follow only one redirection.
        if response.status().is_redirection() {
            let location = response
                .headers()
                .get(header::LOCATION)
                .ok_or(Error::RedirectionFailure)?
                .to_str()
                .map_err(|_| Error::RedirectionFailure)?;
            response = fetch(location).await?;
        }

        // Final response.
        if !response.status().is_success() {
            return Err(Error::InvalidStatusCode {
                code: response.status(),
            });
        }

        let mut page_content = Vec::new();
        while let Some(next) = response.data().await {
            let chunk = next.map_err(|err| Error::ContentStreamingFailure { source: err })?;
            page_content.extend_from_slice(&chunk);
        }

        Self::parse(
            url,
            str::from_utf8(&page_content)
                .map_err(|err| Error::InvalidUtf8Content { source: err })?,
        )
    }

    pub fn parse(url: &str, page_content: &str) -> Result<Self, Error> {
        let script_content = {
            let document = Html::parse_document(page_content);
            let selector =
                Selector::parse("#lrbox script").map_err(|_| Error::InvalidCssSelector)?;

            Ok(document
                .select(&selector)
                .find_map(|elem| {
                    let html = elem.inner_html();
                    LINK_GENERATOR_REGEX.is_match(&html).then(|| html)
                })
                .ok_or(Error::ScriptNotFound)?)
        }?;

        let vars = VARIABLE_REGEX
            .captures_iter(&script_content)
            .map(|groups| {
                Ok((
                    groups
                        .get(1)
                        .map(|g| g.as_str())
                        .ok_or(Error::VariableExtractionFailure)?,
                    tinyexpr::interp(
                        groups
                            .get(2)
                            .ok_or(Error::VariableExtractionFailure)?
                            .as_str(),
                    )
                    .map_err(|err| Error::VariableComputationFailure { source: err })?
                        as i64,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<HashMap<_, _>>();

        let groups = LINK_GENERATOR_REGEX
            .captures(&script_content)
            .ok_or(Error::LinkGeneratorExtractionFailure)?;
        let expression = vars.iter().fold(
            groups
                .get(2)
                .ok_or(Error::LinkGeneratorExtractionFailure)?
                .as_str()
                .to_owned(),
            |acc, (var, val)| acc.replace(var, &val.to_string()),
        );
        let key = tinyexpr::interp(&expression)
            .map_err(|err| Error::LinkComputationFailure { source: err })? as i64;

        Ok(Self {
            domain: Url::parse(url).unwrap().host_str().unwrap().to_owned(),
            id: groups.get(1).unwrap().as_str().to_owned(),
            key,
            name: percent_decode_str(groups.get(3).unwrap().as_str())
                .decode_utf8()
                .unwrap()
                .to_string(),
            encoded_name: groups.get(3).unwrap().as_str().to_owned(),
        })
    }

    pub fn link(&self) -> String {
        format!(
            "https://{}/d/{}/{}/DOWNLOAD",
            self.domain, self.id, self.key
        )
    }

    pub fn link_with_encoded_name(&self) -> String {
        format!(
            "https://{}/d/{}/{}/{}",
            self.domain, self.id, self.key, self.encoded_name
        )
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.link_with_encoded_name())
    }
}
