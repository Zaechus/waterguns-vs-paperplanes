build: fmt wasm_plain ts_plain webpack

run:
    python3 x.py

wasm: fmt wasm_plain webpack

ts: ts_plain webpack

c:
    cargo clippy
    cd crate; cargo clippy

doc:
    cd crate; cargo doc --open --document-private-items

webpack:
    npx webpack

fmt:
    cargo fmt

wasm_plain:
    rm dist/*.wasm || echo 0
    cd crate; wasm-pack build; cd ..

ts_plain:
    cd src/ts; tsc *.ts --outDir ..; cd ../..
