#!/bin/bash

set -e

postgresuser='postgres'
unamestr=`uname`
if [[ "$unamestr" == 'Darwin' ]]; then
    postgresuser=`whoami`
    echo "Using user '${postgresuser}' for postgres" 
fi

export DATABASE_NAME=braid_test
export RUST_BACKTRACE=1
export SECRET=QkrDxgVJCT
export DATABASE_URL="postgres://${postgresuser}@localhost:5432/braid_test"
export BRAID_SCRIPT_ROOT=`pwd`/test_scripts

dropdb --if-exists braid_test
createdb --owner=$postgresuser braid_test
cargo build
./target/debug/braid-db init
./support/with_server.sh cargo test $TEST_NAME
