#!/bin/bash

# TuxPilot Installation Script
# This script installs TuxPilot on Linux systems

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
    echo "║                            TuxPilot Installer                               ║"
    echo "║                     AI-Powered Linux System Administrator                   ║"
    echo "║                                v0.1.0                                       ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

check_requirements() {
    print_info "Checking system requirements..."

    # Check if running on Linux
    if [[ "$OSTYPE" != "linux-gnu"* ]]; then
        print_error "TuxPilot is designed for Linux systems only"
        exit 1
    fi

    # Check for Rust/Cargo
    if ! command -v cargo &> /dev/null; then
        print_warning "Cargo not found. Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    fi

    # Check for Git
    if ! command -v git &> /dev/null; then
        print_error "Git is required but not installed"
        exit 1
    fi

    # Check for Node.js and npm (required for web UI)
    if ! command -v node &> /dev/null; then
        print_warning "Node.js not found. The web UI requires Node.js to build."
        print_info "Please install Node.js from https://nodejs.org/ or using your package manager:"
        print_info "  • Ubuntu/Debian: sudo apt install nodejs npm"
        print_info "  • Arch Linux: sudo pacman -S nodejs npm"
        print_info "  • Fedora: sudo dnf install nodejs npm"
        print_info "  • openSUSE: sudo zypper install nodejs npm"
        print_warning "Continuing without web UI build..."
        WEB_UI_AVAILABLE=false
    elif ! command -v npm &> /dev/null; then
        print_warning "npm not found. The web UI requires npm to build."
        print_info "Please install npm using your package manager or Node.js installer."
        print_warning "Continuing without web UI build..."
        WEB_UI_AVAILABLE=false
    else
        print_success "Node.js and npm found - web UI will be built"
        WEB_UI_AVAILABLE=true
    fi

    print_success "System requirements satisfied"
}

install_dependencies() {
    print_info "Installing system dependencies..."
    
    # Detect package manager and install dependencies
    if command -v pacman &> /dev/null; then
        sudo pacman -S --needed --noconfirm openssl pkg-config
    elif command -v apt &> /dev/null; then
        sudo apt update
        sudo apt install -y libssl-dev pkg-config build-essential
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y openssl-devel pkg-config gcc
    elif command -v zypper &> /dev/null; then
        sudo zypper install -y libopenssl-devel pkg-config gcc
    else
        print_warning "Unknown package manager. Please install openssl-dev and pkg-config manually"
    fi
    
    print_success "Dependencies installed"
}

build_web_ui() {
    if [ "$WEB_UI_AVAILABLE" = true ]; then
        print_info "Building web UI..."

        # Navigate to web-ui directory
        cd web-ui

        # Install dependencies
        print_info "Installing web UI dependencies..."
        npm install

        if [ $? -ne 0 ]; then
            print_error "Failed to install web UI dependencies"
            cd ..
            return 1
        fi

        # Build the web UI
        print_info "Building Svelte application..."
        npm run build

        if [ $? -ne 0 ]; then
            print_error "Failed to build web UI"
            cd ..
            return 1
        fi

        # Copy built files to static directory (for development)
        print_info "Copying web UI files to static directory..."
        cp -r build/* ../static/

        if [ $? -eq 0 ]; then
            print_success "Web UI built successfully"
        else
            print_error "Failed to copy web UI files"
            cd ..
            return 1
        fi

        # Return to project root
        cd ..
    else
        print_warning "Skipping web UI build (Node.js/npm not available)"
        print_info "TuxPilot will still work, but the web interface will not be available"
    fi
}

build_tuxpilot() {
    print_info "Building TuxPilot..."

    # Build web UI first
    build_web_ui

    # Build Rust backend in release mode
    print_info "Building Rust backend..."
    cargo build --release

    if [ $? -eq 0 ]; then
        print_success "TuxPilot built successfully"
    else
        print_error "Build failed"
        exit 1
    fi
}

install_binary() {
    print_info "Installing TuxPilot binary..."
    
    # Create installation directory
    sudo mkdir -p /usr/local/bin
    
    # Copy binary
    sudo cp target/release/tuxpilot /usr/local/bin/
    sudo chmod +x /usr/local/bin/tuxpilot
    
    # Create config directory
    mkdir -p ~/.config/tuxpilot
    
    print_success "TuxPilot installed to /usr/local/bin/tuxpilot"
}

install_web_ui() {
    if [ "$WEB_UI_AVAILABLE" = true ] && [ -d "web-ui/build" ]; then
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
        print_info "Web UI will be served from system location when running 'tuxpilot web'"
    else
        print_warning "Skipping Web UI system installation (not built or Node.js/npm not available)"
        print_info "Web UI will fall back to development location if available"
    fi
}

setup_completion() {
    print_info "Setting up shell completion..."
    
    # Generate completion scripts
    mkdir -p ~/.local/share/bash-completion/completions
    /usr/local/bin/tuxpilot --help > /dev/null 2>&1 || true
    
    print_success "Shell completion configured"
}

verify_installation() {
    print_info "Verifying installation..."
    
    if command -v tuxpilot &> /dev/null; then
        VERSION=$(tuxpilot --version)
        print_success "Installation verified: $VERSION"
        
        print_info "Testing basic functionality..."
        tuxpilot permissions > /dev/null
        tuxpilot config --show > /dev/null
        
        print_success "Basic functionality test passed"
        
        # Verify Web UI installation
        if [ "$WEB_UI_AVAILABLE" = true ]; then
            if [ -f "/usr/local/share/tuxpilot/web-ui/index.html" ]; then
                print_success "Web UI system installation verified"
            else
                print_warning "Web UI system installation not found, will use development fallback"
            fi
        fi
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

show_next_steps() {
    echo ""
    print_success "🎉 TuxPilot installation completed successfully!"
    echo ""
    echo -e "${BLUE}Next Steps:${NC}"
    echo "1. Run 'tuxpilot --help' to see available commands"
    echo "2. Try 'tuxpilot config --show' to view your system configuration"
    echo "3. Use 'tuxpilot permissions' to understand the permission system"
    echo "4. Start with 'tuxpilot chat' for interactive assistance"

    if [ "$WEB_UI_AVAILABLE" = true ]; then
        echo "5. Launch the web interface with 'tuxpilot web' and visit http://127.0.0.1:8080"
    fi

    echo ""
    echo -e "${BLUE}Available Interfaces:${NC}"
    echo "• Command Line: 'tuxpilot chat' for interactive terminal chat"

    if [ "$WEB_UI_AVAILABLE" = true ]; then
        echo "• Web Interface: 'tuxpilot web' for modern browser-based interface"
        echo "  - Installed to: /usr/local/share/tuxpilot/web-ui/"
        echo "  - ChatGPT-inspired dark theme"
        echo "  - Real-time chat with AI agents"
        echo "  - Configuration management"
        echo "  - System dashboard"
    else
        echo "• Web Interface: Not available (Node.js/npm required for building)"
    fi

    echo ""
    echo -e "${BLUE}Optional:${NC}"
    echo "• Install Ollama for local AI: https://ollama.ai/"
    echo "• Read the documentation: docs/GETTING_STARTED.md"
    echo "• Join the community: https://github.com/damachine/tuxpilot"
    echo ""
    echo -e "${GREEN}Happy system administration! 🐧🤖${NC}"
}

main() {
    print_header
    
    check_requirements
    install_dependencies
    build_tuxpilot
    install_binary
    install_web_ui
    setup_completion
    verify_installation
    show_next_steps
}

# Run main function
main "$@"
