#!/bin/bash
set -ex

make test

if [ "$TRAVIS_OS_NAME" == "linux" ] && [ "$TRAVIS_RUST_VERSION" == "stable" ]; then
    cargo fmt -- --check
fi

if [ "$TRAVIS_OS_NAME" == "linux" ] && [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
    zip -0 ccov.zip `find . \( -name "indradb*.gc*" \) -print`
    grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o lcov.info
    # TODO: re-enable codecov when we can figure out how to make the coverage
    # analysis more accurate
    # bash <(curl -s https://codecov.io/bash) -f lcov.info
fi
