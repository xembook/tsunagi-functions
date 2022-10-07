package main

import "testing"

import (
	"fmt"
//	"net/http"
//    "io/ioutil"
)

/*
import "encoding/json"
//import 	"golang.org/x/crypto/sha3"
*/
func TestA(t *testing.T) {
    var nums []int = []int{1, 2, 3, 4, 5}
    if !(Summarize(nums) == 15) {
        t.Error(`miss`)
    }

	map1 := map[string]any{
        "name": "hawksnowlog",
        "age":  20,
    }
	_ = map1
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


	catjson := loadCatjson(tx1,network)
	layout := loadLayout(tx1,catjson,false)
	preparedTx := prepareTransaction(tx1,layout,network); //TX事前準備

	_=catjson
	_ = layout
    fmt.Println(layout)

/*
    req, _ := http.NewRequest(http.MethodGet, "https://xembook.github.io/tsunagi-sdk/catjson/transfer.json", nil)

    // リクエストの送信
    resp, _ := http.DefaultClient.Do(req)

    // ステータスコード
//    resp.StatusCode 
    // ヘッダーを取得
//    resp.Header.Get("Content-Type")
    // resp.Bodyの大きさ len(body) と同じ
//    resp.ContentLength
    // リクエストURL
//    resp.Request.URL.String()

    // レスポンスボディをすべて読み出す
    body, _ := ioutil.ReadAll(resp.Body)

var result []interface{}
json.Unmarshal([]byte(body), &result)



//fish := result["fishes"].(map[string]interface{})

for key := range result {

	
	fmt.Println(result[key])
//  fmt.Println(key, value.(string))
}


    // body は []byte
    fmt.Println(result[0])
*/
}

