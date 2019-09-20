build:
    rm dist/*.wasm
    cd crate; wasm-pack build; cd ..
    cd src/ts; tsc *.ts --outDir ..; cd ../..
    npx webpack

wasm:
    cd crate; wasm-pack build; cd ..
    npx webpack

ts:
    cd src/ts; tsc *.ts --outDir ..; cd ../..
    npx webpack