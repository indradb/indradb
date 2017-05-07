#!/bin/bash

set -e

PG_USER='postgres'
UNAME_STR=`uname`
if [[ "$UNAME_STR" == 'Darwin' ]]; then
    PG_USER=`whoami`
    echo "Using user '${PG_USER}' for postgres" 
fi

export DATABASE_NAME=braid_test
export RUST_BACKTRACE=1
export SECRET=QkrDxgVJCT
export DATABASE_URL="postgres://${PG_USER}@localhost:5432/braid_test"
export BRAID_SCRIPT_ROOT=`pwd`/test_scripts

dropdb --if-exists braid_test
createdb --owner=$PG_USER braid_test
cargo build
./target/debug/braid-db init
./support/with_server.sh cargo test $TEST_NAME
