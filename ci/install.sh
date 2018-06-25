#!/bin/bash

set -ex

if [ $TRAVIS_OS_NAME = linux ] && [ $TRAVIS_RUST_VERSION = nightly ]; then
    if ! type kcov &> /dev/null; then
        pushd $HOME
            wget https://github.com/SimonKagstrom/kcov/archive/v35.tar.gz
            tar xzf v35.tar.gz
            pushd kcov-35
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
