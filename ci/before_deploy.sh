#!/bin/bash
# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    pushd bin
        test -f Cargo.lock || cargo generate-lockfile

        cargo build --release

        cp target/release/braid-db $stage/
        cp target/release/braid-server $stage/
        cp target/release/braid-account $stage/

        pushd $stage
            tar czf $src/braid-$TRAVIS_TAG-$TARGET.tar.gz *
        popd

        rm -rf $stage
    popd
}

main
