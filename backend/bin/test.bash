#!/usr/bin/env bash

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ..

cd_into_work_dir() {
    cd "${WORK_DIR}"
}

# Check if cargo nextest exists
if ! command -v cargo-nextest &> /dev/null; then
    echo "cargo-nextest could not be found. Please install it by following the instructions at: https://nexte.st/docs/installation/pre-built-binaries/"
    exit 1
fi

# Run cargo nextest with or without arguments
if [ $# -eq 0 ]; then
    cargo nextest run --workspace --test-threads 1
else
    cargo nextest run "$@" --workspace --test-threads 1
fi

trap cd_into_work_dir EXIT
