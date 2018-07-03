#!/usr/bin/env bash
application_name=game_of_life
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir .
rm -rf wasm
mkdir wasm
mv ${application_name}_bg.wasm $application_name.js $application_name.d.ts wasm/
