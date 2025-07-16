#!/bin/bash

# Local development server for WASM version
echo "Starting local development server for Space Worm WASM..."

# Check if the WASM build exists
if [ ! -d "www/pkg" ]; then
    echo "❌ WASM build not found. Please run ./scripts/build-wasm.sh first"
    exit 1
fi

# Check for available servers and use the first one found
if command -v python3 &> /dev/null; then
    echo "🌐 Starting Python 3 HTTP server on http://localhost:3000"
    echo "Press Ctrl+C to stop the server"
    cd www && python3 -m http.server 3000
elif command -v python &> /dev/null; then
    echo "🌐 Starting Python HTTP server on http://localhost:3000"
    echo "Press Ctrl+C to stop the server"
    cd www && python -m SimpleHTTPServer 3000
elif command -v node &> /dev/null; then
    # Check if npx is available and try to use http-server
    if command -v npx &> /dev/null; then
        echo "🌐 Starting Node.js HTTP server on http://localhost:3000"
        echo "Press Ctrl+C to stop the server"
        cd www && npx http-server -p 3000 -c-1
    else
        echo "❌ Node.js found but npx not available. Please install http-server globally:"
        echo "   npm install -g http-server"
        exit 1
    fi
elif command -v php &> /dev/null; then
    echo "🌐 Starting PHP built-in server on http://localhost:3000"
    echo "Press Ctrl+C to stop the server"
    cd www && php -S localhost:3000
else
    echo "❌ No suitable HTTP server found."
    echo "Please install one of the following:"
    echo "  - Python 3: python3 -m http.server"
    echo "  - Node.js: npm install -g http-server"
    echo "  - PHP: built-in server"
    echo ""
    echo "Or manually serve the www/ directory with any HTTP server on port 3000"
    exit 1
fi