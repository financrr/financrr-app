#!/usr/bin/env pwsh

# Enable strict error handling
$ErrorActionPreference = "Stop"

# Prepare
$WORK_DIR = Get-Location
Set-Location -Path (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location ..

# Function to return to the original working directory
function cd_into_work_dir
{
    Set-Location -Path $WORK_DIR
}

# Ensure the cleanup function runs on exit
$cd_into_work_dir_action = {
    cd_into_work_dir
}
Register-EngineEvent PowerShell.Exiting -Action $cd_into_work_dir_action

# Check if cargo-nextest exists
if (-Not (Get-Command "cargo-nextest" -ErrorAction SilentlyContinue))
{
    Write-Host "cargo-nextest could not be found. Please install it by following the instructions at: https://nexte.st/docs/installation/pre-built-binaries/" -ForegroundColor Red
    exit 1
}

# Run cargo nextest with or without arguments
if ($args.Count -eq 0)
{
    & cargo nextest run --workspace --test-threads 1
}
else
{
    & cargo nextest run @args --workspace --test-threads 1
}
