#!/bin/bash

# Check if "build" flag is supplied
if [ "$1" == "build" ]; then
    # Existing build logic
    echo "Building WASM module..."
    wasm-pack build --target web --out-dir pkg --out-name space_worm

    if [ $? -ne 0 ]; then
        echo "Error: WASM build failed."
        exit 1
    fi

    echo "Copying WASM files to www directory..."
    cp -r pkg/ www/

    echo "Copying assets to www directory..."
    cp -r assets www/

    touch www/.nojekyll
    echo "✅ Build complete!"
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
echo "  - Mobile: Toggle button in top-right"
echo ""
echo "Press Ctrl+C to stop the server"
echo "========================================"

cd scripts && python3 serve.py