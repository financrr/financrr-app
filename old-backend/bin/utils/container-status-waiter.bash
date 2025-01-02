#!/usr/bin/env bash

#!/bin/bash

# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <container-name> <expected-status>"
    exit 1
fi

CONTAINER_NAME="$1"
EXPECTED_STATUS="$2"
TIMEOUT=60

# Function to check container status
check_status() {
    docker ps --format '{{.Names}}:{{.Status}}' | grep "^$CONTAINER_NAME:" | awk '{print $(NF)}' | tr -d '()'
}

# Wait for the container to reach the expected status
SECONDS=0
while [ "$(check_status)" != "$EXPECTED_STATUS" ]; do
    if [ $SECONDS -ge $TIMEOUT ]; then
        echo "Error: Container $CONTAINER_NAME did not reach the $EXPECTED_STATUS status within $TIMEOUT seconds."
        exit 1
    fi
    sleep 1
done

echo "Container $CONTAINER_NAME has reached the $EXPECTED_STATUS status."
