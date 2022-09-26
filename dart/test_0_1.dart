import 'dart:convert';
import 'dart:developer';
import 'dart:typed_data';
import 'package:sha3/sha3.dart';
import 'package:base32/base32.dart';
import 'package:convert/convert.dart';
import 'package:http/http.dart' as http;
import 'package:ed25519_edwards/ed25519_edwards.dart' as ed;
import 'package:hash/hash.dart';

// テストパッケージとテスト対象のコードを読み込みます
import 'package:test/test.dart';
import 'tsunagi-functions-0.1.dart';

void main() {
	group('Person Test', () {
		test('Init', () async {

			var network = {
				"version":1,
				"network":'TESTNET',
				"generationHash":'7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836',
				"epochAdjustment":1637848847,
				"catjasonBase":"https://xembook.github.io/tsunagi-sdk/catjson/",
			};


		//	var now = (new DateTime.now().millisecondsSinceEpoch / 1000).floor();
			int now = network["epochAdjustment"];
			var deadline = ((now + 7200) - network["epochAdjustment"]) * 1000;

			var tx1 = {
				"type" : "TRANSFER",
				"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
				"fee" : 25000,
				"deadline" : deadline,
				"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
				"mosaics" : [
					{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
					{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
				],
				"message" : "Hello Tsunagi(Catjson) SDK!"
			};

			var catjson = await loadCatjson(tx1,network);
			var layout = await loadLayout(tx1,catjson,false);
			var preparedTx = await prepareTransaction(tx1,layout,network); //TX事前準備
			var parsedTx = await parseTransaction(preparedTx,layout,catjson,network); 
			var builtTx = buildTransaction(parsedTx); 

			var signature = signTransaction(builtTx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",network);
			builtTx = updateTransaction(builtTx,"signature","value",signature);

			var txHash = hashTransaction(tx1["signer_public_key"],signature,builtTx,network);

			var payload = hexlifyTransaction(builtTx,0);



				print(payload);

			// 期待値とチェック
			expect(payload, "dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21");
		});

	});



	group('Function Test', () {
		test('Init', () {


				// 期待値とチェック
			expect(generateNamespaceId("xembook",0), BigInt.parse("11832106220717372293"));
			expect(generateNamespaceId("xembook",0).toRadixString(16), "a43415eb268c7385");
			expect(generateNamespaceId("tomato",generateNamespaceId("xembook",0)).toRadixString(16), "fa547fd28c836431");
			expect(generateKey("key_account").toRadixString(16), "9772b71b058127d7");
			expect(generateMosaicId(generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),1700836761).toRadixString(16), "4daffbe5505de676");
			expect(convertAddressAliasId(generateNamespaceId("xembook",0)), "85738c26eb1534a4000000000000000000000000000000");	

		});

	});




	group('transfer transaction', (){});
	group('aggregate complete transaction', (){

			test('Init', () {

			});

	});
	group('aggregate bonded transaction', (){});
	group('mosaic transaction', (){});
	group('namespace transaction', (){});
	group('metadata transaction', (){});
	group('multisig transaction', (){});
	group('account restriction transaction', (){});
	group('global mosaic restriction transaction', (){});
	group('mosaic supply revocation transaction', (){});
	group('secret lock-proof transaction', (){});












}

