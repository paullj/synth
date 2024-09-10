#!/bin/bash

set -eou pipefail

# Variables from environment
TEMPLATE_FILE="$1"
USER="$2"
COMMAND="$3"
OUTPUT_FILE="$4"

# Replace placeholders in the template file
sed -e "s|{{COMMAND}}|$COMMAND|g" -e "s|{{USER}}|$USER|g" "$TEMPLATE_FILE" > "$OUTPUT_FILE"
