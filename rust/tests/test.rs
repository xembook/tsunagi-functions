#[cfg(test)]
mod unit_test {
    use tsunagi_sdk::*;
    use json::{self, object, JsonValue};
    #[test]
    fn test_load_catjson_1() {
        let network = get_network_info();
    
        // case 1
        let tx = object!{"type": "TRANSFER"};
        let catjson = load_catjson(&tx, &network);
        assert_eq!(catjson.iter().find(|&cj| cj["name"] == "TransferTransaction").unwrap()["layout"][1]["value"], "TRANSFER");
    }
    #[test]
    fn test_load_catjson_2() {
        let network = get_network_info();

        // case 2
        let tx = object!{"type": "AGGREGATE_COMPLETE"};
        let catjson = load_catjson(&tx, &network);
        assert_eq!(catjson.iter().find(|&cj| cj["name"] == "AggregateCompleteTransaction").unwrap()["layout"][1]["value"], "AGGREGATE_COMPLETE");
    }

    #[test]
    fn test_load_layout_1() {
        let network = get_network_info();
        let tx = object!{"type": "TRANSFER"};
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        assert_eq!(layout[1]["value"], "TRANSFER");
        assert_eq!(layout[3]["name"], "verifiable_entity_header_reserved_1");
        let elayout = load_layout(&tx, &catjson, true);
        assert_eq!(elayout[1]["value"], "TRANSFER");
        assert_eq!(elayout[3]["name"], "embedded_transaction_header_reserved_1");

    }
    #[test]
    fn test_load_layout_2() {
        let network = get_network_info();
        let tx = object!{"type": "AGGREGATE_COMPLETE"};
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        assert_eq!(layout[1]["value"], "AGGREGATE_COMPLETE");
        assert_eq!(layout[3]["name"], "verifiable_entity_header_reserved_1");
    }

    #[test]
    fn test_prepare_transaction_1() {
        let network = get_network_info();
        let tx = object! {
            type:"TRANSFER",
            name:"xembook",
            value:"value",
            mosaics:[
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
            ],
            message:"Hello Tsunagi(Catjson) SDK!",
        };
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        let prepared_tx = prepare_transaction(&tx, &layout, &network);
        assert_eq!(prepared_tx["name"], "78656d626f6f6b");
        assert_eq!(prepared_tx["value"], "76616c7565");
        assert_eq!(prepared_tx["mosaics"][0]["mosaic_id"], 3029154504617047234u64);
        assert_eq!(prepared_tx["message"], "0048656c6c6f205473756e616769284361746a736f6e292053444b21");
        assert_eq!(prepared_tx["message_size"], 28);
        assert_eq!(prepared_tx["mosaics_count"], 2);
    }
    #[test]

    fn test_prepare_transaction_2() {
        let network = get_network_info();
        let tx = object! {
            type:"TRANSFER",
            name:"xembook",
            value:"value",
            mosaics:[
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
            ],
            message:"Hello Tsunagi(Catjson) SDK!",
        };
        let agg_tx = object! {
            type: "AGGREGATE_COMPLETE",
			transactions: [tx],
        };
        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        assert_eq!(prepared_tx["payload_size"], 0);
        assert_eq!(prepared_tx["transactions"][0]["name"], "78656d626f6f6b");
        assert_eq!(prepared_tx["transactions"][0]["value"], "76616c7565");
        assert_eq!(prepared_tx["transactions"][0]["mosaics"][0]["mosaic_id"], 3029154504617047234u64);
        assert_eq!(prepared_tx["transactions"][0]["message"], "0048656c6c6f205473756e616769284361746a736f6e292053444b21");
        assert_eq!(prepared_tx["transactions"][0]["message_size"], 28);
        assert_eq!(prepared_tx["transactions"][0]["mosaics_count"], 2);
    }

    #[test]
    fn test_parse_transaction_1() {
        let network = get_network_info();

        // case 1
        let tx = object! {
            type:"TRANSFER",
                signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
                fee:1000000u64,
                deadline: get_deadline(&network),
                recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
                mosaics:[
                    {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
                    {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
                ],
                message:"Hello Tsunagi(Catjson) SDK!",
        };
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        let prepared_tx = prepare_transaction(&tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        
        assert_eq!(parsed_tx[0]["value"], 220); // no
        assert_eq!(parsed_tx[1]["value"], 0); // ok 
        assert_eq!(parsed_tx[6]["value"], 152); // ok
        assert_eq!(parsed_tx[10]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8"); //ok
        assert_eq!(parsed_tx[15]["layout"].len(), 2);
        assert_eq!(parsed_tx[15]["layout"][0][0]["value"], 3029154504617047234u64);
        assert_eq!(parsed_tx[15]["layout"][0][1]["value"], 1u64);
        assert_eq!(parsed_tx[16]["layout"].len(), 28);
        assert_eq!(parsed_tx[16]["layout"][0]["value"], "00");
        assert_eq!(parsed_tx[16]["layout"][1]["value"], "48"); //todo parseのtestを全て作成
    }
    #[test]
    fn test_parse_transaction_2() {
        let network = get_network_info();
        let deadline = get_deadline(&network);
        let tx = object! {
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
            deadline:deadline,
            transactions:[tx],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);

        assert_eq!(parsed_tx[8]["value"], 1000000u64);
        assert_eq!(parsed_tx[9]["value"], 7200000u64);
        assert_eq!(parsed_tx[13]["layout"][0][0]["value"], 140);
        assert_eq!(parsed_tx[13]["layout"][0][1]["value"], 0);
        assert_eq!(parsed_tx[13]["layout"][0][5]["value"], 152);
        assert_eq!(parsed_tx[13]["layout"][0][7]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
        assert_eq!(parsed_tx[13]["layout"][0][9]["value"], 2);
        assert_eq!(parsed_tx[13]["layout"][0][12]["layout"][0][0]["value"], 3029154504617047234u64);
        assert_eq!(parsed_tx[13]["layout"][0][12]["layout"][0][1]["value"], 1u64);
        assert_eq!(parsed_tx[13]["layout"][0][13]["layout"].len(), 28);
        assert_eq!(parsed_tx[13]["layout"][0][13]["layout"][0]["value"], "00");
        assert_eq!(parsed_tx[13]["layout"][0][13]["layout"][1]["value"], "48");
    }
    #[test]
    fn test_parse_transaction_3() {
        let network = get_network_info();
        let tx = object! {
            type:"TRANSFER",
            recipient_address:"85738c26eb1534a4000000000000000000000000000000",
            mosaics:[
                {mosaic_id: 18038182949802959921u64, amount: 1u64},
                {mosaic_id: 16666583871264174062u64, amount: 100u64},
            ],
        };

        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        let prepared_tx = prepare_transaction(&tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);

        assert_eq!(parsed_tx[10]["value"], "9985738c26eb1534a4000000000000000000000000000000");
		assert_eq!(parsed_tx[15]["layout"][0][0]["value"], 16666583871264174062u64);

        let cosignature = object! {
            version:0u64,
            signer_public_key:"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
            signature:"",
        };
        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            transactions:[tx],
            cosignatures:[cosignature]
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);

        assert_eq!(parsed_tx[13]["layout"][0][7]["value"], "9985738c26eb1534a4000000000000000000000000000000");
        assert_eq!(parsed_tx[13]["layout"][0][12]["layout"][0][0]["value"], 16666583871264174062u64);
        assert_eq!(parsed_tx[14]["layout"][0][1]["value"], "6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC");
    }
    #[test]
    fn test_parse_transaction_4() {
        let network = get_network_info();
        let tx = object! {
            type:"MULTISIG_ACCOUNT_MODIFICATION",
            address_additions:[
                "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
                "9869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
            ],
        };
        
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        let prepared_tx = prepare_transaction(&tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);

        assert_eq!(parsed_tx[12]["value"], 2);
        assert_eq!(parsed_tx[15]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
        

        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            transactions:[tx],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        
        assert_eq!(parsed_tx[13]["layout"][0][9]["value"], 2);
        assert_eq!(parsed_tx[13]["layout"][0][12]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
    }


    #[test]
    fn test_parse_transaction_5() {
        let network = get_network_info();
        let tx = object! {
            type:"ACCOUNT_ADDRESS_RESTRICTION",
            restriction_flags:"ADDRESS BLOCK OUTGOING",
            restriction_additions:["989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8","98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"],
            restriction_deletions:[]
        };
                    
        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        let prepared_tx = prepare_transaction(&tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);


        assert_eq!(parsed_tx[10]["value"], 49153);
        assert_eq!(parsed_tx[14]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
        

        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            transactions:[tx],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        
    
        assert_eq!(parsed_tx[13]["layout"][0][7]["value"], 49153);
        assert_eq!(parsed_tx[13]["layout"][0][8]["value"], 2);
        assert_eq!(parsed_tx[13]["layout"][0][11]["layout"][0]["value"], "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8");
    }

    #[test]
    fn test_build_transaction_1() {
        let network = get_network_info();
        let tx = object! {
            type:"TRANSFER",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
            mosaics:[
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
            ],
            message:"Hello Tsunagi(Catjson) SDK!",
        };
        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            fee:1000000u64,
            deadline:get_deadline(&network),
            transactions:[tx],
        };
        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx);

        assert_eq!(built_tx[11]["value"], 144);
        assert_eq!(built_tx[10]["value"], "a5b60f432c88daaf89d3154c5f1e6f7be3090c1af95ba0f21c308ecf119b2222");
    }

    #[test]
    fn test_build_transaction_2() {
        let network = get_network_info();
        //Alice->Bob
        let tx1 = object! {
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
        let tx2 = object! {
            type:"TRANSFER",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            recipient_address:"98f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82",
            mosaics:[
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
            ],
            message:"Hello Carol! This is Alice.",
        };

        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            fee:1000000u64,
            deadline:get_deadline(&network),
            transactions:[tx1,tx2],
        };
        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx);

        assert_eq!(built_tx[11]["value"], 288);
        assert_eq!(built_tx[10]["value"], "00de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe31");
    }

    #[test]
    fn test_build_transaction_3() {
        let network = get_network_info();
        //Alice->Bob
        let tx1 = object! {
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
        let tx2 = object! {
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
        let tx3 = object! {
            type:"TRANSFER",
            signer_public_key:"886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
            recipient_address:"9869762418c5b643eee70e6f20d4d555d5997087d7a686a9",
            mosaics:[
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
            ],
            message:"Hello Alice, This is Carol.",
        };

        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            fee:1000000u64,
            deadline:get_deadline(&network),
            transactions:[tx1,tx2,tx3],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx);

        assert_eq!(built_tx[11]["value"], 432);
		assert_eq!(built_tx[10]["value"], "ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1");
    }

    #[test]
    fn test_verifiable_data_1() {
        let network = get_network_info();
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
            deadline:get_deadline(&network),
            transactions:[tx1,tx2,tx3],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx);
        let verifiable_data = get_verifiable_data(&built_tx);

        assert_eq!(verifiable_data[0]["name"], "version");
        assert_eq!(verifiable_data[5]["value"], "ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1");
    }

    #[test]
    fn test_verifiable_data_2() {
        let network = get_network_info();
    let tx = object!{
            type:"TRANSFER",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            fee:1000000u64,
            deadline:get_deadline(&network),
            recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
            mosaics:[
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
            ],
            message:"Hello Tsunagi(Catjson) SDK!",
        };

        let catjson = load_catjson(&tx, &network);
        let layout = load_layout(&tx, &catjson, false);
        let prepared_tx = prepare_transaction(&tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx);
        let verifiable_data = get_verifiable_data(&built_tx);

        assert_eq!(verifiable_data[0]["name"], "version");
        assert_eq!(verifiable_data[11]["name"], "message");
    }

    // #[test]
    // fn test_() {
    //     let network = get_network_info();

    //     let catjson = load_catjson(&agg_tx, &network);
    //     let layout = load_layout(&agg_tx, &catjson, false);
    //     let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
    //     let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
    //     let built_tx = build_transaction(&parsed_tx);


    // }

    #[test]
    fn test_hexlify_transaction_1(){
        let network = get_network_info();
        let tx = object! {
            type:"TRANSFER",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
            mosaics:[
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
            ],
            message:"Hello Tsunagi(Catjson) SDK!",
        };

        let agg_tx = object! {
            type:"AGGREGATE_COMPLETE",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            fee:1000000u64,
            deadline:get_deadline(&network),
            transactions:[tx],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx); //TX構築
        assert_eq!(hexlify_transaction(&get_verifiable_data(&built_tx).into(), 0), "0198414140420f000000000000dd6d0000000000a5b60f432c88daaf89d3154c5f1e6f7be3090c1af95ba0f21c308ecf119b2222")

    }

    #[test]
    fn test_hexlify_transaction_2(){
        let network = get_network_info();
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
            deadline:get_deadline(&network),
            transactions:[tx1,tx2],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);
        let built_tx = build_transaction(&parsed_tx); //TX構築
        assert_eq!(hexlify_transaction(&get_verifiable_data(&built_tx).into(), 0), "0198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe31");

    }

    #[test]
    fn test_count_size_1() {
        let network = get_network_info();
        let tx = object! {
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
            deadline:get_deadline(&network),
            transactions:[tx],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);

        assert_eq!(count_size(&(parsed_tx.into()), 0), 312);
    }

    #[test]
    fn test_count_size_2() {
        let network = get_network_info();
        let tx1 = object! {
            type:"TRANSFER",
            signer_public_key:"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
            recipient_address:"989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8",
            mosaics:[
                {mosaic_id: 0x2A09B7F9097934C2u64, amount: 1u64},
                {mosaic_id: 0x3A8416DB2D53B6C8u64, amount: 100u64},
            ],
            message:"Hello Tsunagi(Catjson) SDK!",
        };

        let tx2 = object! {
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
            deadline:get_deadline(&network),
            transactions:[tx1,tx2],
        };

        let catjson = load_catjson(&agg_tx, &network);
        let layout = load_layout(&agg_tx, &catjson, false);
        let prepared_tx = prepare_transaction(&agg_tx, &layout, &network);
        let parsed_tx = parse_transaction(&prepared_tx, &layout, &catjson, &network);

        let built_tx    = build_transaction(&parsed_tx); //TX構築
        assert_eq!(count_size(&built_tx.iter().find(|&lf| lf["name"] == "transactions").unwrap(), 0), 288);
    }

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
}