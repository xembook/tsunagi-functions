use url::{Url, ParseError};
use reqwest;
use json::{self, JsonValue};


fn main() {
	let url = Url::parse(&("https://xembook.github.io/tsunagi-sdk/catjson/0.2.0.3/".to_string() + "aggregate.json")).unwrap();
    println!("{}", url.to_string());

    let resp = reqwest::blocking::get(url).unwrap();
    let resp_txt = resp.text().unwrap();
    //println!("{:?}", resp_txt);

    let parsed = json::parse(&resp_txt).unwrap();

    println!("{:?}", parsed[0]["comments"].to_string());
}