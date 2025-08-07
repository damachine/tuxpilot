#!/bin/bash

# TuxPilot Complete Build Script
# This script builds both the Rust backend and Svelte web UI

set -e

echo "🚀 Building TuxPilot (Complete Build)"
echo "======================================"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check dependencies
echo "🔍 Checking dependencies..."

if ! command_exists cargo; then
    echo "❌ Rust/Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

if ! command_exists node; then
    echo "❌ Node.js is not installed. Please install Node.js to build the web UI."
    exit 1
fi

if ! command_exists npm; then
    echo "❌ npm is not installed. Please install npm to build the web UI."
    exit 1
fi

echo "✅ All dependencies found"
echo ""

# Build Web UI
echo "🌐 Building Web UI..."
echo "--------------------"
cd web-ui

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing web UI dependencies..."
    npm install
fi

# Build the web UI
echo "🔨 Building Svelte application..."
npm run build

# Copy to static directory
echo "📁 Copying web UI files to static directory..."
cp -r build/* ../static/

cd ..
echo "✅ Web UI build complete"
echo ""

# Build Rust Backend
echo "🦀 Building Rust Backend..."
echo "---------------------------"
echo "🔨 Building TuxPilot binary..."
cargo build --release

echo "✅ Rust backend build complete"
echo ""

# Final summary
echo "🎉 Build Complete!"
echo "=================="
echo ""
echo "📁 Built files:"
echo "   • Rust binary: ./target/release/tuxpilot"
echo "   • Web UI: ./static/ (served automatically)"
echo ""
echo "🚀 To start TuxPilot:"
echo "   ./target/release/tuxpilot web"
echo ""
echo "🌐 Web interface will be available at:"
echo "   http://127.0.0.1:8080"
