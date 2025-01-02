#!/usr/bin/env pwsh

# Prepare
$ErrorActionPreference = "Stop"
$WORK_DIR = Get-Location
Set-Location -Path (Split-Path -Parent $MyInvocation.MyCommand.Path)

# Check if cargo nextest exists
if (-not (Get-Command cargo-nextest -ErrorAction SilentlyContinue)) {
    Write-Host "cargo-nextest could not be found. Please install it by following the instructions at: https://nexte.st/docs/installation/pre-built-binaries/"
    exit 1
}

# Run cargo nextest with or without arguments
if ($args.Count -eq 0) {
    cargo nextest run --workspace --test-threads 1
} else {
    cargo nextest run $args --workspace --test-threads 1
}

Set-Location -Path $WORK_DIR