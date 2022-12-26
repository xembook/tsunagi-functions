# 'test/unit'とテストするメソッド部品をrequire 
require 'test/unit'
require_relative 'tsunagi-functions-0.1.rb'
require "base32"



# Test::Unit::TestCaseを継承したclassを作成。名前がtest_で始まるメソッドを作成。
class TestTsunagi < Test::Unit::TestCase

	def startup
	end

	def get_network()

		network = {
			"version" => 1,
			"network" => "TESTNET",
			"generationHash" => "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
			"epochAdjustment" => 1637848847,
			"catjasonBase" => "https://xembook.github.io/tsunagi-functions/catjson/",
		};
		return network

	end

	def get_deadline(network)
#		now = Time.now.to_i;
		now = network["epochAdjustment"];
		return ((now  + 7200) - network["epochAdjustment"]) * 1000;

	end

	def get_payload(tx,network)

		catjson = load_catjson(tx,network)
		layout = load_layout(tx,catjson,false)

		prepared_tx = prepare_transaction(tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
		built_tx = build_transaction(parsed_tx)

		private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"
		signature = sign_transaction(built_tx,private_key,network);

		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(tx["signer_public_key"],signature,built_tx,network);
		payload = hexlify_transaction(built_tx)

		return payload
	end 

	def test_transfer

		network = get_network()
		#resolves 2 mosaic transfer
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		expect = "dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)


		#//resolves 2 mosaic transfer by namespace
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => convert_address_alias_id(generate_namespace_id("xembook")),
			"mosaics" => [
				{"mosaic_id" =>  generate_namespace_id("tomato",generate_namespace_id("xembook")) , "amount" => 1},
				{"mosaic_id" =>  generate_namespace_id("xym",generate_namespace_id("symbol")) , "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}


		expect = "dc00000000000000a1bcb56de796c45cd982e79748772cd9a616a084c95fc775a1d003b9f5f2dcbffa95e869e8a2d77873bbe3d26d5c2764e8299bded689037e4ede6095008cc2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d00000000009985738c26eb1534a40000000000000000000000000000001c00020000000000eeaff441ba994be764000000000000003164838cd27f54fa01000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 mosaic transfer by opossite namespace
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => convert_address_alias_id(generate_namespace_id("xembook")),
			"mosaics" => [
				{"mosaic_id" =>  generate_namespace_id("xym",generate_namespace_id("symbol")) , "amount" => 100},
				{"mosaic_id" =>  generate_namespace_id("tomato",generate_namespace_id("xembook")) , "amount" => 1},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}


		expect = "dc00000000000000a1bcb56de796c45cd982e79748772cd9a616a084c95fc775a1d003b9f5f2dcbffa95e869e8a2d77873bbe3d26d5c2764e8299bded689037e4ede6095008cc2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d00000000009985738c26eb1534a40000000000000000000000000000001c00020000000000eeaff441ba994be764000000000000003164838cd27f54fa01000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)

		#//resolves opposite mosaice order
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		expect = "dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)

		#//resolves null message
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "",
		}

		expect = "c100000000000000c086746240315084735ebee633ff541056c5ba0f17c4d924a4b59c9531aa72243eaa7b76e5e0a9e32a15fb475be49a2f1ff1e380c763bcb2ab3ef5d83125b40d5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80100020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a640000000000000000"
		assert_equal expect,get_payload(tx1,network)

		#//resolves undefined message
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
		}

		expect = "c000000000000000fee4646022be8647455bc876a8f7f303233d297a5755cd1eb41999ae6c8cca2f0225e2b93c4aa793c68657c230578dc3af26c3ef32acae96ea1ae10c438278055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a6400000000000000"
		assert_equal expect,get_payload(tx1,network)

		#//resolves null mosaic transfer
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		expect = "bc00000000000000cd5b93e94f053a07a5a132d7f59708b6818d88840c150d6f6dc38a2ca2408fff0e7e3ee39599d1242a0e4a5869dec8a2847b05fb698fa39db2bf1c3bf46ce2005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)

		#//resolves undefined message and null mosaic
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [],
		}

		expect = "a0000000000000002c271a17d41832515a9ad0e995a524a4859a001436a990370c4b53eaa63677b4d69edde0831171a10defc157ea01f1d5528a562c423e38c725fc5b37af35ee055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000000000000000"
		assert_equal expect,get_payload(tx1,network)

		#//resolves null message and null mosaic
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [],
			"message" => "",
		}

		expect = "a100000000000000786d46993afe584dd4e1fd2904d8eb0ea67e27ca3c7ef81fd208a6f27c1450807234093f9be03bbda0b02d96a69bd2766595ac4ab59fbc5119d247181b5596065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8010000000000000000"
		assert_equal expect,get_payload(tx1,network)

	end

	def test_aggregate_complete
		network = get_network()

		#//resolves 3 account transfer

		#//Alice->Bob
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		#//Bob->Caroll
		tx2 = {
			"type" => "TRANSFER",
			"signer_public_key" => "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"recipient_address" => generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Carol! This is Bob.",
		}

		#//Caroll->Alice
		tx3 = {
			"type" => "TRANSFER",
			"signer_public_key" => "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Alice, This is Carol.",
		}

		cosignature1 = {
			"version" => 0,
			"signer_public_key" =>"6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"signature"=>"",
		};

		cosignature2 = {
			"version" => 0,
			"signer_public_key" => "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
			"signature" => "",
		};

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2,tx3],
			"cosignatures" => [cosignature1,cosignature2]

		};

		catjson = load_catjson(agg_tx,network)
		layout = load_layout(agg_tx,catjson,false)
		prepared_tx = prepare_transaction(agg_tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
		built_tx = build_transaction(parsed_tx)

		private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"
		signature = sign_transaction(built_tx,private_key,network);
		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(agg_tx["signer_public_key"],signature,built_tx,network);

		#//連署
		bob_private_key = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";
		carol_private_key = "1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf";

		prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(tx_hash,bob_private_key);
		prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(tx_hash,carol_private_key);

		cosignatures_layout = layout.find{|lf| lf["name"] == "cosignatures"}
		parsed_cosignatures = parse_transaction(prepared_tx,[cosignatures_layout],catjson,network) #//構築
		built_tx = update_transaction(built_tx,"cosignatures","layout",parsed_cosignatures[0]["layout"])

		payload = hexlify_transaction(built_tx)

		expect = "28030000000000006f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afece4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a7080000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e4794f0617eae1f3f862c286c3e75494f0bb8009f8a8bccf8acb3ceb7719234f0282cdddab7bbc6adb8041788a8642729ec53ea8f6e107e8e2615ae592d44a60c"
		assert_equal expect,payload

		#//resolves opposite cosignature

		#//Alice->Bob
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		#//Bob->Caroll
		tx2 = {
			"type" => "TRANSFER",
			"signer_public_key" => "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"recipient_address" => generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Carol! This is Bob.",
		}

		#//Caroll->Alice
		tx3 = {
			"type" => "TRANSFER",
			"signer_public_key" => "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Alice, This is Carol.",
		}

		cosignature1 = {
			"version" => 0,
			"signer_public_key" =>"6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"signature"=>"",
		};

		cosignature2 = {
			"version" => 0,
			"signer_public_key" => "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
			"signature" => "",
		};

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2,tx3],
			"cosignatures" => [cosignature2,cosignature1]

		};

		catjson = load_catjson(agg_tx,network)
		layout = load_layout(agg_tx,catjson,false)
		prepared_tx = prepare_transaction(agg_tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
		built_tx = build_transaction(parsed_tx)

		private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"
		signature = sign_transaction(built_tx,private_key,network);
		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(agg_tx["signer_public_key"],signature,built_tx,network);

		#//連署
		bob_private_key = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";
		carol_private_key = "1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf";

		prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(tx_hash,bob_private_key);
		prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(tx_hash,carol_private_key);

		cosignatures_layout = layout.find{|lf| lf["name"] == "cosignatures"}
		parsed_cosignatures = parse_transaction(prepared_tx,[cosignatures_layout],catjson,network) #//構築
		built_tx = update_transaction(built_tx,"cosignatures","layout",parsed_cosignatures[0]["layout"])

		payload = hexlify_transaction(built_tx)

		expect = "28030000000000006f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e000000000000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e4794f0617eae1f3f862c286c3e75494f0bb8009f8a8bccf8acb3ceb7719234f0282cdddab7bbc6adb8041788a8642729ec53ea8f6e107e8e2615ae592d44a60c00000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afece4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a708"
		assert_equal expect,payload

		#//resolves no cosignature
	
		#//Alice->Bob
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		tx2 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Carol! This is Alice.",
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2],
			"cosignatures" => []

		};

		expect = "c80100000000000083de0648e05d23036b302e5249554f6fc164917021d4cf07f1d19dfefaea34bfb8679fde237115d5ac3885ef4d4d76c16d4a930429970edbc1fb32a967d0d5025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe3120010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000"
		assert_equal expect,get_payload(agg_tx,network)


		#//resolves undefined cosignature
	
		#//Alice->Bob
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		tx2 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Carol! This is Alice.",
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2],

		};

		expect = "c80100000000000083de0648e05d23036b302e5249554f6fc164917021d4cf07f1d19dfefaea34bfb8679fde237115d5ac3885ef4d4d76c16d4a930429970edbc1fb32a967d0d5025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe3120010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000"
		assert_equal expect,get_payload(agg_tx,network)

	end

	def test_bonded_transaction
		network = get_network()
		#//resolves hash lock
		tx1 = {
			"type" => "HASH_LOCK",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"mosaic" => [{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 10000000}],
			"duration" =>  480,
			"hash" => "a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78"
		}

		expect = "b8000000000000008f0e4dc6dc42be7428219f820718d723803b0dde5455adec3f8ed1871318656ccd7fb4aab539ff722384b0cccf2d66603d5458a12ea01e12ffdd7bbbca9c5a0a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841a86100000000000000dd6d0000000000c8b6532ddb16843a8096980000000000e001000000000000a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78"
		assert_equal expect,get_payload(tx1,network)


		#//resolves hash lock by aggregate
		tx1 = {
			"type" => "HASH_LOCK",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"mosaic" => [{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 10000000}],
			"duration" =>  480,
			"hash" => "4ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c"
		}


		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		}

		expect = "1001000000000000c2941402f941376e3b58c9931c45cd768334fe6ac65e9b746fe484e8ec8067795f5ba4b895ff582395a5d74e8f79a861c6239495b3b38e6215d9d9eef699ac055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000064b515ba0d874c0e0db27687514491d0bb74969cb82b767dde37a0b330a9f3ee680000000000000068000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841c8b6532ddb16843a8096980000000000e0010000000000004ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c"
		assert_equal expect,get_payload(agg_tx,network)

		#//resolves 3 account transfer

		#//Alice->Bob
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		#//Bob->Caroll
		tx2 = {
			"type" => "TRANSFER",
			"signer_public_key" => "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"recipient_address" => generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Carol! This is Bob.",
		}


		#//Caroll->Alice
		tx3 = {
			"type" => "TRANSFER",
			"signer_public_key" => "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Alice, This is Carol.",
		}


		agg_tx = {
			"type" => 'AGGREGATE_BONDED',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2,tx3],

		};

		expect = "5802000000000000ffd1ebcc029c4997d904586292aa1aab8c87e992cd736c074d639419aeae7adc82ce4782f1276f2504b0c4548777dd48616754205c7741af5b2a248f89b4c4035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414240420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e00000000"
		assert_equal expect,get_payload(agg_tx,network)

		#//resolves 3 account transfer partial complete

		#//Alice->Bob
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		}

		#//Bob->Caroll
		tx2 = {
			"type" => "TRANSFER",
			"signer_public_key" => "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"recipient_address" => generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Carol! This is Bob.",
		}

		#//Caroll->Alice
		tx3 = {
			"type" => "TRANSFER",
			"signer_public_key" => "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"mosaics" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
			],
			"message" => "Hello Alice, This is Carol.",
		}

		cosignature1 = {
			"version" => 0,
			"signer_public_key" =>"6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"signature"=>"",
		};

		agg_tx = {
			"type" => 'AGGREGATE_BONDED',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2,tx3],
			"cosignatures" => [cosignature1]

		};

		catjson = load_catjson(agg_tx,network)
		layout = load_layout(agg_tx,catjson,false)
		prepared_tx = prepare_transaction(agg_tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
		built_tx = build_transaction(parsed_tx)

		private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"
		signature = sign_transaction(built_tx,private_key,network);
		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(agg_tx["signer_public_key"],signature,built_tx,network);

		#//連署
		bob_private_key = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";
		prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(tx_hash,bob_private_key);

		cosignatures_layout = layout.find{|lf| lf["name"] == "cosignatures"}
		parsed_cosignatures = parse_transaction(prepared_tx,[cosignatures_layout],catjson,network) #//構築
		built_tx = update_transaction(built_tx,"cosignatures","layout",parsed_cosignatures[0]["layout"])

		payload = hexlify_transaction(built_tx)

		expect = "c002000000000000ffd1ebcc029c4997d904586292aa1aab8c87e992cd736c074d639419aeae7adc82ce4782f1276f2504b0c4548777dd48616754205c7741af5b2a248f89b4c4035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414240420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecbdde1c296d3e2e82bb1ae878586f832aa59080290b0f095f7e8d73921b6d2f67742e4588795f41b652500fdc1230ce4f45d2099fe37e182ebbe0f86121336e03"
		assert_equal expect,payload
	end

	def test_mosaic
		nonce = 1700836761

		network = get_network()

		#//resolves mosaic definition
		tx1 = {
			"type" => "MOSAIC_DEFINITION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"duration" => 0,
			"id" => generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce),
			"nonce" => nonce,
			"flags" => 'TRANSFERABLE RESTRICTABLE',
			"divisibility" => 2,
		}

		expect = "96000000000000008400ea1dd86f206c946ae4aacfdd2d9997ceb406028e3d3e67e0b20a2a0dae696d9084f7f38f64c56450a3d6cd305722cb37d60b462358bbe23b5d8e155a3f0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d41a86100000000000000dd6d000000000076e65d50e5fbaf4d000000000000000099b560650602"
		assert_equal expect,get_payload(tx1,network)

		#//resolves mosaic supply change
		tx1 = {
			"type" => "MOSAIC_SUPPLY_CHANGE",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"mosaic_id" => generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce),
			"delta" => 1000 * 100,
			"action" => 'INCREASE',
		}

		expect = "9100000000000000cbec54081f0a62d5c5f84748df4668670fad447b53b44062e58d5cab054a2c8bcd8ab13533231825eda6156d4c73ba98978eccb011b0107f9bc9a0f071888e035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d42a86100000000000000dd6d000000000076e65d50e5fbaf4da08601000000000001"
		assert_equal expect,get_payload(tx1,network)

		#//resolves aggregate mosaic definition and supply change

		tx1 = {
			"type" => "MOSAIC_DEFINITION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"duration" => 0,
			"id" => generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce),
			"nonce" => nonce,
			"flags" => 'TRANSFERABLE RESTRICTABLE',
			"divisibility" => 2,
		}

		tx2 = {
			"type" => "MOSAIC_SUPPLY_CHANGE",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"mosaic_id" => generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce),
			"delta" => 1000 * 100,
			"action" => 'INCREASE',
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2],

		};

		expect = "3801000000000000c4b2ea423fd6eaa69407fb261cdb09b3d039923ad15a120ad1f1da61bcfd69db9b71cddc0bff730b3cd1b421b35f8cbc87a4765a204412b5efc34e221d50b20a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000fc4405540b555f4dde5dc4ce67daeaf207e5485d8da24d5cfd6bf71fa064c9a5900000000000000046000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4176e65d50e5fbaf4d000000000000000099b560650602000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4276e65d50e5fbaf4da0860100000000000100000000000000"
		assert_equal expect,get_payload(agg_tx,network)

	end

	def test_namespace
		network = get_network()
	
		#//resolves root namespace regisration
		tx1 = {
			"type" => "NAMESPACE_REGISTRATION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"duration" => 86400,
			"registration_type" => "ROOT",
			"name" => "xembook",
			"id" => generate_namespace_id("xembook"),
		}

		expect = "99000000000000003983d675dd3affcab71fb09ee51cbddd4e8ee587335e030472dd50370333266ad571c4c94410262c7bb2ecc99b2b4b8eab71245046f41518d52ef6d5355792055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d0000000000805101000000000085738c26eb1534a4000778656d626f6f6b"
		assert_equal expect,get_payload(tx1,network)

		#//resolves sub namespace regisration
		tx1 = {
			"type" => "NAMESPACE_REGISTRATION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"parent_id" => generate_namespace_id("xembook"),
			"registration_type" => "CHILD",
			"name" => "tomato",
			"id" => generate_namespace_id("tomato",generate_namespace_id("xembook")),
		}

		expect = "9800000000000000942e0fe89a3471a075f2cbd06cc64d4d8af5cd8e58c437aa39fa05e47bb9230c9a259d9da70279c8656749792310585b138e19889b3b41e7e01c14a4cbea1b025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d000000000085738c26eb1534a43164838cd27f54fa0106746f6d61746f"
		assert_equal expect,get_payload(tx1,network)

		#//resolves address alias
		tx1 = {
			"type" => "ADDRESS_ALIAS",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"namespace_id" => generate_namespace_id("xembook"),
			"address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"alias_action" => "LINK"
		}

		expect = "a1000000000000008f61856b455c0a57db652844aa761281c019511d7b0cd0ae9b54e4b22585f36f012116860ce9f5300fe0e91521be74d8434032bf73e008b2f52d5f0744ef13045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42a86100000000000000dd6d000000000085738c26eb1534a49869762418c5b643eee70e6f20d4d555d5997087d7a686a901"
		assert_equal expect,get_payload(tx1,network)

		#//resolves address alias
		tx1 = {
			"type" => "MOSAIC_ALIAS",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"namespace_id" => generate_namespace_id("tomato",generate_namespace_id("xembook")),
			"mosaic_id" => 0x4DAFFBE5505DE676,
			"alias_action" => "LINK"
		}

		expect = "910000000000000041f45e3bbbc8073c14b6e05b71fa9299692d1eafc86300b59698207ef044c7db8e346870c57df722497803549e2e3f8d5777c5e1c98fdf27a562d814076acc035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43a86100000000000000dd6d00000000003164838cd27f54fa76e65d50e5fbaf4d01"
		assert_equal expect,get_payload(tx1,network)


		#//aggregate
		tx1 = {
			"type" => "NAMESPACE_REGISTRATION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"duration" => 86400,
			"registration_type" => "ROOT",
			"name" => "xembook1",
			"id" => generate_namespace_id("xembook1"),
		}

		tx2 = {
			"type" => "NAMESPACE_REGISTRATION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"parent_id" => generate_namespace_id("xembook1"),
			"registration_type" => "CHILD",
			"name" => "tomato1",
			"id" => generate_namespace_id("tomato1",generate_namespace_id("xembook1")),
		}

		tx3 = {
			"type" => "ADDRESS_ALIAS",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"namespace_id" => generate_namespace_id("xembook1"),
			"address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"alias_action" => "LINK"
		}

		tx4 = {
			"type" => "MOSAIC_ALIAS",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"namespace_id" => generate_namespace_id("tomato1",generate_namespace_id("xembook1")),
			"mosaic_id" => 0x4DAFFBE5505DE676,
			"alias_action" => "LINK"
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2,tx3,tx4],

		};

		expect = "e801000000000000989f5f8a026d6e5c45301ce06af70406bd9c3694604a9e0718c3bac4dff9b95494d397210817139ebf43306a7bb43242e200afc4205b9a3cb439ffb1e2a14c015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000de415866cedcab9dda7baa97b5bb326ad2647bfafe69d8b3587a789bff9d073c40010000000000004a000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e418051010000000000bd1cf9801594b9ed000878656d626f6f6b3100000000000049000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41bd1cf9801594b9edf47e2f57b78ec1920107746f6d61746f310000000000000051000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42bd1cf9801594b9ed9869762418c5b643eee70e6f20d4d555d5997087d7a686a9010000000000000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43f47e2f57b78ec19276e65d50e5fbaf4d0100000000000000"
		assert_equal expect,get_payload(agg_tx,network)

	end

	def test_metadata
		network = get_network()


		#//resolves mosaic metadata
		tx1 = {
			"type" => "MOSAIC_METADATA",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"target_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"target_mosaic_id" => 0x4DAFFBE5505DE676,
			"scoped_metadata_key" => generate_key("key_mosaic"),
			"value_size_delta" => 27,
			"value" => "Hello Tsunagi(Catjson) SDK!",
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "2001000000000000b6c125c94aed659dc1346151d03e72552dfe57c6882baa924310f3cddea39c2387bf238aba046b6a7a2f849802b38bb5f046de6b455e60a2fbaa5f22eb4d53005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000eadc97a286bf8081b523c4d246cf6ca05f208835b82e1f97ad978a2d638386a2780000000000000077000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844429869762418c5b643eee70e6f20d4d555d5997087d7a686a9e222a46a117e21cf76e65d50e5fbaf4d1b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
		assert_equal expect,get_payload(agg_tx,network)

		#//resolves mosaic metadata without aggregate
		tx1 = {
			"type" => "MOSAIC_METADATA",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"target_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"target_mosaic_id" => 0x4DAFFBE5505DE676,
			"scoped_metadata_key" => generate_key("key_mosaic"),
			"value_size_delta" => 27,
			"value" => "Hello Tsunagi(Catjson) SDK!",
		}

		expect = "c700000000000000dc5ff39f1dc61eb4500f2d19dec8bc36f03dffcb65504e906e6f6e6eecb71022b9e88993f4f55de53ba969563ad48be7fb718aaf1021617a1fa6324814f9b0045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444240420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9e222a46a117e21cf76e65d50e5fbaf4d1b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)

		#//resolves namespace metadata

		tx1 = {
			"type" => "NAMESPACE_METADATA",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"target_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"target_namespace_id" => generate_namespace_id("xembook"),
			"scoped_metadata_key" => generate_key("key_namespace"),
			"value_size_delta" => 27,
			"value" => "Hello Tsunagi(Catjson) SDK!",
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "2001000000000000b80a6851320a00db3e505516813106f3190fbbd349266667373faf85cdd370cec50c3224c8110c62147a3bffb7be8b1bc09aec7d7701bc15299b45c18ddd70015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000accdb81d64d2626a79d546a2380171879f39beecc9b314805ca9b5a0d2b547e4780000000000000077000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019844439869762418c5b643eee70e6f20d4d555d5997087d7a686a9d9d07308378a6a8b85738c26eb1534a41b001b0048656c6c6f205473756e616769284361746a736f6e292053444b2100"
		assert_equal expect,get_payload(agg_tx,network)

		tx1 = {
			"type" => "NAMESPACE_METADATA",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"target_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"target_namespace_id" => generate_namespace_id("xembook"),
			"scoped_metadata_key" => generate_key("key_namespace"),
			"value_size_delta" => 27,
			"value" => "Hello Tsunagi(Catjson) SDK!",
		}

		#//resolves namespace metadata without aggregate
		expect = "c700000000000000093141b56044db8b665d8c86f3119988f505c8ac3ad557f22383b2d8b0e2177fd934059df8e7890632b0ec020a087bf0569f8a992201a9e9a69870aa14351e055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198444340420f000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9d9d07308378a6a8b85738c26eb1534a41b001b0048656c6c6f205473756e616769284361746a736f6e292053444b21"
		assert_equal expect,get_payload(tx1,network)


	end

	def test_multisig

		network = get_network()

		#//resolves multisig account modification address_additions

		tx1 = {
			"type" => "MULTISIG_ACCOUNT_MODIFICATION",
			"signer_public_key" => "66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77",
			"min_removal_delta" => 1,
			"min_approval_delta" => 1,
			"address_additions" => [
				generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			],
			"address_deletions" => [],
		}

		cosignature1 = {
			"version" => 0,
			"signer_public_key" =>"5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"signature"=>"",
		};

		cosignature2 = {
			"version" => 0,
			"signer_public_key" => "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"signature" => "",
		};

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],
			"cosignatures" => [cosignature1,cosignature2]

		};

		catjson = load_catjson(agg_tx,network)
		layout = load_layout(agg_tx,catjson,false)
		prepared_tx = prepare_transaction(agg_tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
		built_tx = build_transaction(parsed_tx)

		private_key = "22F0BA129FE0C66BA596D7127B85961BF8EEF32784364338BACB4E88D6F284D6"
		signature = sign_transaction(built_tx,private_key,network);
		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(agg_tx["signer_public_key"],signature,built_tx,network);

		#//連署
		bob_private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7";
		carol_private_key = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";

		prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(tx_hash,bob_private_key);
		prepared_tx["cosignatures"][1]["signature"] = cosign_transaction(tx_hash,carol_private_key);

		cosignatures_layout = layout.find{|lf| lf["name"] == "cosignatures"}
		parsed_cosignatures = parse_transaction(prepared_tx,[cosignatures_layout],catjson,network) #//構築
		built_tx = update_transaction(built_tx,"cosignatures","layout",parsed_cosignatures[0]["layout"])

		payload = hexlify_transaction(built_tx)

		expect = "e001000000000000bf1f73806cedc96540806bd2535be327889448b282e23168ab543037774e202f8d89245c6d4af99f476cf547decd0ae1f77b809ce2e3ec33eb39f3bcaed7970d66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77000000000198414140420f000000000000dd6d0000000000336c1c549f927fd26a4ff9f3602423cb544e766d6c2c655e261c80679f185cb56800000000000000680000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee7700000000019855410101020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b89869762418c5b643eee70e6f20d4d555d5997087d7a686a900000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cbfa0f1120bdd09578723c07734b3a6bebe9810b2871d24532f7bcaba613f821a1ae9835b5dc3927ff6f350f3ca5e3490067b76d23274e264ba30b58e0643fea0400000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec959793a935b0c02b860238732cff351bef70d82af806ce56223205ee6d89f76fedb8fb4b5ae0004bced6db5e5fc6283c3d8a78a1a54a4de197dc75bef0b1210a"
		assert_equal expect,payload



		#//resolves multisig account modification change delta
		tx1 = {
			"type" => "MULTISIG_ACCOUNT_MODIFICATION",
			"signer_public_key" => "66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77",
			"min_removal_delta" => 1,
			"min_approval_delta" => 1,
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "e00000000000000000e3f627769675a51f98c7a9745e8540e74f33a2ed63932e7de26f87b98dc94af51b133ab7d6826e1de86c9cbaa160a7aec5da4d9972ce915da8bf0c85a3010f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000be95dbbb0adf29fe5f5a766fbf3c10e4a60e0d71c216d263e2b167e06c70dac93800000000000000380000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee7700000000019855410101000000000000"
		assert_equal expect,get_payload(agg_tx,network)

		#//resolves multisig account modification address_deletions

		tx1 = {
			"type" => "MULTISIG_ACCOUNT_MODIFICATION",
			"signer_public_key" => "66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77",
			"min_removal_delta" => -1,
			"min_approval_delta" => -1,
			"address_additions" => [],
			"address_deletions" => [generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")],
		}

		cosignature2 = {
			"version" => 0,
			"signer_public_key" => "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
			"signature" => "",
		};

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],
			"cosignatures" => [cosignature2]

		};

		catjson = load_catjson(agg_tx,network)
		layout = load_layout(agg_tx,catjson,false)
		prepared_tx = prepare_transaction(agg_tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
		built_tx = build_transaction(parsed_tx)

		private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"
		signature = sign_transaction(built_tx,private_key,network);
		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(agg_tx["signer_public_key"],signature,built_tx,network);

		#//連署
		bob_private_key = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b";

		prepared_tx["cosignatures"][0]["signature"] = cosign_transaction(tx_hash,bob_private_key);

		cosignatures_layout = layout.find{|lf| lf["name"] == "cosignatures"}
		parsed_cosignatures = parse_transaction(prepared_tx,[cosignatures_layout],catjson,network) #//構築
		built_tx = update_transaction(built_tx,"cosignatures","layout",parsed_cosignatures[0]["layout"])

		payload = hexlify_transaction(built_tx)

		expect = "6001000000000000a119633807603dffcfa86a981b4e31d97f6d21a024470139aeee0400ea748695943486743bf3fa3c6edf6207b47afbdfec3b086b8560e9ba3460f43a70391c0e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000317c15bcbe4d9edadca95ed3fbeabe47fe41e749fbc120e9b83abf57083163745000000000000000500000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee770000000001985541ffff000100000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b800000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afeccf75de4350bbfd1a8f4e7fb3c42e7c50725a9403ca5234fb71268c5eb6691734007a5fe0f8cc14a93011e76a6477e0e8ae1d7a0f9607e4deed709d1a3937c309"
		assert_equal expect,payload


		#//resolves multisig account modification address_deletions 2
		tx1 = {
			"type" => "MULTISIG_ACCOUNT_MODIFICATION",
			"signer_public_key" => "66adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee77",
			"min_removal_delta" => -1,
			"min_approval_delta" => -1,
			"address_deletions" => [generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI")],
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "f800000000000000434d6772e7f92dc5daf56ec8310afce152203a8c3b1dc25d87d1fe1d2300f452d28801cb6e1e59f77ad4f73cac28cdd027e17a44040c7bbb8185462359f1ad005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000bdbccdc54cb19c89113a1c58ecfa776ded496a0b55568d7338530208137922fb5000000000000000500000000000000066adb706bc9a93e6e803b2b76a1341a8acd98690ef204b402643ae3d4701ee770000000001985541ffff0001000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
		assert_equal expect,get_payload(agg_tx,network)
	end

	def test_account_restriction

		network = get_network()

		#//account restriction transaction
		tx1 = {
			"type" => "ACCOUNT_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "ADDRESS BLOCK OUTGOING",
			"restriction_additions" => [
				generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")
			],
			"restriction_deletions" => [],
		}

		expect = "b8000000000000005dd7b8579be90231ada1d6f4158ff6ce47f17a7946f0f9872a5a2d451ab5920c389e8237e25560e71e4e4e2dfcb3c8e297642f1ed78975ab206a984feef2de095f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 address restriction_additions by namespace
		tx1 = {
			"type" => "ACCOUNT_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "ADDRESS BLOCK OUTGOING",
			"restriction_additions" => [
				convert_address_alias_id(generate_namespace_id("bob",generate_namespace_id("xembook"))),
				generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")
			],
			"restriction_deletions" => [],
		}

		expect = "b8000000000000005cdc385106ea9a1896d12d2c7c171f1975df38298ddb065eab1fa63dad9f87eb19ab72611d3e28d1841b5dd708f489bc8266f27e66d7d8400e15aff0b8da4e045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0020000000000993a7f6395187cb7c800000000000000000000000000000098f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 address restriction_deletions transfer
		tx1 = {
			"type" => "ACCOUNT_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "ADDRESS BLOCK OUTGOING",
			"restriction_additions" => [],
			"restriction_deletions" => [
				generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")
			],
		}

		expect = "b8000000000000000f2d19602306a1418229fa0ba8a67bbd0cbc777e9aa51a14647528ae5477ce26cc1bb5f4ceaf8d4a36b58955f324d76a520278438125b2df676fa6089f6ac3035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985041a86100000000000000dd6d000000000001c0000200000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e82"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 mosaic restriction_additions transfer
		tx1 = {
			"type" => "ACCOUNT_MOSAIC_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "MOSAIC_ID BLOCK",
			"restriction_additions" => [0x4DAFFBE5505DE676,0x2A09B7F9097934C2],
			"restriction_deletions" => [],
		}

		expect = "9800000000000000e876104b9595db102728e39715c548b5992a3743348fe2a01f53e155ca27d955482c1a2ca05d3a2f1784db510427d14431f112a6bf339b1f709e95f4d441f4015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042a86100000000000000dd6d0000000000028002000000000076e65d50e5fbaf4dc2347909f9b7092a"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 mosaic restriction_deletions transfer
		tx1 = {
			"type" => "ACCOUNT_MOSAIC_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "MOSAIC_ID BLOCK",
			"restriction_additions" => [],
			"restriction_deletions" => [0x4DAFFBE5505DE676,0x2A09B7F9097934C2],
		}

		expect = "980000000000000063ccbdc7a6bd545d6751a1ed5dad87eda2efbef52462d19ba2e9a1f39d36999539b833a4732d12b87e8ea6802002fb575ac8d04ab4398827e12a41f0a425600c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042a86100000000000000dd6d0000000000028000020000000076e65d50e5fbaf4dc2347909f9b7092a"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 operation restriction_additions transfer
		tx1 = {
			"type" => "ACCOUNT_OPERATION_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "TRANSACTION_TYPE BLOCK OUTGOING",
			"restriction_additions" => ["TRANSFER","AGGREGATE_COMPLETE"],
			"restriction_deletions" => [],
		}

		expect = "8c00000000000000d78ef15ed98496bc101c340ef9862fccca5e73aea04b93612ec00c4bd745674c1173827b81449a1e120bb960c176d56339f2b3d8ce7b7cd95d4b438f6e96ad075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985043a86100000000000000dd6d000000000004c002000000000054414141"
		assert_equal expect,get_payload(tx1,network)

		#//resolves 2 operation restriction_deletions transfer
		tx1 = {
			"type" => "ACCOUNT_OPERATION_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"restriction_flags" => "TRANSACTION_TYPE BLOCK OUTGOING",
			"restriction_additions" => [],
			"restriction_deletions" => ["TRANSFER","AGGREGATE_COMPLETE"],
		}

		expect = "8c000000000000000d553c66bdddc3e1a91cd7cf8ee3fe1bd92f4a9c25a876b78ad95876d03856bb8bcb7cff0bc64ed8a672d1d8592068140a040dbd83e5970ebd57e30be3a31a0a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985043a86100000000000000dd6d000000000004c000020000000054414141"
		assert_equal expect,get_payload(tx1,network)



		#//aggregate
		tx1 = {
			"type" => "ACCOUNT_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "ADDRESS BLOCK OUTGOING",
			"restriction_additions" => [
				generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")
			],
			"restriction_deletions" => [],
		}

		tx2 = {
			"type" => "ACCOUNT_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "ADDRESS BLOCK OUTGOING",
			"restriction_additions" => [
				convert_address_alias_id(generate_namespace_id("bob",generate_namespace_id("xembook"))),
				generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")
			],
			"restriction_deletions" => [],
		}

		tx3 = {
			"type" => "ACCOUNT_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "ADDRESS BLOCK OUTGOING",
			"restriction_additions" => [],
			"restriction_deletions" => [
				generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				generate_address_id("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")
			],
		}

		tx4 = {
			"type" => "ACCOUNT_MOSAIC_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "MOSAIC_ID BLOCK",
			"restriction_additions" => [0x4DAFFBE5505DE676,0x2A09B7F9097934C2],
			"restriction_deletions" => [],
		}

		tx5 = {
			"type" => "ACCOUNT_MOSAIC_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "MOSAIC_ID BLOCK",
			"restriction_additions" => [],
			"restriction_deletions" => [0x4DAFFBE5505DE676,0x2A09B7F9097934C2],
		}

		tx6 = {
			"type" => "ACCOUNT_OPERATION_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "TRANSACTION_TYPE BLOCK OUTGOING",
			"restriction_additions" => ["TRANSFER","AGGREGATE_COMPLETE"],
			"restriction_deletions" => [],
		}

		tx7 = {
			"type" => "ACCOUNT_OPERATION_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"restriction_flags" => "TRANSACTION_TYPE BLOCK OUTGOING",
			"restriction_additions" => [],
			"restriction_deletions" => ["TRANSFER","AGGREGATE_COMPLETE"],
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2,tx3,tx4,tx5,tx6,tx7],

		};

		expect = "f002000000000000a6b76ba9637d35045fd9dfdbf63ce100598eccbab238199bd465d2a66236814473ed1b504573a4be3f177f1045aa367aae8f915ad56101d96c0f9809c24818015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000940deda2e41ffc266aa8a19507f9da37b4fd1a0f6766ab9b6f5bf6db10ecb053480200000000000068000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198504101c0020000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e8268000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198504101c0020000000000993a7f6395187cb7c800000000000000000000000000000098f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e8268000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198504101c0000200000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b898f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e8248000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042028002000000000076e65d50e5fbaf4dc2347909f9b7092a48000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985042028000020000000076e65d50e5fbaf4dc2347909f9b7092a3c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198504304c002000000000054414141000000003c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198504304c00002000000005441414100000000"
		assert_equal expect,get_payload(agg_tx,network)
	end

	def test_global_mosaic_restriction

		network = get_network()

		#//resolves global mosaic restriction transfer
		tx1 = {
			"type" => "MOSAIC_GLOBAL_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"mosaic_id" => 0x4DAFFBE5505DE676,
			"reference_mosaic_id" => 0,
			"restriction_key" => generate_key("key_account"),
			"previous_restriction_value" => 0,
			"new_restriction_value" => 0x1,
			"previous_restriction_type" => "NONE",
			"new_restriction_type" => "EQ"
		}

		expect = "aa0000000000000083b18a9467dd39067ef18dc9eb5d7ee69b51fc68c954586d4291e68407ab41feca86224ddafcd9fd2b5375f33e1e4bb4de031b47fa42c742d4adb82fc60caf0e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985141a86100000000000000dd6d000000000076e65d50e5fbaf4d0000000000000000d72781051bb77297000000000000000001000000000000000001"
		assert_equal expect,get_payload(tx1,network)

		#//resolves global mosaic restriction transfer
		tx1 = {
			"type" => "MOSAIC_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"mosaic_id" => 0x4DAFFBE5505DE676,
			"restriction_key" => generate_key("key_account"),
			"previous_restriction_value" => 0xFFFFFFFFFFFFFFFF,
			"new_restriction_value" => 0x1,
			"target_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		}

		expect = "b80000000000000040748328e8dab01fee7f82b4e23b3ed2c6336783790f286aa75eab0982c2a60af9eb5a2c524a427ec5d7e451d47a3f1620e5478ae37d048441fd7f2675e4880e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985142a86100000000000000dd6d000000000076e65d50e5fbaf4dd72781051bb77297ffffffffffffffff01000000000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
		assert_equal expect,get_payload(tx1,network)

		#//aggregate
		tx1 = {
			"type" => "MOSAIC_GLOBAL_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"mosaic_id" => 0x4DAFFBE5505DE676,
			"reference_mosaic_id" => 0,
			"restriction_key" => generate_key("key_account"),
			"previous_restriction_value" => 0,
			"new_restriction_value" => 0x1,
			"previous_restriction_type" => "NONE",
			"new_restriction_type" => "EQ"
		}

		tx2 = {
			"type" => "MOSAIC_ADDRESS_RESTRICTION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"mosaic_id" => 0x4DAFFBE5505DE676,
			"restriction_key" => generate_key("key_account"),
			"previous_restriction_value" => 0xFFFFFFFFFFFFFFFF,
			"new_restriction_value" => 0x1,
			"target_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		}


		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1,tx2],

		};

		expect = "7001000000000000ad419685fe429a729725dff6b0d44be438c5f4e5d4edeea42ab8f78bd93c0fa886674824832d6c1cefec35eedd75eb7a142c270a221aed5c0e7dce2234c4d5005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000075aa5d190ff872320ec8047a029772ee17b2f62f9f8da3be67ba7adc17233cfbc8000000000000005a000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198514176e65d50e5fbaf4d0000000000000000d72781051bb7729700000000000000000100000000000000000100000000000068000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198514276e65d50e5fbaf4dd72781051bb77297ffffffffffffffff01000000000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9"
		assert_equal expect,get_payload(agg_tx,network)


	end

	def test_mosaic_supply_revocation

		network = get_network()

		#//resolves global mosaic restriction transfer
		tx1 = {
			"type" => "MOSAIC_SUPPLY_REVOCATION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"source_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"mosaic" => [
				{"mosaic_id" =>  0x0552BC5EF5BD589D, "amount" => 100},
			],
		}

		expect = "a800000000000000fd67cc1e3962d068da002cc79531e8972575a771cddf8b9317492bc1022dfc80944540bfca1aba09c44aedf56aef5a7c00eb7f569ef7f94a7f0dad46eebcb70a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d43a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a99d58bdf55ebc52056400000000000000"
		assert_equal expect,get_payload(tx1,network)

		#aggregate
		tx1 = {
			"type" => "MOSAIC_SUPPLY_REVOCATION",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"source_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"mosaic" => [
				{"mosaic_id" =>  0x0552BC5EF5BD589D, "amount" => 100},
			],
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "0001000000000000bd814e80d7e36dd383348f45a386a63a410453e7530d4bd2c22d38d9597227d6899dd36e48fb6c5bf770a519d85fd8a2783592443a5edade551813cc42e682085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000089aea3333dc6fb52d00c74db39ea3b9c2e34932d5198ad52dacc9459249c0a78580000000000000058000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d439869762418c5b643eee70e6f20d4d555d5997087d7a686a99d58bdf55ebc52056400000000000000"
		assert_equal expect,get_payload(agg_tx,network)


	end

	def test_secret
		network = get_network()

		#//resolves secret lock
		tx1 = {
			"type" => "SECRET_LOCK",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"secret" => "f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240",
			"mosaic" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 10000000},
			],
			"duration" => 480,
			"hash_algorithm" => "SHA3_256",
		}

		expect = "d1000000000000000117860215bbc73d6ab56fa39f5ae1495ff55ad76104c3371701de042d6a0865bfb551ece7549abf636d60d443d690beee087f9417290b65da230abf280039085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985241a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240c8b6532ddb16843a8096980000000000e00100000000000000"
		assert_equal expect,get_payload(tx1,network)

		#//resolves secret proof
		tx1 = {
			"type" => "SECRET_PROOF",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"secret" => "f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e240",
			"hash_algorithm" => "SHA3_256",
			"proof" => "7944496ac0f572173c2549baf9ac18f893aab6d0",
		}

		expect = "cf000000000000008a17b7e88005e436580b8b500bf01da70fb22906065590412c458f31094a11c4fee2b08cc1025f40642f96285ffa54bb1a88c4cf373f11b6c240ce146b41a4055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985242a86100000000000000dd6d00000000009869762418c5b643eee70e6f20d4d555d5997087d7a686a9f260bfb53478f163ee61ee3e5fb7cfcaf7f0b663bc9dd4c537b958d4ce00e2401400007944496ac0f572173c2549baf9ac18f893aab6d0"
		assert_equal expect,get_payload(tx1,network)

		#//resolves secret lock with aggregate
		tx1 = {
			"type" => "SECRET_LOCK",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"secret" => "0debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c",
			"mosaic" => [
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 10000000},
			],
			"duration" => 480,
			"hash_algorithm" => "SHA3_256",
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "3001000000000000185b61702bbf8298e2c117f0b262f644f59269ceaf7dbce9851543a62a01f726a08a3409da1d6ecd9ed30d6aa56289c3cfeba7377e66dbc36751493cdc28920e5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000944643651ac8d446192f0dcbe1c370610552ddd0be94a9ba77ce7063e693cb29880000000000000081000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019852419869762418c5b643eee70e6f20d4d555d5997087d7a686a90debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68cc8b6532ddb16843a8096980000000000e0010000000000000000000000000000"
		assert_equal expect,get_payload(agg_tx,network)

		#//resolves secret proof with aggregate
		tx1 = {
			"type" => "SECRET_PROOF",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
			"secret" => "0debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c",
			"hash_algorithm" => "SHA3_256",
			"proof" => "d91a8258175a6213225bd4ec240f1971c8742dca",
		}

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};

		expect = "2801000000000000b26c51b84114750e4005ab6002c5d6646f6de025bdf1fadbe429953044be5e61c1ecf6adaaf4cba51f0748a1cb17f82c9adf091eb7c05f4f5b5b226ead64f9065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000bd584e6eb97627993d2157bc630a4c95ec783e201678539ce671e3d36367372c80000000000000007f000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb00000000019852429869762418c5b643eee70e6f20d4d555d5997087d7a686a90debb816347a49c5121b07c1ec2e9f7443eef6451d260cc646f32520237fa68c140000d91a8258175a6213225bd4ec240f1971c8742dca00"
		assert_equal expect,get_payload(agg_tx,network)

		#//resolves secret lock by namespace
		tx1 = {
			"type" => "SECRET_LOCK",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => convert_address_alias_id(generate_namespace_id("xembook")),
			"secret" => "760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00",
			"mosaic" => [
				{"mosaic_id" =>  generate_namespace_id("xym",generate_namespace_id("symbol")) , "amount" => 10000000},
			],
			"duration" => 480,
			"hash_algorithm" => "SHA3_256",
		}

		expect = "d100000000000000936ffff90a654017eb900af35ea4f5a687b38b111190e2c1f9992e542c5be0bb25f22c14440b67442149de0cf5c7ea0e9642f19badcbc8b6aec8bda707c4a2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985241a86100000000000000dd6d00000000009985738c26eb1534a4000000000000000000000000000000760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00eeaff441ba994be78096980000000000e00100000000000000"
		assert_equal expect,get_payload(tx1,network)

		#//resolves secret proof by namespace
		tx1 = {
			"type" => "SECRET_PROOF",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => get_deadline(network),
			"recipient_address" => convert_address_alias_id(generate_namespace_id("xembook")),
			"secret" => "760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00",
			"hash_algorithm" => "SHA3_256",
			"proof" => "336b7e682903606a2fef4c91d83c4af7da3e7486",
		}

		expect = "cf0000000000000043d7a84b4c20435ffdd50644a2a0eaaed667326975d8af93015013899f5b4741f92d66ea7b2d23ad57fb5cd8d71344ffe34dd9654dbc58d9a447aaab70814d0b5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985242a86100000000000000dd6d00000000009985738c26eb1534a4000000000000000000000000000000760b4407e82970bc86f5a3063b445c0cfec35c6720cbf1f8b5ca643d51bb5a00140000336b7e682903606a2fef4c91d83c4af7da3e7486"
		assert_equal expect,get_payload(tx1,network)

	end

=begin




=end


	def test_function

		expect = 11832106220717372293
		assert_equal expect,generate_namespace_id("xembook")

		expect = "a43415eb268c7385"
		assert_equal expect,generate_namespace_id("xembook").to_s(16)

		expect = "fa547fd28c836431"
		assert_equal expect,generate_namespace_id("tomato",generate_namespace_id("xembook")).to_s(16)

		expect = "9772b71b058127d7"
		assert_equal expect,generate_key("key_account").to_s(16)

		expect = "4daffbe5505de676"
		assert_equal expect,generate_mosaic_id(generate_address_id("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),1700836761).to_s(16)

		expect = "85738c26eb1534a4000000000000000000000000000000"
		assert_equal expect, convert_address_alias_id(generate_namespace_id("xembook"))


		expect = "989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8"
		assert_equal expect, generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")
		
		expect = "1ff973caeb3dc0"
		assert_equal expect, 8999999999000000.to_s(16)


	end


end
