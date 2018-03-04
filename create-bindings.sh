#! /bin/bash

set -eou pipefail

bindgen wrapper.h -- -I/usr/lib/gcc/x86_64-linux-gnu/5/include-fixed -I/usr/lib/gcc/x86_64-linux-gnu/5/include -I/usr/local/cuda/include >> cuda.rs
rustfmt cuda.rs