#! /bin/bash

set -exu -o pipefail

df -h

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $RUST