require "ed25519"
require 'digest'
require 'sha3'
require "base32"
require 'json'
require "net/http"
require 'rubygems'
require 'active_record'

def load_catjson(tx,network) 

	if tx["type"] === "AGGREGATE_COMPLETE" || tx["type"] === "AGGREGATE_BONDED" then
		json_file =  "aggregate.json"
	else
		json_file =  tx["type"].downcase + ".json"
	end

	uri = URI.parse(network["catjasonBase"] + json_file)
	json = Net::HTTP.get(uri)
	result = JSON.parse(json)
	
	return result

end

def load_layout(tx,catjson,is_embedded) 

	if is_embedded then
		prefix = "Embedded";
	else
		prefix = "";
	end

	if    tx["type"] === "AGGREGATE_COMPLETE" then 
		layout_name = "AggregateCompleteTransaction"
	elsif tx["type"] === "AGGREGATE_BONDED" then 
		layout_name = "AggregateBondedTransaction"
	else
		puts "aaaa".camelize

		layout_name = prefix + tx["type"].downcase.camelize + "Transaction"
		puts layout_name
	end


	factory = catjson.find{|item| item['factory_type'] == prefix + "Transaction" && item["name"]  == layout_name}
	puts factory["layout"]

=begin
	conditions = {"prefix" => prefix,"layoutName" => $layoutName];
	$factory = array_filter($catjson, function($item)use($conditions){
		return isset($item['factory_type']) && $item['factory_type'] == $conditions["prefix"] . "Transaction" && $item["name"] === $conditions["layoutName"];
	});

	return array_values($factory)[0]["layout"];
=end


	return 0
end

def to_camel_case(str) 
	return 0
end

def prepare_transaction(tx,layout,network) 
	return 0
end

def parse_transaction(tx,layout,catjson,network) 
	return 0
end

def count_size(item,alignment = 0) 
	return 0
end

def build_transaction(parsed_tx) 
	return 0
end

def hexlify_transaction(item,alignment = 0) 
	return 0
end

def sign_transaction(built_tx,private_key,network) 
	return 0
end

def get_verifiable_data(built_tx) 
	return 0
end

def hash_transaction(signer,signature,built_tx,network) 
	return 0
end

def update_transaction(built_tx,name,type,value) 
	return 0
end

def cosign_transaction(tx_hash,private_key) 
end

def generate_address_id(address) 
	return 0
end

def generate_namespace_id(name, parent_namespace_id = 0)
	return 0
end

def generate_key(name)
	return 0
end

def generate_mosaic_id(owner_address, nonce)
	return 0
end

def convert_address_alias_id(namespace_id)
	return 0
end

def digest_to_bigint(digest)
	return 0
end
