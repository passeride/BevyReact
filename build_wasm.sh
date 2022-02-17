#! /usr/bin/bash
# Created by Lukas H Langøy
# 2022-02-11

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/BevyTest.wasm
