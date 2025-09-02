#!/bin/bash
# test.sh
#
# Copyright (c) 2025 Junpei Kawamoto
#
# This software is released under the MIT License.
#
# http://opensource.org/licenses/mit-license.php

cargo clean
MKLROOT=$(cargo build -vv | grep "cargo::metadata=ROOT=" | cut -d= -f3-)

if [ ! -f "$MKLROOT/include/mkl.h" ]; then
    echo "Error: include/mkl.h not found." >&2
    exit 1
fi

if [ ! -f "$MKLROOT/lib/libmkl_core.a" ]; then
    echo "Error: lib/libmkl_core.a not found." >&2
    exit 1
fi
