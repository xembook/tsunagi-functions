/*
def load_catjson(tx,network) 

	if tx["type"] === "AGGREGATE_COMPLETE" || tx["type"] === "AGGREGATE_BONDED" then
		json_file =  "aggregate.json"
	else
		json_file =  tx["type"].downcase + ".json"
	end

	uri = URI.parse(network["catjasonBase"] + json_file)
	json = Net::HTTP.get(uri)
	result = JSON.parse(json)
	
	return result
end
 */

/*

fn main() {
    let parsed = json::parse(r#"

    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }

    "#).unwrap();

    println!("{:?}", parsed["code"]);
}
 */

use url::{Url, ParseError};
use reqwest;
use json::{self, JsonValue};

fn load_catjson(tx: JsonValue, network: JsonValue) -> JsonValue {

    let json_file;
	if tx["type"] == "AGGREGATE_COMPLETE" || tx["type"] == "AGGREGATE_BONDED" {
		json_file = "aggregate.json".to_string();
	} else {
		json_file = tx["type"].to_string().to_lowercase() + ".json";
	}

    let url = Url::parse(&(network["catjasonBase"].to_string() + &json_file)).unwrap();
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    let parsed = json::parse(&resp).unwrap();

    parsed
}

fn main() {

	let parsed = json::parse(r#"

    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }

    "#).unwrap();

	println!("{}", "ASDadfaDFA".to_string().to_lowercase())
}