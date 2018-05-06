#!/bin/bash

set -e

export RUST_BACKTRACE=1
export INDRADB_SCRIPT_ROOT=`pwd`/test_scripts

ACTION=test

while true; do
    case "$1" in
        --bench) ACTION=bench; shift ;;
        * ) break ;;
    esac
done

cargo build

if [ "$ACTION" == "test" ]; then
    cargo test --features=test-suite $TEST_NAME
else
    cargo +nightly bench --features=test-suite,bench-suite $TEST_NAME
fi
