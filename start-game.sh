#!/bin/bash

if [ "$1" = "--rebuild" ]; then
    echo "Cleaning project..."
    cargo clean

    echo "Building WASM module..."
    wasm-pack build --target web --out-dir pkg

    if [ $? -ne 0 ]; then
        echo "Error: WASM build failed."
        exit 1
    fi
else
    echo "\nSkipping rebuild. Serving existing build...\n"
fi

echo "Starting Space Worm Game..."
echo "========================================"

# Check if www directory exists
if [ ! -d "www" ]; then
    echo "Error: www directory not found."
    exit 1
fi

# Check if port 3000 is already in use and kill the process if it is
PORT=3000
if lsof -i :$PORT > /dev/null 2>&1; then
    echo "Port $PORT is already in use. Attempting to free it..."
    PID=$(lsof -t -i :$PORT)
    kill -9 $PID
    echo "Freed port $PORT."
fi

echo "Starting web server..."
echo "Game will be available at: http://localhost:3000/www/"
echo ""
echo "Controls:"
echo "  - WASD: Move snake/Select perk"
echo "  - Space: Restart after game over"
echo ""
echo "Press Ctrl+C to stop the server"
echo "========================================"

cd www && python3 serve.py