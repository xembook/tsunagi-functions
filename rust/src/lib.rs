use url::{Url, ParseError};
use reqwest;
use json::{self, JsonValue};
use json::object::Object;

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

fn load_layout(tx: JsonValue, catjson: JsonValue, is_emmbeded: bool) -> JsonValue{
    let prefix;
    if is_emmbeded {
        prefix = "Embedded".to_string();
    } else {
        prefix = "".to_string();
    }

    let layout_name;
    if tx["type"] == "AGGREGATE_COMPLETE" {
		layout_name = "AggregateCompleteTransaction".to_string();
    } else if tx["type"] == "AGGREGATE_BONDED" {
		layout_name = "AggregateBondedTransaction".to_string();
    } else {
		layout_name = prefix.to_string() + &tx["type"].to_string().to_lowercase();
    }

    let factory = match catjson {
        JsonValue::Array(ref json_array) => 
            json_array.iter().find(|&item| (item["factory_type"] == prefix.clone() + "Transaction") &&  (item["name"] == layout_name)).unwrap(),
        _ => &JsonValue::Null
    };
    factory.clone()
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