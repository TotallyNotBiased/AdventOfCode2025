#!/usr/bin/env bash

# Configuration
COOKIE_FILE="cookie.txt"

# 1. Check if the URL argument was provided
if [ -z "$1" ]; then
    echo "Error: No URL provided."
    echo "Usage: ./get_input.sh <URL>"
    echo "Example: ./get_input.sh https://adventofcode.com/2025/day/1/input"
    exit 1
fi

URL="$1"

# 2. Extract Day and Setup Project
if [[ "$URL" =~ /day/([0-9]+) ]]; then
    DAY="${BASH_REMATCH[1]}"
    DIR_NAME="day${DAY}"
    
    echo "Detected Day: $DAY"

    # If the directory doesn't exist, create it as a Rust project
    if [ ! -d "$DIR_NAME" ]; then
        echo "Creating Rust project for Day $DAY..."
        # --vcs none prevents creating a nested git repository
        cargo new --vcs none "$DIR_NAME"
    fi
    
    OUTPUT_FILE="$DIR_NAME/input.txt"
else
    echo "Warning: Could not determine day number from URL."
    echo "Saving to current directory."
    OUTPUT_FILE="input.txt"
fi

# 3. Check if the cookie file exists
if [ ! -f "$COOKIE_FILE" ]; then
    echo "Error: Cookie file '$COOKIE_FILE' not found."
    exit 1
fi

# 4. Execute the curl command
echo "Downloading input to: $OUTPUT_FILE"
curl --cookie "$COOKIE_FILE" "$URL" -L -o "$OUTPUT_FILE"

# 5. Success check
if [ $? -eq 0 ]; then
    echo "Success! Saved to '$OUTPUT_FILE'."
else
    echo "Error: Download failed."
fi