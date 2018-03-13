#!/bin/bash

set -e

PG_USER='postgres'
UNAME_STR=`uname`
if [[ "$UNAME_STR" == 'Darwin' ]]; then
    PG_USER=`whoami`
    echo "Using user '${PG_USER}' for postgres"
fi

export TEST_RDB_DIRECTORY=`mktemp -d 2>/dev/null || mktemp -d -t 'indradb'`
export RUST_BACKTRACE=1
export TEST_POSTGRES_URL="postgres://${PG_USER}@localhost:5432/indradb_test"

ACTION=test

while true; do
    case "$1" in
        --bench) ACTION=bench; shift ;;
        * ) break ;;
    esac
done

function cleanup {
    echo "Cleaning up..."
    rm -r $TEST_RDB_DIRECTORY
}

mkdir -p $TEST_RDB_DIRECTORY
trap cleanup EXIT

dropdb --if-exists indradb_test
createdb --owner=$PG_USER indradb_test
#cargo update
cargo $ACTION --all-features $TEST_NAME
