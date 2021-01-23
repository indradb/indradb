#!/bin/bash

set -ex

os=$1

cargo build --release

if [ "$os" == "ubuntu-latest" ]; then
    stage=$(mktemp -d)
else
    stage=$(mktemp -d -t tmp)
fi

cp target/release/indradb-server $stage/
# TODO: add once CLI client is actually implemented
# cp target/release/indradb-client $stage/

pushd $stage
    tar czf ~/indradb.tar.gz *
popd

cd ~
rm -rf $stage
