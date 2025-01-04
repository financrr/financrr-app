#!/usr/bin/env pwsh

# Enable strict error handling
$ErrorActionPreference = "Stop"

# Prepare
$WORK_DIR = Get-Location
Set-Location -Path (Split-Path -Parent $MyInvocation.MyCommand.Path)
Set-Location ..

# Function to return to the original working directory
function cd_into_work_dir {
    Set-Location -Path $WORK_DIR
}

# Ensure the cleanup function runs on exit
$cd_into_work_dir_action = {
    cd_into_work_dir
}
Register-EngineEvent PowerShell.Exiting -Action $cd_into_work_dir_action

Write-Host "Creating .env file..."
try {
    if (-Not (Test-Path ".env")) {
        Copy-Item -Path ".env.dist" -Destination ".env"
    }
} catch {
    Write-Host "Failed to copy .env.dist to .env" -ForegroundColor Red
}

Write-Host "Creating logs directory..."
New-Item -ItemType Directory -Path "logs" -Force | Out-Null
icacls "logs" /grant "Everyone:(OI)(CI)F" | Out-Null

Write-Host "Creating data directory..."
New-Item -ItemType Directory -Path "data" -Force | Out-Null
icacls "data" /grant "Everyone:(OI)(CI)F" | Out-Null
