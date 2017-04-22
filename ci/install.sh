#!/bin/bash

set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target

    if [ $TRAVIS_OS_NAME = linux ]; then
        sudo apt-get -qq update
        sudo apt-get install -y lua5.1 liblua5.1-0-dev

        # Needed so that `lua.pc` can be found when building via `cross`
        pushd /usr/lib/pkgconfig
            sudo ln -s lua5.1.pc lua.pc
        popd

        # Verify that `lua.pc` is there
        pkg-config --cflags "lua"
    else
        brew update

        # TODO: Currently `rustdoc` fails on osx/rust stable due to an issue
        # with `libjpeg`. Once it's fixed, remove the manually tweaking here.
        brew install lua@5.1 libjpeg
        brew link libjpeg
        pushd /usr/local/lib
            rm libgif.dylib
            ln -s /System/Library/Frameworks/ImageIO.framework/Resources/libGIF.dylib libGIF.dylib
            rm libjpeg.dylib
            ln -s /System/Library/Frameworks/ImageIO.framework/Resources/libJPEG.dylib libJPEG.dylib
            rm libtiff.dylib
            ln -s /System/Library/Frameworks/ImageIO.framework/Resources/libTIFF.dylib libTIFF.dylib
            rm libpng.dylib
            ln -s /System/Library/Frameworks/ImageIO.framework/Resources/libPng.dylib libPng.dylib
        popd
    fi

    source ~/.cargo/env || true
}

main
