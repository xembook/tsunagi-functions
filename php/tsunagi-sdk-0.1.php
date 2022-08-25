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
	$factory = array_filter($catjson, function($item)use($conditions){
		return isset($item['factory_type']) && $item['factory_type'] == $conditions["prefix"] . "Transaction" && $item["name"] === $conditions["layoutName"];
	});

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

	if(isset($tx["transactions"])){
		$txes = [];
		foreach($tx["transactions"] as $e_tx){

			print_r($e_tx);

			$e_catjson = load_catjson($e_tx,$network);
			$e_layout = load_layout($e_tx,$e_catjson,true);
			//再帰処理
			$e_prepared_tx = prepare_transaction($e_tx,$e_layout,$network);
			array_push($txes,$e_prepared_tx);
		}
		$prepared_tx["transactions"] = $txes;
	}
	print_r($prepared_tx);
	return $prepared_tx;
}



function parse_transaction($tx,$layout,$catjson,$network) {

print_r("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
	$parsed_tx = []; //return
	foreach($layout as $layer){

		$layer_type = $layer["type"];
		$layer_disposition;
		if(isset($layer["disposition"])){
			$layer_disposition = $layer["disposition"];
		}
		print_r($layer_type . "\n");
		$catitem = array_filter($catjson, function($cj) use($layer_type){
			return $cj["name"] === $layer_type;
		});
//		$catitem = array_values($filter_item)[0];

		if(isset($layer["condition"])){
			if($layer["condition_operation"] === "equals"){
				if($layer["condition_value"] !== $tx[$layer["condition"]]){
					continue;
				}
			}
		}

		if($layer_disposition === "const"){
			continue;

		}else if($layer_type === "EmbeddedTransaction"){
			
			$tx_layer = $layer;
			$items = [];
			foreach($tx["transactions"] as $e_tx){ //小文字のeはembeddedの略
				$e_catjson = load_catjson($e_tx,$network);//catjsonの更新
				$e_layout = load_layout($e_tx,$e_catjson,true); //isEmbedded:true
				$e_parsed_tx = parse_transaction($e_tx,$e_layout,$e_catjson,$network); //再帰
				array_push($items,$e_parsed_tx);
			}
			$tx_layer["layout"] = $items;
			array_push($parsed_tx,$tx_layer);
			continue;

		}else if(isset($catitem["layout"]) && isset($tx[$layer["name"]]) ){ // else:byte,struct

			$tx_layer = $layer;
			$items = [];
			foreach($tx[$layer["name"]] as $item){

				$filter_layer = array_filter($catjson, function($cj) use($layer_type){
					return $cj["name"] === $layer_type;
				});

				$item_parsed_tx = parse_transaction($item,$filter_layer["layout"],$catjson,$network); //再帰
				array_push($items,$item_parsed_tx);
			}
			$tx_layer["layout"] = $items;
			array_push($parsed_tx,$tx_layer);
			continue;

		}else if($layer_type === "UnresolvedAddress"){
			//アドレスに30個の0が続く場合はネームスペースとみなします。
			if(strpos($tx[$layer["name"]],'000000000000000000000000000000') !== false){

				$network_type = array_filter($catjson, function($cj){
					return $cj["name"] === "NetworkType";
				});

				$network_value = array_filter($network_type["values"], function($cj) use($tx){
					return $cj["name"] === $tx["network"];
				})["value"];

				$prefix = bin2hex($network_value + 1);
				$tx[$layer["name"]] =  $prefix + $tx[$layer["name"]];
			}
		}else if(isset($catitem["type"]) && $catitem["type"] === "enum"){

			if(strpos($catitem["name"],'Flags') !== false){

				$value = 0;
				foreach($catitem["values"] as $item_layer){


					if(strpos($tx[$layer["name"]],$item_layer["name"]) !== false){

						$value += $item_layer["value"];
					}
				}
				$catitem["value"] = $value;
			
			}else if(isset($layer_disposition) &&  strpos($layer_disposition,'array') !== false ){
				$values = [];
				foreach($tx[$layer["name"]] as $item){

					$filter_value = array_filter($catitem["values"], function($cj) use($item){
						return $cj["name"] === $item;
					})["value"];

					array_push($values,$filter_value);
				}
				$tx[$layer["name"]] = $values;
			}else{
			
				$conditions = ["tx" => $tx,"layer_name" => $layer["name"] ];
				$filter_value = array_filter($catitem["values"], function($cj) use($conditions){
					return $cj["name"] === $conditions["tx"][$conditions["layer_name"]];
				})["value"];

				$catitem["value"] = $filter_value;
			}
		}

		//サブルーチンにまとめる構想
		//$parsed_tx = build_layer($layer_disposition,$tx,$layer);

		//layerの配置
		if(isset($layer_disposition) && strpos($layer_disposition,'array') !== false ){

			$size = 0;
			if(isset($tx[$layer["size"]])){
				$size = $tx[$layer["size"]];
			}

			if($layer_type === "byte"){

				if(isset($layer["element_disposition"])){ //message

					$sub_layout = $layer;
					$items = [];
					for($count = 0; $count < $size; $count++){
						$tx_layer = [];
						$tx_layer["signedness"] = $layer["element_disposition"]["signedness"];
						$tx_layer["name"] = "element_disposition";
						$tx_layer["size"] = $layer["element_disposition"]["size"];
						$tx_layer["value"] = substr($tx[$layer["name"]],$count * 2, 2);
						$tx_layer["type"] = $layer_type;
//						array_push($items,[$tx_layer]);
						array_push($items,$tx_layer);
					}
					$sub_layout["layout"] = $items;
					array_push($parsed_tx, $sub_layout);

				}else{print_r("not yet");}
			}else if(isset($tx[$layer["name"]])){

				$sub_layout = $layer;
				$items = [];
				foreach($tx[$layer["name"]] as $tx_item){


					$tx_layer = array_filter($catjson, function($cj) use($layer_type){
						return $cj["name"] === $layer_type;
					});

					$tx_layer["value"] = $tx_item;
					
					if($layer_type === "UnresolvedAddress"){
						//アドレスに30個の0が続く場合はネームスペースとみなします。
						if(strpos($txItem,'000000000000000000000000000000') !== false){

							$network_type = array_filter($catjson, function($cj){
								return $cj["name"] === "NetworkType";
							});

							$network_value = array_filter($network_type["values"], function($cj) use($tx){
								return $cj["name"] === $tx["network"];
							})["value"];

							$prefix = bin2hex($network_value + 1);
							$tx_layer["value"] =  $prefix + $tx_layer["value"];
						}
					}			
//					array_push($items,[$tx_layer]);
					array_push($items,$tx_layer);
				}
				$sub_layout["layout"] = $items;
				array_push($parsed_tx,$sub_layout);

			}// else{console.log("not yet");}
		}else{ //reserved またはそれ以外(定義なし)

			$tx_layer = $layer;
			if(count($catitem) > 0){

				//catjsonのデータを使う
				if(isset($catitem["signedness"])){
				$tx_layer["signedness"]	= $catitem["signedness"];

				}
				if(isset($catitem["size"])){
				$tx_layer["size"]  = $catitem["size"];

				}
				if(isset($catitem["type"])){
				$tx_layer["type"]  = $catitem["type"];

				}
				if(isset($catitem["value"])){
				$tx_layer["value"] = $catitem["value"];

				}
			}

			//txに指定されている場合上書き(enumパラメータは上書きしない)
			if(isset($layer["name"]) && isset($catitem["type"]) && $catitem["type"] !== "enum"){
				$tx_layer["value"] = $tx[$layer["name"]];
			}else{
				// そのままtxLayerを追加 
				print_r($layer["name"]);
			}
			array_push($parsed_tx,$tx_layer);
		}
	}

	$layer_size = array_filter($parsed_tx, function($pf){
		return $pf["name"] === "size";
	});


	if(isset($layer_size) && isset($layer_size["size"])){
		$layer_size["value"] = count_size($parsed_tx);
	}

	print_r($parsed_tx);
	return $parsed_tx;
}

function count_size($item,$alignment) {

	$total_size = 0;
	
	//レイアウトサイズの取得
	if(isset($item)  && isset($item["layout"])){
		foreach( $item["layout"] as $layer){
			$item_alignment;
			if(isset($item["alignment"])){
				$item_alignment = $item["alignment"];
			}else{
				$item_alignment = 0;
			}
			$total_size += count_size($layer,$item_alignment); //再帰
		}
	//レイアウトを構成するレイヤーサイズの取得
	}else if(is_array($item)){
		$layout_size = 0;
		foreach($item as $layout){
			$layout_size += count_size($layout,$alignment);
		}		 
		if(isset($alignment)  && $alignment > 0){
			$layout_size = floor(($layout_size  + $alignment - 1) / $alignment ) * $alignment;
		}
		$total_size += $layout_size;
	
	}else{
		if(isset($item["size"])){
			$total_size += $item["size"];
			//console.log(item.name + ":" + item.size);
		}else{print_r("no size:" + $item["name"]);}
	}
	print_r($total_size);
	return $total_size;


//    return 0;
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


