use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str;

use http::Uri;
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

use crate::error::Error;

lazy_static! {
    static ref SCRIPT_REGEX: Regex = Regex::new(r#"(?s)<script type="text/javascript">(.+?)</script>"#).expect("cannot build script extracting regex");
    static ref VARIABLE_REGEX: Regex = Regex::new(r#"var\s*(\w+)\s*=\s*([\d+\-*/%]+);?"#).expect("cannot build variable matching regex");
    static ref LINK_GENERATOR_REGEX: Regex = Regex::new(r#"document\.getElementById\('dlbutton'\)\.href\s*=\s*"/d/(\w+)/"\s*\+\s*([\d\w\s+\-*/%()]+?)\s*\+\s*"/([/\w%.-]+)";?"#).expect("cannot build link generator regex");
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
    pub async fn fetch_and_parse(uri: Uri) -> Result<Self, Error> {
        async fn fetch(uri: Uri) -> Result<Response<Body>, Error> {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let response = client
                .get(uri)
                .await
                .map_err(|err| Error::ContentFetchingFailure { source: err })?;

            if !(response.status().is_success() || response.status().is_redirection()) {
                return Err(Error::InvalidStatusCode {
                    code: response.status(),
                });
            }
            Ok(response)
        }

        let mut response = fetch(uri.clone()).await?;
        // Follow only one redirection.
        if response.status().is_redirection() {
            let location = response
                .headers()
                .get(header::LOCATION)
                .ok_or(Error::RedirectionFailure)?
                .to_str()
                .map_err(|_| Error::RedirectionFailure)?
                .parse()
                .map_err(|err| Error::InvalidUrl { source: err })?;
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
            &uri,
            str::from_utf8(&page_content)
                .map_err(|err| Error::InvalidUtf8PageContent { source: err })?,
        )
    }

    pub fn parse(uri: &Uri, page_content: &str) -> Result<Self, Error> {
        let script_content = {
            let mut content = Err(Error::ScriptNotFound);
            for cap in SCRIPT_REGEX.captures_iter(page_content) {
                let inner = cap.get(1).ok_or(Error::ScriptNotFound)?.as_str();
                if inner.contains("document.getElementById('dlbutton')") {
                    content = Ok(inner);
                    break;
                }
            }
            content
        }?;

        let vars = VARIABLE_REGEX
            .captures_iter(script_content)
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
            .captures(script_content)
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
            domain: uri.host().ok_or(Error::DomainExtractionFailure)?.to_owned(),
            id: groups
                .get(1)
                .ok_or(Error::FileIdExtractionFailure)?
                .as_str()
                .to_owned(),
            key,
            name: percent_decode_str(
                groups
                    .get(3)
                    .ok_or(Error::FilenameExtractionFailure)?
                    .as_str(),
            )
            .decode_utf8()
            .map_err(|err| Error::InvalidUtf8Filename { source: err })?
            .to_string(),
            encoded_name: groups
                .get(3)
                .ok_or(Error::FilenameExtractionFailure)?
                .as_str()
                .to_owned(),
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

#[cfg(test)]
mod tests {
    use hyper::body::HttpBody;
    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use md5::Context;
    use regex::Regex;

    #[tokio::test]
    async fn file_link() {
        let file = super::File::fetch_and_parse(
            "https://www3.zippyshare.com/v/CDCi2wVT/file.html"
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();
        assert!(
            Regex::new(r#"https://(?:w+\d+\.)?zippyshare\.com/d/[\w\d]+/\d+/DOWNLOAD"#)
                .unwrap()
                .is_match(&file.link())
        );
    }

    #[tokio::test]
    async fn file_checksum() {
        let file = super::File::fetch_and_parse(
            "https://www3.zippyshare.com/v/CDCi2wVT/file.html"
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut response = client
            .get(file.link_with_encoded_name().parse().unwrap())
            .await
            .unwrap();
        assert!(response.status().is_success());

        let mut md5 = Context::new();
        while let Some(next) = response.data().await {
            md5.consume(&next.unwrap());
        }
        assert_eq!(
            md5.compute().0,
            [111, 29, 61, 152, 64, 180, 174, 33, 189, 191, 48, 97, 160, 9, 91, 63],
        );
    }
}
