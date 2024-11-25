#!/usr/bin/env bash

echo "Starting required dependencies..."
docker compose -f compose.yml -f compose.test.yml --env-file .env.test up -d
