#!/bin/bash

set -ex

rust_variant=$1
os=$2
echo "running ch.sh with rust_variant=$rust_variant, os=$os"

mkdir -p $HOME/cached-deps/bin

if [ ! -f $HOME/cached-deps/bin/capnp ] ; then
    pushd ~
        curl -O https://capnproto.org/capnproto-c++-0.6.1.tar.gz
        tar zxf capnproto-c++-0.6.1.tar.gz
        pushd capnproto-c++-0.6.1
            ./configure --prefix=$HOME/cached-deps
            make -j6 check
            sudo make install
        popd
    popd
fi

PATH="${HOME}/.local/bin:${HOME}/cached-deps/bin:$PATH"

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
