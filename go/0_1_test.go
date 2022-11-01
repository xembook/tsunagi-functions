package main

import "testing"

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"golang.org/x/exp/slices"	
)

func getNetworkInfo() map[string]any {
	return map[string]any{
		"version":1,
		"network":"TESTNET",
		"generationHash":"7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
		"epochAdjustment":1637848847,
		"catjasonBase":"https://xembook.github.io/tsunagi-sdk/catjson/",
	}
}

func getDeadline(network map[string]any) int {
	//	now := time.Now()
	now := network["epochAdjustment"].(int)
	deadline := ((now + 7200) - network["epochAdjustment"].(int)) * 1000
	return deadline
}
func getPayload(tx map[string]any,network map[string]any) string {
	catjson := loadCatjson(tx,network)
	layout := loadLayout(tx,catjson,false)
	preparedTx := prepareTransaction(tx,layout,network) //TX事前準備
	parsedTx := parseTransaction(preparedTx,layout,catjson,network)
	builtTx := buildTransaction(parsedTx) 
	signature := signTransaction(builtTx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",network)
	builtTx = updateTransaction(builtTx,"signature","value",signature)
	//	txHash := hashTransaction(tx1["signer_public_key"].(string),signature,builtTx,network);
	payload := hexlifyTransaction(builtTx,0)
	return payload
}

func TestTransferTransaction(t *testing.T) {

	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",
	}

	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21")
}

func TestTransferTransaction2(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : convertAddressAliasId(generateNamespaceId("xembook",0)),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  generateNamespaceId("tomato",generateNamespaceId("xembook",0)), "amount" : 1},
			map[string]any{"mosaic_id" :  generateNamespaceId("xym",generateNamespaceId("symbol",0)), "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"dc00000000000000a1bcb56de796c45cd982e79748772cd9a616a084c95fc775a1d003b9f5f2dcbffa95e869e8a2d77873bbe3d26d5c2764e8299bded689037e4ede6095008cc2075f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d00000000009985738c26eb1534a40000000000000000000000000000001c00020000000000eeaff441ba994be764000000000000003164838cd27f54fa01000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21")

}

//resolves opposite mosaice order
func TestTransferTransaction3(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"dc000000000000001e1a289eef4550fe482ff5a073ba9b91bf38e8623e8767eb54eae5fd48dba354f662dce635ad299efb050cbf187c6b52674613d7e81bb58a4a662d2528d491005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21")

}

//resolves 0 byte message
func TestTransferTransaction4(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"c100000000000000c086746240315084735ebee633ff541056c5ba0f17c4d924a4b59c9531aa72243eaa7b76e5e0a9e32a15fb475be49a2f1ff1e380c763bcb2ab3ef5d83125b40d5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80100020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a640000000000000000")

}

//resolves undefined message
func TestTransferTransaction5(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"c000000000000000fee4646022be8647455bc876a8f7f303233d297a5755cd1eb41999ae6c8cca2f0225e2b93c4aa793c68657c230578dc3af26c3ef32acae96ea1ae10c438278055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a6400000000000000")

}

//resolves null mosaic transfer
func TestTransferTransaction6(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{},
		"message":"Hello Tsunagi(Catjson) SDK!",		
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"bc00000000000000cd5b93e94f053a07a5a132d7f59708b6818d88840c150d6f6dc38a2ca2408fff0e7e3ee39599d1242a0e4a5869dec8a2847b05fb698fa39db2bf1c3bf46ce2005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21")

}

//resolves undefined message and null mosaic
func TestTransferTransaction7(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{},	
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"a0000000000000002c271a17d41832515a9ad0e995a524a4859a001436a990370c4b53eaa63677b4d69edde0831171a10defc157ea01f1d5528a562c423e38c725fc5b37af35ee055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b80000000000000000")

}

//resolves 0 byte message and null mosaic
func TestTransferTransaction8(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{},
		"message":"",			
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"a100000000000000786d46993afe584dd4e1fd2904d8eb0ea67e27ca3c7ef81fd208a6f27c1450807234093f9be03bbda0b02d96a69bd2766595ac4ab59fbc5119d247181b5596065f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441a86100000000000000dd6d0000000000989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b8010000000000000000")
}

////aggregate complete transaction

//resolves siimple complete
func TestAggregateCompleteTransaction1(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1},
	}

	payload := getPayload(aggTx,network)
	assert.Equal(t, payload,"380100000000000048b5dad7211f0ff1aee442484bac4def33fe600b37a52a39966e61ed93e54ddcb3517a60471ba4fb37660e5abf164c1ac364bdc485da5cad00cd1b7282145b085f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000a5b60f432c88daaf89d3154c5f1e6f7be3090c1af95ba0f21c308ecf119b222290000000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b2100000000")
}

//resolves 3 account transfer
func TestAggregateCompleteTransaction2(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	tx2 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"recipient_address" : generateAddressId("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Carol! This is Bob.",		
	}

	tx3 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
		"recipient_address" : generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Alice, This is Carol.",		
	}
	
	cosignature1 := map[string]any{
		"version":0,
		"signer_public_key":"6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"signature":"",
	}

	cosignature2 := map[string]any{
		"version":0,
		"signer_public_key":"886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
		"signature":"",
	}

	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2,tx3},
		"cosignatures": []any{cosignature1,cosignature2},
	}

	catjson := loadCatjson(aggTx,network)
	layout := loadLayout(aggTx,catjson,false)
	preparedTx := prepareTransaction(aggTx,layout,network) //TX事前準備
	parsedTx := parseTransaction(preparedTx,layout,catjson,network)
	builtTx := buildTransaction(parsedTx) 
	signature := signTransaction(builtTx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",network)
	builtTx = updateTransaction(builtTx,"signature","value",signature)
	txHash := hashTransaction(aggTx["signer_public_key"].(string),signature,builtTx,network);
	fmt.Println(txHash)

	preparedTx["cosignatures"].([]any)[0].(map[string]any)["signature"] = cosignTransaction(txHash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b")
	preparedTx["cosignatures"].([]any)[1].(map[string]any)["signature"] = cosignTransaction(txHash,"1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf")

	idx := slices.IndexFunc(layout, func(item any) bool {return item.(map[string]any)["name"] == "cosignatures"})
	if idx >= 0 {
		cosignaturesLayout := layout[idx].(map[string]any)
		parsedCosignatures := parseTransaction(preparedTx,[]any{cosignaturesLayout},catjson,network)
		builtTx = updateTransaction(builtTx,"cosignatures","layout",parsedCosignatures[0].(map[string]any)["layout"])
	}
	payload := hexlifyTransaction(builtTx,0);
	assert.Equal(t, payload,"28030000000000006f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afece4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a7080000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e4794f0617eae1f3f862c286c3e75494f0bb8009f8a8bccf8acb3ceb7719234f0282cdddab7bbc6adb8041788a8642729ec53ea8f6e107e8e2615ae592d44a60c")
}

//resolves opposite cosignature
func TestAggregateCompleteTransaction3(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	tx2 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"recipient_address" : generateAddressId("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Carol! This is Bob.",		
	}

	tx3 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
		"recipient_address" : generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Alice, This is Carol.",		
	}

	cosignature1 := map[string]any{
		"version":0,
		"signer_public_key":"6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"signature":"",
	}

	cosignature2 := map[string]any{
		"version":0,
		"signer_public_key":"886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
		"signature":"",
	}

	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2,tx3},
		"cosignatures": []any{cosignature2,cosignature1},
	}

	catjson := loadCatjson(aggTx,network)
	layout := loadLayout(aggTx,catjson,false)
	preparedTx := prepareTransaction(aggTx,layout,network) //TX事前準備
	parsedTx := parseTransaction(preparedTx,layout,catjson,network)
	builtTx := buildTransaction(parsedTx) 
	signature := signTransaction(builtTx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",network)
	builtTx = updateTransaction(builtTx,"signature","value",signature)
	txHash := hashTransaction(aggTx["signer_public_key"].(string),signature,builtTx,network);

	preparedTx["cosignatures"].([]any)[0].(map[string]any)["signature"] = cosignTransaction(txHash,"1e090b2a266877a9f88a510af2eb0945a63dc69dbce674ccd83272717d4175cf")
	preparedTx["cosignatures"].([]any)[1].(map[string]any)["signature"] = cosignTransaction(txHash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b")

	idx := slices.IndexFunc(layout, func(item any) bool {return item.(map[string]any)["name"] == "cosignatures"})
	if idx >= 0 {
		cosignaturesLayout := layout[idx].(map[string]any)
		parsedCosignatures := parseTransaction(preparedTx,[]any{cosignaturesLayout},catjson,network)
		builtTx = updateTransaction(builtTx,"cosignatures","layout",parsedCosignatures[0].(map[string]any)["layout"])
	}
	payload := hexlifyTransaction(builtTx,0);
	
	assert.Equal(t, payload,"28030000000000006f2651ea4046cbb9eca41fd2e38c4868915cae2ba4d77d00fc91eb5b3d0be60e243bb13248bb26b766ceecfc5f3452f6e25612160d476000694cfe39d867e60c5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e000000000000000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e4794f0617eae1f3f862c286c3e75494f0bb8009f8a8bccf8acb3ceb7719234f0282cdddab7bbc6adb8041788a8642729ec53ea8f6e107e8e2615ae592d44a60c00000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afece4b39b5be018de8141b3b0df3ceb358a197ff70b8be8da99fc9246dd979e6285e3547d01744df5a306150e51f49846bab0b2aecabb4d13ef1f3d49c08478a708")
}

//resolves no cosignature
func TestAggregateCompleteTransaction4(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	tx2 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Carol! This is Alice.",		
	}

	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2},
		"cosignatures": []any{},
	}

	payload := getPayload(aggTx,network)	
	assert.Equal(t, payload,"c80100000000000083de0648e05d23036b302e5249554f6fc164917021d4cf07f1d19dfefaea34bfb8679fde237115d5ac3885ef4d4d76c16d4a930429970edbc1fb32a967d0d5025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe3120010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000")
}

//resolves undefined cosignature
func TestAggregateCompleteTransaction5(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	tx2 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Carol! This is Alice.",		
	}

	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2},
	}

	payload := getPayload(aggTx,network)	
	assert.Equal(t, payload,"c80100000000000083de0648e05d23036b302e5249554f6fc164917021d4cf07f1d19dfefaea34bfb8679fde237115d5ac3885ef4d4d76c16d4a930429970edbc1fb32a967d0d5025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000000de2f57a150d1073330b9d3273c651b675ed9ce2f200cac1d29717dffe6fe3120010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320416c6963652e00000000")
}

////aggregate bonded transaction

//resolves hash lock
func TestAggregateBondedTransaction1(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "HASH_LOCK",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"mosaic" : []any{map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 10000000}},
		"duration": 480,
		"hash":"a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"b8000000000000008f0e4dc6dc42be7428219f820718d723803b0dde5455adec3f8ed1871318656ccd7fb4aab539ff722384b0cccf2d66603d5458a12ea01e12ffdd7bbbca9c5a0a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841a86100000000000000dd6d0000000000c8b6532ddb16843a8096980000000000e001000000000000a3ed27ee26592f6c501349a7de3427fc729e8d625ed214a6331c11b981f59f78")
}

//resolves hash lock by aggregate
func TestAggregateBondedTransaction2(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "HASH_LOCK",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"mosaic" : []any{map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 10000000}},
		"duration": 480,
		"hash":"4ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c",
	}

	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1},
	}

	payload := getPayload(aggTx,network)
	assert.Equal(t, payload,"1001000000000000c2941402f941376e3b58c9931c45cd768334fe6ac65e9b746fe484e8ec8067795f5ba4b895ff582395a5d74e8f79a861c6239495b3b38e6215d9d9eef699ac055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d000000000064b515ba0d874c0e0db27687514491d0bb74969cb82b767dde37a0b330a9f3ee680000000000000068000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984841c8b6532ddb16843a8096980000000000e0010000000000004ecd6d1830d46f21d03906885a25c30d6df48418746105201a77dad65287985c")
}

//resolves 3 account transfer
func TestAggregateBondedTransaction3(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	tx2 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"recipient_address" : generateAddressId("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Carol! This is Bob.",		
	}
	tx3 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
		"recipient_address" : generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Alice, This is Carol.",		
	}
	aggTx := map[string]any{
		"type" : "AGGREGATE_BONDED",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2,tx3},
	}

	payload := getPayload(aggTx,network)
	assert.Equal(t, payload,"5802000000000000ffd1ebcc029c4997d904586292aa1aab8c87e992cd736c074d639419aeae7adc82ce4782f1276f2504b0c4548777dd48616754205c7741af5b2a248f89b4c4035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414240420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e00000000")
}

//resolves 3 account transfer partial complete
func TestAggregateBondedTransaction4(t *testing.T) {
	
	network  := getNetworkInfo()
	deadline := getDeadline(network)

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"recipient_address" : generateAddressId("TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",		
	}

	tx2 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"recipient_address" : generateAddressId("TDZBCWHAVA62R4JFZJJUXQWXLIRTUK5KZHFR5AQ"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Carol! This is Bob.",		
	}
	
	tx3 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e",
		"recipient_address" : generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
		},
		"message" : "Hello Alice, This is Carol.",		
	}

	cosignature1 := map[string]any{
		"version":0,
		"signer_public_key":"6199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec",
		"signature":"",
	}	
	aggTx := map[string]any{
		"type" : "AGGREGATE_BONDED",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2,tx3},
		"cosignatures": []any{cosignature1},

	}

	catjson := loadCatjson(aggTx,network)
	layout := loadLayout(aggTx,catjson,false)
	preparedTx := prepareTransaction(aggTx,layout,network) //TX事前準備
	parsedTx := parseTransaction(preparedTx,layout,catjson,network)
	builtTx := buildTransaction(parsedTx) 
	signature := signTransaction(builtTx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",network)
	builtTx = updateTransaction(builtTx,"signature","value",signature)
	txHash := hashTransaction(aggTx["signer_public_key"].(string),signature,builtTx,network);
	fmt.Println(txHash)

	preparedTx["cosignatures"].([]any)[0].(map[string]any)["signature"] = cosignTransaction(txHash,"fa6373f4f497773c5cc55c103e348b139461d61fd4b45387e69d08a68000e06b")

	idx := slices.IndexFunc(layout, func(item any) bool {return item.(map[string]any)["name"] == "cosignatures"})
	if idx >= 0 {
		cosignaturesLayout := layout[idx].(map[string]any)
		parsedCosignatures := parseTransaction(preparedTx,[]any{cosignaturesLayout},catjson,network)
		builtTx = updateTransaction(builtTx,"cosignatures","layout",parsedCosignatures[0].(map[string]any)["layout"])
	}
	payload := hexlifyTransaction(builtTx,0);
	assert.Equal(t, payload,"c002000000000000ffd1ebcc029c4997d904586292aa1aab8c87e992cd736c074d639419aeae7adc82ce4782f1276f2504b0c4548777dd48616754205c7741af5b2a248f89b4c4035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414240420f000000000000dd6d0000000000ae67220a53b24a241f2da951ba6cfe044aedc42a0afb5f2742c5a454e3c9c6e1b0010000000000008c000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001985441989df3aea3852feafc5fdfc2266eb84ed8a7fa242688a8b81c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f205473756e616769284361746a736f6e292053444b21000000008a000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afec000000000198544198f21158e0a83da8f125ca534bc2d75a233a2baac9cb1e821a00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f204361726f6c21205468697320697320426f622e0000000000008c00000000000000886adfbd4213576d63ea7e7a4bece61c6933c27cd2ff36f85155c8febfb6eb4e00000000019854419869762418c5b643eee70e6f20d4d555d5997087d7a686a91c00020000000000c2347909f9b7092a0100000000000000c8b6532ddb16843a64000000000000000048656c6c6f20416c6963652c2054686973206973204361726f6c2e0000000000000000000000006199bae3b241df60418e258d046c22c8c1a5de2f4f325753554e7fd9c650afecbdde1c296d3e2e82bb1ae878586f832aa59080290b0f095f7e8d73921b6d2f67742e4588795f41b652500fdc1230ce4f45d2099fe37e182ebbe0f86121336e03")
}

////mosaic transaction

//resolves mosaic definition
func TestMosaicTransaction1(t *testing.T) {
	nonce := 1700836761
	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "MOSAIC_DEFINITION",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"duration": 0,
		"id":generateMosaicId(generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676,
		"nonce": nonce,
		"flags": "TRANSFERABLE RESTRICTABLE",
		"divisibility": 2,
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"96000000000000008400ea1dd86f206c946ae4aacfdd2d9997ceb406028e3d3e67e0b20a2a0dae696d9084f7f38f64c56450a3d6cd305722cb37d60b462358bbe23b5d8e155a3f0f5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d41a86100000000000000dd6d000000000076e65d50e5fbaf4d000000000000000099b560650602")
}

//resolves mosaic supply change
func TestMosaicTransaction2(t *testing.T) {
	nonce := 1700836761
	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "MOSAIC_SUPPLY_CHANGE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"mosaic_id":generateMosaicId(generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676,
		"delta": 1000 * 100, // assuming divisibility = 2
		"action": "INCREASE",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"9100000000000000cbec54081f0a62d5c5f84748df4668670fad447b53b44062e58d5cab054a2c8bcd8ab13533231825eda6156d4c73ba98978eccb011b0107f9bc9a0f071888e035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d42a86100000000000000dd6d000000000076e65d50e5fbaf4da08601000000000001")
}

//resolves aggregate mosaic definition and supply change
func TestMosaicTransaction3(t *testing.T) {
	nonce := 1700836761
	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "MOSAIC_DEFINITION",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"duration": 0,
		"id":generateMosaicId(generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676,
		"nonce": nonce,
		"flags": "TRANSFERABLE RESTRICTABLE",
		"divisibility": 2,
	}
	tx2 := map[string]any{
		"type" : "MOSAIC_SUPPLY_CHANGE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"mosaic_id":generateMosaicId(generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),nonce), //0x4DAFFBE5505DE676,
		"delta": 1000 * 100, // assuming divisibility = 2
		"action": "INCREASE",
	}
	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2},
	}
	payload := getPayload(aggTx,network)
	assert.Equal(t, payload,"3801000000000000c4b2ea423fd6eaa69407fb261cdb09b3d039923ad15a120ad1f1da61bcfd69db9b71cddc0bff730b3cd1b421b35f8cbc87a4765a204412b5efc34e221d50b20a5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000fc4405540b555f4dde5dc4ce67daeaf207e5485d8da24d5cfd6bf71fa064c9a5900000000000000046000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4176e65d50e5fbaf4d000000000000000099b560650602000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984d4276e65d50e5fbaf4da0860100000000000100000000000000")
}

////namespace transaction

//resolves root namespace regisration
func TestNamespaceTransaction1(t *testing.T) {

	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "NAMESPACE_REGISTRATION",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"duration": 86400,
		"registration_type": "ROOT",
		"name":"xembook",
		"id":generateNamespaceId("xembook",0), //0xA43415EB268C7385, //BigInt((new sym.NamespaceId("xembook")).id.toString())
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"99000000000000003983d675dd3affcab71fb09ee51cbddd4e8ee587335e030472dd50370333266ad571c4c94410262c7bb2ecc99b2b4b8eab71245046f41518d52ef6d5355792055f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d0000000000805101000000000085738c26eb1534a4000778656d626f6f6b")
}

//resolves sub namespace regisration
func TestNamespaceTransaction2(t *testing.T) {

	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "NAMESPACE_REGISTRATION",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"parent_id": generateNamespaceId("xembook",0), //0xA43415EB268C7385, //BigInt((new sym.NamespaceId("xembook")).id.toString())
		"registration_type": "CHILD",
		"name":"tomato",
		"id":generateNamespaceId("tomato",generateNamespaceId("xembook",0)), //0xFA547FD28C836431, //BigInt((new sym.NamespaceId("xembook.tomato")).id.toString())
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"9800000000000000942e0fe89a3471a075f2cbd06cc64d4d8af5cd8e58c437aa39fa05e47bb9230c9a259d9da70279c8656749792310585b138e19889b3b41e7e01c14a4cbea1b025f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41a86100000000000000dd6d000000000085738c26eb1534a43164838cd27f54fa0106746f6d61746f")
}

//resolves address alias
func TestNamespaceTransaction3(t *testing.T) {

	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "ADDRESS_ALIAS",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"namespace_id": generateNamespaceId("xembook",0), //0xA43415EB268C7385, //BigInt((new sym.NamespaceId("xembook")).id.toString())
		"address":generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		"alias_action":"LINK",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"a1000000000000008f61856b455c0a57db652844aa761281c019511d7b0cd0ae9b54e4b22585f36f012116860ce9f5300fe0e91521be74d8434032bf73e008b2f52d5f0744ef13045f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42a86100000000000000dd6d000000000085738c26eb1534a49869762418c5b643eee70e6f20d4d555d5997087d7a686a901")
}


//resolves mosaic alias
func TestNamespaceTransaction4(t *testing.T) {

	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "MOSAIC_ALIAS",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : deadline,
		"namespace_id":generateNamespaceId("tomato",generateNamespaceId("xembook",0)), //0xFA547FD28C836431, //BigInt((new sym.NamespaceId("xembook.tomato")).id.toString())
		"mosaic_id":0x4DAFFBE5505DE676,
		"alias_action":"LINK",
	}
	payload := getPayload(tx1,network)
	assert.Equal(t, payload,"910000000000000041f45e3bbbc8073c14b6e05b71fa9299692d1eafc86300b59698207ef044c7db8e346870c57df722497803549e2e3f8d5777c5e1c98fdf27a562d814076acc035f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43a86100000000000000dd6d00000000003164838cd27f54fa76e65d50e5fbaf4d01")
}

//resolves namespace by aggregate


func TestNamespaceTransaction5(t *testing.T) {
	network  := getNetworkInfo()
	deadline := getDeadline(network)
	tx1 := map[string]any{
		"type" : "NAMESPACE_REGISTRATION",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"duration": 86400,
		"registration_type": "ROOT",
		"name":"xembook1",
		"id":generateNamespaceId("xembook1",0), //0xA43415EB268C7385, //BigInt((new sym.NamespaceId("xembook")).id.toString())
	}
	tx2 := map[string]any{
		"type" : "NAMESPACE_REGISTRATION",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"parent_id": generateNamespaceId("xembook1",0), //0xA43415EB268C7385, //BigInt((new sym.NamespaceId("xembook")).id.toString())
		"registration_type": "CHILD",
		"name":"tomato1",
		"id":generateNamespaceId("tomato1",generateNamespaceId("xembook1",0)), //0xFA547FD28C836431, //BigInt((new sym.NamespaceId("xembook.tomato")).id.toString())
	}
	tx3 := map[string]any{
		"type" : "ADDRESS_ALIAS",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"namespace_id": generateNamespaceId("xembook1",0), //0xA43415EB268C7385, //BigInt((new sym.NamespaceId("xembook")).id.toString())
		"address":generateAddressId("TBUXMJAYYW3EH3XHBZXSBVGVKXKZS4EH26TINKI"),
		"alias_action":"LINK",
	}
	tx4 := map[string]any{
		"type" : "MOSAIC_ALIAS",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"namespace_id":generateNamespaceId("tomato1",generateNamespaceId("xembook1",0)), //0xFA547FD28C836431, //BigInt((new sym.NamespaceId("xembook.tomato")).id.toString())
		"mosaic_id":0x4DAFFBE5505DE676,
		"alias_action":"LINK",
	}
	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : deadline,
		"transactions": []any{tx1,tx2,tx3,tx4},
	}
	payload := getPayload(aggTx,network)
	assert.Equal(t, payload,"e801000000000000989f5f8a026d6e5c45301ce06af70406bd9c3694604a9e0718c3bac4dff9b95494d397210817139ebf43306a7bb43242e200afc4205b9a3cb439ffb1e2a14c015f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb000000000198414140420f000000000000dd6d0000000000de415866cedcab9dda7baa97b5bb326ad2647bfafe69d8b3587a789bff9d073c40010000000000004a000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e418051010000000000bd1cf9801594b9ed000878656d626f6f6b3100000000000049000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e41bd1cf9801594b9edf47e2f57b78ec1920107746f6d61746f310000000000000051000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e42bd1cf9801594b9ed9869762418c5b643eee70e6f20d4d555d5997087d7a686a9010000000000000041000000000000005f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb0000000001984e43f47e2f57b78ec19276e65d50e5fbaf4d0100000000000000")
}



func xTestB(t *testing.T) {

	network  := map[string]any{
		"version":1,
		"network":"TESTNET",
		"generationHash":"7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836",
		"epochAdjustment":1637848847,
		"catjasonBase":"https://xembook.github.io/tsunagi-sdk/catjson/",
    }
	_ = network

	tx1 := map[string]any{
		"type" : "TRANSFER",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 25000,
		"deadline" : "deadline",
		"recipient_address" : "TCO7HLVDQUX6V7C737BCM3VYJ3MKP6REE2EKROA",
		"mosaics" : []any{
			map[string]any{"mosaic_id" :  0x2A09B7F9097934C2, "amount" : 1},
			map[string]any{"mosaic_id" :  0x3A8416DB2D53B6C8, "amount" : 100},
		},
		"message" : "Hello Tsunagi(Catjson) SDK!",
	};
	_ = tx1



	aggTx := map[string]any{
		"type" : "AGGREGATE_COMPLETE",
		"signer_public_key" : "5f594dfc018578662e0b5a2f5f83ecfb1cda2b32e29ff1d9b2c5e7325c4cf7cb",
		"fee" : 1000000,
		"deadline" : "deadline",
		"transactions": []any{tx1},
	};
	_ = tx1


	catjson := loadCatjson(aggTx,network)
	layout := loadLayout(aggTx,catjson,false)
	preparedTx := prepareTransaction(aggTx,layout,network) //TX事前準備
	parsedTx := parseTransaction(preparedTx,layout,catjson,network)
	builtTx := buildTransaction(parsedTx) 
	signature := signTransaction(builtTx,"94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7",network)
	builtTx = updateTransaction(builtTx,"signature","value",signature)
	txHash := hashTransaction(aggTx["signer_public_key"].(string),signature,builtTx,network);
	payload := hexlifyTransaction(builtTx,0)
	_=signature
	_=builtTx
	_=preparedTx
	_= parsedTx
	_=txHash
	_=payload
	//    fmt.Println(payload)
		fmt.Println("txHash")
}
