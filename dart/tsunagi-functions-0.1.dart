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
	print(res);
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
		preparedTx["name"] = hex.encode(utf8.encode(tx["value"]));
	}

	if(tx.containsKey("mosaics")){

		var castMosaics = tx["mosaics"] as List<dynamic>;
		preparedTx["mosaics"] = castMosaics..sort((a,b) => a["mosaic_id"].compareTo(b["mosaic_id"]));

		print(preparedTx["mosaics"]);

	}
	//レイアウト層ごとの処理
	for (var layer in layout) {

		print(layer["size"]);

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

		var catitem = {}..addAll( catjson.firstWhere((cj)=>cj["name"] == layerType,orElse: () => null) );

		
		if(layer.containsKey("condition") ){
			if(layer["condition_operation"] === "equals"){
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
			if(tx.containsKey(layer["name"]) && tx[layer["name"]].contains("000000000000000000000000000000")){
				var prefix = (catjson.firstWhere((cj)=>cj["name"] == "NetworkType")["values"].firstWhere(vf=>vf["name"]==tx["network"])["value"] + 1).toRadixString(16);
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

			var size = tx[layer["size"]];
			if(layerType == "byte"){

				if(layer.containsKey()){ //message
					var subLayout = {}..addAll(layer);

					var items = [];
					for(let count = 0; count < size; count++){
						var txLayer = {};
						txLayer["signedness"] = layer["element_disposition"]["signedness"];
						txLayer["name"] = "element_disposition";
						txLayer["size"] = layer["element_disposition"]["size"];
						txLayer["value"] = tx[layer["name"]].substring(count * 2, 2);
						txLayer["type"] = layerType;
						items.add([txLayer]);
					}
					subLayout["layout"] = items;
					parsedTx.add(subLayout);

				}else{print("not yet");}
			}else if(tx.containsKey(layer["name"])){

				var subLayout = {}..addAll(layer);
				let items = [];
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
					items.add([txLayer]);
				}
				subLayout["layout"] = items;
				parsedTx.add(subLayout);


			}// else{print("not yet");}
		}else{ //reserved またはそれ以外(定義なし)

			var txLayer = {}..addAll(layer);

			if(catitem.length > 0){

				//catjsonのデータを使う
				txLayer["signedness	= catitem.signedness;
				txLayer["size"]  = catitem["size"];
				txLayer["type"]  = catitem["type"];
				txLayer["value"] = catitem["value"];
			}

			//txに指定されている場合上書き(enumパラメータは上書きしない)
			if(tx.containsKey(layer["name"]) && catitem["type"] !== "enum"){
				txLayer["value"] = tx[layer["name"]];
			}else{
				/* そのままtxLayerを追加 */
				console.log(layer["name"]);
			}
			parsedTx.add(txLayer);
		}

	}

	var layerSize = parsedTx.firstWhere((lf) => lf["name"] == "size", orElse: () => null);
	if(layerSize != null && layerSize.containsKey("size")){
		layerSize["value"] = countSize(parsedTx,0);
	}

	print(parsedTx);
	return parsedTx;
}


countSize(item,alignment){
	var totalSize = 0;

	//レイアウトサイズの取得
	if(item != null &&  item.containsKey("layout")){

		for(var layer in item["layout"]){
			var itemAlignment;
			if(item.containsKey("alignment")){
				itemAlignment = item["alignment"];
			}else{
				itemAlignment = 0;
			}
			totalSize += countSize(layer,itemAlignment); //再帰
		}

	//レイアウトを構成するレイヤーサイズの取得
	}else if(item.runtimeType.toString() == "List<dynamic>"){

		var layoutSize = 0;
		for(var layout in item){
			layoutSize += countSize(layout,alignment);
		}		 
		if(alignment !== null && alignment > 0){
			layoutSize = ((layoutSize  + alignment - 1) / alignment ).floor() * alignment;
		}
		totalSize += layoutSize;

	}else{
		if(item.containsKey("size")){
			totalSize += item["size"];
			print(item.name + ":" + item.size);
		}else{print("no size:" + item.name);}

	}
	console.log(totalSize);
	return totalSize;
}




buildTransaction(parsedTx){
	return 0;
}
getVerifiableData(builtTx){
	return 0;
}
hashTransaction(signer,signature,builtTx,network){
	return 0;
}



updateTransaction(builtTx,name,type,value){
	return 0;
}
hexlifyTransaction(item,alignment){
	return 0;
}
signTransaction(builtTx,priKey,network){
	return 0;
}
cosignTransaction(txhash,priKey){
	return 0;
}

generateAddressId(address){
	return 0;

}