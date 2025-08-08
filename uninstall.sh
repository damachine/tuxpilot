#!/bin/bash

# TuxPilot Uninstallation Script
# This script removes TuxPilot from the system

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
    echo -e "${RED}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                          TuxPilot Uninstaller                               ║"
    echo "║                    Remove TuxPilot from your system                         ║"
    echo "║                                v0.1.0                                       ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

confirm_uninstall() {
    echo ""
    print_warning "This will remove TuxPilot from your system, including:"
    echo "  • Binary: /usr/local/bin/tuxpilot"
    echo "  • Web UI: /usr/local/share/tuxpilot/"
    echo "  • Shell completions"
    echo ""
    print_info "Your configuration files in ~/.config/tuxpilot/ will be preserved."
    echo ""
    
    read -p "Are you sure you want to uninstall TuxPilot? (y/N): " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Uninstallation cancelled."
        exit 0
    fi
}

remove_binary() {
    print_info "Removing TuxPilot binary..."
    
    if [ -f "/usr/local/bin/tuxpilot" ]; then
        sudo rm -f /usr/local/bin/tuxpilot
        print_success "Binary removed: /usr/local/bin/tuxpilot"
    else
        print_warning "Binary not found: /usr/local/bin/tuxpilot"
    fi
}

remove_web_ui() {
    print_info "Removing Web UI files..."
    
    if [ -d "/usr/local/share/tuxpilot" ]; then
        sudo rm -rf /usr/local/share/tuxpilot
        print_success "Web UI removed: /usr/local/share/tuxpilot/"
    else
        print_warning "Web UI directory not found: /usr/local/share/tuxpilot/"
    fi
}

remove_completions() {
    print_info "Removing shell completions..."
    
    # Remove bash completions
    if [ -f "$HOME/.local/share/bash-completion/completions/tuxpilot" ]; then
        rm -f "$HOME/.local/share/bash-completion/completions/tuxpilot"
        print_success "Bash completions removed"
    fi
    
    # Remove zsh completions if they exist
    if [ -f "$HOME/.local/share/zsh/site-functions/_tuxpilot" ]; then
        rm -f "$HOME/.local/share/zsh/site-functions/_tuxpilot"
        print_success "Zsh completions removed"
    fi
}

show_config_info() {
    echo ""
    print_info "Configuration files preserved:"
    
    if [ -d "$HOME/.config/tuxpilot" ]; then
        echo "  • Configuration: $HOME/.config/tuxpilot/"
        echo ""
        print_warning "To completely remove TuxPilot including configuration:"
        echo "  rm -rf $HOME/.config/tuxpilot/"
    else
        print_info "No configuration directory found."
    fi
}

verify_removal() {
    print_info "Verifying removal..."
    
    local all_removed=true
    
    if [ -f "/usr/local/bin/tuxpilot" ]; then
        print_warning "Binary still exists: /usr/local/bin/tuxpilot"
        all_removed=false
    fi
    
    if [ -d "/usr/local/share/tuxpilot" ]; then
        print_warning "Web UI directory still exists: /usr/local/share/tuxpilot/"
        all_removed=false
    fi
    
    if [ "$all_removed" = true ]; then
        print_success "TuxPilot successfully removed from system"
    else
        print_warning "Some files may still exist. Please check manually."
    fi
}

show_completion_message() {
    echo ""
    print_success "🗑️  TuxPilot uninstallation completed!"
    echo ""
    echo -e "${BLUE}What was removed:${NC}"
    echo "• TuxPilot binary from /usr/local/bin/"
    echo "• Web UI files from /usr/local/share/tuxpilot/"
    echo "• Shell completion scripts"
    echo ""
    echo -e "${BLUE}What was preserved:${NC}"
    echo "• Your configuration files in ~/.config/tuxpilot/"
    echo "• Any custom scripts or data you created"
    echo ""
    echo -e "${GREEN}Thank you for using TuxPilot! 🐧${NC}"
    echo "Feel free to reinstall anytime or contribute to the project:"
    echo "https://github.com/damachine/tuxpilot"
}

main() {
    print_header
    
    confirm_uninstall
    remove_binary
    remove_web_ui
    remove_completions
    verify_removal
    show_config_info
    show_completion_message
}

# Run main function
main "$@"