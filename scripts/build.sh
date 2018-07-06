#!/usr/bin/env bash
set -e

application_name=game_of_life
target_dir=wasm_generated

yarn

cargo +nightly build --target wasm32-unknown-unknown
rm -rf $target_dir
mkdir $target_dir
wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir ./$target_dir

yarn webpack
