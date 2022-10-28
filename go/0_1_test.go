package main

import "testing"

import (
//	"fmt"
	"github.com/stretchr/testify/assert"
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



func TestB(t *testing.T) {

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
//		fmt.Println("txHash")
}
