#!/bin/bash

set -e

export DATABASE_NAME=braid_test
export TEST_RDB_DIRECTORY="/tmp/test-rdb"
export RUST_BACKTRACE=1
export SECRET=QkrDxgVJCT
export DATABASE_URL=postgres://postgres@localhost:5432/braid_test
export BRAID_SCRIPT_ROOT=`pwd`/bin/test_scripts

function cleanup {
    echo "Cleaning up..."
    rm -r $TEST_RDB_DIRECTORY
}

mkdir -p $TEST_RDB_DIRECTORY
trap cleanup EXIT

dropdb --if-exists $DATABASE_NAME
createdb --owner=postgres $DATABASE_NAME
cargo test $TEST_NAME
