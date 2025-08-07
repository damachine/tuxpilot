# 🔧 TuxPilot Troubleshooting Guide

**Solutions for common issues and problems**

## 🚨 Common Issues

### **TuxPilot Won't Start**

#### **Problem**: `tuxpilot: command not found`
**Solution:**
```bash
# Check if TuxPilot is built
ls target/release/tuxpilot

# If not built, build it
cargo build --release

# Add to PATH or use full path
export PATH=$PATH:$(pwd)/target/release
# Or
./target/release/tuxpilot --help
```

#### **Problem**: Configuration errors on startup
**Solution:**
```bash
# Check configuration
tuxpilot config --show

# Reset configuration
rm ~/.config/tuxpilot/config.toml
tuxpilot config --show  # Will recreate with defaults

# Check permissions on config directory
ls -la ~/.config/tuxpilot/
chmod 755 ~/.config/tuxpilot/
```

### **AI Integration Issues**

#### **Problem**: "Ollama not available" error
**Solution:**
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# If not running, start Ollama
ollama serve &

# Check if model is available
ollama list

# Pull model if missing
ollama pull llama3.1:8b

# Test Ollama directly
ollama run llama3.1:8b "Hello"
```

#### **Problem**: Slow AI responses
**Solution:**
```bash
# Check system resources
htop

# Use smaller model
ollama pull llama3.1:7b
# Update config to use smaller model

# Increase timeout in config
[ai.ollama]
timeout = 60  # Increase from default 30
```

#### **Problem**: OpenAI/Anthropic API errors
**Solution:**
```bash
# Check API key
echo $OPENAI_API_KEY

# Test API directly
curl -H "Authorization: Bearer $OPENAI_API_KEY" \
     https://api.openai.com/v1/models

# Check configuration
tuxpilot config --show

# Verify API key in config
[ai.openai]
api_key = "your-actual-api-key"
```

### **Command Execution Issues**

#### **Problem**: Commands not executing
**Solution:**
```bash
# Check execution mode
tuxpilot config --show

# Check permissions
tuxpilot permissions --detailed

# Try read-only mode first
tuxpilot execute "show system info" --mode read-only

# Check if in supervised mode (requires approval)
tuxpilot execute "ls" --mode supervised
```

#### **Problem**: Permission denied errors
**Solution:**
```bash
# Check user permissions
tuxpilot permissions --detailed

# Check if user is in sudo group
groups $USER

# Add user to sudo group (if needed)
sudo usermod -aG sudo $USER
# Log out and back in

# Check specific command permissions
tuxpilot permissions --check "sudo systemctl restart nginx"
```

#### **Problem**: Safety checks blocking commands
**Solution:**
```bash
# Check what's being blocked
tuxpilot execute "your command" --dry-run

# Review safety settings
tuxpilot config --show

# Temporarily use read-only mode
tuxpilot execute "your command" --mode read-only

# Check audit logs for details
tuxpilot audit --limit 5
```

### **Distribution Detection Issues**

#### **Problem**: Wrong package manager detected
**Solution:**
```bash
# Check detected distribution
tuxpilot config --show

# Manually set package manager
tuxpilot config --set system.package_manager=Pacman
# Or: Apt, Dnf, Zypper, Portage

# Verify detection
cat /etc/os-release
```

#### **Problem**: Service manager not working
**Solution:**
```bash
# Check service manager
systemctl --version  # For systemd
rc-service --version  # For OpenRC

# Check TuxPilot detection
tuxpilot config --show

# Manually set service manager
tuxpilot config --set system.service_manager=Systemd
```

### **Audit and Logging Issues**

#### **Problem**: Audit logs not working
**Solution:**
```bash
# Check audit log location
ls -la ~/.local/share/tuxpilot/audit/

# Check permissions
chmod 755 ~/.local/share/tuxpilot/
chmod 644 ~/.local/share/tuxpilot/audit/audit.jsonl

# Check disk space
df -h ~/.local/share/

# Enable audit logging
tuxpilot config --set execution.enable_audit_logging=true
```

#### **Problem**: Cannot export audit logs
**Solution:**
```bash
# Check export format
tuxpilot audit --export json > test.json
cat test.json

# Check file permissions
touch test-export.json
ls -la test-export.json

# Try different formats
tuxpilot audit --export csv > audit.csv
tuxpilot audit --export html > audit.html
```

## 🔍 Diagnostic Commands

### **System Health Check**
```bash
# Complete system check
tuxpilot diagnose --auto

# Check configuration
tuxpilot config --show

# Check permissions
tuxpilot permissions --detailed

# Check recent operations
tuxpilot audit --limit 10
```

### **AI Connection Test**
```bash
# Test Ollama
curl http://localhost:11434/api/tags

# Test with TuxPilot
tuxpilot execute "echo hello" --mode read-only

# Check AI provider status
tuxpilot config --show | grep provider
```

### **Verbose Debugging**
```bash
# Enable verbose output
RUST_LOG=debug tuxpilot execute "test command"

# Check system logs
journalctl -u ollama -f  # For Ollama service
tail -f ~/.local/share/tuxpilot/audit/audit.jsonl
```

## 🛠️ Advanced Troubleshooting

### **Reset TuxPilot**
```bash
# Backup current config
cp ~/.config/tuxpilot/config.toml ~/tuxpilot-config-backup.toml

# Remove all TuxPilot data
rm -rf ~/.config/tuxpilot/
rm -rf ~/.local/share/tuxpilot/

# Restart TuxPilot (will recreate defaults)
tuxpilot config --show
```

### **Manual Configuration**
```bash
# Create minimal config
mkdir -p ~/.config/tuxpilot/
cat > ~/.config/tuxpilot/config.toml << 'EOF'
[ai]
provider = "Ollama"

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"

[execution]
default_mode = "supervised"
enable_audit_logging = true

[system]
package_manager = "Pacman"  # Change as needed
service_manager = "Systemd"
EOF
```

### **Build Issues**
```bash
# Clean build
cargo clean
cargo build --release

# Check Rust version
rustc --version
# Update if needed: rustup update

# Check dependencies
cargo check

# Build with verbose output
cargo build --release --verbose
```

## 📊 Performance Optimization

### **Improve AI Response Time**
```bash
# Use smaller model
ollama pull llama3.1:7b

# Increase system resources for Ollama
export OLLAMA_NUM_PARALLEL=4
export OLLAMA_MAX_LOADED_MODELS=2

# Use GPU acceleration (if available)
export OLLAMA_GPU_LAYERS=35
```

### **Reduce Memory Usage**
```bash
# Use smaller model
ollama pull llama3.1:7b

# Limit concurrent operations
tuxpilot config --set execution.max_concurrent_operations=1

# Disable audit logging temporarily
tuxpilot config --set execution.enable_audit_logging=false
```

## 🚨 Emergency Procedures

### **Stop All TuxPilot Operations**
```bash
# Kill all TuxPilot processes
pkill -f tuxpilot

# Stop Ollama if needed
pkill -f ollama

# Check for running operations
ps aux | grep tuxpilot
```

### **Rollback Recent Changes**
```bash
# Check recent operations
tuxpilot audit --limit 10

# Rollback specific operation
tuxpilot audit --rollback <execution-id>

# Manual rollback (if needed)
# Check audit logs for rollback commands
cat ~/.local/share/tuxpilot/audit/audit.jsonl | grep rollback
```

### **Safe Mode**
```bash
# Use read-only mode only
tuxpilot execute "your command" --mode read-only

# Disable all execution
tuxpilot config --set execution.default_mode=read-only

# Chat without execution
tuxpilot chat --no-execute
```

## 📞 Getting Help

### **Collect Debug Information**
```bash
# System information
uname -a
cat /etc/os-release

# TuxPilot information
tuxpilot config --show
tuxpilot permissions --detailed
tuxpilot audit --limit 5

# AI provider status
curl http://localhost:11434/api/tags  # For Ollama
ollama list

# Recent logs
tail -20 ~/.local/share/tuxpilot/audit/audit.jsonl
```

### **Report Issues**
When reporting issues, include:
1. **System information** (OS, distribution, version)
2. **TuxPilot version** and configuration
3. **Error messages** (exact text)
4. **Steps to reproduce** the issue
5. **Expected vs actual behavior**

### **Community Support**
- 🐛 [Issue Tracker](https://github.com/yourusername/tuxpilot/issues)
- 💬 [Discussions](https://github.com/yourusername/tuxpilot/discussions)
- 📧 [Email Support](mailto:support@tuxpilot.dev)

---

**Most issues can be resolved by checking configuration, permissions, and AI provider status. When in doubt, start with read-only mode and work your way up to more autonomous modes.**
