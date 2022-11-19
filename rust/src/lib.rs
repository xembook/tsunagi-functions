use std::cmp::Ordering;
use url::Url;
use reqwest;
use json::{self, JsonValue};
use rustc_serialize::hex::ToHex;

/// catjson(catapult json)をURLにあるjson形式テキストデータからloadする。
/// URLはtx["type"]とnetwork["catjasonBase"]により、一意に定まる。
/// txはトランザクション(transaction)を意味している。
pub fn load_catjson(tx: &JsonValue, network: &JsonValue) -> json::Array {

    let json_file;
	if tx["type"] == "AGGREGATE_COMPLETE" || tx["type"] == "AGGREGATE_BONDED" {
		json_file = "aggregate.json".to_string();
	} else {
		json_file = tx["type"].to_string().to_lowercase() + ".json";
	}

    let url = Url::parse(&(network["catjasonBase"].to_string() + &json_file)).unwrap();
    let json_txt_resp = reqwest::blocking::get(url).unwrap().text().unwrap();

    let json_value_parsed = json::parse(&json_txt_resp).unwrap();
    let catjson = must_json_array_as_ref(&json_value_parsed);

    catjson.clone()
}

/// 列挙型JsonValue内のArray形式である事を確認する。
/// Array形式に違いない時にのみ使用する。
/// Array形式でない時、panicして終了する。
fn must_json_array_as_ref(json_value: &JsonValue) -> &json::Array {
    match json_value {
        JsonValue::Array(ref json_array) => json_array,
        _ => panic!("error: このcaseにmatchすることは想定していない。")
    }
}

fn must_json_array_as_ref_mut(json_value: &mut JsonValue) -> &mut json::Array {
    match json_value {
        JsonValue::Array(ref mut json_array) => json_array,
        _ => panic!("error: このcaseにmatchすることは想定していない。")
    }
}

/// トランザクションレイアウトの取得
pub fn load_layout(tx: &JsonValue, catjson: &json::Array, is_emmbeded: bool) -> json::Array {
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
		layout_name = prefix.to_string() + &to_camelcase(&tx["type"].to_string()) + "Transaction";
    }

    let factory = catjson.iter().find(
        |&item| (item["factory_type"].to_string() == prefix.clone() + "Transaction") && (item["name"].to_string() == layout_name)
    ).unwrap();

    let layout = must_json_array_as_ref(&factory["layout"]);
    layout.clone()
}

/// camelcaseに変換する。
fn to_camelcase(any_case: &str) -> String {
    fn first_letter_to_uppercase(s: &str) -> String{
        let (first_letter, other) = s.split_at(1);
        first_letter.to_uppercase() + &other.to_lowercase()
    }
    any_case.split("_").map(first_letter_to_uppercase).collect()
}

/// トランザクションの事前準備
pub fn prepare_transaction(tx: &JsonValue, layout: &json::Array, network: &JsonValue) -> JsonValue
{
    let mut prepared_tx = tx.clone();
    prepared_tx["network"] = network["network"].clone();
    prepared_tx["version"] = network["version"].clone();

    if prepared_tx.has_key("message") {
        let message = "00".to_string() + &prepared_tx["message"].to_string().as_bytes().to_hex();
        prepared_tx["message"] = message.into();
    }

    if prepared_tx.has_key("name") {
        prepared_tx["name"] = prepared_tx["name"].to_string().as_bytes().to_hex().into();
    }

    if prepared_tx.has_key("value") {
        prepared_tx["value"] = prepared_tx["value"].to_string().as_bytes().to_hex().into();
    }

    if prepared_tx.has_key("mosaics") {
        let mut prepared_tx_mosaics = must_json_array_as_ref(&prepared_tx["mosaics"]).clone();
        prepared_tx_mosaics.sort_by(|a, b|
            if a["mosaic_id"].as_u64() < b["mosaic_id"].as_u64() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        );
        prepared_tx["mosaics"] = prepared_tx_mosaics.into();
    }

    for layer in layout {
        if layer.has_key("size") && !layer["size"].is_number() {
            let mut size = 0;

            if layer.has_key("element_disposition") {
                if prepared_tx.has_key(&layer["name"].to_string()) {
                    let s1 = prepared_tx[layer["name"].to_string()].to_string().len() as u64;
                    let s2 = (layer["element_disposition"]["size"].as_f64().unwrap() * 2.0) as u64;
                    size = s1 / s2;
                }
            } else if layer["size"].to_string().contains("_count") {
                if prepared_tx.has_key(&layer["name"].to_string()) {
                    size = prepared_tx[layer["name"].to_string()].len() as u64;
                } else {
                    size = 0;
                }
            } else {
                //その他のsize値はPayloadの長さを入れるため現時点では不明
            }
            prepared_tx[layer["size"].to_string()] = size.into();
        }
    }

    if tx.has_key("transactions") {
        let mut txes = Vec::new();

        let e_txes = must_json_array_as_ref(&tx["transactions"]);
        for e_tx in e_txes {
            let e_tx_map = e_tx;
            let e_catjson = load_catjson(&e_tx_map, network);
            let e_layout = load_layout(&e_tx_map, &e_catjson, true);

            // 再帰処理
            let e_prepared_tx = prepare_transaction(&e_tx_map, &e_layout, network);
            txes.push(e_prepared_tx);
        }
        prepared_tx["transactions"] = txes.into();
    }
    prepared_tx
}

pub fn parse_transaction(tx: &JsonValue, layout: &json::Array, catjson: &json::Array, network: &JsonValue) -> json::Array 
{
    let mut parsed_tx = Vec::new();
    
    for layer in layout {
        let layer_type = layer["type"].to_string();
        let mut layer_disposition = "".to_string();
        let mut tx_layer_name = tx[layer["name"].to_string()].clone();

        if layer.has_key("disposition") {
            layer_disposition = layer["disposition"].to_string();
        }

        let mut catitem = match catjson.iter().find(|x| x["name"].to_string() == layer_type) {
            Some(x) => x.clone(),
            None => JsonValue::new_object()
        };

        if layer.has_key("condition") {
            if layer["condition_operation"] == "equals" {
                if layer["condition_value"] != tx[layer["condition"].to_string()] {
                    continue;
                }
            }
        }
        if layer_disposition == "const" {
            continue;
        } else if layer_type == "EmbeddedTransaction" {
            let mut tx_layer = layer.clone();

            let mut items = Vec::new();
            for e_tx in must_json_array_as_ref(&tx["transactions"]) {

                let e_tx_map = e_tx;
                let e_catjson = load_catjson(&e_tx_map, network);
                let e_layout = load_layout(&e_tx_map, &e_catjson, true);

                // 再帰処理
                let e_prepared_tx = parse_transaction(&e_tx_map, &e_layout, &e_catjson, network);
                items.push(e_prepared_tx);
            }
            tx_layer["layout"] = items.into();
            parsed_tx.push(tx_layer);
            continue;

        } else if catitem.has_key("layout") && tx.has_key(&layer["name"].to_string()) {
            let mut tx_layer = layer.clone();
            let mut items = Vec::new();

            for item in must_json_array_as_ref(&tx_layer_name) {
                let catjson_at_layer_type = catjson.iter().find(|x| x["name"].to_string() == layer_type).unwrap();
                let catjson_layout = must_json_array_as_ref(&catjson_at_layer_type["layout"]);
                let item_parsed_tx = parse_transaction(&item, &catjson_layout, catjson, network); // 再帰
                items.push(item_parsed_tx);
            }

            tx_layer["layout"] = items.into();
            parsed_tx.push(tx_layer);
            continue;
        } else if layer_type == "UnresolvedAddress" {
            //アドレスに30個の0が続く場合はネームスペースとみなします。
            if tx.has_key(&layer["name"].to_string()) && tx_layer_name.to_string().contains("000000000000000000000000000000") {
                let cat_json_idx_value = catjson.iter().find(|x| x["name"].to_string() == "NetworkType").unwrap()["values"].clone();
                let idx = must_json_array_as_ref(&cat_json_idx_value).iter().position(|x| x["name"].to_string() == tx["network"].to_string()).unwrap();
                let prefix = format!("{:x}", must_json_array_as_ref(&cat_json_idx_value)[idx]["value"].as_u64().unwrap() + 1);
                tx_layer_name = (prefix + &tx_layer_name.to_string()).into();
            }
        } else if catitem["type"] == "enum" {
            if catitem["name"].to_string().contains("Flags") {
                let mut value = 0;
                match catitem["values"] {
                    JsonValue::Array(ref item_layers) => {
                        for item_layer in item_layers {
                            if tx_layer_name.to_string().contains(&item_layer["name"].to_string()) {
                                value += item_layer["value"].as_u64().unwrap();
                            }
                        }
                    }
                    _ => ()
                }
                catitem["value"] = value.into();
            } else if layer_disposition.contains("array") {
                let mut values = Vec::new();
                for item in must_json_array_as_ref(&tx_layer_name) {
                    values.push(must_json_array_as_ref(&catitem["values"]).iter().find(|x| x["name"].to_string() == item.to_string()).unwrap().clone());
                }
            } else {
                catitem["value"] = must_json_array_as_ref(&catitem["values"]).iter().find(|x| x["name"] == tx_layer_name).unwrap()["value"].clone();
            }
        }
        if layer_disposition.to_string().contains("array") {
            if layer_type == "byte" {
                let size = tx[layer["size"].to_string()].as_usize().unwrap();
                if layer.has_key("element_disposition") {
                    let mut sub_layout = layer.clone();

                    let mut items = Vec::new();
                    for i in 0..size {
                        let mut tx_layer = JsonValue::new_object();
                        tx_layer["signedness"] = layer["element_disposition"]["signedness"].clone();
                        tx_layer["name"] = "element_disposition".clone().into();
                        tx_layer["size"] = layer["element_disposition"]["size"].clone();
                        tx_layer["value"] = (&tx_layer_name.to_string())[i*2 .. i*2+2].into();
                        tx_layer["type"] = layer_type.clone().into();

                        items.push(tx_layer);
                    }
                    sub_layout["layout"] = items.into();
                    parsed_tx.push(sub_layout);
                }
            } else if tx.has_key(&layer["name"].to_string()) {
                let mut sub_layout = layer.clone();
                let mut items = Vec::new();

                let tx_items = must_json_array_as_ref(&tx_layer_name);

                for tx_item in tx_items {
                    let mut tx_layer = catjson.iter().find(|x| x["name"].to_string() == layer_type).unwrap().clone();
                    tx_layer["value"] = tx_item.clone();

                    if layer_type == "UnresolvedAddress" {
                        if tx_item.to_string().contains("000000000000000000000000000000") {
                            
                            let catjson_name_networktype = catjson.iter().find(|x| x["name"].to_string() == "NetworkType").unwrap();
                            let catjson_name_networktype_values = must_json_array_as_ref(&catjson_name_networktype["values"]);
                            let catjson_name_networktype_values_name_network = 
                                catjson_name_networktype_values.iter().find(|x| x["name"] == tx["network"]).unwrap();

                            let prefix = format!("{:x}", catjson_name_networktype_values_name_network["value"].as_u64().unwrap() + 1);
                            tx_layer["value"] = (prefix + &tx_layer_name.to_string()).into();
                        }
                    }
                    items.push(tx_layer);
                }
                sub_layout["layout"] = items.into();
                parsed_tx.push(sub_layout);

            }
        } else {
            let mut tx_layer = layer.clone();

            if catitem.len() > 0 {
                tx_layer["signedness"] = catitem["signedness"].clone();
				tx_layer["type"] = catitem["type"].clone();
				tx_layer["value"] = catitem["value"].clone();
				tx_layer["size"] = catitem["size"].clone();
            }

            if tx.has_key(&layer["name"].to_string()) && catitem["type"] != "enum" {
                tx_layer["value"] = tx_layer_name.clone()
            }

            parsed_tx.push(tx_layer);
        }
        
    }
    let idx = parsed_tx.iter().position(|x| x["name"] == "size");
    match idx {
        // TODO: cloneによる計算の無駄遣いを解消すべき
        Some(idx) => parsed_tx[idx]["value"] = count_size(&(parsed_tx.clone().into()), 0).into(),
        _ => ()
    }
    parsed_tx
}

fn count_size(item: &JsonValue, aligment: u64) -> u64{
    let mut total_size = 0;
    match item {
        JsonValue::Array(json_array) => {
            let mut layout_size: u64 = json_array.iter() 
                                                 .map(|layout| count_size(layout, aligment))
                                                 .sum();
            if aligment > 0 {
                layout_size = ((layout_size + aligment - 1) / aligment) * aligment;
            }
            total_size += layout_size;
        }
        _ => ()
    }
    if item.has_key("layout") {
        let layout = must_json_array_as_ref(&item["layout"]);
        for layer in layout {
            let item_alignment;
            if item.has_key("alignment") {
                item_alignment = item["alignment"].as_u64().unwrap();
            } else {
                item_alignment = 0;
            }
            total_size += count_size(layer, item_alignment);
        }
    } else if item.has_key("size") {
        total_size += item["size"].as_u64().unwrap();
    }
    total_size
}


