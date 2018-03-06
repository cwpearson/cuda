#! /bin/bash

set -exu -o pipefail

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $RUST
df -h