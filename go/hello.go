package main

import (
	"fmt"
	"time"
	"bytes"
	"net/http"
	"crypto/ed25519"
	"encoding/hex"
	"encoding/json"
	"encoding/base32"
	"encoding/binary"
)
import 	"golang.org/x/crypto/sha3"

func main() {

	alicePrivateKey, _ := hex.DecodeString("94ee0f4d7fe388ac4b04a6a6ae2ba969617879b83616e4d25710d688a89d80c7")
	aliceKeypair := ed25519.NewKeyFromSeed(alicePrivateKey)
	var alice interface{} = aliceKeypair.Public()
	var alicePulicKey ed25519.PublicKey = alice.(ed25519.PublicKey)
	fmt.Println(hex.EncodeToString(alicePulicKey))

	version := make([]byte, 1)
	binary.PutUvarint(version, uint64(1))

	networkType := make([]byte, 2)
	binary.PutUvarint(networkType, uint64(152))

	transactionType := make([]byte, 2)
	binary.LittleEndian.PutUint16(transactionType, uint16(16724))

	fee := make([]byte, 8)
	binary.LittleEndian.PutUint64(fee, uint64(1000000))

	dt := time.Now()
	secondLater7200 := ((dt.Unix() + 7200) - 1637848847) * 1000
	deadline := make([]byte, 8)
	binary.LittleEndian.PutUint64(deadline, uint64(secondLater7200))

	recipientAddress, _ := base32.StdEncoding.DecodeString("TBIL6D6RURP45YQRWV6Q7YVWIIPLQGLZQFHWFEQ" + "A")

	mosaicCount := make([]byte, 1)
	binary.PutUvarint(mosaicCount, uint64(1))

	mosaicId := make([]byte, 8)
	binary.LittleEndian.PutUint64(mosaicId, uint64(0x3A8416DB2D53B6C8))

	mosaicAmount := make([]byte, 8)
	binary.LittleEndian.PutUint64(mosaicAmount, uint64(100))

	message := []byte("Hello Golang! Welcome to Symbol world!")
	
	messageSize := make([]byte, 2)
	binary.LittleEndian.PutUint16(messageSize, uint16(len(message) + 1))

	verifiableBody := hex.EncodeToString(version) +
		hex.EncodeToString(networkType[:1]) +
		hex.EncodeToString(transactionType) + 
		hex.EncodeToString(fee) + 
		hex.EncodeToString(deadline) +
		hex.EncodeToString(recipientAddress[:len(recipientAddress) - 1]) + 
		hex.EncodeToString(messageSize) + 
		hex.EncodeToString(mosaicCount) + 
		"00" + "00000000" + 
		hex.EncodeToString(mosaicId) + 
		hex.EncodeToString(mosaicAmount) +
		"00" + hex.EncodeToString(message) 

	verifiableString := "7fccd304802016bebbcd342a332f91ff1f3bb5e902988b352697be245f48e836" +
		verifiableBody

	verifiableBuffer, _ := hex.DecodeString(verifiableString)
	signature := ed25519.Sign(aliceKeypair,verifiableBuffer)

	transactionSize := make([]byte, 4)
	binary.LittleEndian.PutUint32(transactionSize, uint32(len(verifiableBody)/2 + 108))

	payloadString := hex.EncodeToString(transactionSize) +
		"00000000" +
		hex.EncodeToString(signature) +
		hex.EncodeToString(alicePulicKey) +
		"00000000" +
		verifiableBody

	payload := map[string]interface{}{
		"payload":payloadString,
	}

	client := &http.Client{}
	json, _ := json.Marshal(payload)
	req , _ := http.NewRequest(http.MethodPut, "https://sym-test-02.opening-line.jp:3001/transactions", bytes.NewBuffer(json))
	req.Header.Set("Content-Type", "application/json; charset=utf-8")
	resp, _ := client.Do(req)
	fmt.Println(resp.StatusCode)

	hashableBuffer, _ := hex.DecodeString(
		hex.EncodeToString(signature) +
		hex.EncodeToString(alicePulicKey) +
		verifiableString,
	)
	transactionHash := sha3.Sum256(hashableBuffer)

	fmt.Printf("transactionStatus: https://sym-test-02.opening-line.jp:3001/transactionStatus/%s\n", fmt.Sprintf("%x", transactionHash))
	fmt.Printf("confirmed: https://sym-test-02.opening-line.jp:3001/transactions/confirmed/%s\n", fmt.Sprintf("%x", transactionHash))
	fmt.Printf("explorer: https://testnet.symbol.fyi/transactions/%s\n", fmt.Sprintf("%x", transactionHash))
}
