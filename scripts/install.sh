#!/bin/bash

set -ex

mkdir -p $HOME/cached-deps/bin

if [ ! -f $HOME/cached-deps/bin/capnp ] ; then
    curl -O https://capnproto.org/capnproto-c++-0.6.1.tar.gz
    tar zxf capnproto-c++-0.6.1.tar.gz
    cd capnproto-c++-0.6.1
    ./configure --prefix=$HOME/cached-deps
    make -j6 check
    sudo make install
fi

if [ ! -f $HOME/cached-deps/bin/grcov ] ; then
    curl -L https://github.com/mozilla/grcov/releases/latest/download/grcov-linux-x86_64.tar.bz2 | tar jxf -
    mv grcov $HOME/cached-deps/bin/grcov
fi

if [ "$TRAVIS_OS_NAME" == "linux" ] && [ "$TRAVIS_RUST_VERSION" == "stable" ]; then
    rustup component add rustfmt
    rustup component add clippy
fi

ls -l $HOME/cached-deps/bin
source ~/.cargo/env || true
