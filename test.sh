#!/bin/bash

set -e

pushd lib
    ./test.sh
popd

pushd bin
    ./test.sh
popd
