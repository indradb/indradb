#!/bin/bash

set -e

export DATABASE_NAME=braid_test
export RUST_BACKTRACE=1
export SECRET=QkrDxgVJCT
export DATABASE_URL=postgres://postgres@localhost:5432/braid_test
export BRAID_SCRIPT_ROOT=`pwd`/test_scripts

dropdb --if-exists braid_test
createdb --owner=postgres braid_test
cargo build
./target/debug/braid-db init
./support/with_server.sh cargo test $TEST_NAME
