#!/usr/bin/env bash

# Check if a file name is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <test-file-name>"
    exit 1
fi

TEST_FILE="$1"

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ../../

# Function to clean up
cleanup() {
    bash bin/hurl/clean-up.bash
}

# Trap EXIT signal to ensure cleanup is always executed
trap cleanup EXIT

bash bin/hurl/clean-up.bash

bash bin/hurl/start-up.bash

echo "Waiting for the services to start..."
bash bin/utils/container-status-waiter.bash financrr-backend-test healthy

# Setting admin token
echo "Setting admin token..."
TOKEN_OUTPUT=$(bash bin/hurl/set-admin-token.bash)
TOKEN=$(echo "$TOKEN_OUTPUT" | grep "Token:" | cut -d ' ' -f 2)
if [ -n "$TOKEN" ]; then
    export HURL_admin_token="$TOKEN"
    echo "HURL_admin_token set to $HURL_admin_token"
else
    echo "Failed to set HURL_admin_token."
    exit 1
fi

echo "Running the test for $TEST_FILE..."
docker compose -f compose.yml -f compose.test.yml run --rm -T hurl --test --color "/tests/$TEST_FILE"
wait

cd "${WORK_DIR}"
