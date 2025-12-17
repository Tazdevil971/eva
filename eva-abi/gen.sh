#!/bin/sh

# TODO: const arrays are not generated!
cbindgen \
    --config cbindgen.toml \
    --crate eva-abi \
    --output include/eva/abi.h