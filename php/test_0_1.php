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

		$deadline_time = ((time()  + 7200) - 1637848847) * 1000;

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
			"deadline" => $deadline_time,
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

		$built_tx    = build_transaction($parsed_tx); //TX構築
		$private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c75f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb";
		$signature = sign_transaction($built_tx,$private_key,$network);
		$built_tx = update_transaction($built_tx,"signature","value",$signature);

		$tx_hash = hash_transaction($tx1["signer_public_key"],$signature,$built_tx,$network);




		$payload = hexlify_transaction($built_tx,0);
		print_r($payload);

//		print_r($built_tx);



        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);

    }

*/
/*
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
		$private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c75f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb";
		$signature = sign_transaction($built_tx,$private_key,$network);
		$built_tx = update_transaction($built_tx,"signature","value",$signature);

		$tx_hash = hash_transaction($agg_tx["signer_public_key"],$signature,$built_tx,$network);









		$payload = hexlify_transaction($built_tx,0);
		print_r($payload);



        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);

    }
*/

    public function testWithTaxAndTip4() {


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

		$deadline_time = ((time()  + 7200) - 1637848847) * 1000;
//		$now = $network["epochAdjustment"] * 1000;
//		$deadline_time = ((intval($now/1000)  + 7200) - 1637848847) * 1000;

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

		//Bob->Caroll
		$tx2 = [
			"type" => "TRANSFER",
			"signer_public_key" => "6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
			"recipient_address" => bin2hex(Base32::decode("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ")),
			"mosaics" => [
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
			],
			"message" => "Hello Carol! This is Bob.",

		];

		//Caroll->Alice
		$tx3 = [
			"type" => "TRANSFER",
			"signer_public_key" => "886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
			"recipient_address" => bin2hex(Base32::decode("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI")),
			"mosaics" => [
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
			],
			"message" => "Hello Alice, This is Carol.",

		];

		$cosignature1 = [
			"version" => 0,
			"signer_public_key" =>"6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC",
			"signature"=>"",
		];

		$cosignature2 = [
			"version" => 0,
			"signer_public_key" => "886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E",
			"signature" => "",
		];



		$agg_tx = [
			"type" => 'AGGREGATE_COMPLETE',
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 1000000,
			"deadline" => $deadline_time,
			"transactions" => [$tx1,$tx2,$tx3],
			"cosignatures" => [$cosignature1,$cosignature2]

		];

		$catjson = load_catjson($agg_tx,$network);
		$layout = load_layout($agg_tx,$catjson,false); //isEmbedded false
		$prepared_tx = prepare_transaction($agg_tx,$layout,$network); //TX事前準備
		$parsed_tx = parse_transaction($prepared_tx,$layout,$catjson,$network);
		$built_tx    = build_transaction($parsed_tx); //TX構築
		$private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c75f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb";
		$signature = sign_transaction($built_tx,$private_key,$network);
		$built_tx = update_transaction($built_tx,"signature","value",$signature);

		$tx_hash = hash_transaction($agg_tx["signer_public_key"],$signature,$built_tx,$network);

		//連署

		$bob_private_key = "fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b6199BAE3B241DF60418E258D046C22C8C1A5DE2F4F325753554E7FD9C650AFEC";
		$carol_private_key = "1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf886ADFBD4213576D63EA7E7A4BECE61C6933C27CD2FF36F85155C8FEBFB6EB4E";

		$prepared_tx["cosignatures"][0]["signature"] = cosign_transaction($tx_hash,$bob_private_key);
		$prepared_tx["cosignatures"][1]["signature"] = cosign_transaction($tx_hash,$carol_private_key);


		$filter_layout = array_filter($layout,function($fl){
			return $fl["name"] === "cosignatures";
		});
		$cosignatures_layout = array_values($filter_layout);

		$parsed_cosignatures = parse_transaction($prepared_tx,$cosignatures_layout,$catjson,$network); //連署TXの構築
		$built_tx = update_transaction($built_tx,"cosignatures","layout",$parsed_cosignatures[0]["layout"]);

		$payload = hexlify_transaction($built_tx,0);
		print_r($payload . PHP_EOL);

		print_r($tx_hash);

        $meal = 100;
        $tax = 10;
        $tip = 20;
        $result = restaurant_check($meal, $tax, $tip);
        $this->assertEquals(130, $result);

    }

}
?>


