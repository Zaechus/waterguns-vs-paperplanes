#!/bin/sh

rm dist/*.wasm

cd crate
wasm-pack build

cd ..
npm install

cd src/ts
tsc *.ts --outDir ..

cd ../..
npx webpack
cargo run
