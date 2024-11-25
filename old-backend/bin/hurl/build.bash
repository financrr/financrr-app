#!/usr/bin/env bash

echo "Rebuilding test containers..."

# Navigate to the project's root directory
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ../../

# Rebuild the test containers
docker compose -f compose.yml -f compose.test.yml --env-file .env.test build

echo "Test containers rebuilt successfully."

# Return to the original working directory
cd "${WORK_DIR}"
