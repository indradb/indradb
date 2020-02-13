#!/bin/bash

set -ex

mkdir -p $HOME/bin

if [ ! -f $HOME/bin/capnp ] ; then
    mkdir -p $HOME/capnp
    curl -O https://capnproto.org/capnproto-c++-0.6.1.tar.gz
    tar zxf capnproto-c++-0.6.1.tar.gz
    cd capnproto-c++-0.6.1
    ./configure --prefix=$HOME/capnp
    make -j6 check
    sudo make install
    cp $HOME/capnp/bin/* $HOME/bin/
fi

ls -l $HOME/bin
source ~/.cargo/env || true
