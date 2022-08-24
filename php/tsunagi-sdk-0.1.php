<?php
function restaurant_check($meal, $tax, $tip) {
    $tax_amount = $meal * ($tax / 100);
    $tip_amount = $meal * ($tip / 100);
    $total_amount = $meal + $tax_amount + $tip_amount;
    return $total_amount;
}



function load_catjson($tx,$network) {

	$jsonFile;
	if($tx["type"] === "AGGREGATE_COMPLETE" || $tx["type"] === "AGGREGATE_BONDED"){
		$jsonFile =  "aggregate.json";
	}else{
		$jsonFile =  strtolower($tx["type"]) . ".json";
	}

	$res = file_get_contents($network["catjasonBase"] . $jsonFile);
	$catjson = json_decode($res,true);
	
	return $catjson;
}

function load_layout($tx,$catjson,$is_embedded) {

	$prefix;
	if($is_embedded){
		$prefix = "Embedded";
	}else{
		$prefix = "";
	}

	$layoutName;
	if(      $tx["type"] === "AGGREGATE_COMPLETE"){ $layoutName = "AggregateCompleteTransaction";
	}else if($tx["type"] === "AGGREGATE_BONDED"){   $layoutName = "AggregateBondedTransaction";
	}else{
		$layoutName = $prefix . to_camel_case(strtolower($tx["type"])) . "Transaction";
	}

	$conditions = ["prefix" => $prefix,"layoutName" => $layoutName];
	$factory = array_filter($catjson, function($item,$key)use($conditions){
		var_dump(isset($item['factory_type']) && $item['factory_type'] == $conditions["prefix"] . "Transaction" && $item["name"] === $conditions["layoutName"]);
	  return isset($item['factory_type']) && $item['factory_type'] == $conditions["prefix"] . "Transaction" && $item["name"] === $conditions["layoutName"];
	}, ARRAY_FILTER_USE_BOTH);

	return array_values($factory)[0]["layout"];
}

function to_camel_case($str) {

	return str_replace(' ', '', ucwords(str_replace('_', ' ', $str)));

}

function prepare_transaction($tx,$layout,$network) {

	$prepared_tx = $tx;
	$prepared_tx["network"] = $network["network"];
	$prepared_tx["version"] = $network["version"];

	if(isset($prepared_tx['message'])){
		$prepared_tx['message'] = "00" . bin2hex($tx['message']);
	}

	if(isset($prepared_tx['name'])){
		$prepared_tx['name'] = bin2hex($tx['name']);
	}

	if(isset($prepared_tx['value'])){
		$prepared_tx['value'] = bin2hex($tx['value']);
	}

	if(isset($tx['mosaics'])){
		$ids = array_column($prepared_tx['mosaics'], 'mosaic_id');
		array_multisort($ids, SORT_ASC, $prepared_tx['mosaics']);
	}

	foreach($layout as $layer){
		print_r($layer);
		if(isset($layer["size"]) && !is_numeric($layer["size"])){
			$size = 0;

			if(isset($layer["element_disposition"])  && isset($prepared_tx[$layer["name"]])){
				$size = strlen($prepared_tx[$layer["name"]]) / ($layer["element_disposition"]["size"] * 2);

			}else if(strpos($layer["size"],'_count') !== false){//暫定 sizeにcountという文字列が含まれている場合はサイズ値を指定する項目が含まれると考える
				
				if(isset($prepared_tx[$layer["name"]])){

					$size = count($prepared_tx[ $layer["name"] ]);
				}else{
					$size = 0;
				}


			}else{
				//その他のsize値はPayloadの長さを入れるため現時点では不明
			}
			$prepared_tx[$layer["size"]] = $size;


		}
	}

/*
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
*/



    return 0;
}



function parse_transaction() {
    return 0;
}

function build_transaction() {
    return 0;
}

function get_verifiable_data() {
    return 0;
}


function hash_transaction() {
    return 0;
}

function update_transaction() {
    return 0;
}

function count_size() {
    return 0;
}

function hexlify_transaction() {
    return 0;
}

function sign_transaction() {
    return 0;
}

function cosign_transaction() {
    return 0;
}

function generate_address_alias_id() {
    return 0;
}

function generate_address_id() {
    return 0;
}



?>


