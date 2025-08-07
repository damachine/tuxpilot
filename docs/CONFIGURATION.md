# TuxPilot Configuration Guide

This guide covers all aspects of configuring TuxPilot for optimal performance and security.

## 📁 Configuration Files

### Primary Configuration

**Location**: `~/.config/tuxpilot/config.toml`

```toml
# TuxPilot Configuration File
# This file controls all aspects of TuxPilot behavior

[ai]
provider = "ollama"           # AI provider: ollama, openai, anthropic
model = "llama3.1:8b"        # Model name (provider-specific)
temperature = 0.8            # Response creativity (0.0-2.0)
max_tokens = 4096           # Maximum response length
timeout = 30                # Request timeout in seconds

[execution]
mode = "supervised"         # supervised, semi-auto, autonomous, read-only
require_confirmation = true # Require user approval for commands
timeout = 300              # Command execution timeout (seconds)
enable_rollback = true     # Enable automatic rollback for failed operations

[safety]
risk_threshold = "medium"   # low, medium, high
enable_audit = true        # Enable audit logging
audit_file = "~/.config/tuxpilot/audit.log"

[permissions]
allow_package_management = true    # Allow package install/remove
allow_service_management = true    # Allow service start/stop
allow_system_modification = false  # Allow system file modifications
allow_network_access = true       # Allow network operations
allow_user_management = false     # Allow user/group management

[web]
port = 8080                # Web server port
bind_address = "127.0.0.1" # Bind address (127.0.0.1 or 0.0.0.0)
ssl_enabled = false        # Enable HTTPS/SSL
ssl_cert_path = ""         # SSL certificate path
ssl_key_path = ""          # SSL private key path

[logging]
level = "info"             # debug, info, warn, error
file = "~/.config/tuxpilot/tuxpilot.log"
max_size = "10MB"          # Log file rotation size
max_files = 5              # Number of log files to keep
```

### AI Provider Configurations

#### Ollama (Local AI - Recommended)

```toml
[ai]
provider = "ollama"
model = "llama3.1:8b"      # or llama3.1:70b, codellama, etc.
base_url = "http://localhost:11434"
temperature = 0.8

[ai.ollama]
# Ollama-specific settings
keep_alive = "5m"          # Keep model loaded for 5 minutes
num_ctx = 4096            # Context window size
num_predict = 2048        # Max tokens to predict
```

#### OpenAI

```toml
[ai]
provider = "openai"
model = "gpt-4"           # gpt-4, gpt-3.5-turbo, etc.
temperature = 0.8

[ai.openai]
api_key = "your-api-key-here"
organization = ""         # Optional organization ID
base_url = "https://api.openai.com/v1"
```

#### Anthropic (Claude)

```toml
[ai]
provider = "anthropic"
model = "claude-3-sonnet-20240229"
temperature = 0.8

[ai.anthropic]
api_key = "your-api-key-here"
base_url = "https://api.anthropic.com"
```

## ⚙️ Configuration Management

### Command Line Configuration

```bash
# View current configuration
tuxpilot config show

# Set configuration values
tuxpilot config set ai.provider ollama
tuxpilot config set ai.model llama3.1:8b
tuxpilot config set execution.mode supervised

# Get specific configuration value
tuxpilot config get ai.provider

# Reset to defaults
tuxpilot config reset

# Validate configuration
tuxpilot config validate
```

### Web Interface Configuration

Access the settings page at `http://127.0.0.1:8080/settings` for:
- **Visual configuration editor** with form validation
- **Real-time configuration updates**
- **Configuration backup and restore**
- **Provider-specific model selection**

## 🔒 Security Configuration

### Execution Modes

| Mode | Description | Security Level | Use Case |
|------|-------------|----------------|----------|
| **read-only** | Analysis only, no execution | Highest | Learning, inspection |
| **supervised** | User approves each command | High | Daily use, learning |
| **semi-auto** | Auto-execute safe commands | Medium | Experienced users |
| **autonomous** | Execute most commands automatically | Lower | Trusted environments |

### Permission Categories

```toml
[permissions]
# System Information (always allowed)
read_system = true

# File Operations
read_files = true
write_files = false
modify_system_files = false

# Package Management
install_packages = true
remove_packages = false
update_packages = true

# Service Management
start_services = true
stop_services = true
restart_services = true
enable_services = false

# System Configuration
modify_kernel_params = false
mount_filesystems = false
network_configuration = false

# User Management
create_users = false
modify_users = false
delete_users = false

# Advanced Operations
execute_as_root = false
modify_bootloader = false
partition_disks = false
```

### Risk Assessment

```toml
[safety]
# Risk threshold for automatic execution
risk_threshold = "medium"  # low, medium, high

# Command pattern detection
dangerous_patterns = [
    "rm -rf /",
    "dd if=.*of=/dev/.*",
    "mkfs.*",
    "fdisk.*",
    "parted.*"
]

# Safe command patterns (always allowed)
safe_patterns = [
    "ls.*",
    "cat.*",
    "grep.*",
    "ps.*",
    "top",
    "htop"
]
```

## 🌐 Web Server Configuration

### Basic Web Configuration

```toml
[web]
port = 8080
bind_address = "127.0.0.1"  # Localhost only
ssl_enabled = false
```

### Production Web Configuration

```toml
[web]
port = 443
bind_address = "0.0.0.0"    # All interfaces
ssl_enabled = true
ssl_cert_path = "/etc/ssl/certs/tuxpilot.crt"
ssl_key_path = "/etc/ssl/private/tuxpilot.key"

[web.security]
session_timeout = 3600      # 1 hour
max_sessions = 100
rate_limit = 60            # Requests per minute
allowed_origins = [
    "https://yourdomain.com",
    "https://admin.yourdomain.com"
]
```

### SSL/TLS Setup

```bash
# Generate self-signed certificate (development)
openssl req -x509 -newkey rsa:4096 -keyout tuxpilot.key -out tuxpilot.crt -days 365 -nodes

# Use Let's Encrypt (production)
certbot certonly --standalone -d yourdomain.com
```

## 📊 Logging Configuration

### Log Levels

```toml
[logging]
level = "info"             # debug, info, warn, error
console_output = true      # Also log to console
structured_logging = true # JSON format logs
```

### Log Rotation

```toml
[logging]
file = "~/.config/tuxpilot/tuxpilot.log"
max_size = "10MB"
max_files = 5
compress_old = true
```

### Audit Logging

```toml
[safety]
enable_audit = true
audit_file = "~/.config/tuxpilot/audit.log"
audit_level = "all"        # all, commands, errors

[audit]
include_environment = true  # Log environment variables
include_user_info = true   # Log user information
include_system_info = true # Log system state
retention_days = 90        # Keep audit logs for 90 days
```

## 🔧 Advanced Configuration

### Performance Tuning

```toml
[performance]
max_concurrent_commands = 5    # Parallel command execution
command_cache_size = 1000     # Cache command results
ai_request_cache_ttl = 300    # Cache AI responses (seconds)
system_monitor_interval = 5   # System monitoring interval

[ai.performance]
request_timeout = 30          # AI request timeout
retry_attempts = 3           # Retry failed requests
batch_requests = false       # Batch multiple requests
```

### Plugin Configuration

```toml
[plugins]
enabled = true
auto_load = true
plugin_dir = "~/.config/tuxpilot/plugins"
max_plugins = 50

[plugins.security]
verify_signatures = true     # Verify plugin signatures
sandbox_plugins = true      # Run plugins in sandbox
allow_network = false       # Allow plugins network access
```

## 🔄 Configuration Backup & Restore

### Automatic Backups

```toml
[backup]
auto_backup = true
backup_interval = "daily"    # daily, weekly, monthly
backup_location = "~/.config/tuxpilot/backups"
max_backups = 30
```

### Manual Backup/Restore

```bash
# Create configuration backup
tuxpilot config backup --file backup-$(date +%Y%m%d).toml

# Restore from backup
tuxpilot config restore --file backup-20240101.toml

# Export configuration
tuxpilot config export --format json > config.json

# Import configuration
tuxpilot config import --file config.json
```

## 🐛 Configuration Troubleshooting

### Common Issues

1. **Configuration not loading**
   ```bash
   # Check file permissions
   ls -la ~/.config/tuxpilot/config.toml
   
   # Validate configuration syntax
   tuxpilot config validate
   ```

2. **AI provider not working**
   ```bash
   # Test AI connection
   tuxpilot ai test
   
   # Check provider-specific settings
   tuxpilot config get ai
   ```

3. **Permission denied errors**
   ```bash
   # Check permission settings
   tuxpilot config get permissions
   
   # Review audit logs
   tail -f ~/.config/tuxpilot/audit.log
   ```

### Configuration Validation

```bash
# Validate entire configuration
tuxpilot config validate

# Check specific sections
tuxpilot config validate --section ai
tuxpilot config validate --section permissions

# Test configuration changes
tuxpilot config test --dry-run
```

## 📚 Configuration Examples

### Gaming Optimization

```toml
[execution]
mode = "semi-auto"
require_confirmation = false

[permissions]
allow_system_modification = true
allow_service_management = true

[performance]
max_concurrent_commands = 10
priority = "high"
```

### Server Administration

```toml
[execution]
mode = "supervised"
require_confirmation = true

[safety]
risk_threshold = "low"
enable_audit = true

[web]
ssl_enabled = true
bind_address = "0.0.0.0"
```

### Development Environment

```toml
[execution]
mode = "semi-auto"

[permissions]
allow_package_management = true
allow_service_management = true

[plugins]
enabled = true
auto_load = true
```
