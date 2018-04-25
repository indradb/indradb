#!/bin/bash

set -ex

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

    curl -OL https://github.com/google/protobuf/releases/download/v3.2.0/protoc-3.2.0-linux-x86_64.zip
    unzip protoc-3.2.0-linux-x86_64.zip -d protoc
    sudo mv protoc/bin/* /usr/local/bin/
    sudo mv protoc/include/* /usr/local/include/
else
    # Ignore errors on this because homebrew is returning exit code 1 on
    # successful installation
    brew install protobuf || true
fi

source ~/.cargo/env || true
