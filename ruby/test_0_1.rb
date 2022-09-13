# 'test/unit'とテストするメソッド部品をrequire 
require 'test/unit'
require_relative 'tsunagi-functions-0.1.rb'

# Test::Unit::TestCaseを継承したclassを作成。名前がtest_で始まるメソッドを作成。
class TestTsunagi < Test::Unit::TestCase
  def test_tsunagi

		network = {
			"version" => 1,
			"network" => "TESTNET",
			"generationHash" => "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
			"epochAdjustment" => 1637848847,
			"catjasonBase" => "https://xembook.github.io/tsunagi-sdk/catjson/",
		};

#		now = Time.now.to_i;
		now = network["epochAdjustment"];
		deadline_time = ((now  + 7200) - network["epochAdjustment"]) * 1000;

		puts [deadline_time]
		puts [deadline_time].pack("Q").unpack('H*')[0]


		tx1 = {
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => deadline_time,
			"recipient_address" => generate_address_id("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
			"mosaics" => [
				{"mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1},
				{"mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100},
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		};

		catjson = load_catjson(tx1,network)
#		puts catjson
		layout = load_layout(tx1,catjson,false)
		puts layout


#    assert_equal 0, load_catjson(0,0)
  end
end
