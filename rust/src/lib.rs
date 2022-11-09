use std::{cmp::Ordering};

use url::Url;
use reqwest;
use json::{self, JsonValue};
use rustc_serialize::{hex::ToHex};

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

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn parse_transaction(tx: &mut JsonValue, layout: &Vec<JsonValue>, catjson: &mut Vec<JsonValue>, network: &JsonValue) -> Vec<JsonValue> 
{
    let mut parsed_tx = Vec::new();

    for layer in layout {
        let mut layer_map = layer.clone();
        let layer_type = layer_map["type"].to_string();
        let mut layer_disposition = "".to_string();

        if layer_map.contains("disposition") {
            layer_disposition = layer_map["disposition"].to_string();
        }

        let idx = catjson.iter().position(|x| x["name"].to_string() == layer_type).unwrap();
        let mut catitem;
        //if idx >= 0 {
            catitem = catjson[idx].clone();
        //}

        if layer_map.contains("condition") {
            if layer_map["condition_operation"] == "equals" {
                if layer_map["condition_value"] != tx[layer_map["condition"].to_string()] {
                    continue;
                }
            }
        }

        if layer_disposition == "const" {
            continue;
        } else if layer_type == "EmbeddedTransaction" {
            let mut tx_layer = layer_map.clone();

            let mut items = Vec::new();
            match tx["transactions"] {
                JsonValue::Array(ref mut  e_txes) => {
                    for e_tx in e_txes {
                        //TODO: e_txがnil辺りがよくわからない
                        //if e_tx != None {
                            let mut e_tx_map = e_tx.clone();
                            let mut e_catjson = load_catjson(&e_tx_map, &network);
                            let e_layout = load_layout(&e_tx_map, &e_catjson, true);

                            // 再帰処理
                            let e_prepared_tx = parse_transaction(&mut e_tx_map, &e_layout, &mut e_catjson, &network);
                            items.push(e_prepared_tx);
                        //}
                    }
                }
                _ => ()
            }
            
            tx_layer["layout"] = items.into();
            parsed_tx.push(tx_layer);
            continue;
        } else if catitem.contains("layout") && tx.contains(layer_map["name"].to_string()) {
            let mut tx_layer = layer_map.clone();
            let mut items = Vec::new();

            match tx[layer_map["name"].to_string()] {
                JsonValue::Array(ref mut json_array) => {
                    for item in json_array {
                        let idx = catjson.iter().position(|x| x["name"].to_string() == layer_type).unwrap();
                        match catjson[idx]["layout"].clone() {
                            JsonValue::Array(ref mut json_array) => {
                                let item_parsed_tx = parse_transaction(item, &json_array, catjson, &network); // 再帰
                                items.push(item_parsed_tx);
                            }
                            _ => panic!("error")
                        }
                        
                    }
                }
                _ => ()
            }

            tx_layer["layout"] = items.into();
            parsed_tx.push(tx_layer);
            continue;
        } else if layer_type == "UnresolvedAddress" {
            //アドレスに30個の0が続く場合はネームスペースとみなします。
            if tx.contains(layer_map["name"].to_string()) && 
                //type_of(tx[layer_map["name"].to_string()]) == "String" && 
                tx[layer_map["name"].to_string()].contains("000000000000000000000000000000") {
                    let idx = catjson.iter().position(|x| x["name"].to_string() == "NetworkType").unwrap();
                    let cat_json_idx_value = catjson[idx]["values"].clone();
                    let idx2 = match cat_json_idx_value {
                        JsonValue::Array(ref json_array) => {
                            json_array.iter().position(|x| x["name"].to_string() == tx["network"].to_string()).unwrap()
                        }
                        _ => panic!("error")
                    };
                    let prefix = match cat_json_idx_value {
                        JsonValue::Array(ref json_array) => {
                            format!("{:x}", json_array[idx2]["value"].as_i32().unwrap() + 1)
                        }
                        _ => panic!("error")
                    };
                    tx[layer_map["name"].to_string()] = (prefix + &tx[layer_map["name"].to_string()].to_string()).into();
            }


        } else if catitem["type"] == "enum" {
            if catitem["name"].contains("Flags") {
                let mut value = 0;
                match catitem["values"] {
                    JsonValue::Array(ref item_layers) => {
                        for item_layer in item_layers {
                            if tx[layer_map["name"].to_string()].contains(item_layer["name"].to_string()) {
                                value += item_layer["value"].as_i32().unwrap();
                            }
                        }
                    }
                    _ => ()
                }
                catitem["value"] = value.into();
            } else if layer_disposition.contains("array") {
                let mut values = Vec::new();
                match tx[layer_map["name"].to_string()] {
                    JsonValue::Array(ref mut json_array) => {
                        for item in json_array {
                            match catitem["values"] {
                                JsonValue::Array(ref mut json_array) => {
                                    let idx = json_array.iter().position(|x| x["name"].to_string() == item.to_string()).unwrap();
                                    values.push(json_array[idx]["value"].clone());
                                }
                                _ => panic!("error")
                            };

                            values.push(catitem["values"].clone());
                        }
                        tx[layer_map["name"].to_string()] = values.into();
                    }
                    _ => panic!("error")
                }
            } else {
                let idx = match catitem["values"] {
                    JsonValue::Array(ref mut json_array) => {
                        json_array.iter().position(|x| x["name"] == tx[layer_map["name"].to_string()]).unwrap()
                    }
                    _ => panic!("error")
                };
                if idx >= 0 {
                    catitem["value"] = match catitem["values"] {
                        JsonValue::Array(ref mut json_array) => {
                            json_array[idx]["value"].clone()
                        }
                        _ => panic!("error")
                    }
                }
            }
        }

        if layer_disposition.contains("array") {
            if layer_type == "byte" {
                let size = tx[layer_map["name"].to_string()].as_usize().unwrap();
                if layer_map.contains("element_disposition") {
                    let mut sub_layout = layer_map.clone();

                    let mut items = Vec::new();
                    for i in 0..size {
                        let mut tx_layer = JsonValue::new_object();
                        tx_layer["signedness"] = layer_map["element_disposition"]["signedness"].clone();
                        tx_layer["name"] = "element_disposition".clone().into();
                        tx_layer["size"] = layer_map["element_disposition"]["size"].clone();
                        tx_layer["value"] = (&tx[layer_map["name"].to_string()].to_string())[i*2 .. i*2+2].into();
                        tx_layer["type"] = layer_type.clone().into();

                        items.push(tx_layer);
                    }
                    sub_layout["layout"] = items.into();
                    parsed_tx.push(sub_layout);
                }
            } else if tx.contains(layer_map["name"].clone()) {
                let mut sub_layout = layer_map.clone();
                let mut items = Vec::new();

                match tx[layer_map["name"].to_string()].clone() {
                    JsonValue::Array(ref mut json_array) => {
                        for tx_item in json_array {
                            let idx = catjson.iter().position(|x| x["name"].to_string() == layer_type).unwrap();
                            let mut tx_layer = if idx >= 0 { catjson[idx].clone() } else { JsonValue::new_object() };
                            tx_layer["value"] = tx_item.clone();

                            if layer_type == "UnresolvedAddress" {
                                if tx_item.contains("000000000000000000000000000000") {
                                    let idx = catjson.iter().position(|x| x["name"].to_string() == "NetworkType").unwrap();
                                    match catjson[idx]["values"] {
                                        JsonValue::Array(ref mut json_array) => {
                                            let idx2 = json_array.iter().position(|x| x["name"] == tx["network"]).unwrap();
                                            let prefix = format!("{:x}", json_array[idx2]["value"].as_i32().unwrap() + 1);
                                            tx_layer["value"] = (prefix + &tx[layer_map["name"].to_string()].to_string()).into();
                                        }
                                        _ => panic!("error")
                                    }
                                }
                            }
                            items.push(tx_layer);
                        }
                        sub_layout["layout"] = items.into();
                        parsed_tx.push(sub_layout);
                    }
                    _ => panic!("error")
                }
            }
        } else {
            let mut tx_layer = layer_map.clone();

            if catitem.len() > 0 {
                tx_layer["signedness"] = catitem["signedness"].clone();
				tx_layer["type"] = catitem["type"].clone();
				tx_layer["value"] = catitem["value"].clone();
				tx_layer["size"] = catitem["size"].clone();
            }

            if tx.contains(layer_map["name"].to_string()) && catitem["type"] != "enum" {
                tx_layer["value"] = tx[layer_map["name"].to_string()].clone()
            }

            parsed_tx.push(tx_layer);
        }
    }
    let idx = parsed_tx.iter().position(|x| x["name"] == "size").unwrap();
    if idx >= 0 {
        parsed_tx[idx]["value"] = count_size(&parsed_tx, 0).into();
    }
    parsed_tx
}

fn count_size(item: &Vec<JsonValue>, aligment: i32) -> i32{
    // TODO: 型による分岐の実装方法を知らない。
    100
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