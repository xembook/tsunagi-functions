<?php

include 'tsunagi-sdk-0.1.php';
require 'vendor/autoload.php';
use Base32\Base32;

class test_0_1 extends \PHPUnit\Framework\TestCase {


/*

    public function testWithTaxAndTip() {
        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);
    }

    public function testWithTaxAndTip2() {

		$network = [
			"version" => 1,
			"network" => "TESTNET",
			"generationHash" => "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
			"currencyMosaicId" => 0x3A8416DB2D53B6C8,
			"currencyNamespaceId" => 0xE74B99BA41F4AFEE,
			"currencyDivisibility" => 6,
			"epochAdjustment" => 1637848847,
			"catjasonBase" => "https://xembook.github.io/tsunagi-sdk/catjson/",
			"wellknownNodes" => [
				"https://sym-test.opening-line.jp:3001",
				"https://sym-test.opening-line.jp:3001",
				"https://sym-test.opening-line.jp:3001",
			]
		];


		//Alice->Bob
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => 0,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",

		];

//		var_dump($tx1);

		$catjson = load_catjson($tx1,$network);
		$layout = load_layout($tx1,$catjson,false); //isEmbedded false
//		var_dump($layout);
		$prepared_tx = prepare_transaction($tx1,$layout,$network); //TX事前準備
		$parsed_tx = parse_transaction($prepared_tx,$layout,$catjson,$network);

        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);

    }
*/

    public function testWithTaxAndTip3() {

		$network = [
			"version" => 1,
			"network" => "TESTNET",
			"generationHash" => "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
			"currencyMosaicId" => 0x3A8416DB2D53B6C8,
			"currencyNamespaceId" => 0xE74B99BA41F4AFEE,
			"currencyDivisibility" => 6,
			"epochAdjustment" => 1637848847,
			"catjasonBase" => "https://xembook.github.io/tsunagi-sdk/catjson/",
			"wellknownNodes" => [
				"https://sym-test.opening-line.jp:3001",
				"https://sym-test.opening-line.jp:3001",
				"https://sym-test.opening-line.jp:3001",
			]
		];


		//Alice->Bob
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",

		];

		$agg_tx = [
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => 0,
			"transactions" => [$tx1],

		];



//		var_dump($tx1);

		$catjson = load_catjson($agg_tx,$network);
		$layout = load_layout($agg_tx,$catjson,false); //isEmbedded false
//		var_dump($layout);
		$prepared_tx = prepare_transaction($agg_tx,$layout,$network); //TX事前準備
		$parsed_tx = parse_transaction($prepared_tx,$layout,$catjson,$network);
		$built_tx    = build_transaction($parsed_tx); //TX構築


        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);

    }


}
?>


