#!/bin/bash

set -ex

rust_variant=$1
os=$2

if [ "$os" == "ubuntu-latest" ]; then
    eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
fi

brew install protobuf
make test

if [ "$os" == "ubuntu-latest" ]; then
    if [ "$rust_variant" == "stable" ]; then
        cargo clippy
        cargo fmt -- --check
    else
        cargo check --all-features
        # These are here because the above `cargo check` doesn't cover them
        (cd lib/fuzz && cargo check)
        (cd proto && cargo bench --no-run --features=bench-suite)
    fi
fi
