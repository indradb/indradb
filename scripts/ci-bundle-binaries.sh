#!/bin/bash

set -ex

os=$1
project_root=$(pwd)

if [ "$os" == "ubuntu-latest" ]; then
    echo "/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin" >> $GITHUB_PATH
fi

brew install protobuf

cargo build --release

if [ "$os" == "ubuntu-latest" ]; then
    stage=$(mktemp -d)
else
    stage=$(mktemp -d -t tmp)
fi

cp target/release/indradb-server $stage/
cp target/release/indradb-client $stage/

pushd $stage
    tar czf $project_root/indradb.tar.gz *
popd

rm -rf $stage
