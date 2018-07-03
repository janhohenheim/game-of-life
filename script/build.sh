#!/usr/bin/env bash
application_name=game_of_life
target_dir=wasm_generated
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir .
rm -rf $target_dir
mkdir $target_dir
mv ${application_name}_bg.wasm $application_name.js $application_name.d.ts $target_dir/

parcel html/index.html
