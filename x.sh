#!/bin/sh

cd crate
wasm-pack build

cd ..
npm install

cd src
tsc *.ts

cd ..
npx webpack
cp static/* dist/
cargo run
