#!/bin/bash

# Define .gitignore path
GITIGNORE_PATH=".gitignore"

# Array of lines to add to .gitignore
IGNORE_LINES=(
    "/archive/"
    "*.tar.gz"
    "/target/"
    "**/*.pkg.tar.zst"
    ".DS_Store"
)

# Function to add line to .gitignore if it doesn't already exist
add_line_to_gitignore() {
    local line=$1
    grep -qxF "$line" "$GITIGNORE_PATH" || echo "$line" >> "$GITIGNORE_PATH"
}

# Ensure .gitignore exists
touch "$GITIGNORE_PATH"

# Add lines to .gitignore
for line in "${IGNORE_LINES[@]}"; do
    add_line_to_gitignore "$line"
done

echo ".gitignore has been updated."

