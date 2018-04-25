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

if [ "$ACTION" == "test" ]; then
    cargo test $TEST_NAME
else
    cargo +nightly bench --features=bench-suite $TEST_NAME
fi
