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

	pushd bin
	    cargo build --release
	popd

	cp target/release/indradb $stage/

	pushd $stage
	    tar czf $src/indradb-$TRAVIS_TAG-$TARGET.tar.gz *
	popd

	rm -rf $stage
}

main
