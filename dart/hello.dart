import 'dart:convert';
import 'dart:developer';
import 'dart:typed_data';
import 'package:sha3/sha3.dart';
import 'package:base32/base32.dart';
import 'package:convert/convert.dart';
import 'package:http/http.dart' as http;
import 'package:ed25519_edwards/ed25519_edwards.dart' as ed;
import 'package:hash/hash.dart';

void main() {

	var keyPair = ed.generateKey();
	var privateKey = keyPair.privateKey;
	var publicKey = keyPair.publicKey;
	print(hex.encode(privateKey.bytes));
	print(hex.encode(publicKey.bytes));

	var alicePrivateKey = ed.newKeyFromSeed(hex.decode("94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7"));
	print(alicePrivateKey.bytes);

	var alicePublicKey = ed.public(alicePrivateKey);
	var aliceKeypair = ed.KeyPair(alicePrivateKey,alicePublicKey);

	print(hex.encode(alicePrivateKey.bytes));
	print(hex.encode(alicePublicKey.bytes));

	var addressHasher = SHA3(256, SHA3_PADDING, 256);
	var publicKeyHash = addressHasher.update(alicePublicKey.bytes).digest();
	print("publicKeyHash");
	print(hex.encode(publicKeyHash));

	var addressBody = RIPEMD160().update(publicKeyHash).digest();
	print("addressBody");
	print(hex.encode(addressBody));

	var sumHasher = SHA3(256, SHA3_PADDING, 256);
	var sumHash = sumHasher.update(hex.decode("98" + hex.encode(addressBody))).digest();
	print(hex.encode(sumHash).substring(0,6));
	var aliceAddress = base32.encode(hex.decode("98" + hex.encode(addressBody) + hex.encode(sumHash).substring(0,6)));
	print("98" + hex.encode(addressBody) + hex.encode(sumHash).substring(0,6));
	aliceAddress = aliceAddress.substring(0, aliceAddress.length - 1);
	print(aliceAddress);

	var version = Uint8List(1)..buffer.asByteData().setInt8(0, 1);
	var networkType  = Uint8List(1)..buffer.asByteData().setInt8(0, 152);
	var transactionType   = Uint8List(2)..buffer.asByteData().setInt16(0, 16724, Endian.little);
	var fee   = Uint8List(8)..buffer.asByteData().setInt64(0, 1000000, Endian.little);
	var dt = (new DateTime.now().millisecondsSinceEpoch / 1000).floor();
	var secondLater7200 = ((dt + 7200) - 1637848847) * 1000;
	var deadline   = Uint8List(8)..buffer.asByteData().setInt64(0, secondLater7200, Endian.little);
	var recipientAddress = base32.decodeAsHexString("TBIL6D6RURP45YQRWV6Q7YVWIIPLQGLZQFHWFEQ");
	print(recipientAddress);
	var mosaicCount = Uint8List(1)..buffer.asByteData().setInt8(0, 1);
	var mosaicId = Uint8List(8)..buffer.asByteData().setInt64(0, 0x3A8416DB2D53B6C8, Endian.little);
	var mosaicAmount = Uint8List(8)..buffer.asByteData().setInt64(0, 100, Endian.little);
	List<int> message = utf8.encode('Hello Dart! Welcome to Symbol world!');
	var messageSize    = Uint8List(2)..buffer.asByteData().setInt16(0, message.length + 1, Endian.little);

	var verifiableBody = hex.encode(version)
		+ hex.encode(networkType)
		+ hex.encode(transactionType)
		+ hex.encode(fee)
		+ hex.encode(deadline)
		+ recipientAddress
		+ hex.encode(messageSize)
		+ hex.encode(mosaicCount)
		+ "00" + "00000000"
		+ hex.encode(mosaicId)
		+ hex.encode(mosaicAmount)
		+ "00" + hex.encode(message);

	var verifiableString = "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836"
		+ verifiableBody;

	var verifiableBuffer = hex.decode(verifiableString);
	var signature = ed.sign(alicePrivateKey, verifiableBuffer);

	var transactionSize = Uint8List(4)..buffer.asByteData().setInt32(0, hex.decode(verifiableBody).length + 108, Endian.little);

	var payloadString = hex.encode(transactionSize)
		+ "00000000"
		+ hex.encode(signature) 
		+ hex.encode(alicePublicKey.bytes) 
		+ "00000000" 
		+ verifiableBody;

	var payload = {
		"payload":payloadString
	};
/*
	var response = http.put(
		Uri.parse("https://sym-test-02.opening-line.jp:3001/transactions"),
		headers: {'Content-Type': 'application/json'},
		body: jsonEncode(payload),
	)
	.then((http.Response response) {
		print("Response status: ${response.statusCode}");
		print("Response body: ${response.contentLength}");
		print(response.headers);
		print(response.request);
	});
*/

	var hashableBuffer = hex.decode(
		 hex.encode(signature) 
		+ hex.encode(alicePublicKey.bytes) 
		+ verifiableString
	);

	var hasher = SHA3(256, SHA3_PADDING, 256);
	var transactionHash = hasher.update(hashableBuffer).digest();

	print("transactionStatus: https://sym-test-02.opening-line.jp:3001/transactionStatus/" + hex.encode(transactionHash));
	print("confirmed: https://sym-test-02.opening-line.jp:3001/transactions/confirmed/" + hex.encode(transactionHash));
	print("explorer: https://testnet.symbol.fyi/transactions/" +  hex.encode(transactionHash));
}
