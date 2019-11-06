build: fmt wasm ts

run:
    python3 x.py

wasm: fmt
    rm dist/*.wasm || echo 0
    cd crate; wasm-pack build; cd ..
    npx webpack

ts:
    cd src/ts; tsc *.ts --outDir ..; cd ../..
    npx webpack

fmt:
    cargo fmt

doc:
    cd crate; cargo doc --open --document-private-items