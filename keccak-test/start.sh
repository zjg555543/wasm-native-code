set -x

cargo wasm

cargo run-script optimize


res=$(exchaincli tx wasm store /Users/oker/workspace/github/wasm-native/wasm-native-code/keccak-test/artifacts/wasm_counter.wasm --fees 0.01okt --from captain --gas=2000000 -b block -y)
code_id=$(echo "$res" | jq '.logs[0].events[1].attributes[0].value' | sed 's/\"//g')
res=$(exchaincli tx wasm instantiate "$code_id" "{}" --label test1 --admin "0xbbE4733d85bc2b90682147779DA49caB38C0aA1F" --fees 0.001okt --from captain -b block -y)
raw_log=$(echo "$res" | jq -r '.raw_log')


contractAddr_0x=$(echo "$res" | jq '.logs[0].events[0].attributes[0].value' | sed 's/\"//g')
res=$(exchaincli tx wasm execute "$contractAddr_0x" '{"test":{"vm":true, "count": "1"}}' --fees 0.001okt --from captain -b block -y)
res=$(exchaincli tx wasm execute "$contractAddr_0x" '{"test":{"vm":false, "count": "1"}}' --fees 0.001okt --from captain -b block -y)