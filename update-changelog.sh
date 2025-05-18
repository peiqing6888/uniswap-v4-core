#!/bin/bash

# Check if git-cliff is installed
if ! command -v git-cliff &> /dev/null; then
    echo "git-cliff is not installed. Installing now..."
    cargo install git-cliff
    
    # Check if installation was successful
    if [ $? -ne 0 ]; then
        echo "Failed to install git-cliff. Please install it manually using 'cargo install git-cliff'."
        exit 1
    fi
fi

echo "Updating .changelog file..."

# Generate changelog
git-cliff --config cliff.toml --output .changelog

echo "Changelog updated successfully!"
echo "To view the changelog, use 'cat .changelog'" 