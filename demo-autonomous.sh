#!/bin/bash

echo "🤖 TuxPilot Autonomous Execution Demo"
echo "====================================="
echo "Showcasing AI-powered command execution with safety controls"
echo ""

# Colors for better output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}[DEMO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

print_feature() {
    echo -e "${PURPLE}[FEATURE]${NC} $1"
}

# 1. Build TuxPilot
print_step "1. Building TuxPilot with autonomous execution capabilities..."
cargo build --release

if [ $? -eq 0 ]; then
    print_success "TuxPilot built successfully!"
else
    echo "❌ Build failed!"
    exit 1
fi

echo ""

# 2. Show current capabilities
print_step "2. Demonstrating TuxPilot's autonomous execution features..."

echo ""
print_feature "🛡️ Safety System Demo"
echo "TuxPilot includes multiple safety layers:"
echo "  ✅ Command risk assessment (Safe → Critical)"
echo "  ✅ Permission verification"
echo "  ✅ User approval workflows"
echo "  ✅ Audit logging with rollback"
echo "  ✅ Pattern matching for dangerous commands"

echo ""
print_feature "🔐 Permission System Demo"
./target/release/tuxpilot permissions --detailed

echo ""
print_feature "📊 Audit System Demo"
./target/release/tuxpilot audit --limit 5

echo ""

# 3. Execution Mode Demonstrations
print_step "3. Execution Mode Demonstrations..."

echo ""
print_info "🔒 Supervised Mode (Default)"
echo "In supervised mode, TuxPilot:"
echo "  • Analyzes your request with AI"
echo "  • Generates appropriate commands"
echo "  • Shows risk assessment and explanation"
echo "  • Asks for your approval before execution"
echo "  • Logs everything for audit trail"

echo ""
print_info "Example supervised execution:"
echo "$ tuxpilot execute \"show me system information\""
./target/release/tuxpilot execute "show me system information" --mode supervised

echo ""
print_info "⚡ Semi-Autonomous Mode"
echo "In semi-autonomous mode, TuxPilot:"
echo "  • Executes safe commands automatically"
echo "  • Asks approval for medium/high risk operations"
echo "  • Perfect balance of efficiency and safety"

echo ""
print_info "🚀 Autonomous Mode"
echo "In autonomous mode, TuxPilot:"
echo "  • Executes most commands automatically"
echo "  • Only asks for critical operations"
echo "  • Maintains complete audit trail"
echo "  • Includes automatic rollback capabilities"

echo ""

# 4. Safety Demonstrations
print_step "4. Safety System Demonstrations..."

echo ""
print_feature "🚫 Dangerous Command Detection"
echo "TuxPilot automatically detects and blocks dangerous commands:"

echo ""
echo "Example dangerous patterns TuxPilot prevents:"
echo "  ❌ rm -rf / (system destruction)"
echo "  ❌ dd if=/dev/zero of=/dev/sda (disk wiping)"
echo "  ❌ chmod -R 777 /etc (security violation)"
echo "  ❌ curl malicious-site.com | sh (untrusted execution)"

echo ""
print_feature "✅ Safe Command Examples"
echo "Commands TuxPilot can execute safely:"
echo "  ✅ System information gathering"
echo "  ✅ Package queries and safe installations"
echo "  ✅ Service status checks and management"
echo "  ✅ Log analysis and monitoring"
echo "  ✅ Performance optimization"

echo ""

# 5. AI Integration Demo
print_step "5. AI Integration Demonstration..."

echo ""
print_feature "🤖 Local AI with Ollama"
if command -v ollama &> /dev/null; then
    if curl -s http://localhost:11434/api/tags &> /dev/null; then
        print_success "Ollama is running and ready!"
        echo "Available models:"
        ollama list | head -5
        
        echo ""
        print_info "Testing AI integration..."
        echo "$ tuxpilot execute \"check if nginx is running\""
        ./target/release/tuxpilot execute "check if nginx is running" --mode read-only
    else
        print_info "Ollama is installed but not running"
        echo "Start with: ollama serve &"
    fi
else
    print_info "Ollama not installed - install with: ./setup-ollama.sh"
fi

echo ""

# 6. Real-world Examples
print_step "6. Real-world Usage Examples..."

echo ""
print_feature "📦 Package Management"
echo "Natural language package operations:"
echo "  • \"install docker and start the service\""
echo "  • \"update all packages safely\""
echo "  • \"find packages related to python development\""
echo "  • \"remove unused packages and clean cache\""

echo ""
print_feature "⚙️ Service Management"
echo "Intelligent service operations:"
echo "  • \"restart nginx and check if it's working\""
echo "  • \"enable ssh service for remote access\""
echo "  • \"troubleshoot why apache won't start\""
echo "  • \"optimize services for better performance\""

echo ""
print_feature "🔍 System Diagnosis"
echo "AI-powered troubleshooting:"
echo "  • \"my system is running slow, fix it\""
echo "  • \"find and fix permission issues\""
echo "  • \"analyze recent errors and suggest solutions\""
echo "  • \"optimize system for gaming performance\""

echo ""

# 7. Configuration Demo
print_step "7. Configuration and Customization..."

echo ""
print_feature "⚙️ TuxPilot Configuration"
./target/release/tuxpilot config --show

echo ""
print_info "Configuration highlights:"
echo "  • Multiple AI providers (Ollama, OpenAI, Anthropic)"
echo "  • Automatic system detection (package manager, services)"
echo "  • Customizable safety levels"
echo "  • Audit log configuration"

echo ""

# 8. Future Features Preview
print_step "8. Upcoming Features Preview..."

echo ""
print_feature "🔮 Coming Soon in TuxPilot"
echo ""
echo "🤖 Multi-Agent System:"
echo "  • Specialized AI agents (Security, Performance, Network)"
echo "  • Agent collaboration for complex tasks"
echo "  • Continuous learning and improvement"

echo ""
echo "🔌 MCP Integration:"
echo "  • Model Context Protocol support"
echo "  • Advanced tool communication"
echo "  • Cross-system integration"

echo ""
echo "🌐 Web Interface:"
echo "  • Remote system management"
echo "  • Real-time monitoring dashboard"
echo "  • Multi-server orchestration"

echo ""
echo "🔧 Plugin System:"
echo "  • Community-driven extensions"
echo "  • Custom tool integration"
echo "  • Enterprise connectors"

echo ""

# 9. Getting Started
print_step "9. Getting Started with TuxPilot..."

echo ""
print_feature "🚀 Quick Start Guide"
echo ""
echo "1. Start with supervised mode to learn:"
echo "   $ tuxpilot chat --execute-mode supervised"

echo ""
echo "2. Try some safe commands:"
echo "   $ tuxpilot execute \"show system information\""
echo "   $ tuxpilot execute \"check disk usage\""

echo ""
echo "3. Review permissions and audit logs:"
echo "   $ tuxpilot permissions --detailed"
echo "   $ tuxpilot audit --limit 10"

echo ""
echo "4. Set up local AI for privacy:"
echo "   $ ./setup-ollama.sh"

echo ""
echo "5. Gradually move to more autonomous modes:"
echo "   $ tuxpilot chat --execute-mode semi-auto"

echo ""

# 10. Summary
print_step "10. Demo Summary"

echo ""
print_success "🎉 TuxPilot Autonomous Execution Demo Complete!"

echo ""
echo "Key Takeaways:"
echo "  ✅ TuxPilot can safely execute commands with AI guidance"
echo "  ✅ Multiple execution modes for different use cases"
echo "  ✅ Comprehensive safety system with audit trails"
echo "  ✅ Local AI support for privacy and offline use"
echo "  ✅ Natural language interface for system management"
echo "  ✅ Extensible architecture for future enhancements"

echo ""
echo "Next Steps:"
echo "  1. Try TuxPilot in your environment"
echo "  2. Start with supervised mode for safety"
echo "  3. Review audit logs to understand operations"
echo "  4. Contribute to the project on GitHub"
echo "  5. Join the community discussions"

echo ""
print_feature "🔗 Resources"
echo "  📖 Documentation: docs/"
echo "  🐛 Issues: github.com/yourusername/tuxpilot/issues"
echo "  💬 Discussions: github.com/yourusername/tuxpilot/discussions"
echo "  📧 Support: support@tuxpilot.dev"

echo ""
print_success "Thank you for exploring TuxPilot! 🐧🚀"
echo ""
echo "TuxPilot: Your trusted AI companion for Linux system administration"
