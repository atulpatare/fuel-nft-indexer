#!/bin/bash

wasm-snip target/wasm32-unknown-unknown/release/fuel_nft_indexer.wasm \
  -o target/wasm32-unknown-unknown/release/fuel_nft_indexer.wasm \
  -p __wbindgen
