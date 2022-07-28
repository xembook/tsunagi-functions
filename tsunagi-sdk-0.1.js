async function loadCatjson(tx,network){

	let jsonFile;
	if(tx.type === 'AGGREGATE_COMPLETE' || tx.type === 'AGGREGATE_BONDED'){
		jsonFile =  'aggregate.json';
	}else{
		jsonFile =  tx.type.toLowerCase() + '.json';
	}

	let res = await fetch(network.catjasonBase + jsonFile);
	let catjson = await res.json();
	
	return catjson;
}

async function loadLayout(tx,catjson,isEmbedded){

	let prefix;
	if(isEmbedded){
		prefix = "Embedded";
	}else{
		prefix = "";
	}

	let layoutName;

	if(      tx.type === "AGGREGATE_COMPLETE"){ layoutName = "AggregateCompleteTransaction";
	}else if(tx.type === "AGGREGATE_BONDED"){   layoutName = "AggregateBondedTransaction";
	}else{
		layoutName = prefix + toCamelCase(tx.type) + "Transaction";
	}

	let factory = catjson.find(item => item.factory_type === prefix + "Transaction" && item.name === layoutName);
	return factory.layout;
}

// スネークケースからキャメルケースに変換（文字列）.
function toCamelCase(str) {
	return str.split('_').map(function(word,index){
		return word.charAt(0).toUpperCase() + word.slice(1).toLowerCase();
	}).join('');
}




//事前準備
async function prepareTransaction(tx,layout,network){

	let preparedTx = Object.assign({}, tx);
	preparedTx.network = network.network;
	preparedTx.version = network.version;
	if('recipient_address' in preparedTx){
		preparedTx.recipient_address = buffer.Buffer(base32.decode(tx.recipient_address + "A").slice(0, -1)).toString("hex");
	}
	if('address' in preparedTx){
		preparedTx.address = buffer.Buffer(base32.decode(tx.address + "A").slice(0, -1)).toString("hex");
	}
	if('message' in preparedTx){
		preparedTx.message = buffer.Buffer.from([0,...(new TextEncoder('utf-8')).encode(tx.message)]).toString("hex");
	}
	//TODO:recipient_addressがネームスペースだった場合の変換も必要

	if('name' in preparedTx){
		preparedTx.name = buffer.Buffer.from((new TextEncoder('utf-8')).encode(tx.name)).toString("hex");
	}

	if("mosaics" in tx){
		tx.mosaics = tx.mosaics.sort(function(a,b){
			if(a.mosaic_id < b.mosaic_id) return -1;
			if(a.mosaic_id > b.mosaic_id) return 1;
			return 0;
		});
	}

	//レイアウト層ごとの処理
	for(let layer of layout){

		//size定義の調査
		if(layer.size !== undefined && isNaN(layer.size)){

			let size = 0;
			//element_dispositionが定義されている場合は、TX内の実データをそのサイズ数で分割する。
			if("element_disposition" in layer && layer.name in preparedTx){
				size = preparedTx[layer.name].length / (layer.element_disposition.size * 2);

			//それ以外は、TX内の実データサイズ数を指定する。
			}else if('sort_key' in layer){//暫定 sort_key できるsize値はカウント数を入れると解釈
				size = preparedTx[layer.name].length;
			}else{
				//その他のsize値はPayloadの長さを入れるため現時点では不明
			}
			preparedTx[layer.size] = size;
		}
	}
	if('transactions' in tx){
		let txes = [];
		for(let eTx of tx.transactions){

			let eCatjson = await loadCatjson(eTx,network);
			let eLayout = await loadLayout(eTx,eCatjson,true);
			//再帰処理
			ePreparedTx = await prepareTransaction(eTx,eLayout,network);
			txes.push(ePreparedTx);
		}
		preparedTx.transactions = txes;
	}
	console.log(preparedTx);
	return preparedTx;
}

async function parseTransaction(tx,layout,catjson,network){

	let builtTx = []; //return
	for(let layer of layout){

		let layerType = layer.type;
		let layerDisposition = layer.disposition;
		let catitem = Object.assign({}, catjson.find(cj=>cj.name === layerType));
		
		if("condition" in layer ){
			if(layer.condition_operation === "equals"){
				if(layer.condition_value !== tx[layer.condition]){
					continue;
				}
			}
		}

		if(layerDisposition === "const"){
			continue;
		}else if(layerType === "EmbeddedTransaction"){
			
			let txLayer = Object.assign({}, layer);
			let items = [];
			for(let eTx of tx.transactions){ //小文字のeはembeddedの略
				let eCatjson = await loadCatjson(eTx,network);//catjsonの更新
				let eLayout = await loadLayout(eTx,eCatjson,true); //isEmbedded:true
				let eBuiltTx = await parseTransaction(eTx,eLayout,eCatjson); //再帰
				items.push(eBuiltTx);
			}
			txLayer.layout = items;
			builtTx.push(txLayer);
			continue;

		}else if("layout" in catitem && layer.name in tx){ // else:byte,struct

			let txLayer = Object.assign({}, layer);
			let items = [];
			for(let item of tx[layer.name]){

				let itemBuiltTx = await parseTransaction(item,catjson.find(cj=>cj.name === layerType).layout,catjson,network); //再帰
				items.push(itemBuiltTx);
			}
			txLayer.layout = items;
			builtTx.push(txLayer);			  
			continue;

		}else if(catitem.type === "enum"){
			if(catitem.name.indexOf('Flags') != -1){

				let value = 0;
				for(let itemLayer of catitem.values){
					if(tx[layer.name].indexOf(itemLayer.name) != -1){
						value += itemLayer.value;
					}
				}
				catitem.value = value;
			}else{
			
				catitem.value = catitem.values.find(cvf=>cvf.name === tx[layer.name]).value;
			}
		}
		//layerの配置
		if(layerDisposition !== undefined && layerDisposition.indexOf('array') != -1){ // "array sized","array fill"

			let size = tx[layer.size];
			if(layerType === "byte"){

				if("element_disposition" in layer){ //message

					let subLayout = Object.assign({}, layer);
					let items = [];
					for(let count = 0; count < size; count++){
						let txLayer = {};
						txLayer.signedness = layer.element_disposition.signedness;
						txLayer.name = "element_disposition";
						txLayer.size = layer.element_disposition.size;
						txLayer.value = tx[layer.name].substr(count * 2, 2);
						txLayer.type = layerType;
						items.push([txLayer]);
					}
					subLayout.layout = items;
					builtTx.push(subLayout);

				}else{console.error("not yet");}
			}else{console.log("not yet");}
		}else{ //reserved またはそれ以外(定義なし)

			let txLayer = Object.assign({}, layer);
			if(Object.keys(catitem).length > 0){

				//catjsonのデータを使う
				txLayer.signedness	= catitem.signedness;
				txLayer.size  = catitem.size;
				txLayer.type  = catitem.type;
				txLayer.value = catitem.value;
			}

			//txに指定されている場合上書き(enumパラメータは上書きしない)
			if(layer.name in tx && catitem.type !== "enum"){
				txLayer.value = tx[layer.name]; 			   
			}else{
				/* そのままtxLayerを追加 */
				console.log(layer.name);
			}
			builtTx.push(txLayer);
		}
	}

	let layerSize = builtTx.find(lf=>lf.name === "size");
	if(layerSize !== undefined && "size" in layerSize){
		layerSize.value = countSize(builtTx);
	}

	console.log(builtTx);
	return builtTx;
}


function buildTransaction(parsedTx){

	let builtTx = Object.assign([], parsedTx);
	
	let layerPayloadSize = builtTx.find(lf=>lf.name === "payload_size");
	if(layerPayloadSize !== undefined && "size" in layerPayloadSize){
		layerPayloadSize.value = countSize(builtTx.find(lf=>lf.name === "transactions"));
	}

	//Merkle Hash Builder
	let layerTransactionsHash = builtTx.find(lf=>lf.name === "transactions_hash");
	if(layerTransactionsHash){

		let hashes = [];
		for(let eTx of builtTx.find(lf=>lf.name === "transactions").layout){
			hashes.push(sha3_256.create().update(buffer.Buffer.from(hexlifyTransaction(eTx),"hex")).digest());
		}

		let numRemainingHashes = hashes.length;
		while (1 < numRemainingHashes) {
			let i = 0;
			while (i < numRemainingHashes) {
				const hasher = sha3_256.create();
				hasher.update(hashes[i]);

				if (i + 1 < numRemainingHashes) {
					hasher.update(hashes[i + 1]);
				} else {
					// if there is an odd number of hashes, duplicate the last one
					hasher.update(hashes[i]);
					numRemainingHashes += 1;
				}
				hashes[Math.trunc(i / 2)] = hasher.digest();
				i += 2;
			}
			numRemainingHashes = Math.trunc(numRemainingHashes / 2);
		}
		layerTransactionsHash.value = buffer.Buffer.from(hashes[0]).toString("hex");
	}
	return builtTx;
}


function getVerifiableData(builtTx){
	let typeLayer = builtTx.find(bf=>bf.name==="type");
	if([16705,16961].includes(typeLayer.value)){
		return builtTx.slice(5,11);
	}else{
		return builtTx.slice(5,builtTx.length);
	}
}


function hashTransaction(signer,signature,builtTx,network){

	let hasher = sha3_256.create();
	hasher.update(buffer.Buffer.from(signature,"hex"));
	hasher.update(buffer.Buffer.from(signer,"hex"));
	hasher.update(buffer.Buffer.from(network.generationHash,"hex"));
	hasher.update(buffer.Buffer.from(hexlifyTransaction(getVerifiableData(builtTx)),"hex")); //verifiableData
	let txHash = hasher.hex();
	return txHash;
}

function updateTransaction(builtTx,name,type,value){
	let layer = builtTx.find(bf=>bf.name === name);
	layer[type] = value;
	console.log(layer);
	return builtTx;
}


function countSize(item,alignment){

	let totalSize = 0;
	
	//レイアウトサイズの取得
	if(item !== undefined && item.layout){
		for(let layer of item.layout){
			let itemAlignment;
			if("alignment" in item){
				itemAlignment = item.alignment;
			}else{
				itemAlignment = 0;
			}
			totalSize += countSize(layer,itemAlignment); //再帰
		}
	//レイアウトを構成するレイヤーサイズの取得
	}else if(Array.isArray(item)){
		let layoutSize = 0;
		for(let layout of item){
			layoutSize += countSize(layout,alignment);
		}		 
		if(alignment !== undefined && alignment > 0){
			layoutSize = Math.floor((layoutSize  + alignment - 1) / alignment ) * alignment;
		}
		totalSize += layoutSize;
	
	}else{
		if("size" in item){
			totalSize += item.size;
			console.log(item.name + ":" + item.size);
		}else{console.error("no size:" + item.name);}
	}
	console.log(totalSize);
	return totalSize;
}

//hex化
function hexlifyTransaction(item,alignment){

	let hex = "";
	if(item !== undefined && item.layout){
		for(let layer of item.layout){
			let itemAlignment;
			if("alignment" in item){
				itemAlignment = item.alignment;
			}else{
				itemAlignment = 0;
			}
			hex += hexlifyTransaction(layer,itemAlignment); //再帰
		}
	}else if(Array.isArray(item)){
		let subLayoutHex = "";
		for(let subLayout of item){
			//subLayoutSize += countSize(subLayout);
			subLayoutHex += hexlifyTransaction(subLayout,alignment);
			hexLength = subLayoutHex.length;
		}		 
		if(alignment !== undefined && alignment > 0){
			let alignedSize = Math.floor((subLayoutHex.length + (alignment * 2) - 2)/ (alignment * 2) ) * (alignment * 2);
			subLayoutHex = subLayoutHex + "0".repeat(alignedSize - hexLength);
		}
		hex += subLayoutHex;
	}else{
		let size = item.size;
		if(item.value === undefined){
			if(size >= 24){
				item.value = "00".repeat(size);
			}else{
				item.value = 0;
			}
		}

		if(size==1){
			if(item.name === "element_disposition"){
				hex = buffer.Buffer.from(item.value,'hex').toString("hex");
			}else{
				hex = buffer.Buffer.from(new Uint8Array([item.value]).buffer).toString("hex");
			}	 
		}else if(size==2){
			hex = buffer.Buffer.from(new Uint16Array([item.value]).buffer).toString("hex");
		}else if(size==4){
			hex = buffer.Buffer.from(new Uint32Array([item.value]).buffer).toString("hex");
		}else if(size==8){
			hex = buffer.Buffer.from(new BigInt64Array([item.value]).buffer).toString("hex");
		}else if(size==24 || size==32 || size==64){
			hex = buffer.Buffer.from(item.value,'hex').toString("hex");
		}else{
			console.error("unknown size order");
		}
	}
	console.log(hex);
	return hex;
}

//署名
function signTransaction(builtTx,priKey,network){
	let sig = nacl.sign.detached(
		new Uint8Array([
			...buffer.Buffer.from(network.generationHash,"hex"),
			...buffer.Buffer.from(hexlifyTransaction(getVerifiableData(builtTx)),"hex"),
		]) ,
		new Uint8Array([
			...buffer.Buffer.from(priKey,"hex"),
			...buffer.Buffer(nacl.sign.keyPair.fromSeed(
				new Uint8Array(buffer.Buffer.from(priKey,"hex"))
			).publicKey)
		])
	);
	let signature = buffer.Buffer(sig).toString("hex");
	console.log(signature);
	return signature; 
}

//連署
function cosignTransaction(txhash,priKey){

	let sig = nacl.sign.detached(
		new Uint8Array(buffer.Buffer.from(txhash,"hex")) ,
		new Uint8Array([
			...buffer.Buffer.from(priKey,"hex"),
			...buffer.Buffer(nacl.sign.keyPair.fromSeed(
				new Uint8Array(buffer.Buffer.from(priKey,"hex"))
			).publicKey)
		])
	);
	let signature = buffer.Buffer(sig).toString("hex");
	return signature; 
}

charMapping = {
	createBuilder: () => {
		const map = {};
		return {
			map,
			addRange: (start, end, base) => {
				const startCode = start.charCodeAt(0);
				const endCode = end.charCodeAt(0);

				for (let code = startCode; code <= endCode; ++code)
					map[String.fromCharCode(code)] = code - startCode + base;
			}
		};
	}
};

//https://github.com/symbol/symbol/blob/dev/sdk/javascript/src/utils/base32.js
DECODED_BLOCK_SIZE = 5;
ENCODED_BLOCK_SIZE = 8;

Char_To_Decoded_Char_Map = (() => {
	const builder = charMapping.createBuilder();
	builder.addRange('A', 'Z', 0);
	builder.addRange('2', '7', 26);
	return builder.map;
})();

decodeChar = c => {
	const decodedChar = Char_To_Decoded_Char_Map[c];
	if (undefined !== decodedChar)
		return decodedChar;

	throw Error(`illegal base32 character ${c}`);
};

decodeBlock = (input, inputOffset, output, outputOffset) => {
	const bytes = new Uint8Array(ENCODED_BLOCK_SIZE);
	for (let i = 0; i < ENCODED_BLOCK_SIZE; ++i)
		bytes[i] = decodeChar(input[inputOffset + i]);

	output[outputOffset + 0] = (bytes[0] << 3) | (bytes[1] >> 2);
	output[outputOffset + 1] = ((bytes[1] & 0x03) << 6) | (bytes[2] << 1) | (bytes[3] >> 4);
	output[outputOffset + 2] = ((bytes[3] & 0x0F) << 4) | (bytes[4] >> 1);
	output[outputOffset + 3] = ((bytes[4] & 0x01) << 7) | (bytes[5] << 2) | (bytes[6] >> 3);
	output[outputOffset + 4] = ((bytes[6] & 0x07) << 5) | bytes[7];
};

base32 = {

	decode: encoded => {
		if (0 !== encoded.length % ENCODED_BLOCK_SIZE)
			throw Error(`encoded size must be multiple of ${ENCODED_BLOCK_SIZE}`);

		const output = new Uint8Array(encoded.length / ENCODED_BLOCK_SIZE * DECODED_BLOCK_SIZE);
		for (let i = 0; i < encoded.length / ENCODED_BLOCK_SIZE; ++i)
			decodeBlock(encoded, i * ENCODED_BLOCK_SIZE, output, i * DECODED_BLOCK_SIZE);

		return output;
	}
};
