use std::cmp::Ordering;
use url::Url;
use reqwest;
use json::{self, JsonValue};
use rustc_serialize::hex::ToHex;
use std::str::FromStr;
use sha3::{Digest, Sha3_256};
use ed25519_dalek::*;

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

pub fn count_size(item: &JsonValue, alignment: u64) -> u64{
    let mut total_size = 0;
    match item {
        JsonValue::Array(json_array) => {
            let mut layout_size: u64 = json_array.iter() 
                                                 .map(|layout| count_size(layout, alignment))
                                                 .sum();
            if alignment > 0 {
                layout_size = ((layout_size + alignment - 1) / alignment) * alignment;
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

pub fn build_transaction(parsed_tx: &json::Array) -> json::Array {
    let mut built_tx = parsed_tx.clone();

    let mut some_layer_payload_size = built_tx.iter_mut().find(|lf| lf["name"] == "payload_size");
    let some_layer_transactions = parsed_tx.iter().find(|&lf| lf["name"] == "transactions");
    match some_layer_payload_size {
        Some(ref mut layer_payload_size) => {
            match some_layer_transactions {
                Some(layer_transactions) => {
                    layer_payload_size["value"] = count_size(&layer_transactions, 0).into();
                }
                None => ()
            }
        }
        None => ()
    }

    let mut some_leyer_transaction_hash = built_tx.iter_mut().find(|lf| lf["name"] == "transactions_hash");
    match some_leyer_transaction_hash {
        Some(ref mut layer_transaction_hash) => {
            let mut hashes = Vec::new();
            match some_layer_transactions {
                Some(layer_transactions) => {
                    let tx_layout = must_json_array_as_ref(&layer_transactions["layout"]);
                    for e_tx in tx_layout {
                        let hexed_vec = hex::decode(hexlify_transaction(e_tx, 0)).unwrap(); 
                        let mut hasher = Sha3_256::new();
                        hasher.update(hexed_vec);
                        hashes.push(hasher.finalize());
                    }
                }
                None => ()
            }

            let mut num_remaining_hashes = hashes.len();
            while num_remaining_hashes > 1 {
                let mut i = 0;
                while i < num_remaining_hashes {
                    let mut hasher = Sha3_256::new();
                    hasher.update(hashes[i]);

                    if i + 1 < num_remaining_hashes {
                        hasher.update(hashes[i + 1]);
                    } else {
                        hasher.update(hashes[i]);
                        num_remaining_hashes += 1;
                    }
                    hashes[i/2] = hasher.finalize();
                    i += 2;
                }
                num_remaining_hashes = num_remaining_hashes / 2;
            }
            layer_transaction_hash["value"] = hashes[0].to_hex().into();
        }
        None => ()
    }
    println!("{:?}", built_tx.len());

    built_tx
}

pub fn hexlify_transaction(item: &JsonValue, alignment: usize) -> String{
    let mut payload = String::new();
    match item {
        JsonValue::Array(item) => {
            let mut sub_layout_hex = "".to_string();
            for layout in item {
                sub_layout_hex += &hexlify_transaction(layout, alignment);
            }
            if alignment > 0 {
                let aligned_size = ((sub_layout_hex.len() + (alignment * 2) - 2)/(alignment * 2)) * alignment * 2; // 浮動小数を削った、テスト時に注意
                sub_layout_hex += &"0".repeat(aligned_size - sub_layout_hex.len());
            }
            payload += &sub_layout_hex;
        }
        _ => {
            if item.has_key("layout") {
                for layer in must_json_array_as_ref(&item["layout"]) {
                    let item_alignment = if item.has_key("alignment") {
                        item["alignment"].as_usize().unwrap() // 浮動小数を削った、テスト時に注意
                    } else {
                        0
                    };
                    payload += &hexlify_transaction(layer, item_alignment);
                }
            } else {
                let size = item["size"].as_usize().unwrap();
                let item_value = if item.has_key("value") {
                    item["value"].clone()
                } else {
                    if size >= 24 {
                        "00".repeat(size).into()
                    } else {
                        0.into()
                    }
                };

                if size == 1 {
                    if item["name"] == "element_disposition" {
                        payload = item_value.to_string();
                    } else {
                        let hex_string = format!("{:02x}", u64::from_str(&item_value.to_string()).unwrap());
                        payload = hex_string;
                    }
                } else if size == 2 || size == 4 || size == 8 {
                    let mut buf = "".to_string();
                    // 文字列(decimal)を数値(uint64)に変換
                    let mut item_value_num = u64::from_str(&item_value.to_string()).unwrap();
                    // 256進数と見なし、数値(uint64)を文字列(hex)に変換する
                    while item_value_num / 256 >= 1 {
                        let b = item_value_num % 256;
                        buf += &format!("{:02x}", b);
                        item_value_num /= 256;
                    }
                    let b = item_value_num % 256;
                    buf += &format!("{:02x}", b);

                    // 足りない分は"00"で埋める
                    let len = buf.len() / 2;
                    assert!(len <= size);
                    let d = size - len;
                    payload = buf + &"00".repeat(d);
                } else if size == 24 || size == 32 || size == 64 {
                    payload = item_value.to_string();
                } else {
                    println!("Unkown size");
                }
            }
        }
    }
    payload
}

pub fn get_verifiable_data(built_tx: &json::Array) -> json::Array {
    let type_layer = built_tx.iter().find(|&bf| bf["name"] == "type").unwrap();
    if ["16705".to_string(), "16961".to_string()].contains(&type_layer["value"].to_string()){
        built_tx[5..11].to_vec()
    } else {
        built_tx[5..].to_vec()
    }
}

pub fn hash_transaction(signer: String, signature: String, built_tx: &json::Array, network: &JsonValue) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(hex::decode(signature).unwrap());
    hasher.update(hex::decode(signer).unwrap());
    hasher.update(hex::decode(network["generationHash"].to_string()).unwrap());
    hasher.update(hex::decode(hexlify_transaction(&get_verifiable_data(&built_tx).into(), 0)).unwrap());

    let tx_hash = hasher.finalize().to_hex(); // 正常に動作するか要確認
    tx_hash
}

pub fn updtae_transaction(built_tx: &json::Array, name: String, type_string: String, value: &JsonValue) -> json::Array {
    let mut update_tx = built_tx.clone();
    update_tx.iter_mut().find(|x| x["name"] == name).unwrap()[type_string] = value.clone();
    update_tx
}

pub fn sign_transaction(built_tx: &json::Array, my_secret_key: String, network: &JsonValue) -> String {
    let tmp_sec_seed = hex::decode(my_secret_key).unwrap();
    let tmp_key_pair = Keypair::from_bytes(&tmp_sec_seed).unwrap();
    let verifiable_data = get_verifiable_data(built_tx);
    let payload = network["generationHash"].to_string() + &hexlify_transaction(&verifiable_data.into(), 0);

    let verifiable_buffer = hex::decode(payload).unwrap();
    let signature = tmp_key_pair.sign(&verifiable_buffer);

    signature.to_string()
}

pub fn cosign_transaction(tx_hash: String, my_secret_key: String) -> String {
    let tmp_sec_seed = hex::decode(my_secret_key).unwrap();
    let tmp_key_pair = Keypair::from_bytes(&tmp_sec_seed).unwrap();
    let tx_hash_bytes = hex::decode(tx_hash).unwrap();
    let signature = tmp_key_pair.sign(&tx_hash_bytes);

    signature.to_string()
}