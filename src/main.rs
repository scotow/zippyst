use scraper::{Html, Selector};
use regex::Regex;
use percent_encoding::percent_decode_str;
use mexprp::Term;

const GILETTE_URL: &str = "https://www3.zippyshare.com/v/CDCi2wVT/file.html";

fn main() {
    direct_link(GILETTE_URL);
}

fn direct_link(url: &str) -> String {
    let body = reqwest::get(url).unwrap().text().unwrap();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("#lrbox .right script").unwrap();

    let script_content = document
        .select(&selector)
        .take(1)
        .collect::<Vec<_>>()[0]
        .inner_html();

    let re = Regex::new(r#"document.getElementById\('dlbutton'\)\.href = "/d/(\w+)/" \+ \(([\d\(\)+\-*/% ]+)\) \+ "/([/\w%.]+)";"#).unwrap();
    let groups = re.captures(script_content.as_str()).unwrap();

    let key = &groups[1];
    let file_name = percent_decode_str(&groups[3]).decode_utf8().unwrap();
    let secret: f64 = Term::parse(&groups[2]).unwrap().eval().unwrap().unwrap_single();

    println!("{}, {}, {}", key, secret, file_name);

    String::from("OK")
}