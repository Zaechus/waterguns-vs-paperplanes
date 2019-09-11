#!/bin/sh

cd crate
wasm-pack build

cd ..
npm install
npx webpack
cp static/* dist/
cargo run
