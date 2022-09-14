require "ed25519"
require 'digest'
require 'sha3'
require "base32"
require 'json'
require "net/http"

signing_key = Ed25519::SigningKey.generate
verify_key = signing_key.verify_key

private_key = signing_key.to_bytes
public_key = verify_key.to_bytes

puts private_key.unpack('H*')
puts public_key.unpack('H*')


#アカウント復元
alice_priavte_key = Ed25519::SigningKey.new(
	"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7" \
	.scan(/../).map{ |b| b.to_i(16) }.pack('C*')
)
alice_public_key = alice_priavte_key.verify_key
puts alice_public_key.to_bytes.unpack('H*')

public_key_hash = SHA3::Digest.hexdigest(:sha256, alice_public_key.to_bytes)
address_body = Digest::RMD160.hexdigest public_key_hash.scan(/../).map{ |b| b.to_i(16) }.pack('C*')
sum_hash = SHA3::Digest.hexdigest(:sha256, ("98" + address_body).scan(/../).map{ |b| b.to_i(16) }.pack('C*'))
alice_address = Base32.encode(("98" + address_body + sum_hash.slice(0..5)).scan(/../).map{ |b| b.to_i(16) }.pack('C*')) 
alice_address = alice_address.slice(0..alice_address.length-2)
puts alice_address


version = [1].pack("C").unpack('H*')[0]
network_type = [152].pack("C").unpack('H*')[0]
transaction_type = [16724].pack("v").unpack('H*')[0]
fee = [1000000].pack("Q").unpack('H*')[0]
second_later_7200 = ((Time.now.to_i + 7200) - 1637848847) * 1000;
deadline = [second_later_7200].pack("Q").unpack('H*')[0]
mosaic_count = [1].pack("C").unpack('H*')[0]
mosaic_id = [0x3A8416DB2D53B6C8].pack("Q").unpack('H*')[0]
recipient_address = Base32.decode("TBIL6D6RURP45YQRWV6Q7YVWIIPLQGLZQFHWFEQ").unpack('H*')[0]
mosaic_amount = [100].pack("Q").unpack('H*')[0]
message = ('Hello Ruby! Welcome to Symbol world!').unpack('H*')[0]
message_size = [message.length / 2 + 1].pack("v").unpack('H*')[0]

verifiable_body = version \
	+ network_type \
	+ transaction_type \
	+ fee \
	+ deadline \
	+ recipient_address \
	+ message_size \
	+ mosaic_count \
	+ "00" + "00000000" \
	+ mosaic_id \
	+ mosaic_amount \
	+ "00" + message


verifiable_string = "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836" \
		+ verifiable_body;

verifiable_buffer = verifiable_string.scan(/../).map{ |b| b.to_i(16) }.pack('C*')
signature = alice_priavte_key.sign(verifiable_buffer).unpack('H*')[0]
transaction_size = [verifiable_body.length / 2 + 108].pack("L").unpack('H*')[0]

payload_string = transaction_size \
	+ "00000000" \
	+ signature \
	+ alice_public_key.to_bytes.unpack('H*')[0] \
	+ "00000000" \
	+ verifiable_body

puts payload_string

payload = { "payload" => payload_string}
uri = URI.parse('https://sym-test-02.opening-line.jp:3001/transactions')
req = Net::HTTP::Put.new(uri.request_uri)
req["Content-Type"] = "application/json" 
req.body = payload.to_json

http = Net::HTTP.new(uri.host, uri.port)
http.use_ssl = true
response = http.request(req)
puts response

hashable_string = signature \
	+ alice_public_key.to_bytes.unpack('H*')[0] \
	+ verifiable_string

puts hashable_string
transactionHash  = SHA3::Digest.hexdigest(:sha256, hashable_string.scan(/../).map{ |b| b.to_i(16) }.pack('C*'))

puts "transactionStatus: https://sym-test-02.opening-line.jp:3001/transactionStatus/" + transactionHash;
puts "confirmed: https://sym-test-02.opening-line.jp:3001/transactions/confirmed/" + transactionHash;
puts "explorer: https://testnet.symbol.fyi/transactions/" +  transactionHash;

