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
			"catjasonBase" => "https://xembook.github.io/tsunagi-sdk/catjson/",
		};
		return network

	end

	def get_deadline(network)
#		now = Time.now.to_i;
		now = network["epochAdjustment"];
		return ((now  + 7200) - network["epochAdjustment"]) * 1000;

	end


  def test_tsunagi




=begin
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => deadline_time,
			"recipient_address" => Base32.decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA").unpack('H*')[0],
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		};

		catjson = load_catjson(tx1,network)
#		puts catjson
		layout = load_layout(tx1,catjson,false)
#		puts layout
		prepared_tx = prepare_transaction(tx1,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
#		puts parsed_tx
		built_tx = build_transaction(parsed_tx)
#    assert_equal 0, load_catjson(0,0)


		network = get_network()
		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		};

		agg_tx = {
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => get_deadline(network),
			"transactions" => [tx1],

		};


		catjson = load_catjson(agg_tx,network)
#		puts catjson
		layout = load_layout(agg_tx,catjson,false)
#		puts layout
		prepared_tx = prepare_transaction(agg_tx,layout,network) 
		parsed_tx = parse_transaction(prepared_tx,layout,catjson,network) 
#		puts parsed_tx
		built_tx = build_transaction(parsed_tx)
#		puts built_tx

		private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"
		signature = sign_transaction(built_tx,private_key,network);
#		puts "============================="
#		puts signature
		built_tx = update_transaction(built_tx,"signature","value",signature);
		tx_hash = hash_transaction(agg_tx["signer_public_key"],signature,built_tx,network);
		payload = hexlify_transaction(built_tx)
#		puts payload

=end

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
		

	end


end
