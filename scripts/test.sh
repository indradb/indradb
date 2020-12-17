#!/bin/bash
set -ex

make test

if [ "$TRAVIS_OS_NAME" == "linux" ] && [ "$TRAVIS_RUST_VERSION" == "stable" ]; then
    cargo fmt -- --check
    cargo clippy
fi

if [ "$TRAVIS_OS_NAME" == "linux" ] && [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
    # Make sure benches and fuzzers can compile. We don't actually run them
    # because they're heavy.
    pushd lib
        cargo check --features=bench-suite
        pushd fuzz
            cargo check
        popd
    popd
    pushd bin
        cargo check --features=bench-suite
    popd

    zip -0 ccov.zip `find . \( -name "indradb*.gc*" \) -print`
    grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o lcov.info
    # TODO: re-enable codecov when we can figure out how to make the coverage
    # analysis more accurate
    # bash <(curl -s https://codecov.io/bash) -f lcov.info
fi
