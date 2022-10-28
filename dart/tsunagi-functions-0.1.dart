import 'dart:convert';
import 'dart:developer';
import 'dart:typed_data';
import 'dart:collection';
import 'dart:io';
import 'package:sha3/sha3.dart';
import 'package:base32/base32.dart';
import 'package:convert/convert.dart';
import 'package:http/http.dart' as http;
import 'package:ed25519_edwards/ed25519_edwards.dart' as ed;
import 'package:hash/hash.dart';

loadCatjson(tx,network) async{



	var jsonFile;
	if(tx["type"] == 'AGGREGATE_COMPLETE' || tx["type"] == 'AGGREGATE_BONDED'){
		jsonFile =  'aggregate.json';
	}else{
		jsonFile =  tx["type"].toLowerCase() + '.json';
	}

	var result = await http.get(Uri.parse(network["catjasonBase"] + jsonFile));


	return json.decode(result.body);

}
loadLayout(tx,catjson,isEmbedded){
	
	var prefix;
	if(isEmbedded){
		prefix = "Embedded";
	}else{
		prefix = "";
	}

	var layoutName;

	if(      tx["type"] == "AGGREGATE_COMPLETE"){ layoutName = "AggregateCompleteTransaction";
	}else if(tx["type"] == "AGGREGATE_BONDED"){   layoutName = "AggregateBondedTransaction";
	}else{
		layoutName = prefix + toCamelCase(tx["type"]) + "Transaction";
	}

//	let factory = catjson.find(item => item.factory_type === prefix + "Transaction" && item.name === layoutName);
	
	var factory = catjson.firstWhere((item) => item["factory_type"] == prefix + "Transaction" && item["name"] == layoutName);
	return factory["layout"];

//return layoutName;	
	
	
//	return 0;
}

toCamelCase(str) {
	var res =  str.split('_').map((word) =>word[0].toUpperCase() + word.substring(1).toLowerCase()).toList().join('');
//	print(res);
	return res;
}

prepareTransaction(tx,layout,network) async{

	var preparedTx = {}..addAll(tx);

	preparedTx["network"] = network["network"];
	preparedTx["version"] = network["version"];

	if(preparedTx.containsKey("message")){
		preparedTx["message"] = "00" + hex.encode(utf8.encode(tx["message"]));
	}

	if(preparedTx.containsKey("name")){
		preparedTx["name"] = hex.encode(utf8.encode(tx["name"]));
	}

	if(preparedTx.containsKey("value")){
		preparedTx["value"] = hex.encode(utf8.encode(tx["value"]));
	}

	if(tx.containsKey("mosaics")){

		var castMosaics = tx["mosaics"] as List<dynamic>;
		preparedTx["mosaics"] = castMosaics..sort((a,b) => a["mosaic_id"].compareTo(b["mosaic_id"]));

//		print(preparedTx["mosaics"]);

	}
	//レイアウト層ごとの処理
	for (var layer in layout) {

//		print(layer["size"]);

		//size定義の調査
		if(layer.containsKey("size") && int.tryParse(layer["size"].toString()) == null){

			var size = 0;

			//element_dispositionが定義されている場合は、TX内の実データをそのサイズ数で分割する。
			if(layer.containsKey("element_disposition") && preparedTx.containsKey(layer["name"])){

				size = (preparedTx[layer["name"]].length / (layer["element_disposition"]["size"] * 2)).toInt();

			}else if(layer["size"].contains('_count')){//暫定 sizeにcountという文字列が含まれている場合はサイズ値を指定する項目が含まれると考える
				
				if(preparedTx.containsKey(layer["name"])){
					size = preparedTx[layer["name"]].length;
				}else{
					size = 0;
				}

			}else{
				//その他のsize値はPayloadの長さを入れるため現時点では不明
			}
			preparedTx[layer["size"]] = size;
		}

	}


	if(tx.containsKey('transactions')){
		var txes = [];
		for(var eTx in tx["transactions"]){

			var eCatjson = await loadCatjson(eTx,network);
			var eLayout = loadLayout(eTx,eCatjson,true);
			//再帰処理
			var ePreparedTx = await prepareTransaction(eTx,eLayout,network);
			txes.add(ePreparedTx);
		}
		preparedTx["transactions"] = txes;
	}

	return preparedTx;

}
parseTransaction(tx,layout,catjson,network) async{

	var parsedTx = []; //return
	for(var layer in layout){
		var layerType = layer["type"];
		var layerDisposition = "";

		if(layer.containsKey("disposition")){
			layerDisposition = layer["disposition"];
		}
		
		var catitem = {}..addAll( catjson.firstWhere((cj)=>cj["name"] == layerType,orElse: () => {}) );

		
		if(layer.containsKey("condition") ){
			if(layer["condition_operation"] == "equals"){
				if(layer["condition_value"] != tx[layer["condition"]]){
					continue;
				}
			}
		}


		if(layerDisposition == "const"){
			continue;


		}else if(layerType == "EmbeddedTransaction"){

			var txLayer = {}..addAll(layer);
			var items = [];
			for(var eTx in tx["transactions"]){ //小文字のeはembeddedの略
				var eCatjson = await loadCatjson(eTx,network);//catjsonの更新
				var eLayout = loadLayout(eTx,eCatjson,true); //isEmbedded:true
				var eParsedTx = await parseTransaction(eTx,eLayout,eCatjson,network); //再帰
				items.add(eParsedTx);
			}
			txLayer["layout"] = items;
			parsedTx.add(txLayer);
			continue;

		}else if(catitem.containsKey("layout") && tx.containsKey(layer["name"])){ // else:byte,struct

			var txLayer = {}..addAll(layer);
			var items = [];
			for(var item in tx[layer["name"]]){

				var itemParsedTx = await parseTransaction(item,catjson.firstWhere((cj)=>cj["name"] == layerType)["layout"],catjson,network); //再帰
				items.add(itemParsedTx);
			}
			txLayer["layout"] = items;
			parsedTx.add(txLayer);
			continue;

		}else if(layerType == "UnresolvedAddress"){
			//アドレスに30個の0が続く場合はネームスペースとみなします。
//			print("■■■■■■■■■■■■■■■■■■");
//			print(tx);
			if(tx.containsKey(layer["name"]) && tx[layer["name"]].contains("000000000000000000000000000000")){
				var prefix = (catjson.firstWhere((cj)=>cj["name"] == "NetworkType")["values"].firstWhere((vf)=>vf["name"]==tx["network"])["value"] + 1).toRadixString(16);
				tx[layer["name"]] =  prefix + tx[layer["name"]];
			}
		}else if(catitem["type"] == "enum"){

			if(catitem["name"].contains('Flags')){

				var value = 0;
				for(var itemLayer in catitem["values"]){
					if(tx[layer["name"]].contains(itemLayer["name"])){
						value += itemLayer["value"];
					}
				}
				catitem["value"] = value;
			}else if(layerDisposition.contains('array')){
				var values = [];
				for(var item in tx[layer["name"]]){
					values.add(catitem["values"].firstWhere((cvf)=>cvf["name"] == item)["value"]);
				}
				tx[layer["name"]] = values;
			}else{
			
				catitem["value"] = catitem["values"].firstWhere((cvf)=>cvf["name"] == tx[layer["name"]])["value"];
			}
		}


		//layerの配置
		if(layerDisposition.contains('array')){ // "array sized","array fill"

			if(layerType == "byte"){

				if(layer.containsKey("element_disposition")){ //message
					var size = tx[layer["size"]];
					var subLayout = {}..addAll(layer);

					var items = [];
					for(var count = 0; count < size; count++){
						var txLayer = {};
						txLayer["signedness"] = layer["element_disposition"]["signedness"];
						txLayer["name"] = "element_disposition";
						txLayer["size"] = layer["element_disposition"]["size"];
						txLayer["value"] = tx[layer["name"]].substring(count * 2, count * 2 + 2);
						txLayer["type"] = layerType;
						items.add(txLayer);
					}
					subLayout["layout"] = items;
					parsedTx.add(subLayout);

				}else{print("not yet");}
			}else if(tx.containsKey(layer["name"])){

				var subLayout = {}..addAll(layer);
				var items = [];
				for(var txItem in tx[layer["name"]]){
					var txLayer = {}..addAll(catjson.firstWhere((cj)=>cj["name"] == layerType));
					txLayer["value"] = txItem;
					
					if(layerType == "UnresolvedAddress"){
						//アドレスに30個の0が続く場合はネームスペースとみなします。
						if(txItem.contains("000000000000000000000000000000")){
							var prefix = (catjson.firstWhere((cf)=>cf["name"]=="NetworkType")["values"].firstWhere((vf)=>vf["name"]==tx["network"])["value"] + 1).toRadixString(16);
							txLayer["value"] =  prefix + txLayer["value"];
						}
					}			
//					items.add([txLayer]);
					items.add(txLayer);
				}
				subLayout["layout"] = items;
				parsedTx.add(subLayout);


			}// else{print("not yet");}
		}else{ //reserved またはそれ以外(定義なし)

			var txLayer = {}..addAll(layer);

			if(catitem.length > 0){

				//catjsonのデータを使う
				txLayer["signedness"]	= catitem["signedness"];
				txLayer["size"]  = catitem["size"];
				txLayer["type"]  = catitem["type"];
				txLayer["value"] = catitem["value"];
			}

			//txに指定されている場合上書き(enumパラメータは上書きしない)
			if(tx.containsKey(layer["name"]) && catitem["type"] != "enum"){
				txLayer["value"] = tx[layer["name"]];
			}else{
				/* そのままtxLayerを追加 */
//				print(layer["name"]);
			}
			parsedTx.add(txLayer);
		}

	}

	var layerSize = parsedTx.firstWhere((lf) => lf["name"] == "size", orElse: () => null);
	if(layerSize != null && layerSize.containsKey("size")){
		layerSize["value"] = countSize(parsedTx,0);
	}

//	print(parsedTx);
	return parsedTx;
}


countSize(item,alignment){
	var totalSize = 0;

//	print("start : countSize");
//	print(item.runtimeType.toString());
//	print(item);

	//レイアウトを構成するレイヤーサイズの取得
	if(item.runtimeType.toString() == "List<dynamic>"){

		var layoutSize = 0;
		for(var layout in item){
			layoutSize += countSize(layout,alignment);
		}		 
		if(alignment != null && alignment > 0){
			layoutSize = ((layoutSize  + alignment - 1) / alignment ).floor() * alignment;
		}
		totalSize += layoutSize;


	//レイアウトサイズの取得
	}else if(item != null &&  item.containsKey("layout")){

		for(var layer in item["layout"]){
			var itemAlignment;
			if(item.containsKey("alignment")){
				itemAlignment = item["alignment"];
			}else{
				itemAlignment = 0;
			}
			totalSize += countSize(layer,itemAlignment); //再帰
		}

	}else{
		if(item.containsKey("size")){
			totalSize += item["size"];
//			print(item["name"] + ":" + item["size"].toString());
		}else{print("no size:" + item["name"]);}

	}
//	print(totalSize);
	return totalSize;
}




buildTransaction(parsedTx){
	var builtTx = []..addAll(parsedTx);

	var layerPayloadSize = builtTx.firstWhere((lf)=>lf["name"] == "payload_size", orElse: () => null);
	if(layerPayloadSize != null && layerPayloadSize.containsKey("size")){
		layerPayloadSize["value"] = countSize(builtTx.firstWhere((lf)=>lf["name"] == "transactions"),0);
	}

	var layerTransactionsHash = builtTx.firstWhere((lf)=>lf["name"] == "transactions_hash", orElse: () => null);
	if(layerTransactionsHash != null){

		var hashes = [];
		for(var eTx in builtTx.firstWhere((lf)=>lf["name"] == "transactions")["layout"]){
			var hasher = SHA3(256, SHA3_PADDING, 256);
			hashes.add(hasher.update(hex.decode(hexlifyTransaction(eTx,0))).digest());
		}

		var numRemainingHashes = hashes.length;
		while (1 < numRemainingHashes) {
			var i = 0;
			while (i < numRemainingHashes) {
				var hasher = SHA3(256, SHA3_PADDING, 256);
				hasher.update(hashes[i]);

				if (i + 1 < numRemainingHashes) {
					hasher.update(hashes[i + 1]);
				} else {
					// if there is an odd number of hashes, duplicate the last one
					hasher.update(hashes[i]);
					numRemainingHashes += 1;
				}
				hashes[(i / 2).floor()] = hasher.digest();
				i += 2;
			}
			numRemainingHashes = (numRemainingHashes / 2).floor();
		}
		layerTransactionsHash["value"] = hex.encode(hashes[0]);
	}

	return builtTx;
}

hexlifyTransaction(item,alignment){
	var payload = "";

	if(item.runtimeType.toString() == "List<dynamic>"){
		var subLayoutHex = "";
		for(var subLayout in item){
			subLayoutHex += hexlifyTransaction(subLayout,alignment);
		}
		if(alignment != null && alignment > 0){
			var alignedSize = ((subLayoutHex.length + (alignment * 2) - 2)/ (alignment * 2) ).floor() * (alignment * 2);
			subLayoutHex = subLayoutHex + ("0" * (alignedSize - subLayoutHex.length));
		}
		payload += subLayoutHex;

	}else if(item != null && item.containsKey("layout")){
		for(var layer in item["layout"]){
			var itemAlignment;
			if(item.containsKey("alignment")){
				itemAlignment = item["alignment"];
			}else{
				itemAlignment = 0;
			}
			payload += hexlifyTransaction(layer,itemAlignment); //再帰
		}

	}else{
		var size = item["size"];
//		if(item["value"] == null){
		if(!item.containsKey("value")){
			if(size >= 24){
				item["value"] = "00" * size;
			}else{
				item["value"] = 0;
			}
		}

		if(size==1){
			if(item["name"] == "element_disposition"){
				payload = item["value"];
			}else{
				payload = hex.encode(Uint8List(1)..buffer.asByteData().setInt8(0, item["value"]));
			}	 
		}else if(size==2){
			payload = hex.encode(Uint8List(2)..buffer.asByteData().setInt16(0, item["value"], Endian.little));
		}else if(size==4){
			payload = hex.encode(Uint8List(4)..buffer.asByteData().setInt32(0, item["value"], Endian.little));
		}else if(size==8){
			if(item["value"].runtimeType.toString() == "int"){
				payload = hex.encode(Uint8List(8)..buffer.asByteData().setInt64(0, item["value"], Endian.little));
			}else{
				payload = hex.encode(bigIntToUint8List(item["value"]));
			}
		}else if(size==24 || size==32 || size==64){
			payload = item["value"];
		}else{
			print("unknown size order");
		}
	}
//	print(payload);

	return payload;

}


signTransaction(builtTx,priKey,network){

	var privateKey = ed.newKeyFromSeed(hex.decode(priKey));

	var verifiableData = getVerifiableData(builtTx);
	var payload = network["generationHash"]  +  hexlifyTransaction(verifiableData,0);
	var signature = hex.encode(ed.sign(privateKey, hex.decode(payload)));
	return signature; 

}

getVerifiableData(builtTx){

	var typeLayer = builtTx.firstWhere((bf) => bf["name"] == "type");
	if([16705,16961].contains(typeLayer["value"])){

		return builtTx.sublist(5,11);
	}else{
//		return builtTx.sublist(5,builtTx.length);
		return builtTx.sublist(5);
	}
}
hashTransaction(signer,signature,builtTx,network){


	var hasher = SHA3(256, SHA3_PADDING, 256);

	hasher.update(hex.decode(signature));
	hasher.update(hex.decode(signer));
	hasher.update(hex.decode(network["generationHash"]));
	hasher.update(hex.decode(hexlifyTransaction(getVerifiableData(builtTx),0)));
	return hex.encode(hasher.digest());
}



updateTransaction(builtTx,name,type,value){
	
	var updatedTx = []..addAll(builtTx);

	var layer = updatedTx.firstWhere((bf)=>bf["name"] == name);
	layer[type] = value;
	return updatedTx;
}

cosignTransaction(txhash,priKey){

	var privateKey = ed.newKeyFromSeed(hex.decode(priKey));
	var signature = hex.encode(ed.sign(privateKey, hex.decode(txhash)));
	return signature; 
}

generateAddressId(address){

	return base32.decodeAsHexString(address);

}


generateNamespaceId(name, parentNamespaceId){

	var namespace_flag = BigInt.from(1) << 63;

	var hasher = SHA3(256, SHA3_PADDING, 256);

	hasher.update(Uint8List(4)..buffer.asByteData().setInt32(0, ( BigInt.parse(parentNamespaceId.toString()) & BigInt.from(0xFFFFFFFF)).toInt(), Endian.little));
	hasher.update(Uint8List(4)..buffer.asByteData().setInt32(0, ((BigInt.parse(parentNamespaceId.toString()) >> 32) & BigInt.from(0xFFFFFFFF)).toInt(), Endian.little));


	hasher.update(utf8.encode(name));
	var digest =  digestToBigint(hasher.digest());
	return digest | namespace_flag;
}


digestToBigint(digest){

	var result = BigInt.from(0);
	for(var count = 0; count < 8; count++){
		result += BigInt.from(digest[count]) << 8 * count;
	}

	return result;
}

generateKey(name){
	
	var namespace_flag = BigInt.from(1) << 63;

	var hasher = SHA3(256, SHA3_PADDING, 256);
	hasher.update(utf8.encode(name));

	var digest =  digestToBigint(hasher.digest());
	return digest | namespace_flag;
}

generateMosaicId(ownerAddress, nonce){

	var namespace_flag = BigInt.from(1) << 63;

	var hasher = SHA3(256, SHA3_PADDING, 256);
	hasher.update(Uint8List(4)..buffer.asByteData().setInt32(0, nonce, Endian.little));
	hasher.update(hex.decode(ownerAddress));

	var result =  digestToBigint(hasher.digest());

	if( result & namespace_flag > BigInt.from(0)){
		result -= namespace_flag;
	}
	return result;


}

convertAddressAliasId(namespaceId){

	return hex.encode( bigIntToUint8List(namespaceId)) + "000000000000000000000000000000";
}


Uint8List bigIntToUint8List(BigInt bigInt) =>
	bigIntToByteData(bigInt).buffer.asUint8List();

ByteData bigIntToByteData(BigInt bigInt) {
	final data = ByteData((bigInt.bitLength / 8).ceil());
	var _bigInt = bigInt;

	for (var i = 0; i < data.lengthInBytes; i++) {
		data.setUint8(i, _bigInt.toUnsigned(8).toInt());
		_bigInt = _bigInt >> 8;
	}

	return data;
}

