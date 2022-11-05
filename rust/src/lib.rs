use std::cmp::Ordering;

use url::Url;
use reqwest;
use json::{self, JsonValue};
use rustc_serialize::hex::ToHex;

pub fn load_catjson(tx: &JsonValue, network: &JsonValue) -> Vec<JsonValue> {

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

pub fn load_layout(tx: &JsonValue, catjson: &Vec<JsonValue>, is_emmbeded: bool) -> Vec<JsonValue> {
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
		layout_name = prefix.to_string() + &to_camelcase(tx["type"].to_string()) + "Transaction";
    }

    let factory = catjson.iter().find(|&item| (item["factory_type"].to_string() == prefix.clone() + "Transaction") && (item["name"].to_string() == layout_name)).unwrap();

    match factory["layout"] {
        JsonValue::Array(ref json_array) => json_array.clone(),
        _ => Vec::new()
    }
    
}

fn to_camelcase(snake_case: String) -> String {
    let lowercase = snake_case.replace("_", "").to_lowercase();
    let (lowercase_head, lowercase_other) = lowercase.split_at(1);
    let camelcase = lowercase_head.to_uppercase() + lowercase_other;
    camelcase
}

pub fn prepare_transaction(tx: &JsonValue, layout: &Vec<JsonValue>, network: &JsonValue) -> JsonValue
{
    
    let mut prepared_tx = tx.clone();
    prepared_tx["network"] = network["network"].clone();
    prepared_tx["version"] = network["version"].clone();

    if prepared_tx.contains("message") {
        let message = "00".to_string() + &prepared_tx["message"].to_string().as_bytes().to_hex();
        prepared_tx["message"] = message.into();
    }

    if prepared_tx.contains("name") {
        prepared_tx["name"] = prepared_tx["name"].to_string().as_bytes().to_hex().into();
    }

    if prepared_tx.contains("value") {
        prepared_tx["value"] = prepared_tx["value"].to_string().as_bytes().to_hex().into();
    }

    if prepared_tx.contains("mosaics") {
        match prepared_tx["mosaics"] {
            JsonValue::Array(ref mut json_array) => {
                json_array.sort_by(|a, b|
                    if a["mosaic_id"].as_u64() < b["mosaic_id"].as_u64() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                )
            }
            _ => ()
        }
        println!("{:?}", prepared_tx["mosaics"]);
    }

    for layer in layout {
        let layer_map = layer.clone();
        if layer_map.contains("size") {
            let mut size = 0;

            if layer_map.contains("element_disposition") {
                if prepared_tx.contains(layer_map["name"].to_string()) {
                    let s1 = prepared_tx[layer_map["name"].to_string()].len() as u64;
                    let s2 = (layer_map["element_disposition"].as_f64().unwrap() * 2.0) as u64;
                    size = (s1 / s2) as u64;
                }
            } else if layer_map["size"].to_string().contains("_count") {
                if prepared_tx.contains(layer_map["name"].to_string()) {
                    size = prepared_tx[layer_map["name"].to_string()].len() as u64;
                } else {
                    size = 0;
                }
            } else {
                //その他のsize値はPayloadの長さを入れるため現時点では不明
            }
            prepared_tx[layer_map["size"].to_string()] = size.into();
        }
    }

    if tx.contains("transactions") {
        let mut txes = Vec::new();
        let mut e_tx_map;
        let mut e_catjson;
        let mut e_layout;
        let mut e_prepared_tx;

        match tx["transactions"] {
            JsonValue::Array(ref e_txes) => {
                for e_tx in e_txes {
                    e_tx_map = e_tx;
                    e_catjson = load_catjson(&e_tx_map, &network);
                    e_layout = load_layout(&e_tx_map, &e_catjson, true);

                    // 再帰処理
                    e_prepared_tx = prepare_transaction(&e_tx_map, &e_layout, network);
                    txes.push(e_prepared_tx);
                }
                prepared_tx["transactions"] = txes.into();
            }   
            _ => ()
        }
    }
    prepared_tx
}

#[cfg(test)]
mod tests {
    use super::*;

    // let mut network = JsonValue::new_object();
    // network["version"] = 1.into();
    // network["network"] = "TESTNET".into();
    // network["generationHash"] = "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836".into();
    // network["currencyMosaicId"] = 0x3A8416DB2D53B6C8u64.into();
    // network["currencyNamespaceId"] = 0xE74B99BA41F4AFEEu64.into();
    // network["currencyDivisibility"] = 6.into();
    // network["epochAdjustment"] = 1637848847.into();
    // network["catjasonBase"] = "https://xembook.github.io/tsunagi-sdk/catjson/".into();
    // network["wellknownNodes"] = vec![
    //     "https://sym-test.opening-line.jp:3001",
    //     "https://sym-test.opening-line.jp:3001",
    //     "https://sym-test.opening-line.jp:3001",].into();

    #[test]
    fn test_load_catjson() {

        let mut network = JsonValue::new_object();
        network["catjasonBase"] = "https://xembook.github.io/tsunagi-sdk/catjson/".into();

    
        // case 1
        let mut tx = JsonValue::new_object();
        tx["type"] = "TRANSFER".into();
        let catjson = load_catjson(&tx, &network);
        assert_eq!(catjson.iter().find(|&cj| cj["name"] == "TransferTransaction").unwrap()["layout"][1]["value"], "TRANSFER");

        // case 2
        let mut tx = JsonValue::new_object();
        tx["type"] = "AGGREGATE_COMPLETE".into();
        let catjson = load_catjson(&tx, &network);
        assert_eq!(catjson.iter().find(|&cj| cj["name"] == "AggregateCompleteTransaction").unwrap()["layout"][1]["value"], "AGGREGATE_COMPLETE");
    }

    #[test]
    fn test_load_layout() {
        let mut network = JsonValue::new_object();
        network["catjasonBase"] = "https://xembook.github.io/tsunagi-sdk/catjson/".into();

        // case 1
        let mut tx = JsonValue::new_object();
        tx["type"] = "TRANSFER".into();

        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        assert_eq!(layout[1]["value"], "TRANSFER");
        assert_eq!(layout[3]["name"], "verifiable_entity_header_reserved_1");

        let elayout = load_layout(&tx, &catjson, true);
        assert_eq!(elayout[1]["value"], "TRANSFER");
        assert_eq!(elayout[3]["name"], "embedded_transaction_header_reserved_1");

        // case 2
        let mut tx = JsonValue::new_object();
        tx["type"] = "AGGREGATE_COMPLETE".into();
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        assert_eq!(layout[1]["value"], "AGGREGATE_COMPLETE");
        assert_eq!(layout[3]["name"], "verifiable_entity_header_reserved_1");
    }

    // #[test]
    // fn test_prepare_transaction() {
    // }
}