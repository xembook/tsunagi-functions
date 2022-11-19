use tsunagi_sdk::*;
use json::JsonValue;

fn main() {
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