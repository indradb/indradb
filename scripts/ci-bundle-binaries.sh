#!/bin/bash

set -ex

os=$1
project_root=$(pwd)

# install protobuf
if [ "$os" == "ubuntu-latest" ]; then
    eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
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
