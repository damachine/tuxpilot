# TuxPilot Installation Guide

## Prerequisites

### System Requirements
- Linux operating system (tested on Arch Linux, should work on most distributions)
- Rust 1.70 or later
- Internet connection for cloud AI features
- Optional: API keys for OpenAI or Anthropic

### Installing Rust
If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Installation Methods

### Method 1: Build from Source (Recommended)

1. **Clone the repository:**
```bash
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot
```

2. **Build the project:**
```bash
# Standard build with cloud AI support
cargo build --release

# Or build with local AI support (experimental)
cargo build --release --features local-ai
```

3. **Install system-wide:**
```bash
sudo cp target/release/tuxpilot /usr/local/bin/
```

4. **Verify installation:**
```bash
tuxpilot --version
```

### Method 2: Install via Cargo

```bash
cargo install --git https://github.com/yourusername/tuxpilot.git
```

## Configuration

### Initial Setup

1. **Run TuxPilot for the first time:**
```bash
tuxpilot config --show
```

This will create a default configuration file at `~/.config/tuxpilot/config.toml`.

2. **Configure AI Provider:**

Edit the configuration file:
```bash
nano ~/.config/tuxpilot/config.toml
```

### OpenAI Configuration

```toml
[ai]
provider = "OpenAI"

[ai.openai]
api_key = "sk-your-openai-api-key-here"
model = "gpt-4"
```

### Anthropic Configuration

```toml
[ai]
provider = "Anthropic"

[ai.anthropic]
api_key = "your-anthropic-api-key-here"
model = "claude-3-sonnet-20240229"
```

### Local AI Configuration (Experimental)

```toml
[ai]
provider = "Local"

[ai.local]
model_path = "/path/to/your/model.gguf"
context_size = 4096
temperature = 0.7
```

## System-Specific Setup

### Arch Linux

TuxPilot automatically detects Arch Linux and configures pacman as the package manager:

```toml
[system]
package_manager = "Pacman"
service_manager = "Systemd"
```

### Ubuntu/Debian

For apt-based systems:

```toml
[system]
package_manager = "Apt"
service_manager = "Systemd"
```

### Fedora/RHEL

For dnf-based systems:

```toml
[system]
package_manager = "Dnf"
service_manager = "Systemd"
```

### openSUSE

For zypper-based systems:

```toml
[system]
package_manager = "Zypper"
service_manager = "Systemd"
```

## Verification

### Test Basic Functionality

```bash
# Show help
tuxpilot --help

# Test package suggestions (works without AI)
tuxpilot package install firefox

# Test system monitoring
tuxpilot monitor

# Interactive mode (requires AI configuration)
tuxpilot chat
```

### Run Demo

```bash
./demo.sh
```

## Troubleshooting

### Common Issues

1. **"Command not found" error:**
   - Ensure `/usr/local/bin` is in your PATH
   - Or use the full path: `/usr/local/bin/tuxpilot`

2. **"Invalid response format" error:**
   - Check your AI API key configuration
   - Verify internet connection for cloud AI
   - Try using `--local` flag for local AI

3. **Permission errors:**
   - Some system monitoring features require elevated permissions
   - Run with `sudo` if needed for system log access

4. **Build errors:**
   - Update Rust: `rustup update`
   - Clear cargo cache: `cargo clean`
   - Check system dependencies

### Getting Help

- Check the [README.md](README.md) for usage examples
- Open an issue on GitHub for bugs
- Join discussions for questions and feature requests

## Uninstallation

```bash
# Remove binary
sudo rm /usr/local/bin/tuxpilot

# Remove configuration (optional)
rm -rf ~/.config/tuxpilot

# Remove cargo installation (if installed via cargo)
cargo uninstall tuxpilot
```

## Development Setup

For contributors:

```bash
# Clone and setup
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot

# Install development dependencies
cargo install cargo-watch

# Run tests
cargo test

# Run with auto-reload during development
cargo watch -x run
```
