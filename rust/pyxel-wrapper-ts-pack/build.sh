#!/bin/zsh
set -e
pushd .

cd ../  # rust/ へ移動

# 1. tsbind_types.json生成
cargo build -p pyxel-wrapper-ts --release --target wasm32-unknown-emscripten --target-dir target

# 2. bindgenで各種ファイル生成
cargo run -p pyxel-wrapper-ts-bindgen

cd pyxel-wrapper-ts-pack

# 3. Rust + TypeScript ファイルを一括コピー・変換

# *.rs のうち #[ts...] マクロを削除してコピー
for file in ../pyxel-wrapper-ts/src/*.rs(.); do
  name=$(basename "$file")
  sed '/#\[ts/d' "$file" > "src/$name"
  sed '/#\[ts/d;/use pyxel_wrapper_ts_macros::/d' "$file" > "src/$name"
done

# lib.rs に mod generated を追加
echo '#[path = "generated.rs"]
mod generated;' >> src/lib.rs

# bindgenで生成された各種ファイルをコピー
cp ../pyxel-wrapper-ts-bindgen/src/generated.rs src/
cp ../pyxel-wrapper-ts-bindgen/pkg/pyxel_wrapper_ts.d.ts ts/pkg/
cp ../pyxel-wrapper-ts-bindgen/pkg/pyxel.ts ts/pkg/
cp ../pyxel-wrapper-ts-bindgen/pkg/EXPORTED_FUNCTIONS.txt pkg/

# 4. Rust crateとしてビルド (.a生成)
cargo build --release --target wasm32-unknown-emscripten --target-dir target

# 5. emccで wasm + js 出力
EXPORTED_FUNCTIONS=$(cat pkg/EXPORTED_FUNCTIONS.txt)
emcc \
  target/wasm32-unknown-emscripten/release/libpyxel_wrapper_ts_pack.a \
  -O3 \
  --no-entry \
  -s FORCE_FILESYSTEM=1 \
  -s MODULARIZE=1 \
  -s EXPORT_ES6=1 \
  -s WASM=1 \
  -s WASM_BIGINT=1 \
  -s ENVIRONMENT=web \
  -s USE_SDL=2 \
  -s EXCEPTION_CATCHING_ALLOWED="['*']" \
  -s SUPPORT_LONGJMP=1 \
  -s INITIAL_MEMORY=64MB \
  -s MAXIMUM_MEMORY=512MB \
  -s ALLOW_MEMORY_GROWTH=1 \
  -s "EXPORTED_FUNCTIONS=$EXPORTED_FUNCTIONS" \
  -s EXPORTED_RUNTIME_METHODS="['ccall', 'cwrap', 'FS', 'UTF8ToString']" \
  -o ts/pkg/pyxel_wrapper_ts.js

popd

echo "✅ Build complete!"
