#!/bin/bash

set -e

export RUST_BACKTRACE=1

ACTION=test

while true; do
    case "$1" in
        --bench) ACTION=bench; shift ;;
        * ) break ;;
    esac
done

cargo build
cargo $ACTION $TEST_NAME
