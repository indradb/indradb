#!/bin/bash

set -ex

main() {
    if [ $TRAVIS_OS_NAME = linux ]; then
        sudo apt-get -qq update
        sudo apt-get install -y lua5.1 liblua5.1-0-dev
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
