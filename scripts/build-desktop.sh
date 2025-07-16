#!/bin/bash

# Build script for desktop target
echo "Building Space Worm for desktop..."

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build the desktop version
echo "Building desktop executable..."
cargo build --release --bin space_worm

if [ $? -eq 0 ]; then
    echo "✅ Desktop build successful!"
    echo ""
    echo "🎮 To run the game:"
    echo "   cargo run --bin space_worm"
    echo "   OR"
    echo "   ./target/release/space_worm"
    echo ""
    echo "📁 Executable created at: target/release/space_worm"
else
    echo "❌ Desktop build failed!"
    exit 1
fi