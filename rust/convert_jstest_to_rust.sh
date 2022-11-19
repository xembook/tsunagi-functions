#! /bin/bash

TARGET="src/lib.rs"

sed -ie "s/'/\"/g" $TARGET
sed -ie "s/\([0-9]\)n/\1u64/g" $TARGET
sed -ie "s/).toEqual(/, /g" $TARGET
sed -ie "s/expect/assert_eq!/g" $TARGET
sed -ie "s/parsedTx[0-9]*/parsed_tx/g" $TARGET
sed -ie "s/aggTx/agg_tx/g" $TARGET