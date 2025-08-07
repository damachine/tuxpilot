#!/bin/bash

# TuxPilot Web UI Build Script
# This script builds the Svelte web UI and copies it to the static directory

set -e

echo "🌐 Building TuxPilot Web UI..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js to build the web UI."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install npm to build the web UI."
    exit 1
fi

# Navigate to web-ui directory
cd web-ui

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Build the web UI
echo "🔨 Building web UI..."
npm run build

# Copy built files to static directory
echo "📁 Copying files to static directory..."
cp -r build/* ../static/

echo "✅ Web UI build complete!"
echo ""
echo "The web UI has been built and is ready to be served by TuxPilot."
echo "You can now run: ./target/release/tuxpilot web"
