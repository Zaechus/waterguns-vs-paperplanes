build:
    cargo fmt
    rm dist/*.wasm || echo 0
    cd crate; wasm-pack build; cd ..
    cd src/ts; tsc *.ts --outDir ..; cd ../..
    npx webpack

run:
    python3 x.py

wasm:
    rm dist/*.wasm || echo 0
    cd crate; wasm-pack build; cd ..
    npx webpack

ts:
    cd src/ts; tsc *.ts --outDir ..; cd ../..
    npx webpack