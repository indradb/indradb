#!/bin/bash

set -e

export RUST_BACKTRACE=1
export INDRADB_SCRIPT_ROOT=`pwd`/test_scripts

cargo build
cargo test $TEST_NAME -- --nocapture
