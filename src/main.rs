use std::env::args;
// use std::error::Error;
use url::{Url};

use scraper::{Html, Selector};
use regex::Regex;
use percent_encoding::percent_decode_str;
use minicalc::compute;

// const GILETTE_URL: &str = "https://www3.zippyshare.com/v/CDCi2wVT/file.html";

fn main() {
    for link in args().skip(1) {
        println!("{}", direct_link(&link));
    }
}

fn direct_link(url: &String) -> String {
    let body = reqwest::get(url).unwrap().text().unwrap();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("#lrbox .right script").unwrap();

    let script_content = document
        .select(&selector)
        .take(1)
        .collect::<Vec<_>>()[0]
        .inner_html();

    let re = Regex::new(r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/" \+ \(([\d+% ]+)\) \+ "/([/\w%.]+)";"#).unwrap();
    let groups = re.captures(script_content.as_str()).unwrap();

    let domain = Url::parse(url).unwrap().host_str().unwrap().to_owned();
    let key = &groups[1];
    let file_name_encoded = &groups[3];
    // let file_name = percent_decode_str(file_name_encoded).decode_utf8().unwrap();
    let secret = compute(&groups[2]).unwrap();

    //println!("{}, {}, {}, {}", domain, key, secret, file_name);

    format!("https://{}/d/{}/{}/{}", domain, key, secret, file_name_encoded)
}