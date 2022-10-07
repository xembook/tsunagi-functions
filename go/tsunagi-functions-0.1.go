package main

import (
	"fmt"
	"strings"
	"net/http"
    "io/ioutil"
	"encoding/json"
	"encoding/hex"
	"golang.org/x/exp/slices"
    "golang.org/x/text/cases"
    "golang.org/x/text/language"

)
func Summarize(nums []int) int {
    var total int
    for _, num := range nums {
        total += num
    }
    return total
}


func loadCatjson( tx map[string]any, network map[string]any) []any{

	var jsonFile string
	if(tx["type"] == "AGGREGATE_COMPLETE" || tx["type"] == "AGGREGATE_BONDED"){
		jsonFile =  "aggregate.json"
	}else{
		jsonFile =  strings.ToLower(tx["type"].(string)) + ".json"
	}

	req, _ := http.NewRequest(http.MethodGet, "https://xembook.github.io/tsunagi-sdk/catjson/" + jsonFile, nil)
	resp, _ := http.DefaultClient.Do(req)
	body, _ := ioutil.ReadAll(resp.Body)

	var result []interface{}
	json.Unmarshal([]byte(body), &result)

	fmt.Println(result)
	return result
}

func loadLayout(tx map[string]any,catjson  []any,isEmbedded bool) any{
	
	var prefix string
	if(isEmbedded){
		prefix = "Embedded"
	}else{
		prefix = ""
	}


	var layoutName string
	if(      tx["type"].(string) == "AGGREGATE_COMPLETE"){ layoutName = "AggregateCompleteTransaction"
	}else if(tx["type"].(string) == "AGGREGATE_BONDED"){   layoutName = "AggregateBondedTransaction"
	}else{
		layoutName = prefix + toCamelCase(tx["type"].(string)) + "Transaction"
	}
	
	idx := slices.IndexFunc(catjson, func(item any) bool {return item.(map[string]any)["factory_type"] == prefix + "Transaction" && item.(map[string]any)["name"] == layoutName})
	return catjson[idx].(map[string]any)
}

func toCamelCase(str string) string {
	res := strings.Replace(cases.Title(language.Und).String(strings.Replace(str, "_", " ", -1))," ","",-1)
	return res;
}



func prepareTransaction(tx map[string]any,layout any,network map[string]any) map[string]any{

	preparedTx := make(map[string]any)
	for k, v := range tx {
		preparedTx[k] = v
	}

	preparedTx["network"] = network["network"];
	preparedTx["version"] = network["version"];

    if val, ok := preparedTx["message"]; ok {
        fmt.Printf("foo exists. The value is %#v", val)
		message := []byte("Hello Golang! Welcome to Symbol world!")
		preparedTx["message"] = "00" + hex.EncodeToString(message) 
    }


/*
	var preparedTx = {}..addAll(tx);

	preparedTx["network"] = network["network"];
	preparedTx["version"] = network["version"];

	if(preparedTx.containsKey("message")){
		preparedTx["message"] = "00" + hex.encode(utf8.encode(tx["message"]));
	}

	if(preparedTx.containsKey("name")){
		preparedTx["name"] = hex.encode(utf8.encode(tx["name"]));
	}

	if(preparedTx.containsKey("value")){
		preparedTx["value"] = hex.encode(utf8.encode(tx["value"]));
	}

	if(tx.containsKey("mosaics")){

		var castMosaics = tx["mosaics"] as List<dynamic>;
		preparedTx["mosaics"] = castMosaics..sort((a,b) => a["mosaic_id"].compareTo(b["mosaic_id"]));

//		print(preparedTx["mosaics"]);

	}
	//レイアウト層ごとの処理
	for (var layer in layout) {

//		print(layer["size"]);

		//size定義の調査
		if(layer.containsKey("size") && int.tryParse(layer["size"].toString()) == null){

			var size = 0;

			//element_dispositionが定義されている場合は、TX内の実データをそのサイズ数で分割する。
			if(layer.containsKey("element_disposition") && preparedTx.containsKey(layer["name"])){

				size = (preparedTx[layer["name"]].length / (layer["element_disposition"]["size"] * 2)).toInt();

			}else if(layer["size"].contains('_count')){//暫定 sizeにcountという文字列が含まれている場合はサイズ値を指定する項目が含まれると考える
				
				if(preparedTx.containsKey(layer["name"])){
					size = preparedTx[layer["name"]].length;
				}else{
					size = 0;
				}

			}else{
				//その他のsize値はPayloadの長さを入れるため現時点では不明
			}
			preparedTx[layer["size"]] = size;
		}

	}


	if(tx.containsKey('transactions')){
		var txes = [];
		for(var eTx in tx["transactions"]){

			var eCatjson = await loadCatjson(eTx,network);
			var eLayout = loadLayout(eTx,eCatjson,true);
			//再帰処理
			var ePreparedTx = await prepareTransaction(eTx,eLayout,network);
			txes.add(ePreparedTx);
		}
		preparedTx["transactions"] = txes;
	}
*/
	return preparedTx;

}