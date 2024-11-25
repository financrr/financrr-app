#!/usr/bin/env bash

# prepare
set -e
WORK_DIR="$(pwd)"
cd "$(dirname "$0")"
cd ../../

set +e
# Execute the curl command inside the hurl container and capture the token
TOKEN=$(docker compose -f compose.yml -f compose.test.yml run -T --entrypoint curl hurl -s -X POST 'http://rust:8080/api/v1/session' \
    -H 'Content-Type: application/json' \
    -d '{"username": "admin", "password": "Financrr123", "name": "test_session"}' | jq -r '.token')
set -e

if [ "$TOKEN" != "null" ] && [ -n "$TOKEN" ]; then
    # Set the HURL_admin_token environment variable on the host machine
    export HURL_admin_token="$TOKEN"
    echo "Token set successfully."
    echo "Token: $HURL_admin_token" # This line has been added to show the token
else
    echo "Failed to extract token."
fi

cd "${WORK_DIR}"
