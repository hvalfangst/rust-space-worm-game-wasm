#!/bin/bash

# Deployment script for web hosting
echo "Preparing Space Worm for web deployment..."

# Build the WASM version first
echo "Building WASM version..."
./scripts/build-wasm.sh

if [ $? -ne 0 ]; then
    echo "❌ WASM build failed, cannot deploy"
    exit 1
fi

# Create deployment directory
DEPLOY_DIR="deploy"
echo "Creating deployment package in $DEPLOY_DIR/..."
rm -rf $DEPLOY_DIR
mkdir -p $DEPLOY_DIR

# Copy all necessary files
cp -r www/* $DEPLOY_DIR/

# Copy assets if they exist
if [ -d "assets" ]; then
    echo "Copying game assets..."
    cp -r assets/ $DEPLOY_DIR/assets/
fi

# Create a simple .htaccess for Apache servers (optional)
cat > $DEPLOY_DIR/.htaccess << 'EOF'
# Enable CORS for WASM files
<FilesMatch "\.(wasm)$">
    Header set Access-Control-Allow-Origin "*"
</FilesMatch>

# Set correct MIME types
AddType application/wasm .wasm

# Enable compression
<FilesMatch "\.(js|css|html|wasm)$">
    SetOutputFilter DEFLATE
</FilesMatch>
EOF

echo "✅ Deployment package ready in $DEPLOY_DIR/ directory"
echo ""
echo "📁 Contents:"
ls -la $DEPLOY_DIR/
echo ""
echo "🌐 Upload the contents of $DEPLOY_DIR/ to your web server"
echo "   Make sure your server serves .wasm files with the correct MIME type"
echo ""
echo "🔧 For GitHub Pages, you can:"
echo "   1. Push the contents to gh-pages branch"
echo "   2. Enable GitHub Pages in repository settings"
echo ""
echo "⚡ For other hosting platforms:"
echo "   - Netlify: Drag and drop the $DEPLOY_DIR folder"
echo "   - Vercel: Deploy the $DEPLOY_DIR folder"
echo "   - Firebase: firebase deploy --public $DEPLOY_DIR"