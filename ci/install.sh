#! /bin/bash

set -exu -o pipefail

df -h

rustup component add rustfmt-preview
cargo install --force bindgen

export CUDA80_URL=https://developer.nvidia.com/compute/cuda/8.0/Prod2/local_installers/cuda_8.0.61_375.26_linux-run
export CUDA91_URL=https://developer.nvidia.com/compute/cuda/9.1/Prod/local_installers/cuda_9.1.85_387.26_linux

if [ "$CUDA" == "80" ]; then
    export CUDA_URL="$CUDA80_URL"
elif [ "$CUDA" == "91" ]; then
    export CUDA_URL="$CUDA91_URL"
fi

wget -O cuda.run "$CUDA_URL"
chmod +x cuda.run
./cuda.run --toolkit --silent --toolkitpath="$CUDA_ROOT"
rm -v cuda.run

