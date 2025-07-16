#!/bin/bash

# Build script for WebAssembly target
echo "Building Space Worm for WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Please install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf pkg/
rm -rf www/pkg/

# Build the WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg --out-name space_worm

if [ $? -eq 0 ]; then
    echo "✅ WASM build successful!"
    
    # Copy the generated files to www directory
    echo "Copying files to www directory..."
    cp -r pkg/ www/
    
    echo "✅ WASM files copied to www/pkg/"
    echo ""
    echo "🌐 To serve the game locally, run:"
    echo "   ./scripts/serve.sh"
    echo ""
    echo "📁 Files ready for deployment in www/ directory"
else
    echo "❌ WASM build failed!"
    exit 1
fi