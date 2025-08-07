# TuxPilot 🐧

An AI-powered copilot for Linux systems that assists with error diagnosis, troubleshooting, and system administration tasks.

## Features

- **🔍 Error Diagnosis**: Intelligent analysis of system errors and log files
- **💬 Interactive Chat**: Natural language interface for Linux help
- **📦 Package Management**: Smart assistance with package operations (pacman, apt, dnf, etc.)
- **⚙️ Service Management**: Help with systemd and other service managers
- **📊 System Monitoring**: Real-time system health monitoring and alerts
- **🤖 AI Integration**: Support for both local and cloud-based AI models
- **🎯 Arch Linux Optimized**: Special focus on Arch Linux best practices

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Linux system (tested on Arch Linux)
- Optional: API keys for OpenAI or Anthropic for cloud AI features

### Build from Source

```bash
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot
cargo build --release
sudo cp target/release/tuxpilot /usr/local/bin/
```

### Configuration

On first run, TuxPilot will create a configuration file at `~/.config/tuxpilot/config.toml`:

```toml
[ai]
provider = "OpenAI"

[ai.openai]
api_key = "your-api-key-here"
model = "gpt-4"

[system]
package_manager = "Pacman"
service_manager = "Systemd"
log_paths = [
    "/var/log/syslog",
    "/var/log/messages",
    "/var/log/kern.log",
    "/var/log/auth.log",
    "/var/log/pacman.log"
]

[ui]
theme = "default"
show_tips = true
auto_suggest = true
```

## Usage

### Interactive Mode

```bash
tuxpilot
```

This starts an interactive chat session where you can ask questions in natural language.

### Command Line Usage

```bash
# Diagnose system errors automatically
tuxpilot diagnose --auto

# Analyze a specific error
tuxpilot diagnose --input "systemd failed to start nginx.service"

# Get help with a command
tuxpilot help systemctl

# Monitor system health
tuxpilot monitor

# Package management help
tuxpilot package install firefox
tuxpilot package search "text editor"

# Service management
tuxpilot service nginx status
tuxpilot service docker start

# Configuration
tuxpilot config --show
```

### Example Interactions

```
tuxpilot> My system is running slow
🤖 TuxPilot: I'll help you diagnose performance issues. Let me check your system status...

tuxpilot> How do I install Docker on Arch Linux?
🤖 TuxPilot: To install Docker on Arch Linux, use: sudo pacman -S docker
Then enable and start the service: sudo systemctl enable --now docker.service

tuxpilot> nginx won't start
🤖 TuxPilot: Let me check the nginx service status and logs to diagnose the issue...
```

## AI Providers

### 🚀 Ollama (Empfohlen - Lokal & Kostenlos)

**Komplett offline, keine API-Kosten!**

```bash
# Schnell-Setup (alles automatisch)
./setup-ollama.sh

# Oder manuell:
curl -fsSL https://ollama.ai/install.sh | sh
ollama serve &
ollama pull llama3.1:8b
```

Siehe [OLLAMA-SETUP.md](OLLAMA-SETUP.md) für detaillierte Anleitung.

### Cloud Providers

- **OpenAI**: GPT-4, GPT-3.5-turbo
- **Anthropic**: Claude-3, Claude-2

### Local AI (Experimental)

TuxPilot supports local AI models through the `candle` framework:

```bash
# Build with local AI support
cargo build --release --features local-ai

# Use local model
tuxpilot --local
```

## Architecture

```
src/
├── main.rs              # Entry point and CLI argument parsing
├── cli.rs               # Interactive CLI interface
├── config.rs            # Configuration management
├── ai/                  # AI integration modules
│   └── mod.rs          # AI client and provider abstraction
├── error_diagnosis.rs   # Error detection and analysis
├── linux_integration.rs # System integration and command execution
└── system_monitor.rs    # System monitoring and health checks
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Local AI model integration with Ollama
- [ ] Web interface for remote system management
- [ ] Plugin system for custom integrations
- [ ] Multi-language support
- [ ] Advanced log analysis with ML
- [ ] Integration with monitoring tools (Prometheus, Grafana)
- [ ] Automated system optimization suggestions

## Support

- 📖 [Documentation](docs/)
- 🐛 [Issue Tracker](https://github.com/yourusername/tuxpilot/issues)
- 💬 [Discussions](https://github.com/yourusername/tuxpilot/discussions)

---

Made with ❤️ for the Linux community
