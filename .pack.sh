#!/bin/sh

rm dist/*.wasm

cd crate
wasm-pack build

cd ../src
tsc *.ts

cd ..
npx webpack
cp static/* dist/