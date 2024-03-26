#!/usr/bin/env bash

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ..

echo "Installing..."
bin/install.bash


echo "Building containers"
bin/build.bash

echo "Stopping containers"
docker compose --profile all down -v

echo "Pulling images"
docker compose --profile all pull

echo "Starting containers"
docker compose --profile all up -d

cd "${WORK_DIR}"
