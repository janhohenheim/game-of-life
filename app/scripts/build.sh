#!/usr/bin/env bash
application_name=game
target_dir=wasm_generated
rust_crates=(./app/client/game)

yarn

for index in ${!rust_crates[*]}
do
    crate=${rust_crates[$index]}
    cd $crate
    cargo +nightly build --target wasm32-unknown-unknown
    rm -rf $target_dir
    mkdir $target_dir
    wasm-bindgen target/wasm32-unknown-unknown/debug/$application_name.wasm --out-dir ./$target_dir
    cd -
done

yarn webpack
