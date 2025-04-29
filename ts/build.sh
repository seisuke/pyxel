#!/bin/zsh
set -e
pushd .

cd ../rust

cargo build -p pyxel-wrapper-ts --release --target wasm32-unknown-emscripten --target-dir target
cargo run -p pyxel-wrapper-ts-bindgen

cd pyxel-wrapper-ts

emcc \
    target/wasm32-unknown-emscripten/release/libpyxel_wrapper_ts.a \
    -O0 \
    --no-entry \
    -s WASM=1 \
    -s STANDALONE_WASM \
    -s ERROR_ON_UNDEFINED_SYMBOLS=0 \
    -s EXPORTED_FUNCTIONS=$(cat pkg/EXPORTED_FUNCTIONS.txt) \
    -s EXPORTED_RUNTIME_METHODS="['ccall', 'cwrap']" \
    -o pkg/pyxel_wrapper_ts.js

cp pkg/pyxel_wrapper_ts.* ../../ts/pkg/

popd

echo "✅ Build complete!"
