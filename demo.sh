#!/bin/bash

echo "🐧 TuxPilot Demo - Linux AI Copilot"
echo "=================================="
echo ""

echo "1. Building TuxPilot..."
cargo build --release

echo ""
echo "2. Showing help information:"
echo "$ tuxpilot --help"
./target/release/tuxpilot --help

echo ""
echo "3. Showing version:"
echo "$ tuxpilot --version"
./target/release/tuxpilot --version

echo ""
echo "4. Testing package management suggestions (without AI):"
echo "This would normally provide AI-powered advice, but shows the basic command structure:"
echo ""

echo "For Arch Linux (pacman):"
echo "- Install package: sudo pacman -S firefox"
echo "- Remove package: sudo pacman -R firefox"
echo "- Update system: sudo pacman -Syu"
echo "- Search packages: pacman -Ss firefox"

echo ""
echo "5. System Information (basic):"
echo "$ uname -a"
uname -a

echo ""
echo "$ uptime"
uptime

echo ""
echo "$ free -h"
free -h

echo ""
echo "$ df -h /"
df -h /

echo ""
echo "6. Service Management Examples:"
echo "$ systemctl status nginx"
systemctl status nginx 2>/dev/null || echo "nginx service not found (expected)"

echo ""
echo "7. Log Analysis Examples:"
echo "Recent system errors (last 5 lines):"
echo "$ journalctl -p err -n 5 --no-pager"
journalctl -p err -n 5 --no-pager 2>/dev/null || echo "No recent errors or insufficient permissions"

echo ""
echo "=================================="
echo "🎉 TuxPilot Demo Complete!"
echo ""
echo "To use TuxPilot with AI features:"
echo "1. Get an API key from OpenAI or Anthropic"
echo "2. Configure it in ~/.config/tuxpilot/config.toml"
echo "3. Run: tuxpilot chat"
echo ""
echo "For local AI (experimental):"
echo "1. Build with: cargo build --release --features local-ai"
echo "2. Run with: tuxpilot --local"
echo ""
echo "Project structure created successfully!"
echo "Ready for development and customization."
