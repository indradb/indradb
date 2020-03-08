#!/bin/bash

set -ex

mkdir -p $HOME/cached-deps

if [ ! -f $HOME/cached-deps/bin/capnp ] ; then
    curl -O https://capnproto.org/capnproto-c++-0.6.1.tar.gz
    tar zxf capnproto-c++-0.6.1.tar.gz
    cd capnproto-c++-0.6.1
    ./configure --prefix=$HOME/cached-deps
    make -j6 check
    sudo make install
fi

ls -l $HOME/cached-deps/bin
source ~/.cargo/env || true
