#!/usr/bin/env pwsh

# PowerShell script to download the latest PCI IDs database
# Usage: .\update_pci_ids.ps1

param(
    [string]$OutputPath = "pci.ids",
    [string]$Url = "https://pci-ids.ucw.cz/v2.2/pci.ids",
    [switch]$Force = $false
)

$ErrorActionPreference = "Stop"

Write-Host "PCI IDs Database Updater" -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green

# Check if file exists and get modification time
$shouldDownload = $true
if (Test-Path $OutputPath -and -not $Force) {
    $fileInfo = Get-Item $OutputPath
    $daysSinceModified = (Get-Date).Subtract($fileInfo.LastWriteTime).Days

    Write-Host "Existing file found: $OutputPath" -ForegroundColor Yellow
    Write-Host "Last modified: $($fileInfo.LastWriteTime)" -ForegroundColor Yellow
    Write-Host "Days since last update: $daysSinceModified" -ForegroundColor Yellow

    if ($daysSinceModified -lt 7) {
        Write-Host "File is less than 7 days old. Use -Force to download anyway." -ForegroundColor Cyan
        $shouldDownload = $false
    }
}

if ($shouldDownload) {
    Write-Host "Downloading PCI IDs database from: $Url" -ForegroundColor Cyan

    try {
        # Create a web request with proper headers
        $webClient = New-Object System.Net.WebClient
        $webClient.Headers.Add("User-Agent", "IDS_RS/0.1.0 PCI-IDs-Updater")

        # Download the file
        $webClient.DownloadFile($Url, $OutputPath)

        $fileSize = (Get-Item $OutputPath).Length
        Write-Host "Download completed successfully!" -ForegroundColor Green
        Write-Host "File size: $([math]::Round($fileSize / 1KB, 2)) KB" -ForegroundColor Green

        # Verify the file is valid by checking for expected header
        $firstLine = Get-Content $OutputPath -TotalCount 1
        if ($firstLine -match "^#.*PCI.*ID") {
            Write-Host "File validation: PASSED" -ForegroundColor Green
        } else {
            Write-Warning "File validation: FAILED - Unexpected format"
            exit 1
        }

        # Show some basic statistics
        $totalLines = (Get-Content $OutputPath | Measure-Object).Count
        $vendorLines = (Get-Content $OutputPath | Where-Object { $_ -match "^[0-9a-fA-F]{4}\s" }).Count
        $deviceLines = (Get-Content $OutputPath | Where-Object { $_ -match "^\t[0-9a-fA-F]{4}\s" }).Count

        Write-Host "" -ForegroundColor Green
        Write-Host "Database Statistics:" -ForegroundColor Green
        Write-Host "  Total lines: $totalLines" -ForegroundColor White
        Write-Host "  Vendors: $vendorLines" -ForegroundColor White
        Write-Host "  Devices: $deviceLines" -ForegroundColor White

    } catch {
        Write-Error "Failed to download PCI IDs database: $($_.Exception.Message)"
        exit 1
    } finally {
        if ($webClient) {
            $webClient.Dispose()
        }
    }
} else {
    Write-Host "Skipping download." -ForegroundColor Cyan
}

Write-Host "PCI IDs database is ready at: $OutputPath" -ForegroundColor Green