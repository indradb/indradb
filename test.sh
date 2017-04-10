#!/bin/bash

set -e

export TEST_RDB_DIRECTORY=`mktemp -d 2>/dev/null || mktemp -d -t 'braid'`
export RUST_BACKTRACE=1
export SECRET=QkrDxgVJCT
export TEST_POSTGRES_URL=postgres://postgres@localhost:5432/braid_test

function cleanup {
    echo "Cleaning up..."
    rm -r $TEST_RDB_DIRECTORY
}

mkdir -p $TEST_RDB_DIRECTORY
trap cleanup EXIT

dropdb --if-exists braid_test
createdb --owner=postgres braid_test
cargo test $TEST_NAME
