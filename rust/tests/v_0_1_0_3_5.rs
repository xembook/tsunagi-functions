use tsunagi_sdk::v0_1_0_3_5::*;
use json::{self, object, JsonValue};

fn get_network_info() -> JsonValue {
    let network = object!{
		network:"TESTNET",
		generationHash:"49D6E1CE276A85B70EAFE52349AACCA389302E7A9754BCF1221E79494FC665A4",
		currencyMosaicId:0x72C0212E67A08BCEu64,
		currencyNamespaceId:0xE74B99BA41F4AFEEu64,
		currencyDivisibility:6,
		epochAdjustment:1667250467,
		catjasonBase:"https://xembook.github.io/tsunagi-sdk/catjson/0.1.0.3.4/",
		wellknownNodes:[
			"https://d-testnet.d-world.fun:3001",
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
    let built_tx = update_transaction(&built_tx, "signature", "value", &signature);

    let _tx_hash = hash_transaction(&tx["signer_public_key"].to_string(), &signature.to_string(), &built_tx, &network);
    let payload = hexlify_transaction(&built_tx.into(), 0);
    payload
}

const PRIVATE_KEY: &str = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7";
const BOB_PRIVATE_KEY: &str = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";
const CAROL_PRIVATE_KEY: &str = "1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf";


#[cfg(test)]
mod tsunagi_sdk_0_1_0_3_5 {
    use super::*;

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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"dc00000000000000bd41cf3a6baf502884747063380835605dc9c977f5ace60b481305078a501a2b910bf7744d9a0aa7a5530edae4f0ee05169ed608c07f63aa8241e34def6e8a065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
				"dc000000000000008caa3fbf848a932c340c02b5399bf464c754ca447767d4ce18e21d42235bb93015b2d4d3c69ab35368db8561a2a1d3667b44c81d5d5d00ce9837d6d399b3f9005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d00000000009985738c26eb1534a40000000000000000000000000000001c00020000000000eeaff441ba994be764000000000000003164838cd27f54fa01000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};

			assert_eq!(&get_payload(&tx1), 
				"dc00000000000000bd41cf3a6baf502884747063380835605dc9c977f5ace60b481305078a501a2b910bf7744d9a0aa7a5530edae4f0ee05169ed608c07f63aa8241e34def6e8a065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
				message:"",
			};

			assert_eq!(&get_payload(&tx1), 
				"c100000000000000e090dad4ca0d57046dcd98af576c1de8987ae386207369796c133082433e0a273aec474e35d3d655a327f03ca5e1d1411ed8e0a7304eec2a14eead6c67e1d40d5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b801000200000000000e3362701d5303090100000000000000ce8ba0672e21c072640000000000000000"
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
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
			};

			assert_eq!(&get_payload(&tx1), 
				"c00000000000000078703034b334b964bf4b9842a2c51eb7c71ceb031e3136a9505d75ea28fb08d450d11c5456a463d03d9370723fb2ce06af9e1b9945eeb0e5df6489392f28cc015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b800000200000000000e3362701d5303090100000000000000ce8ba0672e21c0726400000000000000"
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
				"bc00000000000000d5567b070c6b3b4004f6951b12e4b2c0f7047003d6a73c68e2bcc829fb0ddaeaa145931a9fe20c2b7ee14fae3fda560bbac2b614fb6490db5ffc25cf0135ea055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
				"a0000000000000004c1f7e1f247b3eca8e4663d801215e8942aed5c9fd15a79b17e3b1857f5c2d2c2d992f3b1772f508ae36fc0dda54e845217cca58ab9057009b4c3c089990cf015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000000000000000"
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
				"a10000000000000051dcc5f85e43b9da3f04ee24d8d847ef0bc24526fc5043a30e8ab3a535895e90dd0992062d9ed898f90d2c7e58ae7b8008af9fa2bd1c9102d1914d7e80f1850a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8010000000000000000"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
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
				"380100000000000077d5ef84458181b3f3a3e5965609e474d5edcfb98eaf3086b5a03199c5600e023ec44edae767a2b3bd9ea376147f58b897b4fbcf74c7b085567dbd7ddd8bb90c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d000000000030df2911573eb91106810924854281ff6f709644a83b9c4f58690f6b630c11c890000000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b2100000000"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
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
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());


			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,BOB_PRIVATE_KEY).into();
			prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(&tx_hash,CAROL_PRIVATE_KEY).into();
			
			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			assert_eq!(payload, 
				"280300000000000021a82d135a86bd7c393f06d935365e40f43d10ac312fc51e333c53c7819e3e114ae5638f914a6f31568df53bdbe3c38999fcebe6bb026dbb6a630355c207ee0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d00000000004f215d93f14b9e86130d761e350b1f6c41488e09f4261e0d3e03be413f1a27e3b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecff95f609a25f82bc9c048fedc07358224940bf812654ea75cc168142df3d30fb3af9274ac702fbfc167b7da2ff35e35105ae36f8d719f224682c35c3e727930c0000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4ee73a2aaf7d9103088dec6d7ce51ebe40844c00ee0707832c3916eaf4cf910d764d5ec0c7490c22c0d12a35cc4b0eb5422127c2f517567c1d1d9fdcc4645b800b"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
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
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,CAROL_PRIVATE_KEY).into();
			prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(&tx_hash,BOB_PRIVATE_KEY).into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			assert_eq!(payload, 
				"280300000000000021a82d135a86bd7c393f06d935365e40f43d10ac312fc51e333c53c7819e3e114ae5638f914a6f31568df53bdbe3c38999fcebe6bb026dbb6a630355c207ee0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d00000000004f215d93f14b9e86130d761e350b1f6c41488e09f4261e0d3e03be413f1a27e3b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e000000000000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4ee73a2aaf7d9103088dec6d7ce51ebe40844c00ee0707832c3916eaf4cf910d764d5ec0c7490c22c0d12a35cc4b0eb5422127c2f517567c1d1d9fdcc4645b800b00000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecff95f609a25f82bc9c048fedc07358224940bf812654ea75cc168142df3d30fb3af9274ac702fbfc167b7da2ff35e35105ae36f8d719f224682c35c3e727930c"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
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

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"c801000000000000811f52f76187393ed5a272a2c6d7298b88e157f90abb59931370c42fc8193b5036bda35b23d581b2828228ca2eb8e92a78655651950fe399d956ed086b67fd095f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d000000000002c11b3eaf098e517c8b3edd4a531a9b72ef9a09bb6b6ce795d11bb9f8d845dc20010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
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
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			assert_eq!(payload, 
				"c801000000000000811f52f76187393ed5a272a2c6d7298b88e157f90abb59931370c42fc8193b5036bda35b23d581b2828228ca2eb8e92a78655651950fe399d956ed086b67fd095f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d000000000002c11b3eaf098e517c8b3edd4a531a9b72ef9a09bb6b6ce795d11bb9f8d845dc20010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000"
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
				mosaic:[{mosaic_id: 0x72C0212E67A08BCEu64, amount: 10000000u64}],
				duration: 480u64,
				hash:"a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78"

			};

			assert_eq!(&get_payload(&tx1), 
				"b80000000000000072b485134459d9d446eb4e43e4ccdcf9c4d6cf2fcffc6bae7fd20dd1881fdb9686fcad36d4d3fd99642a3bf1ed202db10a900fe3b8026343d23fc5613ebf7d0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841a86100000000000000dd6d0000000000ce8ba0672e21c0728096980000000000e001000000000000a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78"
			);
		}


		#[test]
        fn test_resolves_hash_lock_by_aggregate() {

			let tx1 = object!{
				type:"HASH_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				mosaic:[{mosaic_id: 0x72C0212E67A08BCEu64, amount: 10000000u64}],
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


			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"10010000000000001d12a749ddebf175404430a1fc216df35576f2f3affcf1efefefc302a2479b936413f6b1c9652934e5bf02b9be1c093749ddf28d3f70f342b96f6b45ba47ed085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000e45792b55c83a86a6590f6094a2192b4075f515614d77f432d475a671eec43d5680000000000000068000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841ce8ba0672e21c0728096980000000000e0010000000000004ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
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

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"5802000000000000e5cfb484206223df20dcbfb416a43bb295c0c052df118b1592c52beea4c11f3bac147de1c2b227acb5531ab05f78eb6633d3325113dc04e18f4bcc6c9d21470c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414240420f000000000000dd6d00000000004f215d93f14b9e86130d761e350b1f6c41488e09f4261e0d3e03be413f1a27e3b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e00000000"
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
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
				],
				message:"Hello Tsunagi(Catjson) SDK!",
			};
			//Bob->Caroll
			let tx2 = object!{
				type:"TRANSFER",
				signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
				recipient_address:generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
				],
				message:"Hello Carol! This is Bob.",
			};
			//Caroll->Alice
			let tx3 = object!{
				type:"TRANSFER",
				signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaics:[
					{mosaic_id: 0x72C0212E67A08BCEu64, amount: 100u64},
					{mosaic_id: 0x0903531D7062330Eu64, amount: 1u64},
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
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());


			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,BOB_PRIVATE_KEY).into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);


			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"c002000000000000e5cfb484206223df20dcbfb416a43bb295c0c052df118b1592c52beea4c11f3bac147de1c2b227acb5531ab05f78eb6633d3325113dc04e18f4bcc6c9d21470c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414240420f000000000000dd6d00000000004f215d93f14b9e86130d761e350b1f6c41488e09f4261e0d3e03be413f1a27e3b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c000200000000000e3362701d5303090100000000000000ce8ba0672e21c07264000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecea2dc91f73de119d3bbdd71d7b8c414f7039fe29667468f8cd19220a66d410f9b1f2b145e4c7c14281b8c3c19dfbce959990611abba7aded9ae656ef7879ee04"
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
				"960000000000000036042771fac43d72e20e2db9006e549d07f4f5736a725155fccf96d7d3ce778c6985717be37c7bd711d0e372109b8e8ff9e4bb13bcbef479f3e4376dac7896075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d41a86100000000000000dd6d000000000076e65d50e5fbaf4d000000000000000099b560650602"
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
				"9100000000000000d960c8d7939a4562020bc12cf03096af087230e258af9e3fb68cbf2e05c5aef1f30a744c63d258233208c756e5dc3d915206d57873907c25a768b87affb773035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d42a86100000000000000dd6d000000000076e65d50e5fbaf4da08601000000000001"
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

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,PRIVATE_KEY,&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			

			assert_eq!(payload, 
				"38010000000000004c70db5e6151db0561ce26b11fb21e4ee670eba06ff5bbd467c30b77d55d8db6cd087ebaa83b386118aeb82fdf80f9c06d5f8ed66e93514dd2c8d3708c6b5a035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000fc4405540b555f4dde5dc4ce67daeaf207e5485d8da24d5cfd6bf71fa064c9a5900000000000000046000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4176e65d50e5fbaf4d000000000000000099b560650602000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4276e65d50e5fbaf4da0860100000000000100000000000000"
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
				"9900000000000000f8252165bf6ebdae4dd8a00c5550166ec14e10bb23246e9fa4a0fea5be5f022d8eec3f85663694f2de764f5ca6aa97e02ac17dc269798bf9780ec8433a2f9d0d5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d0000000000805101000000000085738c26eb1534a4000778656d626f6f6b"
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
				"9800000000000000ed38356655ef9444ce5a340d687a889e692bc5ad4a92a2e2c9d47a08b5f0dcb530543c895e9335350e3230957a189f36547adde3763029101a46b777bf00940c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d000000000085738c26eb1534a43164838cd27f54fa0106746f6d61746f"
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
				"a10000000000000096b34f684b92975319366a899644594f2458ea59823167e4ca8e9bdbe1ca73a5ffd0e4ea5db1fd0fb63680864c799c7a28e593d694d27d7231c614a5879ab7015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42a86100000000000000dd6d000000000085738c26eb1534a49869762418c5b643eee70e6f20d4d555d5997087d7a686a901"
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
				"910000000000000008e404f03cd005394f15ab902c4681b665e5dfcbcf5ef07abdc91da2c3afb49819b5593a2b0ecf3339c7e93077fd24b95f5fffee73180a6ab440616209ea4d045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43a86100000000000000dd6d00000000003164838cd27f54fa76e65d50e5fbaf4d01"
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
				"e8010000000000006c4ce112e856dd1c2b8f9668910f60f05b9645baf407425ad10d1ed870c97c745abb3a88754419cd03f4a4653c620c5874498f8790fac87ff8fb4f2d9f172c055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000de415866cedcab9dda7baa97b5bb326ad2647bfafe69d8b3587a789bff9d073c40010000000000004a000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e418051010000000000bd1cf9801594b9ed000878656d626f6f6b3100000000000049000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41bd1cf9801594b9edf47e2f57b78ec1920107746f6d61746f310000000000000051000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42bd1cf9801594b9ed9869762418c5b643eee70e6f20d4d555d5997087d7a686a9010000000000000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43f47e2f57b78ec19276e65d50e5fbaf4d0100000000000000"
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
				"18010000000000003d16c333de66d956f2b764a43d00e23e077b48718ce5530f093313d8c2adb525d3cf89de614d353a28a3812751926294d373023698054caf58cdaaf469ad5d0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000765f150d97dd08f64258c5632403090fd6f36e7d4845b7d9c0a24c1c320e9b2b70000000000000006f000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844419869762418c5b643eee70e6f20d4d555d5997087d7a686a9d72781051bb772971b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
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
				"bf000000000000005f3c77c37e335518e804631fead83bb6a754744c91be4b3e3abd131ad554767a0f011f897b56e582872112867fae2c0c4ad59eed5aa73775e476edf27b97f70e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444140420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9d72781051bb772971b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
				"20010000000000008e0ca659e08c18daffe52f5ca8cc7c7de2a8abd2481d8ad208ec6ff0f60e2c157c319935c4191c026639acbfdcb1223048bb80f344013e275ed54f27615f66025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000eadc97a286bf8081b523c4d246cf6ca05f208835b82e1f97ad978a2d638386a2780000000000000077000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844429869762418c5b643eee70e6f20d4d555d5997087d7a686a9e222a46a117e21cf76e65d50e5fbaf4d1b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
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
				"c7000000000000008f514c163b196762688400af26abd5fcdad69cea5d19ceb495798bdd25d1bd72393def5a656224416e1118b688168959b9156568c04f360c6d19cd5c5826500a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444240420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9e222a46a117e21cf76e65d50e5fbaf4d1b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
				"20010000000000007b7e80f8a26190c54e99537393860feec6629856bbfcc41d0461a9d1752e6394d2a5edccd967cd50e018d7282d3e3feff719ff4bfaaea20d0aa6952a3029c9025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000accdb81d64d2626a79d546a2380171879f39beecc9b314805ca9b5a0d2b547e4780000000000000077000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844439869762418c5b643eee70e6f20d4d555d5997087d7a686a9d9d07308378a6a8b85738c26eb1534a41b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
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
				"c70000000000000066b829b81fec0520ba4e8bff1dd46fdba2b500c6fa07017aa12683f8bcfa31406b417b93db5d2e0ecce5385812508244a6de557a5cc43fd4dbb46a1f45f8b60b5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444340420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9d9d07308378a6a8b85738c26eb1534a41b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
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
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7").into();
			prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(&tx_hash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b").into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"e001000000000000bd5c75a5987de3f81fceeb5c09d86a1f0e5c8c4c50b20e5ed4d7b5e1f39818ad3336e4d28cd75873959e6f5168fc0a44189ea1dae9fd693f557ddb343ac2100066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77000000000298414140420f000000000000dd6d0000000000336c1c549f927fd26a4ff9f3602423cb544e766d6c2c655e261c80679f185cb56800000000000000680000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee7700000000019855410101020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b89869762418c5b643eee70e6f20d4d555d5997087d7a686a900000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cbc9516c0a0364cdd944b0213151d0bb62aa22c48ab58a4ec2537f5e362e82bea3fb00de636a47f8b717bd678d539306be3509bf1016b203edf969087c8854350e00000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec3eae659a42b5d0c620229621fe15744ec5f05ce22813ddafb8c531f589d6f427d9b24f2e7d3a44e7bfadeae37b72525e663bd9041167afa5123e56dbe113a40b"
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

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());


			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"e000000000000000948d8236ee551842e8e34d11afd53601d384177c69905f4900c1f9dbbb2a7345a590e10247b71efbd981b082f69d68640bb6d0860114bd8c8372c69e8a8c2d005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000be95dbbb0adf29fe5f5a766fbf3c10e4a60e0d71c216d263e2b167e06c70dac93800000000000000380000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee7700000000019855410101000000000000"
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
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());

			//連署
			prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(&tx_hash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b").into();

			let cosignatures_layout = layout.iter().find(|&lf| lf["name"] == "cosignatures").unwrap().clone();
			let parsed_cosignatures = parse_transaction(&mut prepared_tx,&vec![cosignatures_layout],&catjson,&get_network_info()); //構築
			built_tx = update_transaction(&built_tx,"cosignatures","layout",&parsed_cosignatures[0]["layout"]);

			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			assert_eq!(payload, 
				"6001000000000000347b97a91669746f0ff2b5960df8333fde0a9afa5f83e366d29015e6fbb312c15b0205df3128f1456c5026446adead0a3693faa905ed11d59735b5fc91fdbe0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000317c15bcbe4d9edadca95ed3fbeabe47fe41e749fbc120e9b83abf57083163745000000000000000500000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee770000000001985541ffff000100000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b800000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecd49ab3ed6979ba6691dea4eba0f9be502534adc9bb4917f7bc74f8040e96ac4bd51c86f78265a0a8ef114c8a38a6726fb014bd845493d795b6767b4d0f76f604"
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

			let catjson = load_catjson(&agg_tx,&get_network_info());
			let layout = load_layout(&agg_tx,&catjson,false); //isEmbedded false

			let mut prepared_tx = prepare_transaction(&agg_tx,&layout,&get_network_info()); //TX事前準備
			let parsed_tx   = parse_transaction(&mut prepared_tx,&layout,&catjson,&get_network_info()); //TX解析
			let mut built_tx    = build_transaction(&parsed_tx); //TX構築

			let signature = sign_transaction(&built_tx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",&get_network_info());
			built_tx = update_transaction(&built_tx,"signature","value",&signature);

			//トランザクションハッシュ作成
			let _tx_hash = hash_transaction(&agg_tx["signer_public_key"].to_string(),&signature.to_string(),&built_tx,&get_network_info());


			let payload = hexlify_transaction(&built_tx.into(), 0);
			
			
			assert_eq!(payload, 
				"f8000000000000005e5a8bd2e6b74472862605623a4651fd28efaa5dc0c136753626b1246c733f1e8f0d6cc76b16f9ea3f360ac652d74a0a30c30f76b1c6afb710c4c581fe2a470f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000bdbccdc54cb19c89113a1c58ecfa776ded496a0b55568d7338530208137922fb5000000000000000500000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee770000000001985541ffff0001000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
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
				"b800000000000000feaebe027c9c2d1cafd5e8eeaadc6c886baf3c7fc7897451010c33227742fe743f69094c33b3528b36633a361f010c0e6107e29c4c165b1d93b09d7dd047c10a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
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
				"b800000000000000cdf9ff486963f1bcf0e77725137b198c2d9d868f8d61eea25b020b5a2aea0d288a9d1b3737f991bcf508f00413cd5b62ceeb79004f587d8321d4a26c4391460c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0020000000000993a7f6395187cb7c800000000000000000000000000000098f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
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
				"b800000000000000c6895eac6b6634e18d15fc188bc56c0aa8e84e7a2fe5d63af734981487f07740f5ff74e2923c8150b9f968e755350641fb4d01113fda9f000235f5c79a1eee045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0000200000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
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
				restriction_additions:[0x4DAFFBE5505DE676u64,0x0903531D7062330Eu64],
				restriction_deletions:[]
			};

			assert_eq!(&get_payload(&tx1), 
				"9800000000000000d6f89896b651db894c27879e1c2d2b2201039ef27eb4aae22f70b31e7a28a18730e0d41c8fc5319eac8f99d7fc2e650d8faefe36b638ff705d9c8b9b0d8934085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042a86100000000000000dd6d0000000000028002000000000076e65d50e5fbaf4d0e3362701d530309"
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
				restriction_deletions:[0x4DAFFBE5505DE676u64,0x0903531D7062330Eu64]
			};

			assert_eq!(&get_payload(&tx1), 
				"9800000000000000db86cc357a83d9a12fb2d9659e01e204cbd33dbbb0a9f29987ac7555412836beb3a083097a32c9c9195ab13c114141e4dc500847dfab82098fa6af2374ac8e085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042a86100000000000000dd6d0000000000028000020000000076e65d50e5fbaf4d0e3362701d530309"
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
				"8c000000000000002bfbc341548c32bfd8f6a8b015cea856cad09373f5bcbaf4792cd8001b6b3282c2b60fb79bed85c172d3443a78e696e71ddcb091d71e308108db2354c416b9005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985043a86100000000000000dd6d000000000004c002000000000054414141"
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
				"8c0000000000000080c528a1698677f875228f21884b4c3377e76a5e7236bf5bf9901c1e546190fe32c474c1baa44b8d4f10b93d94753bc6caf9e45362c73d51287f5ea127f7f6095f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985043a86100000000000000dd6d000000000004c000020000000054414141"
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
				"aa00000000000000676b3b3db2b78c44a7b7ac90bcca2207596847316666f82ce36491f718712e9803f720f6a90e7e7d07eb5a2c106dca3b70f099549433174b98aa358624b3250a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985141a86100000000000000dd6d000000000076e65d50e5fbaf4d0000000000000000d72781051bb77297000000000000000001000000000000000001"
			);
		}

		#[test]
        fn test_resolves_global_mosaic_restriction_transfer_2() {

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
				"b800000000000000a6bec6c4d6b10ea6dd527e59ea818f5397711ef9f623d19e9af606f965a0fff87d9ba26f6a5d2d17072fd05a50807c4bf1d017af6e7dcc3e5ed421d9a3d499025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985142a86100000000000000dd6d000000000076e65d50e5fbaf4dd72781051bb77297ffffffffffffffff01000000000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
			);
		}
	}

	#[cfg(test)]
    mod mosaic_supply_revocation_transaction {
        use super::*;
	
		#[test]
        fn test_resolves_mosaic_supply_revocation() {

			let tx1 = object!{
				type:"MOSAIC_SUPPLY_REVOCATION",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				fee:25000u64,
				deadline:get_deadline(&get_network_info()),
				source_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				mosaic:[{mosaic_id: 0x0552BC5EF5BD589Du64, amount: 100u64}]
			};

			assert_eq!(&get_payload(&tx1), 
				"a800000000000000e189bfa03404fa4ae502fe0369ba8f5ef79fde5dc0b881a6287b52133b29b64ca7bec6ff7a017a29376c96ea0f9c57b891883060e5bddca07f5e7bb9a2933b0c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d43a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a99d58bdf55ebc52056400000000000000"
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
				mosaic:[{mosaic_id: 0x72C0212E67A08BCEu64, amount: 10000000u64}],
				duration: 480u64,
				hash_algorithm:"SHA3_256"

			};

			assert_eq!(&get_payload(&tx1), 
				"d100000000000000abdc6bdc9bbcd3fec927e34eb387ad94d072d40b0eb168b78aee0851d220c07997cce258805c02ce50ba6d8947788536cbefbdbc008e188e5cc0fd89bcf123055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985241a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240ce8ba0672e21c0728096980000000000e00100000000000000"
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
				"cf0000000000000067d62f92fda708e2f61990fcc77b504a21e952aa7d2360c439bfccaea4fef01ee609a92c772edfdd964d28f6c566b936ed6e16ca6dcd3f83801073adae3637045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985242a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e2401400007944496ac0f572173c2549baf9ac18f893aab6d0"
			);
		}


		#[test]
        fn test_resolves_secret_lock_with_aggregate() {

			let tx1 = object!{
				type:"SECRET_LOCK",
				signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				recipient_address:generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
				secret:"0debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c",
				mosaic:[{mosaic_id: 0x72C0212E67A08BCEu64, amount: 10000000u64}],
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
				"3001000000000000772dd15e233b047029037f1fd144305687cf6172883ac0879539972e5f5b113255ccb40da50ecfa05795c421b3c5e00ed1812f9cc71f036aefad678e79b163015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d000000000057b02d03a808bd698f317b515642db890f58bb70efe9e0a33560a6f5e23b1125880000000000000081000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019852419869762418c5b643eee70e6f20d4d555d5997087d7a686a90debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68cce8ba0672e21c0728096980000000000e0010000000000000000000000000000"
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
				"2801000000000000e92c7f9d64c82b1b2e0172957df5ece0ee4e64c6a0b4a8731642421c2d1d5989fb5aca6ff48e6a1661cb0557705d8b094283b3c880a6582e1cb71e31545afc0a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000298414140420f000000000000dd6d0000000000bd584e6eb97627993d2157bc630a4c95ec783e201678539ce671e3d36367372c80000000000000007f000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019852429869762418c5b643eee70e6f20d4d555d5997087d7a686a90debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c140000d91a8258175a6213225bd4ec240f1971c8742dca00"
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
				"d100000000000000f1c8adf4a9280669a82f90b31a3696553daf230d87394ca6c484a21de044e6f740fcce22a52ac08bf98b94ef8bb47ba0e3dcd37387ea9ed2ed8616f84cdaf20b5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985241a86100000000000000dd6d00000000009985738c26eb1534a4000000000000000000000000000000760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00eeaff441ba994be78096980000000000e00100000000000000"
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
				"cf000000000000007bdebf21df930d0bfedfca3d7a3118323f4a99969696c08bd8e3b4f5d0070fe2030b541f99d7f87d540b51f5503b8f2d3778eb1b3e87ff66f58ad6f0e87e2b085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985242a86100000000000000dd6d00000000009985738c26eb1534a4000000000000000000000000000000760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00140000336b7e682903606a2fef4c91d83c4af7da3e7486"
			);
		}
	}
}//symbol-sdk