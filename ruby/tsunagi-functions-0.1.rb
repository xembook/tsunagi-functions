

def load_catjson(tx,network) 
	return 0
end

def load_layout(tx,catjson,is_embedded) 
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
