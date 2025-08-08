---
description: Repository Information Overview
alwaysApply: true
---

# TuxPilot Information

## Summary
TuxPilot is an AI-powered Linux system administrator assistant that helps with everyday system administration tasks, from security and gaming optimization to professional server management. It features natural language processing, autonomous command execution, and comprehensive safety controls to make Linux system administration accessible to everyone.

## Structure
- **src/**: Core Rust backend code organized by modules (agents, ai, automation, containers, etc.)
- **web-ui/**: Svelte-based frontend interface with TypeScript and Tailwind CSS
- **docs/**: Documentation files for API, configuration, and usage
- **tests/**: Integration tests for the application
- **examples/**: Example configuration files including Ollama setup
- **static/**: Static web assets for the web interface

## Language & Runtime
**Language**: Rust
**Version**: 1.70+
**Build System**: Cargo
**Package Manager**: Cargo (backend), npm (frontend)
**Web Framework**: Svelte 5 with SvelteKit

## Dependencies

### Backend (Rust)
**Main Dependencies**:
- tokio (1.0): Async runtime with full features
- axum (0.7): Web framework for API and server
- clap (4.4): CLI framework with derive features
- serde (1.0): Serialization/deserialization
- reqwest (0.11): HTTP client for AI APIs
- sysinfo (0.30): System information collection
- ratatui (0.24): Terminal UI framework
- nix (0.27): Unix API bindings
- candle-core (0.3, optional): Local AI model support

**Development Dependencies**:
- tempfile (3.8): Temporary file creation for tests
- assert_cmd (2.0): Command-line testing
- predicates (3.0): Test assertions

### Frontend (Svelte)
**Main Dependencies**:
- svelte (5.19.3): UI framework
- @sveltejs/kit (2.20.2): Full-stack framework
- tailwindcss (3.4.17): Utility-first CSS
- @tabler/icons-svelte (3.29.0): Icon library
- marked (15.0.4): Markdown parser
- highlight.js (11.10.0): Syntax highlighting

## Build & Installation

### Complete Installation
```bash
# Clone and install with integrated web UI
git clone https://github.com/damachine/tuxpilot.git
cd tuxpilot
chmod +x install.sh
./install.sh
```

### Manual Build
```bash
# Backend only
cargo build --release

# Frontend only
cd web-ui
npm install
npm run build
cp -r build/* ../static/

# Complete build (backend + web UI)
./build.sh
```

### System Requirements
- Linux distribution (any major distro)
- Rust 1.70+
- Node.js 18+ (for web interface)
- 2GB RAM minimum (4GB+ recommended)

## Configuration
```toml
# ~/.config/tuxpilot/config.toml
[ai]
provider = "OpenAI"  # or "Anthropic", "Ollama"
model = "gpt-4"      # or "claude-3-sonnet", "llama3.1:8b"

[safety]
execution_mode = "supervised"  # supervised, semi-autonomous, autonomous, read-only
risk_threshold = "medium"      # low, medium, high
```

## Testing
**Framework**: Rust's built-in testing with assert_cmd and predicates
**Test Location**: tests/ directory
**Naming Convention**: *_test.rs files
**Run Command**:
```bash
cargo test
```

## Usage
**CLI Commands**:
```bash
# Start the web server
tuxpilot web

# Interactive chat mode
tuxpilot chat

# System diagnostics
tuxpilot diagnose --auto-fix

# Package management
tuxpilot package install <package>

# Security operations
tuxpilot security scan
tuxpilot security harden --level medium
```

**Web Interface**: Modern Svelte-based UI with chat interface, system dashboard, and configuration management. Available at http://127.0.0.1:8080 when running `tuxpilot web`.