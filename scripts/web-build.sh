#!/bin/bash

# This script builds rustydoku for WebAssembly (WASM), generates the necessary JS glue code using wasm-bindgen,
# copies assets and an index.html file to the output directory, and then packages everything into a zip file.
# The resulting zip file can be deployed to a web server or a platform like itch.io.

# Step 1: Build the project for the WASM target in release mode.
# This compiles your Rust code to WebAssembly.
cargo build --release --target wasm32-unknown-unknown

# Step 2: Run wasm-bindgen to generate JavaScript glue code.
# --no-typescript: Do not generate TypeScript declaration files.
# --target web: Generate output suitable for running in the browser.
# --out-dir ./web-build/: Specify the output directory for the generated files.
# --out-name "rustydoku": Base name for the generated .wasm and JS files.
wasm-bindgen --no-typescript --target web \
    --out-dir ./web-build/ \
    --out-name "rustydoku" \
    ./target/wasm32-unknown-unknown/release/rustydoku.wasm

# Step 3: Copy the assets directory into the web-build folder.
# This ensures that all static resources are available for the web version.
cp -r ./assets ./web-build/

# Step 4: Copy the index.html file from the web-deploy folder to the web-build folder.
# This index.html serves as the entry point for your web application.
cp ./web-deploy/index.html ./web-build/

# Step 5: Change directory to web-build.
cd web-build

# Step 6: Package the contents of web-build into a zip file.
# The zip file (rustydoku.zip) can then be uploaded to a web server or deployed to itch.io.
zip -r ../rustydoku.zip .
