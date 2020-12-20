#!/bin/bash

set -ex

rust_variant=$1
os=$2
echo "running ch.sh with rust_variant=$rust_variant, os=$os"

mkdir -p $HOME/cached-deps/bin

if [ ! -f $HOME/cached-deps/bin/capnp ] ; then
    curl -O https://capnproto.org/capnproto-c++-0.6.1.tar.gz
    tar zxf capnproto-c++-0.6.1.tar.gz
    cd capnproto-c++-0.6.1
    ./configure --prefix=$HOME/cached-deps
    make -j6 check
    sudo make install
fi

PATH="${HOME}/.local/bin:${HOME}/cached-deps/bin:$PATH"

make test

if [ "$os" == "linux" ]; then
    if [ "$rust_variant" == "stable" ]; then
        cargo clippy
        cargo fmt -- --check
    else
        (cd lib && cargo check --features=bench-suite)
        (cd lib/fuzz && cargo check)
        (cd bin && cargo check --features=bench-suite)
    fi
fi
