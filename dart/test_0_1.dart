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


//				print(layout);
			expect(preparedTx, "mario");

			// 期待値とチェック
			expect("mario", "mario");
		});

	});

	group('transfer transaction', (){});
	group('aggregate complete transaction', (){

			test('Init', () {

				// 期待値とチェック
				expect("mario", "mario");
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

