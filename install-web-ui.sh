#!/bin/bash

# TuxPilot Web UI Installation Script
# This script builds and installs only the Web UI component

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${GREEN}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                        TuxPilot Web UI Installer                            ║"
    echo "║                     Build and Install Web Interface                         ║"
    echo "║                                v0.1.0                                       ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

check_requirements() {
    print_info "Checking requirements for Web UI..."

    # Check for Node.js and npm
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js from https://nodejs.org/"
        print_info "Package manager installation:"
        print_info "  • Ubuntu/Debian: sudo apt install nodejs npm"
        print_info "  • Arch Linux: sudo pacman -S nodejs npm"
        print_info "  • Fedora: sudo dnf install nodejs npm"
        print_info "  • openSUSE: sudo zypper install nodejs npm"
        exit 1
    fi

    if ! command -v npm &> /dev/null; then
        print_error "npm not found. Please install npm."
        exit 1
    fi

    # Check if we're in the right directory
    if [ ! -d "web-ui" ]; then
        print_error "web-ui directory not found. Please run this script from the TuxPilot root directory."
        exit 1
    fi

    print_success "Requirements satisfied"
}

build_web_ui() {
    print_info "Building Web UI..."

    # Navigate to web-ui directory
    cd web-ui

    # Install dependencies
    print_info "Installing dependencies..."
    npm install

    if [ $? -ne 0 ]; then
        print_error "Failed to install dependencies"
        exit 1
    fi

    # Build the web UI
    print_info "Building Svelte application..."
    npm run build

    if [ $? -ne 0 ]; then
        print_error "Failed to build Web UI"
        exit 1
    fi

    print_success "Web UI built successfully"

    # Return to project root
    cd ..
}

install_web_ui() {
    print_info "Installing Web UI to system location..."
    
    # Create web UI installation directory
    sudo mkdir -p /usr/local/share/tuxpilot/web-ui
    
    # Remove old installation if it exists
    if [ -d "/usr/local/share/tuxpilot/web-ui" ]; then
        print_info "Removing previous Web UI installation..."
        sudo rm -rf /usr/local/share/tuxpilot/web-ui/*
    fi
    
    # Copy built web UI files
    sudo cp -r web-ui/build/* /usr/local/share/tuxpilot/web-ui/
    
    # Set appropriate permissions
    sudo chmod -R 644 /usr/local/share/tuxpilot/web-ui/
    sudo find /usr/local/share/tuxpilot/web-ui/ -type d -exec chmod 755 {} \;
    
    print_success "Web UI installed to /usr/local/share/tuxpilot/web-ui/"
}

copy_to_static() {
    print_info "Copying Web UI files to static directory for development..."
    
    # Copy built files to static directory for development use
    cp -r web-ui/build/* static/
    
    print_success "Web UI files copied to static/ directory"
}

verify_installation() {
    print_info "Verifying Web UI installation..."
    
    # Check if system installation exists
    if [ -f "/usr/local/share/tuxpilot/web-ui/index.html" ]; then
        print_success "System installation verified: /usr/local/share/tuxpilot/web-ui/"
    else
        print_warning "System installation not found"
    fi
    
    # Check if static files exist for development
    if [ -f "static/index.html" ]; then
        print_success "Development files verified: static/"
    else
        print_warning "Development files not found in static/"
    fi
}

show_next_steps() {
    echo ""
    print_success "🎉 Web UI installation completed!"
    echo ""
    echo -e "${BLUE}Next Steps:${NC}"
    echo "1. Start TuxPilot web server: 'tuxpilot web'"
    echo "2. Open your browser and visit: http://127.0.0.1:8080"
    echo "3. The Web UI will automatically load from the system installation"
    echo ""
    echo -e "${BLUE}Features:${NC}"
    echo "• Modern Svelte-based interface"
    echo "• ChatGPT-inspired dark theme"
    echo "• Real-time chat with AI agents"
    echo "• Configuration management"
    echo "• System dashboard"
    echo ""
    echo -e "${GREEN}Enjoy the new Web UI! 🌐✨${NC}"
}

main() {
    print_header
    
    check_requirements
    build_web_ui
    install_web_ui
    copy_to_static
    verify_installation
    show_next_steps
}

# Run main function
main "$@"