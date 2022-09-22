import 'dart:convert';
import 'dart:developer';
import 'dart:typed_data';
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

prepareTransaction(tx,layout,network){
	return 0;
}
parseTransaction(tx,layout,catjson,network){
	return 0;
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
countSize(item,alignment){
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