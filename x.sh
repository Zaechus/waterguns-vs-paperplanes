#!/bin/sh

cd waterguns-vs-paperplanes
wasm-pack build

cd ..
npm install
npx webpack
cargo run
