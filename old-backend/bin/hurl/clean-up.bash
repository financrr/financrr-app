#!/usr/bin/env bash

echo "Cleaning up..."
docker compose -f compose.yml -f compose.test.yml --env-file .env.test down -v
