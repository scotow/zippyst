use boa::Context;
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
    static ref SCRIPT_REGEX: Regex =
        Regex::new(r#"(?s)<script type="text/javascript">(.+?)</script>"#)
            .expect("cannot build script extraction regex");
    static ref OMG_SPAN_REGEX: Regex =
        Regex::new(r#"<span id="omg"\s+class="(\d+)"\s(:?style=".*")?>\s*</span>"#)
            .expect("cannot build span extraction regex");
    static ref PATH_REGEX: Regex =
        Regex::new(r#"/d/(\w+)/(\d+)/([/\w%.-]+)"#).expect("cannot build uri regex");
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
            let client = Client::builder().build::<_, Body>(https);
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

        let mut context = Context::default();
        let mut modified_script_content = format!(
            "{}\n{}\n{}\n{}",
            "let button = {};",
            "let fimage = {};",
            script_content
                .replace("document.getElementById('dlbutton')", "button")
                .replace("document.getElementById('fimage')", "fimage"),
            "button.href"
        );

        if script_content.contains("document.getElementById('omg').getAttribute('class')") {
            modified_script_content = modified_script_content.replace(
                "document.getElementById('omg').getAttribute('class')",
                OMG_SPAN_REGEX
                    .captures(page_content)
                    .ok_or(Error::LinkComputationFailure)?
                    .get(1)
                    .ok_or(Error::LinkComputationFailure)?
                    .as_str(),
            )
        }

        let path = context
            .eval(modified_script_content)
            .map_err(|_| Error::LinkComputationFailure)?
            .to_string(&mut context)
            .map_err(|_| Error::LinkComputationFailure)?;
        let groups = PATH_REGEX
            .captures(&*path)
            .ok_or(Error::LinkGeneratorExtractionFailure)?;

        Ok(Self {
            domain: uri.host().ok_or(Error::DomainExtractionFailure)?.to_owned(),
            id: groups
                .get(1)
                .ok_or(Error::FileIdExtractionFailure)?
                .as_str()
                .to_owned(),
            key: groups
                .get(2)
                .ok_or(Error::FileKeyExtractionFailure)?
                .as_str()
                .to_owned()
                .parse()
                .map_err(|_| Error::FileKeyExtractionFailure)?,
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
    use super::File;
    use hyper::body::HttpBody;
    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use md5::Context;
    use regex::Regex;

    fn match_direct_download_format(file: &File) -> bool {
        Regex::new(r#"https://(?:w+\d+\.)?zippyshare\.com/d/[\w\d]+/\d+/DOWNLOAD"#)
            .unwrap()
            .is_match(&file.link())
    }

    #[tokio::test]
    async fn file_link() {
        let file = File::fetch_and_parse(
            "https://www3.zippyshare.com/v/CDCi2wVT/file.html"
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();
        assert!(match_direct_download_format(&file));
    }

    #[tokio::test]
    async fn file_checksum() {
        let file = File::fetch_and_parse(
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

    #[test]
    fn old_formats() {
        for format in [
            include_str!("../page_payloads/2022_07_18.html"),
            include_str!("../page_payloads/2022_07_23.html"),
        ] {
            assert!(match_direct_download_format(
                &File::parse(
                    &"https://www3.zippyshare.com/v/CDCi2wVT/file.html"
                        .parse()
                        .unwrap(),
                    format
                )
                .unwrap()
            ));
        }
    }
}
