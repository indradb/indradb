#!/bin/bash

set -ex

rust_variant=$1
os=$2

# this works on linux too
brew install capnp

if [ "$os" == "ubuntu-latest" ]; then
    if [ "$rust_variant" == "stable" ]; then
        cargo clippy
        cargo fmt -- --check
    else
        cargo check --all-features
        (cd lib/fuzz && cargo check)
    fi
fi
