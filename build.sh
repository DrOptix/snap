#!/bin/bash

# Determine the container tool (podman or docker)
if command -v podman &> /dev/null; then
    BUILDER="podman"
elif command -v docker &> /dev/null; then
    BUILDER="docker"
else
    echo "Error: neither podman nor docker found."
    exit 1
fi

echo "Building image 'snap:latest' using $BUILDER..."
$BUILDER build -t snap:latest -f Containerfile .
