#!/bin/bash

set -ex

main() {
    if [ $TRAVIS_OS_NAME = linux ]; then
        sudo apt-get -qq update
        sudo apt-get install -y lua5.1 liblua5.1-0-dev
    else
        brew update
        brew install lua@5.1
    fi

    source ~/.cargo/env || true
}

main
