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

=end

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

=begin
		#//resolves 2 mosaic transfer by namespace
		$tx1 = [
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
		];

		$payload = $helper->get_payload($tx1);

		$this->assertEquals(
			"dc00000000000000a1bcb56de796c45cd982e79748772cd9a616a084c95fc775a1d003b9f5f2dcbffa95e869e8a2d77873bbe3d26d5c2764e8299bded689037e4ede6095008cc2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d00000000009985738c26eb1534a40000000000000000000000000000001c00020000000000eeaff441ba994be764000000000000003164838cd27f54fa01000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			, $payload
		);
=end



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

	end


end
