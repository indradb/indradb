#!/bin/bash

set -ex

rust_variant=$1
os=$2

apt-get install capnproto

make test

if [ "$os" == "linux-latest" ]; then
    if [ "$rust_variant" == "stable" ]; then
        cargo clippy
        cargo fmt -- --check
    else
        pushd lib
            cargo check --features=bench-suite
            pushd fuzz
                cargo check
            popd
        popd
        pushd bin
            cargo check --features=bench-suite
        popd
    fi
fi
