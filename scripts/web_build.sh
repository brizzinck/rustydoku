cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web \
    --out-dir ./web-build/ \
    --out-name "rustydoku" \
    ./target/wasm32-unknown-unknown/release/rustydoku.wasm

cp -r ./assets ./web-build/

cp ./web-deploy/index.html ./web-build/

cd web-build

zip -r ../rustydoku.zip .
