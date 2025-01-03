#!/usr/bin/env bash

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ..

echo "Building containers."

# Enable Docker's experimental features
#export DOCKER_CLI_EXPERIMENTAL=enabled

# Create a new builder which gives access to the new multi-architecture features
docker buildx create --use

# Build for multiple platforms
docker buildx build --platform linux/amd64,linux/arm64 -t backend:latest  . -f docker/rust/Dockerfile

cd "${WORK_DIR}"
