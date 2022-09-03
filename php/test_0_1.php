<?php

include 'tsunagi-sdk-0.1.php';
require 'vendor/autoload.php';
use Base32\Base32;

class helper{

	public $network;
    public function __construct($network){
        $this->network = $network;
    }

    public function bar(){
        echo 'bar';
    }

	public	function get_payload($tx1){
	
		$catjson = load_catjson($tx1,$this->network);
		$layout = load_layout($tx1,$catjson,false); //isEmbedded false
		$prepared_tx = prepare_transaction($tx1,$layout,$this->network); //TX事前準備
		$parsed_tx = parse_transaction($prepared_tx,$layout,$catjson,$this->network);

		$built_tx    = build_transaction($parsed_tx); //TX構築
		$private_key = "94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c75f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb";
		$signature = sign_transaction($built_tx,$private_key,$this->network);
		$built_tx = update_transaction($built_tx,"signature","value",$signature);
		$tx_hash = hash_transaction($tx1["signer_public_key"],$signature,$built_tx,$this->network);
		$payload = hexlify_transaction($built_tx,0);
		print_r($payload);
		print_r($tx_hash);

		return $payload;
	}
}

class test_0_1 extends \PHPUnit\Framework\TestCase {


	protected function setUp() :void{

		$this->network = [
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
		$now = $this->network["epochAdjustment"] * 1000;
		$this->deadline_time = ((intval($now/1000)  + 7200) - 1637848847) * 1000;
	}

	public function testTransfer(){

		$helper = new helper($this->network);

		//resolves 2 mosaic transfer
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			, $payload
		);

		//resolves 2 mosaic transfer by namespace
		//TODO

		//resolves opposite mosaice order
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
			],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			, $payload
		);

		//resolves null message
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
			],
			"message" => "",
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"c100000000000000c086746240315084735ebee633ff541056c5ba0f17c4d924a4b59c9531aa72243eaa7b76e5e0a9e32a15fb475be49a2f1ff1e380c763bcb2ab3ef5d83125b40d5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80100020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a640000000000000000"
			, $payload
		);

		//resolves undefined message
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [
				["mosaic_id" =>  0x3A8416DB2D53B6C8, "amount" => 100],
				["mosaic_id" =>  0x2A09B7F9097934C2, "amount" => 1],
			],
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"c000000000000000fee4646022be8647455bc876a8f7f303233d297a5755cd1eb41999ae6c8cca2f0225e2b93c4aa793c68657c230578dc3af26c3ef32acae96ea1ae10c438278055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a6400000000000000"
			, $payload
		);

		//resolves null mosaic transfer
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [],
			"message" => "Hello Tsunagi(Catjson) SDK!",
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"bc00000000000000cd5b93e94f053a07a5a132d7f59708b6818d88840c150d6f6dc38a2ca2408fff0e7e3ee39599d1242a0e4a5869dec8a2847b05fb698fa39db2bf1c3bf46ce2005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21"
			, $payload
		);

		//resolves undefined message and null mosaic
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [],
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"a0000000000000002c271a17d41832515a9ad0e995a524a4859a001436a990370c4b53eaa63677b4d69edde0831171a10defc157ea01f1d5528a562c423e38c725fc5b37af35ee055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000000000000000"
			, $payload
		);


		//resolves null message and null mosaic
		$tx1 = [
			"type" => "TRANSFER",
			"signer_public_key" => "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
			"fee" => 25000,
			"deadline" => $this->deadline_time,
			"recipient_address" => bin2hex(Base32::decode("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA")),
			"mosaics" => [],
			"message" => "",
		];

		$payload = $helper->get_payload($tx1);
		print_r($payload);

		$this->assertEquals(
			"a100000000000000786d46993afe584dd4e1fd2904d8eb0ea67e27ca3c7ef81fd208a6f27c1450807234093f9be03bbda0b02d96a69bd2766595ac4ab59fbc5119d247181b5596065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8010000000000000000"
			, $payload
		);
	}

	public function testAggregateComplete(){
		$this->markTestIncomplete();
	}

	public function testAggregateBoded(){
		$this->markTestIncomplete();
	}

	public function testMosaic(){
		$this->markTestIncomplete();
	}

	public function testNamespace(){
		$this->markTestIncomplete();
	}

	public function testMetadata(){
		$this->markTestIncomplete();
	}

	public function testMultisig(){
		$this->markTestIncomplete();
	}

	public function testAccountRestriction(){
		$this->markTestIncomplete();
	}

	public function testGlobalMosaicRestriction(){
		$this->markTestIncomplete();
	}

	public function testMosaicSupplyRevocation(){
		$this->markTestIncomplete();
	}

	public function testSecret(){
		$this->markTestIncomplete();
	}


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
*/
}
?>


