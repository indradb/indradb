#!/bin/bash

set -ex

mkdir -p $HOME/cached-deps

if [ $TRAVIS_OS_NAME = linux ] && [ $TRAVIS_RUST_VERSION = stable ] && [ ! -f $HOME/cached-deps/bin/kcov ] ; then
    pushd $HOME
        wget https://github.com/SimonKagstrom/kcov/archive/v35.tar.gz
        tar xzf v35.tar.gz

        pushd kcov-35
            mkdir -p build
            pushd build
                cmake -DCMAKE_INSTALL_PREFIX=$HOME/cached-deps ..
                make
                sudo make install
            popd
        popd
    popd
fi

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
