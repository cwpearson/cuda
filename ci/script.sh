#! /bin/bash

set -exu -o pipefail

df -h

cargo check --target $TARGET