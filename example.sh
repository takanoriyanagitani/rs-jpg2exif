#!/bin/sh

ijpeg=./input.jpg

cat "${ijpeg}" |
    wasmtime \
        run \
        ./rs-jpg2exif.wasm \
        --input-img-bytes-max 16777216 |
    xxd
