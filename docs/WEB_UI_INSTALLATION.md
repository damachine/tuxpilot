# TuxPilot Web UI Installation Guide

This guide explains how to install and manage the TuxPilot Web UI in system-wide locations following Linux standards.

## 🎯 Overview

The TuxPilot Web UI is now installed to `/usr/local/share/tuxpilot/web-ui/` for system-wide access, providing:

- **Standard Linux Installation**: Follows FHS (Filesystem Hierarchy Standard)
- **System-wide Access**: Available to all users on the system
- **Automatic Fallback**: Development mode still supported
- **Easy Updates**: Dedicated scripts for Web UI management

## 📁 Installation Locations

### System Installation
```
/usr/local/share/tuxpilot/web-ui/
├── index.html              # Main SPA entry point
├── _app/                   # Svelte application assets
│   ├── immutable/          # Versioned static assets
│   ├── env.js              # Environment configuration
│   └── version.json        # Build version info
└── favicon.png             # Application icon
```

### Binary Location
```
/usr/local/bin/tuxpilot     # Main executable
```

### Configuration
```
~/.config/tuxpilot/         # User configuration directory
```

## 🚀 Installation Methods

### 1. Complete Installation (Recommended)

Install everything including Web UI:

```bash
git clone https://github.com/damachine/tuxpilot.git
cd tuxpilot
chmod +x install.sh
./install.sh
```

**What it does:**
- ✅ Builds Rust backend
- ✅ Builds Svelte Web UI (if Node.js available)
- ✅ Installs binary to `/usr/local/bin/tuxpilot`
- ✅ Installs Web UI to `/usr/local/share/tuxpilot/web-ui/`
- ✅ Sets up configuration directories
- ✅ Configures shell completions

### 2. Web UI Only Installation

Update or install only the Web UI:

```bash
./install-web-ui.sh
```

**What it does:**
- ✅ Builds the Svelte Web UI
- ✅ Installs to system location
- ✅ Copies to development location
- ✅ Verifies installation

### 3. Manual Installation

For advanced users or custom setups:

```bash
# Build Web UI
cd web-ui
npm install
npm run build
cd ..

# Install to system location
sudo mkdir -p /usr/local/share/tuxpilot/web-ui
sudo cp -r web-ui/build/* /usr/local/share/tuxpilot/web-ui/
sudo chmod -R 644 /usr/local/share/tuxpilot/web-ui/
sudo find /usr/local/share/tuxpilot/web-ui/ -type d -exec chmod 755 {} \\;
```

## 🔧 How It Works

### Automatic Path Resolution

The TuxPilot server automatically detects and uses the appropriate Web UI location:

1. **System Installation** (Priority 1): `/usr/local/share/tuxpilot/web-ui/`
2. **Development Fallback** (Priority 2): `web-ui/build/`

### Server Configuration

The server code includes intelligent path resolution:

```rust
fn get_web_ui_assets_path() -> String {
    // Try system installation path first
    let system_path = "/usr/local/share/tuxpilot/web-ui/_app";
    if std::path::Path::new(system_path).exists() {
        return system_path.to_string();
    }
    
    // Fallback to development path
    "web-ui/build/_app".to_string()
}
```

## 🌐 Starting the Web UI

After installation, start the web server:

```bash
tuxpilot web
```

Then open your browser to: **http://127.0.0.1:8080**

The server will automatically serve from the system installation if available.

## 🔄 Updating the Web UI

### Update Web UI Only

```bash
cd /path/to/tuxpilot
./install-web-ui.sh
```

### Update Everything

```bash
cd /path/to/tuxpilot
./install.sh
```

## 🗑️ Uninstallation

### Complete Removal

```bash
./uninstall.sh
```

**What it removes:**
- ❌ Binary: `/usr/local/bin/tuxpilot`
- ❌ Web UI: `/usr/local/share/tuxpilot/`
- ❌ Shell completions
- ✅ **Preserves**: Configuration in `~/.config/tuxpilot/`

### Web UI Only Removal

```bash
sudo rm -rf /usr/local/share/tuxpilot/web-ui/
```

## 🛠️ Development Mode

For developers working on the Web UI:

### Development Server
```bash
cd web-ui
npm run dev
```

### Build and Test
```bash
cd web-ui
npm run build
cd ..
cp -r web-ui/build/* static/  # Copy to development location
tuxpilot web                  # Test with TuxPilot server
```

## 📋 Requirements

### System Requirements
- **Linux** (any major distribution)
- **Node.js** 18+ (for building Web UI)
- **npm** (package manager)
- **Rust** 1.70+ (for backend)

### Package Installation

**Arch Linux:**
```bash
sudo pacman -S nodejs npm
```

**Ubuntu/Debian:**
```bash
sudo apt install nodejs npm
```

**Fedora:**
```bash
sudo dnf install nodejs npm
```

**openSUSE:**
```bash
sudo zypper install nodejs npm
```

## 🔍 Troubleshooting

### Web UI Not Loading

1. **Check installation:**
   ```bash
   ls -la /usr/local/share/tuxpilot/web-ui/
   ```

2. **Verify permissions:**
   ```bash
   sudo chmod -R 644 /usr/local/share/tuxpilot/web-ui/
   sudo find /usr/local/share/tuxpilot/web-ui/ -type d -exec chmod 755 {} \\;
   ```

3. **Rebuild Web UI:**
   ```bash
   ./install-web-ui.sh
   ```

### Development Issues

1. **Clear build cache:**
   ```bash
   cd web-ui
   rm -rf .svelte-kit build node_modules
   npm install
   npm run build
   ```

2. **Check Node.js version:**
   ```bash
   node --version  # Should be 18+
   npm --version
   ```

### Server Issues

1. **Check server logs:**
   ```bash
   tuxpilot web --verbose
   ```

2. **Verify binary installation:**
   ```bash
   which tuxpilot
   tuxpilot --version
   ```

## 🎨 Features

The installed Web UI includes:

- **Modern Svelte 5 Interface**: Fast, reactive UI
- **ChatGPT-inspired Design**: Dark theme with modern aesthetics
- **Real-time Chat**: WebSocket-based communication
- **Configuration Management**: Complete system configuration
- **System Dashboard**: Real-time system monitoring
- **Responsive Design**: Works on desktop and mobile
- **Progressive Web App**: Can be installed as desktop app

## 📚 Additional Resources

- **Main Documentation**: [README.md](../README.md)
- **Configuration Guide**: [CONFIGURATION.md](CONFIGURATION.md)
- **API Reference**: [API_REFERENCE.md](API_REFERENCE.md)
- **Troubleshooting**: [TROUBLESHOOTING.md](TROUBLESHOOTING.md)

## 🤝 Contributing

To contribute to the Web UI:

1. Fork the repository
2. Make changes in the `web-ui/` directory
3. Test with `npm run dev`
4. Build with `npm run build`
5. Test with TuxPilot server
6. Submit a pull request

---

**Happy system administration with TuxPilot! 🐧🌐**