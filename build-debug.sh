#!/usr/bin/env bash

trap 'exit' ERR

cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/webgl_gui_demo.wasm --out-dir html/generated --no-modules --no-typescript --debug
