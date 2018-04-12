#!/bin/bash

set -e

export RUST_BACKTRACE=1

cargo build
cargo test $TEST_NAME
