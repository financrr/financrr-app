#!/usr/bin/env bash
# This script exists due to Postgres creating directories with root permissions
# We mount these directories to the docker container, so we need to change the permissions

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ..

echo "Building containers."
docker compose --profile all build

cd "${WORK_DIR}"
