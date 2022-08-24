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

//	var_dump($catjson);
	$factory = array_filter($catjson, function($item,$key)use($conditions){
		var_dump(isset($item['factory_type']) && $item['factory_type'] == $conditions["prefix"] . "Transaction" && $item["name"] === $conditions["layoutName"]);
	  return isset($item['factory_type']) && $item['factory_type'] == $conditions["prefix"] . "Transaction" && $item["name"] === $conditions["layoutName"];
	}, ARRAY_FILTER_USE_BOTH);

//	var_dump($conditions["prefix"] . "Transaction");
//	var_dump($conditions["layoutName"]);
	var_dump("========================================");
	print_r(array_values($factory));
	

	return array_values($factory)[0]["layout"];
}

function to_camel_case($str) {
//    return lcfirst(strtr(ucwords(strtr($str, ['_' => ' '])), [' ' => '']));

	return str_replace(' ', '', ucwords(str_replace('_', ' ', $str)));

}

function prepare_transaction() {
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


