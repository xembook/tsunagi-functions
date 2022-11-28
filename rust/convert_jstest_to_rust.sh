#! /bin/bash

TARGET="tests/test.rs"

sed -ie "s/'/\"/g" $TARGET
sed -ie "s/\([0-9]\)n/\1u64/g" $TARGET
sed -ie "s/).toEqual(/, /g" $TARGET
sed -ie "s/expect/assert_eq!/g" $TARGET
sed -ie "s/parsedTx[0-9]*/parsed_tx/g" $TARGET
sed -ie "s/aggTx/agg_tx/g" $TARGET
sed -ie "s/this.deadline/get_deadline(\&network)/g" $TARGET
sed -ie "s/builtTx/built_tx/g" $TARGET
sed -ie "s/let tx1 = {/let tx1 = object!{/g" $TARGET
sed -ie "s/let tx2 = {/let tx2 = object!{/g" $TARGET
sed -ie "s/let tx3 = {/let tx3 = object!{/g" $TARGET
sed -ie "s/let agg_tx = {/let agg_tx = object!{/g" $TARGET
sed -ie "s/verifiableData/verifiable_data/g" $TARGET