#!/bin/bash

set -ex

if [ $TRAVIS_OS_NAME = linux ]; then
    sudo ln -s /usr/include/x86_64-linux-gnu/zconf.h /usr/include/zconf.h

    if ! type kcov &> /dev/null; then
        pushd $HOME
            wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
            tar xzf master.tar.gz
            pushd kcov-master
                mkdir -p build
                pushd build
                    cmake ..
                    make
                    sudo make install
                popd
            popd
        popd
    fi
fi

source ~/.cargo/env || true
