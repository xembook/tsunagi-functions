use url::{Url, ParseError};
use reqwest;
use json::{self, JsonValue};
use json::object::Object;

fn load_catjson(tx: &JsonValue, network: &JsonValue) -> Vec<JsonValue> {

    let json_file;
	if tx["type"] == "AGGREGATE_COMPLETE" || tx["type"] == "AGGREGATE_BONDED" {
		json_file = "aggregate.json".to_string();
	} else {
		json_file = tx["type"].to_string().to_lowercase() + ".json";
	}

    let url = Url::parse(&(network["catjasonBase"].to_string() + &json_file)).unwrap();
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

    match json::parse(&resp).unwrap() {
        JsonValue::Array(ref json_array) => json_array.clone(),
        _ => Vec::new()
    }
}

fn load_layout(tx: &JsonValue, catjson: &Vec<JsonValue>, is_emmbeded: bool) -> JsonValue{
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

    let factory = catjson.iter().find(|&item| (item["factory_type"] == prefix.clone() + "Transaction") &&  (item["name"] == layout_name)).unwrap();
    factory.clone()
}

// fn prepare_transaction(tx: &JsonValue, layout: &JsonValue, network: &JsonValue) {
//     let mut prepared_tx = tx.clone();
//     prepared_tx["network"] = network["network"].clone();
//     prepared_tx["version"] = network["version"].clone();

//     if prepared_tx.contains("message") {
//         prepared_tx["message"] = "00".to_string() + tx["message"].
//     }
// }

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