use json::object;
use tsunagi_functions::v0_1_0_3_5::*; // tsunagi_functionsの関数群をインポート
use chrono::{DateTime, Local};

fn main() {

    // network情報を定義
    let network = object! {
        version:1,
        network:"TESTNET",
        generationHash:"49D6E1CE276A85B70EAFE52349AACCA389302E7A9754BCF1221E79494FC665A4",
        currencyMosaicId:0x72C0212E67A08BCEu64,
        currencyNamespaceId:0xE74B99BA41F4AFEEu64,
        currencyDivisibility:6,
        epochAdjustment:1667250467u64,
        catjasonBase:"https://xembook.github.io/tsunagi-functions/catjson/0.1.0.3.4/",
        wellknownNodes:[
            "https://sym-test-03.opening-line.jp:3001"
        ]
    };

    // 自分の秘密鍵
    let private_key: &str = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7";

    //有効期限の設定
    let dt: DateTime<Local> = Local::now();
    let timestamp: u64 = dt.timestamp().try_into().unwrap();
    let deadline_time = ((timestamp  + 7200) - &network["epochAdjustment"].as_u64().unwrap()) * 1000;

    // トランザクションを定義
    let tx = object! {
        type:"TRANSFER",
        signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
        fee:25000u64,
        deadline:deadline_time,
        recipient_address:generate_address_id("TBDSEOYRWKY5KYALQPE7QB2O36622V3YFJDF6XI"),
        mosaics:[
            {mosaic_id: network["currencyMosaicId"].as_u64().unwrap(), amount: 1u64},
        ],
        message:"Hello Tsunagi(Catjson) SDK!",
    };

    // catjsonの取得
    let catjson = load_catjson(&tx, &network);

    // トランザクションレイアウトの取得
    let layout = load_layout(&tx, &catjson, false);

    // トランザクションの事前準備
    let mut prepared_tx = prepare_transaction(&tx, &layout, &network);

    // レイアウトの解析とトランザクションデータの注入
    let parsed_tx = parse_transaction(&mut prepared_tx, &layout, &catjson, &network);

    // トランザクションの構築
    let built_tx = build_transaction(&parsed_tx);
    
    // 署名
    let signature = sign_transaction(&built_tx, private_key, &network);

    // トランザクションの更新
    let built_tx = update_transaction(&built_tx, "signature", "value", &signature);

    //hash値取得
    let _tx_hash = hash_transaction(&tx["signer_public_key"].to_string(), &signature.to_string(), &built_tx, &network);
    println!("hash: {}", _tx_hash);

    //ペイロード出力
    let payload = hexlify_transaction(&built_tx.into(), 0);
    println!("{}", payload);

    //ネットワークへ通知
    let json_request = format!(r#"{{"payload":"{}"}}"#, payload);
    let r = ureq::put(&(network["wellknownNodes"][0].to_string().to_owned() + "/transactions"))
        .set("Content-Type", "application/json")
        .send_string(&json_request);
    println!("{}", json_request);
    println!("{:?}", r);
}
