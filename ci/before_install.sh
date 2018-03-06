#! /bin/bash

set -exu -o pipefail

curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path --default-toolchain $RUST
df -h