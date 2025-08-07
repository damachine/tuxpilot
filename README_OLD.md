# TuxPilot 🐧

**Ihr intelligenter AI-Linux-Systemadministrator-Assistent**

TuxPilot ist ein fortschrittlicher KI-gestützter Assistent für Linux-Systeme, der Ihnen bei alltäglichen Systemverwaltungsaufgaben hilft - von Sicherheit über Gaming bis hin zu professioneller Serveradministration. Mit natürlicher Sprachverarbeitung, autonomer Befehlsausführung und umfassenden Sicherheitskontrollen.

## 🎯 **Für wen ist TuxPilot?**

- **🏠 Heimanwender**: Einfache Systemwartung und Problemlösung
- **🎮 Gamer**: Optimierung für Gaming-Performance und Hardware
- **👨‍💻 Entwickler**: Automatisierte Entwicklungsumgebung-Setup
- **🏢 Systemadministratoren**: Professionelle Server- und Netzwerkverwaltung
- **🔒 Sicherheitsexperten**: Umfassende Sicherheitsanalysen und Härtung
- **📚 Linux-Lernende**: Interaktive Hilfe und Erklärungen

## 🚀 **Hauptfunktionen**

### **🤖 Intelligente Systemverwaltung**
- **Autonome Befehlsausführung** mit mehrstufigen Sicherheitskontrollen
- **Natürliche Sprachverarbeitung** - sprechen Sie mit Ihrem System auf Deutsch
- **Intelligente Fehlerdiagnose** mit KI-gestützter Problemlösung
- **Automatische Systemoptimierung** für verschiedene Anwendungsfälle

### **🛡️ Sicherheit & Compliance**
- **Mehrstufiges Sicherheitssystem** mit Risikoanalyse
- **Granulare Berechtigungen** mit Benutzer-Genehmigungsworkflows
- **Vollständige Audit-Protokolle** aller Systemoperationen
- **Sicherheitsscans** und automatische Härtungsempfehlungen

### **📦 Universelle Paketverwaltung**
- **Multi-Distributionen-Support**: Arch, Ubuntu, Fedora, openSUSE, Debian
- **Intelligente Paketoperationen** mit Abhängigkeitsauflösung
- **Automatische Updates** mit Sicherheitsprüfungen
- **Rollback-Funktionen** für sichere Systemänderungen

## 🎯 Execution Modes

TuxPilot offers multiple execution modes to balance automation with safety:

### **🔒 Supervised Mode** (Default)
- AI suggests commands and asks for approval before execution
- Perfect for learning and maintaining control
- Shows detailed explanations and risk assessments

### **⚡ Semi-Autonomous Mode**
- Executes safe commands automatically (read-only operations)
- Asks for approval on medium/high-risk operations
- Ideal for experienced users who want efficiency with safety

### **🚀 Autonomous Mode**
- Executes most commands automatically with safety checks
- Only asks for approval on critical operations
- Best for trusted environments and routine maintenance

### **📖 Read-Only Mode**
- Only performs read operations and system analysis
- No system modifications allowed
- Perfect for system inspection and learning

## 🛠️ Installation

### Prerequisites

- Linux operating system (tested on Arch Linux, Ubuntu, Fedora)
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Optional: Ollama for local AI (recommended)

### Quick Setup with Ollama (Recommended)

```bash
# Clone and build TuxPilot
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot
cargo build --release

# Setup Ollama for local AI (no API keys needed!)
./setup-ollama.sh

# Start using TuxPilot
./target/release/tuxpilot chat --execute-mode supervised
```

### Manual Installation

```bash
# Build TuxPilot
cargo build --release

# Install system-wide (optional)
sudo cp target/release/tuxpilot /usr/local/bin/

# Configure for your system
tuxpilot config --show
```

## 🎮 Usage Examples

### **Autonomous System Management**

```bash
# Interactive AI assistant with command execution
tuxpilot chat --execute-mode supervised  # Ask before executing
tuxpilot chat --execute-mode auto        # Execute automatically

# Natural language system management
tuxpilot execute "install docker and start the service"
tuxpilot execute "find and fix permission issues in /var/log"
tuxpilot execute "optimize system for gaming performance"
tuxpilot execute "update all packages safely"

# Autonomous maintenance
tuxpilot diagnose --auto --fix           # Find and fix issues
tuxpilot optimize --auto                 # Optimize performance
tuxpilot update --auto --safe            # Safe system updates
```

### **Safety and Permissions**

```bash
# Check your permissions and capabilities
tuxpilot permissions --detailed

# View audit log of all operations
tuxpilot audit --limit 20
tuxpilot audit --export json > audit.json

# Review safety settings
tuxpilot config --show
```

### **Traditional Assistance Mode**

```bash
# Get help without execution
tuxpilot package install firefox         # Installation guidance
tuxpilot service nginx troubleshoot      # Service help
tuxpilot monitor --continuous            # System monitoring
tuxpilot diagnose --input "error message" # Error analysis
```

### **Example Conversations**

```
user@system:~$ tuxpilot chat --execute-mode supervised

🤖 TuxPilot: Hi! I'm your AI Linux assistant. I can help you manage your system 
with autonomous command execution. What would you like to do?

You: My system is running slow, can you help optimize it?

🤖 TuxPilot: I'll analyze your system performance and suggest optimizations.

🔍 Analyzing system performance...
📊 Found: High memory usage (87%), several unnecessary services running
⚡ Optimization plan:
  1. Stop unused services (cups, bluetooth)
  2. Clear package cache (pacman -Sc)
  3. Optimize swappiness setting

🔧 Command: systemctl stop cups bluetooth
⚠️  Risk Level: Medium
🎯 Expected: Free ~200MB RAM, improve responsiveness
🔄 Rollback: systemctl start cups bluetooth

Execute this command? [y/N]: y

✅ Command executed successfully!
📈 Memory usage reduced to 72%

Continue with package cache cleanup? [y/N]: y
```

## 🛡️ Safety Features

### **Multi-Layer Safety System**

1. **Command Analysis**: AI analyzes every command for potential risks
2. **Pattern Matching**: Detects dangerous patterns (rm -rf /, dd to devices, etc.)
3. **Permission Checking**: Verifies user has required permissions
4. **Risk Assessment**: Categorizes commands by risk level (Safe → Critical)
5. **User Approval**: Requests confirmation for risky operations
6. **Rollback Planning**: Provides rollback commands when possible

### **Audit and Compliance**

- **Complete Audit Trail**: Every command logged with timestamp and user
- **Permission Tracking**: All permission requests and grants logged
- **Safety Violations**: Blocked dangerous commands recorded
- **Export Capabilities**: Audit logs exportable in JSON, CSV, HTML formats
- **Rollback Support**: Automatic rollback for reversible operations

### **Permission System**

```
🔐 Permission Categories:
├── ReadSystem        # System information, file reading
├── WriteSystem       # File modifications, system changes
├── PackageManagement # Install/remove packages
├── ServiceManagement # Start/stop/configure services
├── NetworkAccess     # Network operations
├── UserManagement    # User/group management
└── SystemConfiguration # Kernel params, mount points
```

## 🔌 AI Integration

### **Local AI with Ollama** (Recommended)

```bash
# Automatic setup
./setup-ollama.sh

# Manual setup
curl -fsSL https://ollama.ai/install.sh | sh
ollama serve &
ollama pull llama3.1:8b
```

**Benefits of Local AI:**
- ✅ **Completely offline** - no internet required
- ✅ **No API costs** - free forever
- ✅ **Privacy** - all data stays local
- ✅ **Fast responses** - no network latency
- ✅ **Always available** - no rate limits

### **Cloud AI Support**

```toml
# ~/.config/tuxpilot/config.toml
[ai]
provider = "OpenAI"  # or "Anthropic"

[ai.openai]
api_key = "your-api-key"
model = "gpt-4"
```

## 🏗️ Architecture

### **Core Components**

```
TuxPilot Architecture:
├── 🤖 AI Engine (Ollama/OpenAI/Anthropic)
├── 🔧 Command Executor
│   ├── Safety Checker
│   ├── Permission Manager
│   └── Audit Logger
├── 🔍 Error Diagnosis Engine
├── 📊 System Monitor
├── 📦 Package Manager Integration
├── ⚙️ Service Manager Integration
└── 💬 Natural Language Interface
```

### **Safety Architecture**

```
Command Execution Pipeline:
User Input → AI Analysis → Safety Check → Permission Check → User Approval → Execute → Audit Log
     ↓            ↓            ↓              ↓               ↓           ↓         ↓
Natural      Command      Risk         Permission      Confirmation   Safe      Complete
Language     Generation   Assessment   Verification    Dialog         Execution Logging
```

## 🎯 Roadmap

### **Current (v0.1.0)**
- ✅ Basic command execution with safety checks
- ✅ Ollama integration for local AI
- ✅ Permission system foundation
- ✅ Audit logging
- ✅ Multi-distribution support

### **Next (v0.2.0)**
- 🔄 **MCP Integration** - Model Context Protocol support
- 🔄 **Multi-Agent System** - Specialized AI agents
- 🔄 **Advanced Safety** - ML-based risk assessment
- 🔄 **Web Interface** - Remote system management

### **Future (v1.0.0)**
- 🔮 **Autonomous Operations** - Self-managing systems
- 🔮 **Plugin Ecosystem** - Community extensions
- 🔮 **Enterprise Features** - Multi-server management
- 🔮 **Cloud Integration** - Multi-cloud support

## 🤝 Contributing

We welcome contributions! TuxPilot is designed to be the ultimate Linux AI assistant.

### **Areas for Contribution**
- 🔧 **Core Features** - Command execution, safety systems
- 🤖 **AI Integration** - New AI providers, local models
- 🛡️ **Security** - Safety checks, permission systems
- 📦 **Distribution Support** - New package managers
- 🔌 **Plugins** - Extensions and integrations
- 📚 **Documentation** - Guides, tutorials, examples

### **Development Setup**

```bash
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot
cargo build
cargo test
./demo-ollama.sh  # Test with Ollama
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## ⚠️ Important Notes

- **Always review commands** before execution in autonomous mode
- **Start with supervised mode** to understand TuxPilot's capabilities
- **Keep backups** of important data before system modifications
- **Test in safe environments** before production use
- **Review audit logs** regularly for security monitoring

## 🆘 Support

- 📖 [Documentation](docs/)
- 🐛 [Issue Tracker](https://github.com/yourusername/tuxpilot/issues)
- 💬 [Discussions](https://github.com/yourusername/tuxpilot/discussions)
- 📧 [Email Support](mailto:support@tuxpilot.dev)

---

**TuxPilot: Your trusted AI companion for Linux system administration** 🐧🚀

*Made with ❤️ for the Linux community*
