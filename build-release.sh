#!/usr/bin/env bash

trap 'exit' ERR

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/webgl_gui_demo.wasm --out-dir html --no-modules --no-typescript

# A real application should run wasm-opt on the wasm file, and a minifier such as 'terser' on the JS.
