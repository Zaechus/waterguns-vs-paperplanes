build: fmt wasm_plain ts_plain webpack

serve:
    python3 x.py

wasm: fmt wasm_plain webpack

ts: ts_plain webpack

c: fmt
    cargo clippy
    cd crate; cargo clippy

doc: fmt
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
