#!/bin/bash

cargo build --profile release-wasm --target wasm32-unknown-unknown --features js
wasm-bindgen --out-dir ./target/web --target web ./target/wasm32-unknown-unknown/release-wasm/maze-3d.wasm
cp target/web/* site/
cp assets/* site/