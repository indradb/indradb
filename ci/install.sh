#!/bin/bash

set -ex

ls /usr/include/x86_64-linux-gnu
ls /usr/include/sys
cc1 -v
cpp -v

if [ $TRAVIS_OS_NAME = linux ]; then
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
