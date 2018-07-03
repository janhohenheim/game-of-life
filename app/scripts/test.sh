#!/usr/bin/env bash
set -e

rust_crates=(./app/client/game)

for index in ${!rust_crates[*]}
do
    crate=${rust_crates[$index]}
    cd $crate
    cargo +nightly test
    cd -
done
