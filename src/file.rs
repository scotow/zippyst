use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::io::Write;
use hyper::body::HttpBody;
use crate::error::Error;
use crate::info::Information;

use regex::Regex;
use url::Url;
use hyper::client::Client;
use hyper_tls::HttpsConnector;

use percent_encoding::percent_decode_str;
use scraper::{Html, Selector};

pub struct File {
    origin: String,
}

impl File {
    pub async fn fetch_and_parse(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut res = client.get(url.parse().unwrap()).await?;

        let mut page_content = Vec::new();
        while let Some(next) = res.data().await {
            let chunk = next.unwrap();
            page_content.extend_from_slice(&chunk);
        }
        
        Self::parse(url, std::str::from_utf8(&page_content).unwrap())
    }
    
    pub fn parse(url: &str, page_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let script_content = {
            let document = Html::parse_document(page_content);
            let selector =
                Selector::parse("#lrbox .right script").map_err(|_| Error::InvalidSelector)?;
    
            Ok::<String, Error>(
                document
                    .select(&selector)
                    .nth(0)
                    .ok_or(Error::CannotFindScriptTag)?
                    .inner_html(),
            )
        }?;
        
        let var_regex = Regex::new(r#"var\s*(\w+)\s*=\s*([\d+\-*/%]+);?"#)?;
        let vars = var_regex
            .captures_iter(&script_content)
            .map(|groups| (
                groups.get(1).unwrap().as_str(),
                tinyexpr::interp(groups.get(2).unwrap().as_str()).unwrap() as i64
            ))
            .collect::<HashMap<_, _>>();
        dbg!(&vars);
        
        let href_regex = Regex::new(r#"document\.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s*\+\s*([\d\w\s+\-*/%()]+)\s*\+".+";"#).unwrap();
        let groups = href_regex
            .captures(&script_content)
            .unwrap();
        
        let expression = vars
            .iter()
            .fold(groups.get(2).unwrap().as_str().to_owned(), |acc, (var, val)| {
                acc.replace(var, &val.to_string())
            });
        let id = tinyexpr::interp(&expression).unwrap() as i64;

        dbg!(
            format!(
                "https://{}/d/{}/{}/DOWNLOAD",
                Url::parse(url).unwrap().host_str().unwrap(), groups.get(1).unwrap().as_str(), id
            )
        );
        
        Err("".into())
    }
    
    
    
    
    
    
    
    
    // pub fn new(origin: &str) -> File {
    //     File {
    //         origin: origin.to_string(),
    //     }
    // }
    // 
    // fn parse_information(
    //     &self,
    //     page_content: &str,
    // ) -> Result<Information, Box<dyn std::error::Error>> {
    //     let script_content = {
    //         let document = Html::parse_document(page_content);
    //         let selector =
    //             Selector::parse("#lrbox .right script").map_err(|_| Error::InvalidSelector)?;
    // 
    //         Ok::<String, Error>(
    //             document
    //                 .select(&selector)
    //                 .nth(0)
    //                 .ok_or(Error::CannotFindScriptTag)?
    //                 .inner_html(),
    //         )
    //     }?;
    // 
    //     let re1 = Regex::new(
    //         r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s?\+\s?\(a \* b \+ c \+ d\)\s?\+\s?"/([/\w%.-]+)";"#,
    //     )?;
    //     let re2 = Regex::new(
    //         r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s?\+\s?\(([\d+% ]+)\)\s?\+\s?"/([/\w%.-]+)";"#,
    //     )?;
    //     let re3 = Regex::new(
    //         r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/"\s?\+\s?\(([\w\d+\- ]+)\)\s?\+\s?"/([/\w%.-]+)";"#,
    //     )?;
    // 
    //     if re1.is_match(&script_content) {
    //         let re_var = Regex::new(r#"var\s?a\s?=\s?(\d+)\s?%\s?900\s?;"#)?;
    //         let var = {
    //             let groups = re_var
    //                 .captures(&script_content)
    //                 .ok_or(Error::InvalidScriptContent)?;
    //             groups[1].parse::<i64>()
    //         }?;
    // 
    //         let groups = re1
    //             .captures(&script_content)
    //             .ok_or(Error::InvalidScriptContent)?;
    //         Ok(Information {
    //             domain: String::from(
    //                 Url::parse(&self.origin)?
    //                     .host_str()
    //                     .ok_or(Error::InvalidDomain)?,
    //             ),
    //             id: String::from(&groups[1]),
    //             key: (var % 900) * (var % 53) + 8 + (var % 13),
    //             name: String::from(percent_decode_str(&groups[2]).decode_utf8()?),
    //             encoded_name: String::from(&groups[2]),
    //         })
    //     } else if re2.is_match(&script_content) {
    //         let groups = re2
    //             .captures(&script_content)
    //             .ok_or(Error::InvalidScriptContent)?;
    //         Ok(Information {
    //             domain: String::from(
    //                 Url::parse(&self.origin)?
    //                     .host_str()
    //                     .ok_or(Error::InvalidDomain)?,
    //             ),
    //             id: String::from(&groups[1]),
    //             key: compute(&groups[2])?,
    //             name: String::from(percent_decode_str(&groups[3]).decode_utf8()?),
    //             encoded_name: String::from(&groups[3]),
    //         })
    //     } else if re3.is_match(&script_content) {
    //         let re_var = Regex::new(r#"var\s?\w\s?=\s?(\d+)(?:\s?%\s?\d)?\s?;"#)?;            
    //         let [n, b, z]: [i64; 3] = re_var
    //             .captures_iter(&script_content)
    //             .map(|groups| groups[1].parse::<i64>())
    //             .collect::<Result<Vec<_>, _>>()?.try_into().map_err(|_| Error::InvalidScriptContent)?;
    // 
    //         let groups = re3
    //             .captures(&script_content)
    //             .ok_or(Error::InvalidScriptContent)?;
    //         Ok(Information {
    //             domain: String::from(
    //                 Url::parse(&self.origin)?
    //                     .host_str()
    //                     .ok_or(Error::InvalidDomain)?,
    //             ),
    //             id: String::from(&groups[1]),
    //             key: n % 2 + b % 3 + z - 3,
    //             name: String::from(percent_decode_str(&groups[3]).decode_utf8()?),
    //             encoded_name: String::from(&groups[3]),
    //         })
    //     } else {
    //         Err(Error::ScriptContentNotMatching.into())
    //     }
    // }
    // 
    // pub fn get_information(&self) -> Result<Information, Box<dyn std::error::Error>> {
    //     let page_content = fetch_content(&self.origin, 0)?;
    //     self.parse_information(&page_content)
    // }
    // 
    // pub fn get_information_retry(
    //     &self,
    //     retry: u16,
    // ) -> Result<Information, Box<dyn std::error::Error>> {
    //     let page_content = fetch_content(&self.origin, retry)?;
    //     self.parse_information(&page_content)
    // }
}

// fn fetch_content(url: &str, retry: u16) -> Result<String, Box<dyn std::error::Error>> {
//     let resp = ureq::get(url).call();
//     if resp.ok() {
//         Ok(resp.into_string()?)
//     } else if resp.error() {
//         if retry > 0 {
//             fetch_content(url, retry - 1)
//         } else {
//             Err(resp.status_text().into())
//         }
//     } else if let Some(err) = resp.synthetic_error() {
//         Err(err.status_text().into())
//     } else {
//         Err("invalid HTTP call".into())
//     }
// }
