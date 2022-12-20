## Example

Example of get_payload() function implementation.
```rust
use tsunagi_sdk::v0_1::*;
use json::object;

fn main() {
    let network = object!{
        version:1,
        network:"TESTNET",
        generationHash:"7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
        currencyMosaicId:0x3A8416DB2D53B6C8u64,
        currencyNamespaceId:0xE74B99BA41F4AFEEu64,
        currencyDivisibility:6,
        epochAdjustment:1637848847,
        catjasonBase:"https://xembook.github.io/tsunagi-sdk/catjson/",
        wellknownNodes:[
            "https://sym-test.opening-line.jp:3001",
            "https://sym-test.opening-line.jp:3001",
            "https://sym-test.opening-line.jp:3001",
        ]
    };
    let private_key: &str = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7";
    let tx = object!{
        type:"TRANSFER",
        signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
        fee:25000u64,
        deadline:7200000u64,
        recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
        mosaics:[
            {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
            {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
        ],
        message:"Hello Tsunagi(Catjson) SDK!",
    };
    let catjson = load_catjson(&tx, &network);
    let layout = load_layout(&tx, &catjson, false);
    let mut prepared_tx = prepare_transaction(&tx, &layout, &network);
    let parsed_tx = parse_transaction(&mut prepared_tx, &layout, &catjson, &network);
    let built_tx = build_transaction(&parsed_tx);
    let signature = sign_transaction(&built_tx, private_key, &network);
    let built_tx = update_transaction(&built_tx, "signature", "value", &signature);
    
    let payload = hexlify_transaction(&built_tx.into(), 0);
    // payloadを任意の方法でJson形式でSymbolネットワークへ送信してください。
    // Send the payload to the Symbol network in Json format in any way you wish.
    
    assert_eq!(payload, 
        "dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
    );
}
```

Check the "tests" directory for details.