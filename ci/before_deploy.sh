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

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin braid-db --target $TARGET --release -- -C lto
    cross rustc --bin braid-server --target $TARGET --release -- -C lto
    cross rustc --bin braid-account --target $TARGET --release -- -C lto

    cp target/$TARGET/release/braid-db $stage/
    cp target/$TARGET/release/braid-server $stage/
    cp target/$TARGET/release/braid-account $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
