#!/bin/bash

set -ex

if [ $TRAVIS_OS_NAME = linux ] && [ $TRAVIS_RUST_VERSION = nightly ]; then
    if ! type $HOME/kcov-35 &> /dev/null; then
        pushd $HOME
            wget https://github.com/SimonKagstrom/kcov/archive/v35.tar.gz
            tar xzf v35.tar.gz
        popd
    fi

    pushd $HOME/kcov-35
        mkdir -p build
        pushd build
            cmake ..
            make
            sudo make install
        popd
    popd
else
    # Just make an empty directory so that caching doesn't fail
    mkdir -p $HOME/kcov-35
fi

source ~/.cargo/env || true
