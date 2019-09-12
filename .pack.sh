#!/bin/sh

rm dist/*.wasm

cd crate
wasm-pack build

cd ../src/ts
tsc *.ts --outDir ..

cd ../..
npx webpack