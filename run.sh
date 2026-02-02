#!/bin/bash

# Default port
PORT=3000

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -p|--port) PORT="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Determine the container tool (podman or docker)
if command -v podman &> /dev/null; then
    RUNTIME="podman"
elif command -v docker &> /dev/null; then
    RUNTIME="docker"
else
    echo "Error: neither podman nor docker found."
    exit 1
fi

echo "Running image 'snap:latest' using $RUNTIME on port $PORT..."

# `exec` replaces the shell process with the container process
# `--init` acts as a PID 1 to forward signals (like Ctrl+C) to the application
# `-it` allocates a TTY and keeps stdin open
# `-v` mounts a persistent volume for the database
exec $RUNTIME run --rm --init -it \
    -p "$PORT:3000" \
    -v snap-data:/app/data \
    snap:latest
