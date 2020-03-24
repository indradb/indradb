#!/bin/bash
set -ex
cargo build
cargo test

if [ "$TRAVIS_OS_NAME" == "linux" ] && [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
    zip -0 ccov.zip `find . \( -name "indradb*.gc*" \) -print`
    grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o lcov.info
    bash <(curl -s https://codecov.io/bash) -f lcov.info
fi
