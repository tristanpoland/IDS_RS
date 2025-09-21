#!/bin/bash

# Shell script to download the latest PCI IDs database
# Usage: ./update_pci_ids.sh [output_path]

set -euo pipefail

# Configuration
OUTPUT_PATH="${1:-pci.ids}"
URL="https://pci-ids.ucw.cz/v2.2/pci.ids"
FORCE_DOWNLOAD=false

# Parse command line options
while [[ $# -gt 0 ]]; do
    case $1 in
        -f|--force)
            FORCE_DOWNLOAD=true
            shift
            ;;
        -o|--output)
            OUTPUT_PATH="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [-f|--force] [-o|--output OUTPUT_PATH]"
            echo "  -f, --force     Force download even if file is recent"
            echo "  -o, --output    Specify output file path (default: pci.ids)"
            echo "  -h, --help      Show this help message"
            exit 0
            ;;
        *)
            OUTPUT_PATH="$1"
            shift
            ;;
    esac
done

echo -e "\033[32mPCI IDs Database Updater\033[0m"
echo -e "\033[32m========================\033[0m"

# Check if file exists and get modification time
should_download=true
if [[ -f "$OUTPUT_PATH" && "$FORCE_DOWNLOAD" != true ]]; then
    if command -v stat > /dev/null 2>&1; then
        # Use stat to get file modification time
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS
            file_time=$(stat -f %m "$OUTPUT_PATH")
        else
            # Linux
            file_time=$(stat -c %Y "$OUTPUT_PATH")
        fi

        current_time=$(date +%s)
        days_old=$(( (current_time - file_time) / 86400 ))

        echo -e "\033[33mExisting file found: $OUTPUT_PATH\033[0m"
        echo -e "\033[33mDays since last update: $days_old\033[0m"

        if [[ $days_old -lt 7 ]]; then
            echo -e "\033[36mFile is less than 7 days old. Use --force to download anyway.\033[0m"
            should_download=false
        fi
    fi
fi

if [[ "$should_download" == true ]]; then
    echo -e "\033[36mDownloading PCI IDs database from: $URL\033[0m"

    # Download with curl or wget
    if command -v curl > /dev/null 2>&1; then
        curl -L -A "IDS_RS/0.1.0 PCI-IDs-Updater" -o "$OUTPUT_PATH" "$URL"
    elif command -v wget > /dev/null 2>&1; then
        wget -U "IDS_RS/0.1.0 PCI-IDs-Updater" -O "$OUTPUT_PATH" "$URL"
    else
        echo -e "\033[31mError: Neither curl nor wget found. Please install one of them.\033[0m"
        exit 1
    fi

    # Verify download was successful
    if [[ ! -f "$OUTPUT_PATH" ]]; then
        echo -e "\033[31mError: Download failed - file not created\033[0m"
        exit 1
    fi

    file_size=$(wc -c < "$OUTPUT_PATH")
    file_size_kb=$((file_size / 1024))

    echo -e "\033[32mDownload completed successfully!\033[0m"
    echo -e "\033[32mFile size: ${file_size_kb} KB\033[0m"

    # Verify the file is valid by checking for expected header
    first_line=$(head -n 1 "$OUTPUT_PATH")
    if echo "$first_line" | grep -q "PCI.*ID"; then
        echo -e "\033[32mFile validation: PASSED\033[0m"
    else
        echo -e "\033[31mFile validation: FAILED - Unexpected format\033[0m"
        exit 1
    fi

    # Show some basic statistics
    total_lines=$(wc -l < "$OUTPUT_PATH")
    vendor_lines=$(grep -c "^[0-9a-fA-F]\{4\}[[:space:]]" "$OUTPUT_PATH" || true)
    device_lines=$(grep -c "^[[:space:]][0-9a-fA-F]\{4\}[[:space:]]" "$OUTPUT_PATH" || true)

    echo ""
    echo -e "\033[32mDatabase Statistics:\033[0m"
    echo -e "  Total lines: $total_lines"
    echo -e "  Vendors: $vendor_lines"
    echo -e "  Devices: $device_lines"

else
    echo -e "\033[36mSkipping download.\033[0m"
fi

echo -e "\033[32mPCI IDs database is ready at: $OUTPUT_PATH\033[0m"