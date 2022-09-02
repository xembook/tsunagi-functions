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

	if('message' in preparedTx){
		preparedTx.message = buffer.Buffer.from([0,...(new TextEncoder('utf-8')).encode(tx.message)]).toString("hex");
	}

	if('name' in preparedTx){
		preparedTx.name = buffer.Buffer.from((new TextEncoder('utf-8')).encode(tx.name)).toString("hex");
	}

	if('value' in preparedTx){
		preparedTx.value = buffer.Buffer.from((new TextEncoder('utf-8')).encode(tx.value)).toString("hex");
	}

	if("mosaics" in tx){
		preparedTx.mosaics = tx.mosaics.sort(function(a,b){
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

			}else if(layer.size.indexOf('_count') != -1){//暫定 sizeにcountという文字列が含まれている場合はサイズ値を指定する項目が含まれると考える
				
				if(layer.name in preparedTx){
					size = preparedTx[layer.name].length;
				}else{
					size = 0;
				}


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

	let parsedTx = []; //return
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
				let eParsedTx = await parseTransaction(eTx,eLayout,eCatjson,network); //再帰
				items.push(eParsedTx);
			}
			txLayer.layout = items;
			parsedTx.push(txLayer);
			continue;

		}else if("layout" in catitem && layer.name in tx){ // else:byte,struct

			let txLayer = Object.assign({}, layer);
			let items = [];
			for(let item of tx[layer.name]){

				let itemParsedTx = await parseTransaction(item,catjson.find(cj=>cj.name === layerType).layout,catjson,network); //再帰
				items.push(itemParsedTx);
			}
			txLayer.layout = items;
			parsedTx.push(txLayer);
			continue;

		}else if(layerType === "UnresolvedAddress"){
			//アドレスに30個の0が続く場合はネームスペースとみなします。
			if(tx[layer.name] !== undefined  && tx[layer.name].indexOf("000000000000000000000000000000")>=0){
				let prefix = (catjson.find(cf=>cf.name==="NetworkType").values.find(vf=>vf.name===tx.network).value + 1).toString(16);
				tx[layer.name] =  prefix + tx[layer.name];
			}
		}else if(catitem.type === "enum"){
			if(catitem.name.indexOf('Flags') != -1){

				let value = 0;
				for(let itemLayer of catitem.values){
					if(tx[layer.name].indexOf(itemLayer.name) != -1){
						value += itemLayer.value;
					}
				}
				catitem.value = value;
			}else if(layerDisposition !== undefined && layerDisposition.indexOf('array') != -1){
				values = [];
				for(let item of  tx[layer.name]){
					values.push(catitem.values.find(cvf=>cvf.name === item).value);
				}
				tx[layer.name] = values;
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
					parsedTx.push(subLayout);

				}else{console.error("not yet");}
			}else if(layer.name in tx){

				let subLayout = Object.assign({}, layer);
				let items = [];
				for(let txItem of tx[layer.name]){
					let txLayer = Object.assign({}, catjson.find(cj=>cj.name === layerType));
					txLayer.value = txItem;
					
					if(layerType === "UnresolvedAddress"){
						//アドレスに30個の0が続く場合はネームスペースとみなします。
						if(txItem.indexOf("000000000000000000000000000000") >= 0){
							let prefix = (catjson.find(cf=>cf.name==="NetworkType").values.find(vf=>vf.name===tx.network).value + 1).toString(16);
							txLayer.value =  prefix + txLayer.value;
						}
					}			
					items.push([txLayer]);
				}
				subLayout.layout = items;
				parsedTx.push(subLayout);


			}// else{console.log("not yet");}
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
			parsedTx.push(txLayer);
		}
	}

	let layerSize = parsedTx.find(lf=>lf.name === "size");
	if(layerSize !== undefined && "size" in layerSize){
		layerSize.value = countSize(parsedTx);
	}

	console.log(parsedTx);
	return parsedTx;
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

	let updatedTx = Object.assign([], builtTx);

	let layer = updatedTx.find(bf=>bf.name === name);
	layer[type] = value;
	console.log(layer);
	return updatedTx;
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


//ネームスペースを16進数のIDにコンバート
const convertAddressAliasId = namespaceId => {
	
	return buffer.Buffer.from(new BigInt64Array([namespaceId]).buffer).toString("hex") + "000000000000000000000000000000";
};


//BASE32アドレスを16進数のIDにデコード
const generateAddressId = address => {
	return buffer.Buffer(base32.decode(address + "A").slice(0, -1)).toString("hex");
};

const generateKey = (name) => {
	const hasher = sha3_256.create();
	hasher.update(name);
	const digest = new Uint8Array(hasher.digest());
	const result = digestToBigInt(digest);
	return result | NAMESPACE_FLAG;
};

//https://github.com/symbol/symbol/blob/dev/sdk/javascript/src/utils/charMapping.js
const charMapping = {
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
const ALPHABET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
const DECODED_BLOCK_SIZE = 5;
const ENCODED_BLOCK_SIZE = 8;

// region encode

const encodeBlock = (input, inputOffset, output, outputOffset) => {
	output[outputOffset + 0] = ALPHABET[input[inputOffset + 0] >> 3];
	output[outputOffset + 1] = ALPHABET[((input[inputOffset + 0] & 0x07) << 2) | (input[inputOffset + 1] >> 6)];
	output[outputOffset + 2] = ALPHABET[(input[inputOffset + 1] & 0x3E) >> 1];
	output[outputOffset + 3] = ALPHABET[((input[inputOffset + 1] & 0x01) << 4) | (input[inputOffset + 2] >> 4)];
	output[outputOffset + 4] = ALPHABET[((input[inputOffset + 2] & 0x0F) << 1) | (input[inputOffset + 3] >> 7)];
	output[outputOffset + 5] = ALPHABET[(input[inputOffset + 3] & 0x7F) >> 2];
	output[outputOffset + 6] = ALPHABET[((input[inputOffset + 3] & 0x03) << 3) | (input[inputOffset + 4] >> 5)];
	output[outputOffset + 7] = ALPHABET[input[inputOffset + 4] & 0x1F];
};

// endregion

// region decode

const Char_To_Decoded_Char_Map = (() => {
	const builder = charMapping.createBuilder();
	builder.addRange('A', 'Z', 0);
	builder.addRange('2', '7', 26);
	return builder.map;
})();

const decodeChar = c => {
	const decodedChar = Char_To_Decoded_Char_Map[c];
	if (undefined !== decodedChar)
		return decodedChar;

	throw Error(`illegal base32 character ${c}`);
};

const decodeBlock = (input, inputOffset, output, outputOffset) => {
	const bytes = new Uint8Array(ENCODED_BLOCK_SIZE);
	for (let i = 0; i < ENCODED_BLOCK_SIZE; ++i)
		bytes[i] = decodeChar(input[inputOffset + i]);

	output[outputOffset + 0] = (bytes[0] << 3) | (bytes[1] >> 2);
	output[outputOffset + 1] = ((bytes[1] & 0x03) << 6) | (bytes[2] << 1) | (bytes[3] >> 4);
	output[outputOffset + 2] = ((bytes[3] & 0x0F) << 4) | (bytes[4] >> 1);
	output[outputOffset + 3] = ((bytes[4] & 0x01) << 7) | (bytes[5] << 2) | (bytes[6] >> 3);
	output[outputOffset + 4] = ((bytes[6] & 0x07) << 5) | bytes[7];
};

// endregion

const base32 = {
	/**
	 * Base32 encodes a binary buffer.
	 * @param {Uint8Array} data Binary data to encode.
	 * @returns {string} Base32 encoded string corresponding to the input data.
	 */
	encode: data => {
		if (0 !== data.length % DECODED_BLOCK_SIZE)
			throw Error(`decoded size must be multiple of ${DECODED_BLOCK_SIZE}`);

		const output = new Array(data.length / DECODED_BLOCK_SIZE * ENCODED_BLOCK_SIZE);
		for (let i = 0; i < data.length / DECODED_BLOCK_SIZE; ++i)
			encodeBlock(data, i * DECODED_BLOCK_SIZE, output, i * ENCODED_BLOCK_SIZE);

		return output.join('');
	},

	/**
	 * Base32 decodes a base32 encoded string.
	 * @param {string} encoded Base32 encoded string to decode.
	 * @returns {Uint8Array} Binary data corresponding to the input string.
	 */
	decode: encoded => {
		if (0 !== encoded.length % ENCODED_BLOCK_SIZE)
			throw Error(`encoded size must be multiple of ${ENCODED_BLOCK_SIZE}`);

		const output = new Uint8Array(encoded.length / ENCODED_BLOCK_SIZE * DECODED_BLOCK_SIZE);
		for (let i = 0; i < encoded.length / ENCODED_BLOCK_SIZE; ++i)
			decodeBlock(encoded, i * ENCODED_BLOCK_SIZE, output, i * DECODED_BLOCK_SIZE);

		return output;
	}
};

//https://github.com/symbol/symbol/blob/dev/sdk/javascript/src/symbol/idGenerator.js
const NAMESPACE_FLAG = 1n << 63n;

const uint32ToBytes = value => new Uint8Array([
	value & 0xFF,
	(value >> 8) & 0xFF,
	(value >> 16) & 0xFF,
	(value >> 24) & 0xFF
]);

const digestToBigInt = digest => {
	let result = 0n;
	for (let i = 0; 8 > i; ++i)
		result += (BigInt(digest[i]) << BigInt(8 * i));

	return result;
};

/**
 * Generates a mosaic id from an owner address and a nonce.
 * @param {Address} ownerAddress Owner address.
 * @param {number} nonce Nonce.
 * @returns {BigInt} Computed mosaic id.
 */
const generateMosaicId = (ownerAddress, nonce) => {
	const hasher = sha3_256.create();
	hasher.update(uint32ToBytes(nonce));
	hasher.update(buffer.Buffer(ownerAddress,"hex"));
	const digest = hasher.digest();

	let result = digestToBigInt(digest);
	if (result & NAMESPACE_FLAG)
		result -= NAMESPACE_FLAG;

	return result;
};

/**
 * Generates a namespace id from a name and an optional parent namespace id.
 * @param {string} name Namespace name.
 * @param {BigInt} parentNamespaceId Parent namespace id.
 * @returns {BigInt} Computed namespace id.
 */
const generateNamespaceId = (name, parentNamespaceId = 0n) => {
	const hasher = sha3_256.create();
	hasher.update(uint32ToBytes(Number(parentNamespaceId & 0xFFFFFFFFn)));
	hasher.update(uint32ToBytes(Number((parentNamespaceId >> 32n) & 0xFFFFFFFFn)));
	hasher.update(name);
	const digest = new Uint8Array(hasher.digest());

	const result = digestToBigInt(digest);
	return result | NAMESPACE_FLAG;
};

/**
 * Returns true if a name is a valid namespace name.
 * @param {string} name Namespace name to check.
 * @returns {boolean} true if the specified name is valid.
 */
const isValidNamespaceName = name => {
	const isAlphanum = character => ('a' <= character && 'z' >= character) || ('0' <= character && '9' >= character);
	if (!name || !isAlphanum(name[0]))
		return false;

	for (let i = 0; i < name.length; ++i) {
		const ch = name[i];
		if (!isAlphanum(ch) && '_' !== ch && '-' !== ch)
			return false;
	}

	return true;
};

/**
 * Parses a fully qualified namespace name into a path.
 * @param {string} fullyQualifiedName Fully qualified namespace name.
 * @returns {array<BigInt>} Computed namespace path.
 */
const generateNamespacePath = fullyQualifiedName => {
	const path = [];
	let parentNamespaceId = 0n;
	fullyQualifiedName.split('.').forEach(name => {
		if (!isValidNamespaceName(name))
			throw Error(`fully qualified name is invalid due to invalid part name (${fullyQualifiedName})`);

		path.push(generateNamespaceId(name, parentNamespaceId));
		parentNamespaceId = path[path.length - 1];
	});

	return path;
};

/**
 * Generates a mosaic id from a fully qualified mosaic alias name.
 * @param {string} fullyQualifiedName Fully qualified mosaic name.
 * @returns {BigInt} Computed mosaic id.
 */
const generateMosaicAliasId = fullyQualifiedName => {
	const path = generateNamespacePath(fullyQualifiedName);
	return path[path.length - 1];
};


