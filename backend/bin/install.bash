#!/usr/bin/env bash

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ..

cd_into_work_dir() {
    cd "${WORK_DIR}"
}

echo "Creating .env file..."
set +e
cp -n .env.dist .env
set -e

echo "Creating logs directory..."
mkdir -p logs
chmod +rw logs

echo "Creating data directory..."
mkdir -p data
chmod +rw data

trap cd_into_work_dir EXIT
