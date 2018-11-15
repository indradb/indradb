#!/bin/bash

set -ex

if [ ! -d "$HOME/bin" ]; then
    mkdir -p $HOME/bin

    if [ $TRAVIS_OS_NAME = linux ] && [ $TRAVIS_RUST_VERSION = stable ] ; then
        pushd $HOME
            wget https://github.com/SimonKagstrom/kcov/archive/v35.tar.gz
            tar xzf v35.tar.gz
        popd

        pushd $HOME/kcov-35
            mkdir -p build
            pushd build
                cmake -DCMAKE_INSTALL_PREFIX=$HOME/bin ..
                make
                sudo make install
            popd
        popd
    fi

    curl -O https://capnproto.org/capnproto-c++-0.6.1.tar.gz
    tar zxf capnproto-c++-0.6.1.tar.gz
    cd capnproto-c++-0.6.1
    ./configure --prefix=$HOME/bin
    make -j6 check
    sudo make install
fi

source ~/.cargo/env || true
