
use tsunagi_sdk::*;
use json::{self, object, JsonValue};

fn get_network_info() -> JsonValue {
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
    network
}

fn get_deadline(network: &JsonValue) -> u64 {
    let now = network["epochAdjustment"].as_u64().unwrap();
    let deadline = ((now + 7200) - network["epochAdjustment"].as_u64().unwrap()) * 1000;
    deadline
}

fn get_payload(tx: &JsonValue) -> String {
    let network = get_network_info();
    let catjson = load_catjson(&tx, &network);
    let layout = load_layout(&tx, &catjson, false);
    let mut prepared_tx = prepare_transaction(&tx, &layout, &network);

    let parsed_tx = parse_transaction(&mut prepared_tx, &layout, &catjson, &network);
    let built_tx = build_transaction(&parsed_tx);
    let signature = sign_transaction(&built_tx, PRIVATE_KEY, &network);
    let built_tx = update_transaction(&built_tx, "signature", "value", &signature.clone().into());

    let _tx_hash = hash_transaction(&tx["signer_public_key"].to_string(), &signature, &built_tx, &network);
    let payload = hexlify_transaction(&built_tx.into(), 0);
    payload
}

const PRIVATE_KEY: &str = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7";
const BOB_PRIVATE_KEY: &str = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";
const CAROL_PRIVATE_KEY: &str = "1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf";


#[cfg(test)]
mod tsunagi_sdk_0_1 {
use super::*;

	

	

	

	#[cfg(test)]
mod function_unit_test {
use super::*;
		#[test]
fn test_load_catjson() {
			let tx1 = object!{type:"TRANSFER"};
			let catjson = load_catjson(&tx1,&get_network_info());
			assert_eq!(catjson.iter().find(|&cj| cj["name"] == "TransferTransaction").unwrap()["layout"][1]["value"], "TRANSFER");
	
			let tx2 = object!{type:"AGGREGATE_COMPLETE"};
			let catjson2 = load_catjson(&tx2,&get_network_info());
			assert_eq!(catjson2.iter().find(|&cj| cj["name"] == "AggregateCompleteTransaction").unwrap()["layout"][1]["value"], "AGGREGATE_COMPLETE");
		}	

		#[test]
fn test_load_layout() {
			let tx1 = object!{type:"TRANSFER"};

			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false
			assert_eq!(layout[1]["value"], "TRANSFER");
			assert_eq!(layout[3]["name"], "verifiable_entity_header_reserved_1");

			let elayout = load_layout(&tx1,&catjson,true); //isEmbedded true
			assert_eq!(elayout[1]["value"], "TRANSFER");
			assert_eq!(elayout[3]["name"], "embedded_transaction_header_reserved_1");

	
			let tx2 = object!{type:"AGGREGATE_COMPLETE"};
			let catjson2 = load_catjson(&tx2,&get_network_info());
			let layout2 = load_layout(&tx2,&catjson2,false); //isEmbedded false
			assert_eq!(layout2[1]["value"], "AGGREGATE_COMPLETE");
			assert_eq!(layout2[3]["name"], "verifiable_entity_header_reserved_1");
		}	

		#[test]
fn test_prepare_transaction() {
			let tx1 = object!{
				type:"TRANSFER",
				name:"xembook",
				value:"value",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false
			let prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			
			assert_eq!(prepared_tx["name"], "78656d626f6f6b");
			assert_eq!(prepared_tx["value"], "76616c7565");
			assert_eq!(prepared_tx["mosaics"][0]["mosaic_id"], 3029154504617047234u64);
			assert_eq!(prepared_tx["message"], "0048656c6c6f205473756e616769284361746a736f6e292053444b21");
			assert_eq!(prepared_tx["message_size"], 28);
			assert_eq!(prepared_tx["mosaics_count"], 2);
			
			let tx2 = object!{
				type:"AGGREGATE_COMPLETE",
				transactions:[tx1],
			};

			let catjson2 = load_catjson(&tx2,&get_network_info());
			let layout2 = load_layout(&tx2,&catjson2,false); //isEmbedded false
			let prepared_tx2 = prepare_transaction(&tx2,&layout2,&get_network_info()); //TX事前準備
			
			assert_eq!(prepared_tx2["payload_size"], 0);
			assert_eq!(prepared_tx2["transactions"][0]["name"], "78656d626f6f6b");
			assert_eq!(prepared_tx2["transactions"][0]["value"], "76616c7565");
			assert_eq!(prepared_tx2["transactions"][0]["mosaics"][0]["mosaic_id"], 3029154504617047234u64);
			assert_eq!(prepared_tx2["transactions"][0]["message"], "0048656c6c6f205473756e616769284361746a736f6e292053444b21");
			assert_eq!(prepared_tx2["transactions"][0]["message_size"], 28);
			assert_eq!(prepared_tx2["transactions"][0]["mosaics_count"], 2);

		}	


		#[test]
fn test_parse_transaction() {


			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			
			
			assert_eq!(parsed_tx[0]["value"], 220);
			assert_eq!(parsed_tx[1]["value"], 0);
			assert_eq!(parsed_tx[6]["value"], 152);
			assert_eq!(parsed_tx[10]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
			assert_eq!(parsed_tx[15]["layout"].len(), 2);
			assert_eq!(parsed_tx[15]["layout"][0][0]["value"], 3029154504617047234u64);
			assert_eq!(parsed_tx[15]["layout"][0][1]["value"], 1u64);
			assert_eq!(parsed_tx[16]["layout"].len(), 28);
			assert_eq!(parsed_tx[16]["layout"][0]["value"], "00");
			assert_eq!(parsed_tx[16]["layout"][1]["value"], "48");
			
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx2],
			};


			let catjson2 = load_catjson(&agg_tx,&get_network_info());
			let layout2 = load_layout(&agg_tx,&catjson2,false); //isEmbedded false

			let mut prepared_tx2 = prepare_transaction(&agg_tx,&layout2,&get_network_info()); //TX事前準備
			let parsed_tx2   = parse_transaction(&mut prepared_tx2,&layout2,&catjson2,&get_network_info()); //TX解析
			
			
			assert_eq!(parsed_tx2[8]["value"], 1000000u64);
			assert_eq!(parsed_tx2[9]["value"], 7200000u64);
			assert_eq!(parsed_tx2[13]["layout"][0][0]["value"], 140);
			
			assert_eq!(parsed_tx2[13]["layout"][0][1]["value"], 0);
			assert_eq!(parsed_tx2[13]["layout"][0][5]["value"], 152);
			assert_eq!(parsed_tx2[13]["layout"][0][7]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
			assert_eq!(parsed_tx2[13]["layout"][0][9]["value"], 2);
			assert_eq!(parsed_tx2[13]["layout"][0][12]["layout"][0][0]["value"], 3029154504617047234u64);
			assert_eq!(parsed_tx2[13]["layout"][0][12]["layout"][0][1]["value"], 1u64);
			assert_eq!(parsed_tx2[13]["layout"][0][13]["layout"].len(), 28);
			assert_eq!(parsed_tx2[13]["layout"][0][13]["layout"][0]["value"], "00");
			assert_eq!(parsed_tx2[13]["layout"][0][13]["layout"][1]["value"], "48");
		}	


		#[test]
fn test_parse_transaction2() {

			let tx1 = object!{
				type:"TRANSFER",
				recipient_address:"85738c26eb1534a4000000000000000000000000000000",
				mosaics:[
					{mosaic_id: 18038182949802959921u64, amount: 1u64},
					{mosaic_id: 16666583871264174062u64, amount: 100u64},
				],
			};

			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			
			assert_eq!(parsed_tx[10]["value"], "9985738c26eb1534a4000000000000000000000000000000");
			assert_eq!(parsed_tx[15]["layout"][0][0]["value"], 16666583871264174062u64);

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				transactions:[tx1],
				cosignatures:[cosignature1]
			};

			let catjson2 = load_catjson(&agg_tx,&get_network_info());
			let layout2 = load_layout(&agg_tx,&catjson2,false); //isEmbedded false

			let mut prepared_tx2 = prepare_transaction(&agg_tx,&layout2,&get_network_info()); //TX事前準備
			let parsed_tx2   = parse_transaction(&mut prepared_tx2,&layout2,&catjson2,&get_network_info()); //TX解析
			
			assert_eq!(parsed_tx2[13]["layout"][0][7]["value"], "9985738c26eb1534a4000000000000000000000000000000");
			assert_eq!(parsed_tx2[13]["layout"][0][12]["layout"][0][0]["value"], 16666583871264174062u64);
			assert_eq!(parsed_tx2[14]["layout"][0][1]["value"], "6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC");

		}	



		#[test]
fn test_parse_transaction3() {

			let tx1 = object!{
				type:"MULTISIG_ACCOUNT_MODIFICATION",
				address_additions:[
					"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
					"9869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
				],
			};
			
			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析

			

			assert_eq!(parsed_tx[12]["value"], 2);
			assert_eq!(parsed_tx[15]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
			

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				transactions:[tx1],
			};

			let catjson2 = load_catjson(&agg_tx,&get_network_info());
			let layout2 = load_layout(&agg_tx,&catjson2,false); //isEmbedded false

			let mut prepared_tx2 = prepare_transaction(&agg_tx,&layout2,&get_network_info()); //TX事前準備
			let parsed_tx2   = parse_transaction(&mut prepared_tx2,&layout2,&catjson2,&get_network_info()); //TX解析
			
			
			
			assert_eq!(parsed_tx2[13]["layout"][0][9]["value"], 2);
			assert_eq!(parsed_tx2[13]["layout"][0][12]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");

		}	

		#[test]
fn test_parse_transaction4() {

			let tx1 = object!{
				type:"ACCOUNT_ADDRESS_RESTRICTION",
				restriction_flags:"ADDRESS BLOCK OUTGOING",
				restriction_additions:["989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8","98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"],
				restriction_deletions:[]
			};
						
			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析

			


			assert_eq!(parsed_tx[10]["value"], 49153);
			assert_eq!(parsed_tx[14]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
			

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				transactions:[tx1],
			};

			let catjson2 = load_catjson(&agg_tx,&get_network_info());
			let layout2 = load_layout(&agg_tx,&catjson2,false); //isEmbedded false

			let mut prepared_tx2 = prepare_transaction(&agg_tx,&layout2,&get_network_info()); //TX事前準備
			let parsed_tx2   = parse_transaction(&mut prepared_tx2,&layout2,&catjson2,&get_network_info()); //TX解析
			
			
		
			assert_eq!(parsed_tx2[13]["layout"][0][7]["value"], 49153);
			assert_eq!(parsed_tx2[13]["layout"][0][8]["value"], 2);
			assert_eq!(parsed_tx2[13]["layout"][0][11]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");

		}	


		#[test]
fn test_build_transaction() {
			
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			assert_eq!(built_tx[11]["value"], 144);
			assert_eq!(built_tx[10]["value"], "a5b60f432c88daaf89d3154c5f1e6f7be3090c1af95ba0f21c308ecf119b2222");

			

		}	

		#[test]
fn test_build_transaction2() {
			
			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Alice.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2],
			};


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			assert_eq!(built_tx[11]["value"], 288);
			assert_eq!(built_tx[10]["value"], "00de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe31");

			

		}	

		#[test]
fn test_build_transaction3() {
			
			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:"98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:"9869762418c5b643eee70e6f20d4d555d5997087d7a686a9",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
			};


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			assert_eq!(built_tx[11]["value"], 432);
			assert_eq!(built_tx[10]["value"], "ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1");

			

		}	

		#[test]
fn test_get_verifiable_data() {
			
			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:"98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:"9869762418c5b643eee70e6f20d4d555d5997087d7a686a9",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
			};


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築
			let verifiable_data = get_verifiable_data(&built_tx);

			assert_eq!(verifiable_data[0]["name"], "version");
			assert_eq!(verifiable_data[5]["value"], "ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1");


		}	



		#[test]
fn test_get_verifiable_data2() {
			
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};


			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築
			let verifiable_data = get_verifiable_data(&built_tx);

			assert_eq!(verifiable_data[0]["name"], "version");
			assert_eq!(verifiable_data[11]["name"], "message");

		}	

		#[test]
fn test_sign_transaction() {
			
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};


			let catjson = load_catjson(&tx1,&get_network_info());
			let layout = load_layout(&tx1,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&tx1,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());

			assert_eq!(&signature, "478839283a58f4167d6f308e472c0a8e5ba410e8ac20b252af2102bfd955efc56d0250b80e07e83fecaff4d63be2f607823da0aadaa1ed13e96a75be8770780e");

		}	

		#[test]
fn test_sign_transaction2() {
			
			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築
			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());

			assert_eq!(&signature, "48b5dad7211f0ff1aee442484bac4def33fe600b37a52a39966e61ed93e54ddcb3517a60471ba4fb37660e5abf164c1ac364bdc485da5cad00cd1b7282145b08");
		}	

		#[test]
fn test_hexlify_transaction() {
			
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			assert_eq!(hexlify_transaction(&get_verifiable_data(&built_tx.into()).into(), 0), "0198414140420f000000000000dd6d0000000000a5b60f432c88daaf89d3154c5f1e6f7be3090c1af95ba0f21c308ecf119b2222");
		}	

		#[test]
fn test_hexlify_transaction2() {
			
			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Alice.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2],
			};


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			assert_eq!(hexlify_transaction(&get_verifiable_data(&built_tx.into()).into(), 0), "0198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe31");
		}	


		#[test]
fn test_count_size() {
			
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析

			assert_eq!(count_size(&parsed_tx.into(), 0), 312);
		}	

		#[test]
fn test_count_size2() {
			
			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:"98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82",
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Alice.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2],
			};


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let built_tx    = build_transaction(&parsed_tx); //TX構築

			assert_eq!(count_size(built_tx.iter().find(|&lf| lf["name"] == "transactions").unwrap(), 0), 288);
		}	

		#[test]
fn test_update_transaction() {
			

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let cosignature2 = object!{
				version:0u64,
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
				cosignatures:[cosignature1,cosignature2]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.into());

			assert_eq!(built_tx[2]["value"], "6f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c");
		}	

		#[test]
fn test_cosign_transaction() {
			

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let cosignature2 = object!{
				version:0u64,
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
				cosignatures:[cosignature1,cosignature2]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.clone().into());

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature,&built_tx,&get_network_info());


			assert_eq!(cosign_transaction(&tx_hash,BOB_PRIVATE_KEY), "e4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a708");
		}	

		#[test]
fn test_convert() {
			assert_eq!(generate_namespace_id("xembook", 0), 11832106220717372293u64);
			assert_eq!(convert_address_alias_id(generate_namespace_id("xembook", 0)), "85738c26eb1534a4000000000000000000000000000000");
			assert_eq!(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"), "9869762418c5b643eee70e6f20d4d555d5997087d7a686a9");
			assert_eq!(generate_namespace_id("tomato",generate_namespace_id("xembook", 0)), 18038182949802959921u64);

			let nonce = 1700836761;
			assert_eq!(generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), 5597969824159229558u64);

			assert_eq!(generate_key("key_account"), 10912986173756483543u64);
		}	
	}

	#[cfg(test)]
mod transfer_transaction {
use super::*;

		#[test]
fn test_resolves_2_mosaic_transfer() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}

		#[test]
fn test_resolves_2_mosaic_transfer_by_namespace() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:convert_address_alias_id(generate_namespace_id("xembook", 0)),

				mosaics:[
					{mosaic_id: generate_namespace_id("xym",generate_namespace_id("symbol", 0)), amount: 100u64},
					{mosaic_id: generate_namespace_id("tomato",generate_namespace_id("xembook", 0)), amount: 1u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"dc00000000000000a1bcb56de796c45cd982e79748772cd9a616a084c95fc775a1d003b9f5f2dcbffa95e869e8a2d77873bbe3d26d5c2764e8299bded689037e4ede6095008cc2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d00000000009985738c26eb1534a40000000000000000000000000000001c00020000000000eeaff441ba994be764000000000000003164838cd27f54fa01000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}


		#[test]
fn test_resolves_opposite_mosaice_order() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}

		#[test]
fn test_resolves_0_byte_message() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"",
			};

			assert_eq!(&get_payload(&tx1), 
				"c100000000000000c086746240315084735ebee633ff541056c5ba0f17c4d924a4b59c9531aa72243eaa7b76e5e0a9e32a15fb475be49a2f1ff1e380c763bcb2ab3ef5d83125b40d5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80100020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a640000000000000000"
			);
		}


		#[test]
fn test_resolves_undefined_message() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
			};

			assert_eq!(&get_payload(&tx1), 
				"c000000000000000fee4646022be8647455bc876a8f7f303233d297a5755cd1eb41999ae6c8cca2f0225e2b93c4aa793c68657c230578dc3af26c3ef32acae96ea1ae10c438278055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a6400000000000000"
			);
		}

		#[test]
fn test_resolves_null_mosaic_transfer() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"bc00000000000000cd5b93e94f053a07a5a132d7f59708b6818d88840c150d6f6dc38a2ca2408fff0e7e3ee39599d1242a0e4a5869dec8a2847b05fb698fa39db2bf1c3bf46ce2005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}


		#[test]
fn test_resolves_undefined_message_and_null_mosaic() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[],
			};

			assert_eq!(&get_payload(&tx1), 
				"a0000000000000002c271a17d41832515a9ad0e995a524a4859a001436a990370c4b53eaa63677b4d69edde0831171a10defc157ea01f1d5528a562c423e38c725fc5b37af35ee055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000000000000000"
			);
		}

		#[test]
fn test_resolves_0_byte_message_and_null_mosaic() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[],
				message:""
			};

			assert_eq!(&get_payload(&tx1), 
				"a100000000000000786d46993afe584dd4e1fd2904d8eb0ea67e27ca3c7ef81fd208a6f27c1450807234093f9be03bbda0b02d96a69bd2766595ac4ab59fbc5119d247181b5596065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8010000000000000000"
			);
		}


	}


	#[cfg(test)]
mod aggregate_complete_transaction {
use super::*;

		#[test]
fn test_resolves_siimple_complete() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"380100000000000048b5dad7211f0ff1aee442484bac4def33fe600b37a52a39966e61ed93e54ddcb3517a60471ba4fb37660e5abf164c1ac364bdc485da5cad00cd1b7282145b085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000a5b60f432c88daaf89d3154c5f1e6f7be3090c1af95ba0f21c308ecf119b222290000000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b2100000000"
			);

		}


		#[test]
fn test_resolves_3_account_transfer() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let cosignature2 = object!{
				version:0u64,
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
				cosignatures:[cosignature1,cosignature2]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.clone().into());

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature,&built_tx,&get_network_info());


			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,BOB_PRIVATE_KEY).into();
			prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(&tx_hash,CAROL_PRIVATE_KEY).into();
			
			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			assert_eq!(payload, 
				"28030000000000006f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afece4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a7080000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e4794f0617eae1f3f862c286c3e75494f0bb8009f8a8bccf8acb3ceb7719234f0282cdddab7bbc6adb8041788a8642729ec53ea8f6e107e8e2615ae592d44a60c"
			);

		}

		#[test]
fn test_resolves_opposite_cosignature() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let cosignature2 = object!{
				version:0u64,
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
				cosignatures:[cosignature2,cosignature1]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.clone().into());

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature,&built_tx,&get_network_info());

			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,CAROL_PRIVATE_KEY).into();
			prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(&tx_hash,BOB_PRIVATE_KEY).into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			assert_eq!(payload, 
				"28030000000000006f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e000000000000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e4794f0617eae1f3f862c286c3e75494f0bb8009f8a8bccf8acb3ceb7719234f0282cdddab7bbc6adb8041788a8642729ec53ea8f6e107e8e2615ae592d44a60c00000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afece4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a708"
			);

		}

		#[test]
fn test_resolves_no_cosignature() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Alice.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2],
				cosignatures:[]

			};

			assert_eq!(&get_payload(&agg_tx), 
				"c80100000000000083de0648e05d23036b302e5249554f6fc164917021d4cf07f1d19dfefaea34bfb8679fde237115d5ac3885ef4d4d76c16d4a930429970edbc1fb32a967d0d5025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe3120010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000"
			);
		}

		#[test]
fn test_resolves_undefined_cosignature() {

			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Alice.",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"c80100000000000083de0648e05d23036b302e5249554f6fc164917021d4cf07f1d19dfefaea34bfb8679fde237115d5ac3885ef4d4d76c16d4a930429970edbc1fb32a967d0d5025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe3120010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000"
			);
			
		}

	} // aggregate complete transaction

	#[cfg(test)]
mod aggregate_bonded_transaction {
use super::*;

		#[test]
fn test_resolves_hash_lock() {

			let tx1 = object!{
				type:"HASH_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				mosaic:[{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 10000000u64}],
				duration: 480u64,
				hash:"a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78"

			};

			assert_eq!(&get_payload(&tx1), 
				"b8000000000000008f0e4dc6dc42be7428219f820718d723803b0dde5455adec3f8ed1871318656ccd7fb4aab539ff722384b0cccf2d66603d5458a12ea01e12ffdd7bbbca9c5a0a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841a86100000000000000dd6d0000000000c8b6532ddb16843a8096980000000000e001000000000000a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78"
			);
		}


		#[test]
fn test_resolves_hash_lock_by_aggregate() {

			let tx1 = object!{
				type:"HASH_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				mosaic:[{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 10000000u64}],
				duration: 480u64,
				hash:"4ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c"

			};
			
			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"1001000000000000c2941402f941376e3b58c9931c45cd768334fe6ac65e9b746fe484e8ec8067795f5ba4b895ff582395a5d74e8f79a861c6239495b3b38e6215d9d9eef699ac055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000064b515ba0d874c0e0db27687514491d0bb74969cb82b767dde37a0b330a9f3ee680000000000000068000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841c8b6532ddb16843a8096980000000000e0010000000000004ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c"
			);


		}

		#[test]
fn test_resolves_3_account_transfer() {


			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};


			let agg_tx = object!{
				type:"AGGREGATE_BONDED",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"5802000000000000ffd1ebcc029c4997d904586292aa1aab8c87e992cd736c074d639419aeae7adc82ce4782f1276f2504b0c4548777dd48616754205c7741af5b2a248f89b4c4035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414240420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e00000000"
			);

		}


		#[test]
fn test_resolves_3_account_transfer_partial_complete() {


			//Alice->Bob
			let tx1 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				mosaics:[
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
					{mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
				],
				message:"Hello Alice, This is Carol.",
			};

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_BONDED",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3],
				cosignatures:[cosignature1]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.clone().into());

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature,&built_tx,&get_network_info());


			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,BOB_PRIVATE_KEY).into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);


			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"c002000000000000ffd1ebcc029c4997d904586292aa1aab8c87e992cd736c074d639419aeae7adc82ce4782f1276f2504b0c4548777dd48616754205c7741af5b2a248f89b4c4035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414240420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecbdde1c296d3e2e82bb1ae878586f832aa59080290b0f095f7e8d73921b6d2f67742e4588795f41b652500fdc1230ce4f45d2099fe37e182ebbe0f86121336e03"
			);

		}

	} // aggregate bonded transaction


	#[cfg(test)]
mod mosaic_transaction {
use super::*;

		//Failure_Mosaic_Modification_Disallowed
		#[test]
fn test_resolves_mosaic_definition() {
			let nonce = 1700836761;
			let tx1 = object!{
				type:"MOSAIC_DEFINITION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				duration: 0u64,
				id:generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676u64,
				nonce: nonce,
				flags: "TRANSFERABLE RESTRICTABLE",
				divisibility: 2
			};

			assert_eq!(&get_payload(&tx1), 
				"96000000000000008400ea1dd86f206c946ae4aacfdd2d9997ceb406028e3d3e67e0b20a2a0dae696d9084f7f38f64c56450a3d6cd305722cb37d60b462358bbe23b5d8e155a3f0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d41a86100000000000000dd6d000000000076e65d50e5fbaf4d000000000000000099b560650602"
			);
		}

		#[test]
fn test_resolves_mosaic_supply_change() {
			let nonce = 1700836761;
			let tx1 = object!{
				type: "MOSAIC_SUPPLY_CHANGE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				mosaic_id:generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676u64,
				delta: 1000u64 * 100u64, // assuming divisibility = 2
				action: "INCREASE"
			};

			assert_eq!(&get_payload(&tx1), 
				"9100000000000000cbec54081f0a62d5c5f84748df4668670fad447b53b44062e58d5cab054a2c8bcd8ab13533231825eda6156d4c73ba98978eccb011b0107f9bc9a0f071888e035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d42a86100000000000000dd6d000000000076e65d50e5fbaf4da08601000000000001"
			);
		}

		#[test]
fn test_resolves_aggregate_mosaic_definition_and_supply_change() {

			//buffer.Buffer.from(nonce.nonce).read_u_int32_le();
			//buffer.Buffer.from(new Uint32_array([699275411]).buffer).to_string("hex");
			//sym.MosaicId.create_from_nonce(nonce, alice.address).to_hex();
			let nonce = 1700836761;
			let tx1 = object!{
				type:"MOSAIC_DEFINITION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				duration: 0u64,
				id:generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676u64,
				nonce: 1700836761,
				flags: "TRANSFERABLE RESTRICTABLE",
				divisibility: 2
			};

			let tx2 = object!{
				type: "MOSAIC_SUPPLY_CHANGE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				mosaic_id:generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676u64,
				delta: 1000u64 * 100u64, // assuming divisibility = 2
				action: "INCREASE"

			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"3801000000000000c4b2ea423fd6eaa69407fb261cdb09b3d039923ad15a120ad1f1da61bcfd69db9b71cddc0bff730b3cd1b421b35f8cbc87a4765a204412b5efc34e221d50b20a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000fc4405540b555f4dde5dc4ce67daeaf207e5485d8da24d5cfd6bf71fa064c9a5900000000000000046000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4176e65d50e5fbaf4d000000000000000099b560650602000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4276e65d50e5fbaf4da0860100000000000100000000000000"
			);

		}



	}

	#[cfg(test)]
mod namespace_transaction {
use super::*;

		#[test]
fn test_resolves_root_namespace_regisration() {

			let tx1 = object!{
				type:"NAMESPACE_REGISTRATION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				duration: 86400u64,
				registration_type: "ROOT",
				name:"xembook",
				id:generate_namespace_id("xembook", 0) //0xA43415EB268C7385u64, //Big_int((new sym.Namespace_id("xembook")).id.to_string())
			};

			assert_eq!(&get_payload(&tx1), 
				"99000000000000003983d675dd3affcab71fb09ee51cbddd4e8ee587335e030472dd50370333266ad571c4c94410262c7bb2ecc99b2b4b8eab71245046f41518d52ef6d5355792055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d0000000000805101000000000085738c26eb1534a4000778656d626f6f6b"
			);
		}

		#[test]
fn test_resolves_sub_namespace_regisration() {

			let tx1 = object!{
				type:"NAMESPACE_REGISTRATION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				parent_id: generate_namespace_id("xembook", 0), //0xA43415EB268C7385u64, //Big_int((new sym.Namespace_id("xembook")).id.to_string())
				registration_type: "CHILD",
				name:"tomato",
				id:generate_namespace_id("tomato",generate_namespace_id("xembook", 0)) //0xFA547FD28C836431u64, //Big_int((new sym.Namespace_id("xembook.tomato")).id.to_string())
			};

			assert_eq!(&get_payload(&tx1), 
				"9800000000000000942e0fe89a3471a075f2cbd06cc64d4d8af5cd8e58c437aa39fa05e47bb9230c9a259d9da70279c8656749792310585b138e19889b3b41e7e01c14a4cbea1b025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d000000000085738c26eb1534a43164838cd27f54fa0106746f6d61746f"
			);
		}

		#[test]
fn test_resolves_address_alias() {

			let tx1 = object!{
				type:"ADDRESS_ALIAS",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				namespace_id:generate_namespace_id("xembook", 0), //0xA43415EB268C7385u64, //Big_int((new sym.Namespace_id("xembook")).id.to_string()),
				address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				alias_action:"LINK"
			};

			assert_eq!(&get_payload(&tx1), 
				"a1000000000000008f61856b455c0a57db652844aa761281c019511d7b0cd0ae9b54e4b22585f36f012116860ce9f5300fe0e91521be74d8434032bf73e008b2f52d5f0744ef13045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42a86100000000000000dd6d000000000085738c26eb1534a49869762418c5b643eee70e6f20d4d555d5997087d7a686a901"
			);
		}

		#[test]
fn test_resolves_mosaic_alias() {

			let tx1 = object!{
				type:"MOSAIC_ALIAS",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				namespace_id:generate_namespace_id("tomato",generate_namespace_id("xembook", 0)), //0xFA547FD28C836431u64, //Big_int((new sym.Namespace_id("xembook.tomato")).id.to_string())
				mosaic_id:0x4DAFFBE5505DE676u64,
				alias_action:"LINK"
			};

			assert_eq!(&get_payload(&tx1), 
				"910000000000000041f45e3bbbc8073c14b6e05b71fa9299692d1eafc86300b59698207ef044c7db8e346870c57df722497803549e2e3f8d5777c5e1c98fdf27a562d814076acc035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43a86100000000000000dd6d00000000003164838cd27f54fa76e65d50e5fbaf4d01"
			);
		}

		#[test]
fn test_resolves_namespace_by_aggregate() {

			let tx1 = object!{
				type:"NAMESPACE_REGISTRATION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				duration: 86400u64,
				registration_type: "ROOT",
				name:"xembook1",
				id:generate_namespace_id("xembook1", 0) //0xA43415EB268C7385u64, //Big_int((new sym.Namespace_id("xembook")).id.to_string())
			};

			let tx2 = object!{
				type:"NAMESPACE_REGISTRATION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				parent_id: generate_namespace_id("xembook1", 0), //0xA43415EB268C7385u64, //Big_int((new sym.Namespace_id("xembook")).id.to_string())
				registration_type: "CHILD",
				name:"tomato1",
				id:generate_namespace_id("tomato1",generate_namespace_id("xembook1", 0)) //0xFA547FD28C836431u64, //Big_int((new sym.Namespace_id("xembook.tomato")).id.to_string())
			};

			let tx3 = object!{
				type:"ADDRESS_ALIAS",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				namespace_id:generate_namespace_id("xembook1", 0), //0xA43415EB268C7385u64, //Big_int((new sym.Namespace_id("xembook")).id.to_string()),
				address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				alias_action:"LINK"
			};

			let tx4 = object!{
				type:"MOSAIC_ALIAS",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				namespace_id:generate_namespace_id("tomato1",generate_namespace_id("xembook1", 0)), //0xFA547FD28C836431u64, //Big_int((new sym.Namespace_id("xembook.tomato")).id.to_string())
				mosaic_id:0x4DAFFBE5505DE676u64,
				alias_action:"LINK"
			};


			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1,tx2,tx3,tx4],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"e801000000000000989f5f8a026d6e5c45301ce06af70406bd9c3694604a9e0718c3bac4dff9b95494d397210817139ebf43306a7bb43242e200afc4205b9a3cb439ffb1e2a14c015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000de415866cedcab9dda7baa97b5bb326ad2647bfafe69d8b3587a789bff9d073c40010000000000004a000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e418051010000000000bd1cf9801594b9ed000878656d626f6f6b3100000000000049000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41bd1cf9801594b9edf47e2f57b78ec1920107746f6d61746f310000000000000051000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42bd1cf9801594b9ed9869762418c5b643eee70e6f20d4d555d5997087d7a686a9010000000000000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43f47e2f57b78ec19276e65d50e5fbaf4d0100000000000000"
			);
		}
	}
	
	#[cfg(test)]
mod metadata_transaction {
use super::*;

		#[test]
fn test_resolves_account_metadata() {

			let tx1 = object!{
				type:"ACCOUNT_METADATA",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				scoped_metadata_key:generate_key("key_account"), //0x9772B71B058127D7u64, //"key_account"
				value_size_delta:27,
				value:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"1801000000000000f6f159ba7929828c37c4c81987ffe73709c9cbf5c4139827236d780c0ce3d6cfae5d9d76619055ca31caeed995da7aafba3a2635da58af8820d6127fc1f96d025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000765f150d97dd08f64258c5632403090fd6f36e7d4845b7d9c0a24c1c320e9b2b70000000000000006f000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844419869762418c5b643eee70e6f20d4d555d5997087d7a686a9d72781051bb772971b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
			);
		}

		#[test]
fn test_resolves_account_metadata_without_aggregate() {
			//ResourceNotFound

			let tx1 = object!{
				type:"ACCOUNT_METADATA",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				scoped_metadata_key:generate_key("key_account"), //0x9772B71B058127D7u64, //"key_account"
				value_size_delta:27,
				value:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"bf00000000000000eff25fc449d936edd1003af36b028a53dad3b5ee1a0f4502682a5a79159fac4712a528cebdf4351f28aa7b417b6906d08022135b9bbe55abeb2fb50831e555035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444140420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9d72781051bb772971b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}


		#[test]
fn test_resolves_mosaic_metadata() {

			let tx1 = object!{
				type:"MOSAIC_METADATA",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				target_mosaic_id:0x4DAFFBE5505DE676u64,
				scoped_metadata_key:generate_key("key_mosaic"), //0xCF217E116AA422E2u64, //"key_mosaic"
				value_size_delta:27,
				value:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"2001000000000000b6c125c94aed659dc1346151d03e72552dfe57c6882baa924310f3cddea39c2387bf238aba046b6a7a2f849802b38bb5f046de6b455e60a2fbaa5f22eb4d53005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000eadc97a286bf8081b523c4d246cf6ca05f208835b82e1f97ad978a2d638386a2780000000000000077000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844429869762418c5b643eee70e6f20d4d555d5997087d7a686a9e222a46a117e21cf76e65d50e5fbaf4d1b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
			);
		}

		#[test]
fn test_resolves_mosaic_metadata_without_aggregate() {
			//ResourceNotFound

			let tx1 = object!{
				type:"MOSAIC_METADATA",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				target_mosaic_id:0x4DAFFBE5505DE676u64,
				scoped_metadata_key:generate_key("key_mosaic"), //0xCF217E116AA422E2u64, //"key_mosaic"
				value_size_delta:27,
				value:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"c700000000000000dc5ff39f1dc61eb4500f2d19dec8bc36f03dffcb65504e906e6f6e6eecb71022b9e88993f4f55de53ba969563ad48be7fb718aaf1021617a1fa6324814f9b0045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444240420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9e222a46a117e21cf76e65d50e5fbaf4d1b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}

		#[test]
fn test_resolves_namespace_metadata() {

			let tx1 = object!{
				type:"NAMESPACE_METADATA",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				target_namespace_id:generate_namespace_id("xembook", 0), //xembook
				scoped_metadata_key:generate_key("key_namespace"), //0x8B6A8A370873D0D9u64, //"key_namespace"
				value_size_delta:27,
				value:"Hello Tsunagi(Catjson) SDK!",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"2001000000000000b80a6851320a00db3e505516813106f3190fbbd349266667373faf85cdd370cec50c3224c8110c62147a3bffb7be8b1bc09aec7d7701bc15299b45c18ddd70015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000accdb81d64d2626a79d546a2380171879f39beecc9b314805ca9b5a0d2b547e4780000000000000077000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844439869762418c5b643eee70e6f20d4d555d5997087d7a686a9d9d07308378a6a8b85738c26eb1534a41b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
			);
		}

		#[test]
fn test_resolves_namespace_metadata_without_aggregate() {
			//ResourceNotFound

			let tx1 = object!{
				type:"NAMESPACE_METADATA",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				target_namespace_id:generate_namespace_id("xembook", 0), //xembook
				scoped_metadata_key:generate_key("key_namespace"), //0x8B6A8A370873D0D9u64, //"key_namespace"
				value_size_delta:27,
				value:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"c700000000000000093141b56044db8b665d8c86f3119988f505c8ac3ad557f22383b2d8b0e2177fd934059df8e7890632b0ec020a087bf0569f8a992201a9e9a69870aa14351e055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444340420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9d9d07308378a6a8b85738c26eb1534a41b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
			);
		}

	}

	#[cfg(test)]
mod multisig_transaction {
use super::*;
	

		#[test]
fn test_resolves_multisig_account_modification_address_additions() {

			let tx1 = object!{
				type:"MULTISIG_ACCOUNT_MODIFICATION",
				signer_public_key:"66ADB706BC9A93E6E803B2B76A1341A8ACD98690EF204B402643AE3D4701EE77",
				min_removal_delta:1,
				min_approval_delta:1,
				address_additions:[
					generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
					generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI")
				],
				address_deletions:[]
			};

			let cosignature1 = object!{
				version:0u64,
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				signature:"",
			};

			let cosignature2 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"66ADB706BC9A93E6E803B2B76A1341A8ACD98690EF204B402643AE3D4701EE77",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
				cosignatures:[cosignature1,cosignature2]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,"22F0BA129FE0C66BA596D7127B85961BF8EEF32784364338BACB4E88D6F284D6",&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.clone().into());
			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature,&built_tx,&get_network_info());

			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7").into();
			prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(&tx_hash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b").into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"e001000000000000bf1f73806cedc96540806bd2535be327889448b282e23168ab543037774e202f8d89245c6d4af99f476cf547decd0ae1f77b809ce2e3ec33eb39f3bcaed7970d66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77000000000198414140420f000000000000dd6d0000000000336c1c549f927fd26a4ff9f3602423cb544e766d6c2c655e261c80679f185cb56800000000000000680000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee7700000000019855410101020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b89869762418c5b643eee70e6f20d4d555d5997087d7a686a900000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cbfa0f1120bdd09578723c07734b3a6bebe9810b2871d24532f7bcaba613f821a1ae9835b5dc3927ff6f350f3ca5e3490067b76d23274e264ba30b58e0643fea0400000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec959793a935b0c02b860238732cff351bef70d82af806ce56223205ee6d89f76fedb8fb4b5ae0004bced6db5e5fc6283c3d8a78a1a54a4de197dc75bef0b1210a"
			);
		}

		#[test]
fn test_resolves_multisig_account_modification_change_delta() {

			let tx1 = object!{
				type:"MULTISIG_ACCOUNT_MODIFICATION",
				signer_public_key:"66ADB706BC9A93E6E803B2B76A1341A8ACD98690EF204B402643AE3D4701EE77",
				min_removal_delta:1,
				min_approval_delta:1,
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"e00000000000000000e3f627769675a51f98c7a9745e8540e74f33a2ed63932e7de26f87b98dc94af51b133ab7d6826e1de86c9cbaa160a7aec5da4d9972ce915da8bf0c85a3010f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000be95dbbb0adf29fe5f5a766fbf3c10e4a60e0d71c216d263e2b167e06c70dac93800000000000000380000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee7700000000019855410101000000000000"
			);
		}

		#[test]
fn test_resolves_multisig_account_modification_address_deletions() {

			let tx1 = object!{
				type:"MULTISIG_ACCOUNT_MODIFICATION",
				signer_public_key:"66ADB706BC9A93E6E803B2B76A1341A8ACD98690EF204B402643AE3D4701EE77",
				min_removal_delta:-1,
				min_approval_delta:-1,
				address_additions:[],
				address_deletions:[
					generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")
				]
			};

			let cosignature2 = object!{
				version:0u64,
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				signature:"",
			};


			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
				cosignatures:[cosignature2]
			};

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature.clone().into());

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature,&built_tx,&get_network_info());

			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b").into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"6001000000000000a119633807603dffcfa86a981b4e31d97f6d21a024470139aeee0400ea748695943486743bf3fa3c6edf6207b47afbdfec3b086b8560e9ba3460f43a70391c0e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000317c15bcbe4d9edadca95ed3fbeabe47fe41e749fbc120e9b83abf57083163745000000000000000500000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee770000000001985541ffff000100000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b800000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afeccf75de4350bbfd1a8f4e7fb3c42e7c50725a9403ca5234fb71268c5eb6691734007a5fe0f8cc14a93011e76a6477e0e8ae1d7a0f9607e4deed709d1a3937c309"
			);
		}


		#[test]
fn test_resolves_multisig_account_modification_address_deletions_2() {

			let tx1 = object!{
				type:"MULTISIG_ACCOUNT_MODIFICATION",
				signer_public_key:"66ADB706BC9A93E6E803B2B76A1341A8ACD98690EF204B402643AE3D4701EE77",
				min_removal_delta:-1,
				min_approval_delta:-1,
				address_deletions:[generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI")]
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"f800000000000000434d6772e7f92dc5daf56ec8310afce152203a8c3b1dc25d87d1fe1d2300f452d28801cb6e1e59f77ad4f73cac28cdd027e17a44040c7bbb8185462359f1ad005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000bdbccdc54cb19c89113a1c58ecfa776ded496a0b55568d7338530208137922fb5000000000000000500000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee770000000001985541ffff0001000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
			);
		}

	
	}

	#[cfg(test)]
mod account_restriction_transaction {
use super::*;
	
	
		#[test]
fn test_resolves_2_address_restriction_additions_transfer() {

			let tx1 = object!{
				type:"ACCOUNT_ADDRESS_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"ADDRESS BLOCK OUTGOING",
				restriction_additions:[generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")],
				restriction_deletions:[]
			};

			assert_eq!(&get_payload(&tx1), 
				"b8000000000000005dd7b8579be90231ada1d6f4158ff6ce47f17a7946f0f9872a5a2d451ab5920c389e8237e25560e71e4e4e2dfcb3c8e297642f1ed78975ab206a984feef2de095f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
			);
		}

		#[test]
fn test_resolves_2_address_restriction_additions_by_namespace() {

			let tx1 = object!{
				type:"ACCOUNT_ADDRESS_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"ADDRESS BLOCK OUTGOING",
				restriction_additions:[

					convert_address_alias_id(
						generate_namespace_id("bob",generate_namespace_id("xembook", 0))
					),
					generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")],
				restriction_deletions:[]
			};

			assert_eq!(&get_payload(&tx1), 
				"b8000000000000005cdc385106ea9a1896d12d2c7c171f1975df38298ddb065eab1fa63dad9f87eb19ab72611d3e28d1841b5dd708f489bc8266f27e66d7d8400e15aff0b8da4e045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0020000000000993a7f6395187cb7c800000000000000000000000000000098f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
			);
		}


		#[test]
fn test_resolves_2_address_restriction_deletions_transfer() {

			let tx1 = object!{
				type:"ACCOUNT_ADDRESS_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"ADDRESS BLOCK OUTGOING",
				restriction_additions:[],
				restriction_deletions:[generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")]
			};

			assert_eq!(&get_payload(&tx1), 
				"b8000000000000000f2d19602306a1418229fa0ba8a67bbd0cbc777e9aa51a14647528ae5477ce26cc1bb5f4ceaf8d4a36b58955f324d76a520278438125b2df676fa6089f6ac3035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0000200000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
			);
		}

		#[test]
fn test_resolves_2_mosaic_restriction_additions_transfer() {

			let tx1 = object!{
				type:"ACCOUNT_MOSAIC_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"MOSAIC_ID BLOCK",
				restriction_additions:[0x4DAFFBE5505DE676u64,0x2A09B7F9097934C2u64],
				restriction_deletions:[]
			};

			assert_eq!(&get_payload(&tx1), 
				"9800000000000000e876104b9595db102728e39715c548b5992a3743348fe2a01f53e155ca27d955482c1a2ca05d3a2f1784db510427d14431f112a6bf339b1f709e95f4d441f4015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042a86100000000000000dd6d0000000000028002000000000076e65d50e5fbaf4dc2347909f9b7092a"
			);
		}

		#[test]
fn test_resolves_2_mosaic_restriction_deletions_transfer() {

			let tx1 = object!{
				type:"ACCOUNT_MOSAIC_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"MOSAIC_ID BLOCK",
				restriction_additions:[],
				restriction_deletions:[0x4DAFFBE5505DE676u64,0x2A09B7F9097934C2u64]
			};

			assert_eq!(&get_payload(&tx1), 
				"980000000000000063ccbdc7a6bd545d6751a1ed5dad87eda2efbef52462d19ba2e9a1f39d36999539b833a4732d12b87e8ea6802002fb575ac8d04ab4398827e12a41f0a425600c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042a86100000000000000dd6d0000000000028000020000000076e65d50e5fbaf4dc2347909f9b7092a"
			);
		}
	
		#[test]
fn test_resolves_2_operation_restriction_additions_transfer() {

			let tx1 = object!{
				type:"ACCOUNT_OPERATION_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"TRANSACTION_TYPE BLOCK OUTGOING",
				restriction_additions:["TRANSFER","AGGREGATE_COMPLETE"],
				restriction_deletions:[]
			};

			assert_eq!(&get_payload(&tx1), 
				"8c00000000000000d78ef15ed98496bc101c340ef9862fccca5e73aea04b93612ec00c4bd745674c1173827b81449a1e120bb960c176d56339f2b3d8ce7b7cd95d4b438f6e96ad075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985043a86100000000000000dd6d000000000004c002000000000054414141"
			);
		}
	
		#[test]
fn test_resolves_2_operation_restriction_deletions_transfer() {

			let tx1 = object!{
				type:"ACCOUNT_OPERATION_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				restriction_flags:"TRANSACTION_TYPE BLOCK OUTGOING",
				restriction_additions:[],
				restriction_deletions:["TRANSFER","AGGREGATE_COMPLETE"]
			};

			assert_eq!(&get_payload(&tx1), 
				"8c000000000000000d553c66bdddc3e1a91cd7cf8ee3fe1bd92f4a9c25a876b78ad95876d03856bb8bcb7cff0bc64ed8a672d1d8592068140a040dbd83e5970ebd57e30be3a31a0a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985043a86100000000000000dd6d000000000004c000020000000054414141"
			);
		}
	}

	#[cfg(test)]
mod global_mosaic_restriction_transaction {
use super::*;
	
		#[test]
fn test_resolves_global_mosaic_restriction_transfer() {

			let tx1 = object!{
				type:"MOSAIC_GLOBAL_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				mosaic_id:0x4DAFFBE5505DE676u64,
				reference_mosaic_id:0u64,
				restriction_key:0x9772B71B058127D7u64,
				previous_restriction_value:0u64,
				new_restriction_value:0x1u64,
				previous_restriction_type:"NONE",
				new_restriction_type:"EQ"
			};

			assert_eq!(&get_payload(&tx1), 
				"aa0000000000000083b18a9467dd39067ef18dc9eb5d7ee69b51fc68c954586d4291e68407ab41feca86224ddafcd9fd2b5375f33e1e4bb4de031b47fa42c742d4adb82fc60caf0e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985141a86100000000000000dd6d000000000076e65d50e5fbaf4d0000000000000000d72781051bb77297000000000000000001000000000000000001"
			);
		}

		#[test]
fn test_resolves_global_mosaic_restriction_transfer_1() {

			let tx1 = object!{
				type:"MOSAIC_ADDRESS_RESTRICTION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				mosaic_id:0x4DAFFBE5505DE676u64,
				restriction_key:0x9772B71B058127D7u64,
				previous_restriction_value:0xFFFFFFFFFFFFFFFFu64,
				new_restriction_value:0x1u64,
				target_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI")
			};

			assert_eq!(&get_payload(&tx1), 
				"b80000000000000040748328e8dab01fee7f82b4e23b3ed2c6336783790f286aa75eab0982c2a60af9eb5a2c524a427ec5d7e451d47a3f1620e5478ae37d048441fd7f2675e4880e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985142a86100000000000000dd6d000000000076e65d50e5fbaf4dd72781051bb77297ffffffffffffffff01000000000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
			);
		}

	
	
	
	}

	#[cfg(test)]
mod mosaic_supply_revocation_transaction {
use super::*;
	
		#[test]
fn test_resolves_mosaic_supply_revocation_2() {

			let tx1 = object!{
				type:"MOSAIC_SUPPLY_REVOCATION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				source_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaic:[{mosaic_id: 0x0552BC5EF5BD589Du64, amount: 100u64}]
			};

			assert_eq!(&get_payload(&tx1), 
				"a800000000000000fd67cc1e3962d068da002cc79531e8972575a771cddf8b9317492bc1022dfc80944540bfca1aba09c44aedf56aef5a7c00eb7f569ef7f94a7f0dad46eebcb70a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d43a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a99d58bdf55ebc52056400000000000000"
			);
		}

	
	}

	#[cfg(test)]
mod secret_lock_proof_transaction {
use super::*;
	
		#[test]
fn test_resolves_secret_lock() {

			let tx1 = object!{
				type:"SECRET_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				secret:"f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240",
				mosaic:[{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 10000000u64}],
				duration: 480u64,
				hash_algorithm:"SHA3_256"

			};

			assert_eq!(&get_payload(&tx1), 
				"d1000000000000000117860215bbc73d6ab56fa39f5ae1495ff55ad76104c3371701de042d6a0865bfb551ece7549abf636d60d443d690beee087f9417290b65da230abf280039085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985241a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240c8b6532ddb16843a8096980000000000e00100000000000000"
			);

		}

		#[test]
fn test_resolves_secret_proof() {

			let tx1 = object!{
				type:"SECRET_PROOF",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				secret:"f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240",
				hash_algorithm:"SHA3_256",
				proof:"7944496ac0f572173c2549baf9ac18f893aab6d0"
			};

			assert_eq!(&get_payload(&tx1), 
				"cf000000000000008a17b7e88005e436580b8b500bf01da70fb22906065590412c458f31094a11c4fee2b08cc1025f40642f96285ffa54bb1a88c4cf373f11b6c240ce146b41a4055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985242a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e2401400007944496ac0f572173c2549baf9ac18f893aab6d0"
			);

		}


		#[test]
fn test_resolves_secret_lock_with_aggregate() {

			let tx1 = object!{
				type:"SECRET_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				secret:"0debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c",
				mosaic:[{mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 10000000u64}],
				duration: 480u64,
				hash_algorithm:"SHA3_256"
			};

			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"3001000000000000185b61702bbf8298e2c117f0b262f644f59269ceaf7dbce9851543a62a01f726a08a3409da1d6ecd9ed30d6aa56289c3cfeba7377e66dbc36751493cdc28920e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000944643651ac8d446192f0dcbe1c370610552ddd0be94a9ba77ce7063e693cb29880000000000000081000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019852419869762418c5b643eee70e6f20d4d555d5997087d7a686a90debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68cc8b6532ddb16843a8096980000000000e0010000000000000000000000000000"
			);
		}


		#[test]
fn test_resolves_secret_proof_with_aggregate() {

			let tx1 = object!{
				type:"SECRET_PROOF",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				secret:"0debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c",
				hash_algorithm:"SHA3_256",
				proof:"d91a8258175a6213225bd4ec240f1971c8742dca"
			};


			let agg_tx = object!{
				type:"AGGREGATE_COMPLETE",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:1000000u64,
				deadline:get_deadline(&get_network_info()),
				transactions:[tx1],
			};

			assert_eq!(&get_payload(&agg_tx), 
				"2801000000000000b26c51b84114750e4005ab6002c5d6646f6de025bdf1fadbe429953044be5e61c1ecf6adaaf4cba51f0748a1cb17f82c9adf091eb7c05f4f5b5b226ead64f9065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000bd584e6eb97627993d2157bc630a4c95ec783e201678539ce671e3d36367372c80000000000000007f000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019852429869762418c5b643eee70e6f20d4d555d5997087d7a686a90debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c140000d91a8258175a6213225bd4ec240f1971c8742dca00"
			);
		}

		#[test]
fn test_resolves_secret_lock_by_namespace() {

			let tx1 = object!{
				type:"SECRET_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:convert_address_alias_id(generate_namespace_id("xembook", 0)),
				secret:"760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00",
				mosaic:[{mosaic_id: generate_namespace_id("xym",generate_namespace_id("symbol", 0)), amount: 10000000u64}],
				duration: 480u64,
				hash_algorithm:"SHA3_256"

			};

			assert_eq!(&get_payload(&tx1), 
				"d100000000000000936ffff90a654017eb900af35ea4f5a687b38b111190e2c1f9992e542c5be0bb25f22c14440b67442149de0cf5c7ea0e9642f19badcbc8b6aec8bda707c4a2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985241a86100000000000000dd6d00000000009985738c26eb1534a4000000000000000000000000000000760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00eeaff441ba994be78096980000000000e00100000000000000"
			);

		}

		#[test]
fn test_resolves_secret_proof_by_namespace() {

			let tx1 = object!{
				type:"SECRET_PROOF",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				recipient_address:convert_address_alias_id(generate_namespace_id("xembook", 0)),
				secret:"760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00",
				hash_algorithm:"SHA3_256",
				proof:"336b7e682903606a2fef4c91d83c4af7da3e7486"
			};

			assert_eq!(&get_payload(&tx1), 
				"cf0000000000000043d7a84b4c20435ffdd50644a2a0eaaed667326975d8af93015013899f5b4741f92d66ea7b2d23ad57fb5cd8d71344ffe34dd9654dbc58d9a447aaab70814d0b5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985242a86100000000000000dd6d00000000009985738c26eb1534a4000000000000000000000000000000760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00140000336b7e682903606a2fef4c91d83c4af7da3e7486"
			);

		}

	}

}//symbol-sdk