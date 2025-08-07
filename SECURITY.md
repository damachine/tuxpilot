# 🛡️ TuxPilot Security & Safety Framework

**Comprehensive security architecture for autonomous command execution**

## 🎯 Security Philosophy

TuxPilot is designed with **security-first principles** to enable safe autonomous command execution while maintaining system integrity and user control.

### **Core Security Principles**

1. **🔒 Principle of Least Privilege** - Only grant minimum required permissions
2. **🛡️ Defense in Depth** - Multiple security layers for comprehensive protection
3. **📊 Complete Transparency** - Full audit trail of all operations
4. **👤 User Control** - User always has final authority over system changes
5. **🔄 Reversibility** - Provide rollback mechanisms where possible
6. **🚫 Fail Safe** - Default to safe behavior when in doubt

---

## 🏗️ Security Architecture

### **Multi-Layer Security Model**

```
┌─────────────────────────────────────────────────────────────┐
│                    User Input Layer                         │
├─────────────────────────────────────────────────────────────┤
│                  AI Analysis Layer                          │
├─────────────────────────────────────────────────────────────┤
│                 Command Safety Layer                        │
├─────────────────────────────────────────────────────────────┤
│                Permission Control Layer                     │
├─────────────────────────────────────────────────────────────┤
│                 User Approval Layer                         │
├─────────────────────────────────────────────────────────────┤
│                Execution Sandbox Layer                      │
├─────────────────────────────────────────────────────────────┤
│                  Audit Logging Layer                        │
└─────────────────────────────────────────────────────────────┘
```

### **Security Components**

#### **1. Command Safety Checker**
```rust
pub struct SafetyChecker {
    dangerous_commands: HashSet<String>,     // Known dangerous commands
    dangerous_patterns: Vec<Regex>,          // Dangerous command patterns
    safe_commands: HashSet<String>,          // Explicitly safe commands
    risk_analyzer: RiskAnalyzer,             // AI-powered risk assessment
}
```

**Safety Features:**
- ✅ **Pattern Matching** - Detects dangerous command patterns
- ✅ **Risk Assessment** - Categorizes commands by risk level
- ✅ **Context Analysis** - Considers command context and arguments
- ✅ **Whitelist/Blacklist** - Explicit safe and dangerous command lists

#### **2. Permission Manager**
```rust
pub struct PermissionManager {
    granted_permissions: HashSet<Permission>,
    user_permissions: UserPermissions,
    system_permissions: SystemPermissions,
}

pub enum Permission {
    ReadSystem,           // System information access
    WriteSystem,          // System modification
    PackageManagement,    // Package install/remove
    ServiceManagement,    // Service control
    FileSystemRead,       // File reading
    FileSystemWrite,      // File modification
    NetworkAccess,        // Network operations
    UserManagement,       // User/group management
    SystemConfiguration, // System config changes
}
```

**Permission Features:**
- ✅ **Granular Control** - Fine-grained permission categories
- ✅ **Dynamic Checking** - Real-time permission verification
- ✅ **Elevation Requests** - Secure privilege escalation
- ✅ **Audit Trail** - Complete permission usage logging

#### **3. Audit Logger**
```rust
pub struct AuditLogger {
    log_file: PathBuf,
    encryption: Option<EncryptionKey>,
    integrity_checker: IntegrityChecker,
}

pub enum AuditEntryType {
    ExecutionRequest,     // Command execution requests
    ExecutionResult,      // Command execution results
    PermissionRequest,    // Permission requests
    PermissionGranted,    // Permission grants
    PermissionDenied,     // Permission denials
    SafetyViolation,      // Safety check failures
    SystemChange,         // System modifications
    Error,               // Error conditions
}
```

**Audit Features:**
- ✅ **Complete Logging** - Every operation logged
- ✅ **Tamper Detection** - Integrity verification
- ✅ **Export Capabilities** - Multiple export formats
- ✅ **Retention Policies** - Configurable log retention

---

## 🚨 Risk Assessment Framework

### **Risk Levels**

| Level | Description | Examples | Approval Required |
|-------|-------------|----------|-------------------|
| **Safe** | Read-only operations | `ls`, `cat`, `ps`, `df` | No |
| **Low** | Minor modifications | Package queries, service status | No (Semi-Auto) |
| **Medium** | System changes | Package install, service restart | Yes (Supervised) |
| **High** | Significant changes | Config modifications, user changes | Yes |
| **Critical** | Dangerous operations | System formatting, recursive deletion | Always |

### **Risk Assessment Algorithm**

```rust
impl SafetyChecker {
    pub async fn assess_risk(&self, request: &ExecutionRequest) -> RiskLevel {
        let mut risk_score = 0;
        
        // Base command risk
        risk_score += self.get_command_base_risk(&request.command);
        
        // Argument analysis
        risk_score += self.analyze_arguments(&request.args);
        
        // Target analysis
        risk_score += self.analyze_targets(&request.args);
        
        // Context analysis
        risk_score += self.analyze_context(&request.context);
        
        // Convert score to risk level
        match risk_score {
            0..=20 => RiskLevel::Safe,
            21..=40 => RiskLevel::Low,
            41..=60 => RiskLevel::Medium,
            61..=80 => RiskLevel::High,
            81..=100 => RiskLevel::Critical,
        }
    }
}
```

---

## 🔍 Dangerous Pattern Detection

### **Command Blacklist**

```rust
// Explicitly dangerous commands
const DANGEROUS_COMMANDS: &[&str] = &[
    "rm",           // File deletion
    "dd",           // Low-level disk operations
    "mkfs",         // Filesystem creation
    "fdisk",        // Disk partitioning
    "shred",        // Secure deletion
    "wipefs",       // Filesystem signature removal
    "iptables",     // Firewall modification
    "userdel",      // User deletion
];
```

### **Pattern Detection**

```rust
// Dangerous patterns
const DANGEROUS_PATTERNS: &[&str] = &[
    r"rm\s+.*-r.*\s*/",              // Recursive deletion of root
    r"dd\s+.*of=/dev/",              // Writing to devices
    r"chmod\s+.*777.*",              // Overly permissive permissions
    r".*\|\s*sh",                    // Piping to shell
    r"curl.*\|\s*sh",                // Download and execute
    r".*--force.*",                  // Force operations
    r".*>/dev/sd[a-z]",             // Writing to disk devices
];
```

### **Safe Command Whitelist**

```rust
// Explicitly safe commands
const SAFE_COMMANDS: &[&str] = &[
    "ls", "cat", "head", "tail", "grep", "find", "locate",
    "ps", "top", "htop", "free", "df", "du", "uptime",
    "uname", "whoami", "id", "groups", "which", "whereis",
    "ping", "traceroute", "nslookup", "dig", "ss", "netstat",
];
```

---

## 🔐 Permission System

### **Permission Categories**

#### **Read Permissions**
- `ReadSystem` - System information access
- `FileSystemRead` - File and directory reading
- `NetworkAccess` - Network connectivity (read-only)

#### **Write Permissions**
- `WriteSystem` - System modifications
- `FileSystemWrite` - File and directory modifications
- `SystemConfiguration` - System configuration changes

#### **Management Permissions**
- `PackageManagement` - Package installation/removal
- `ServiceManagement` - Service control
- `UserManagement` - User and group management

### **Permission Checking**

```rust
impl PermissionManager {
    pub fn check_command_permissions(&self, command: &str, args: &[String]) -> Result<Vec<Permission>> {
        let mut required = Vec::new();
        
        match command {
            "pacman" | "apt" | "dnf" => {
                required.push(Permission::ReadSystem);
                if args.iter().any(|arg| matches!(arg.as_str(), "-S" | "install" | "remove")) {
                    required.push(Permission::PackageManagement);
                    required.push(Permission::WriteSystem);
                }
            }
            
            "systemctl" | "service" => {
                required.push(Permission::ReadSystem);
                if args.iter().any(|arg| matches!(arg.as_str(), "start" | "stop" | "restart")) {
                    required.push(Permission::ServiceManagement);
                    required.push(Permission::WriteSystem);
                }
            }
            
            "rm" | "cp" | "mv" | "mkdir" => {
                required.push(Permission::FileSystemWrite);
                required.push(Permission::WriteSystem);
            }
            
            _ => {
                required.push(Permission::ReadSystem);
            }
        }
        
        // Verify permissions
        for permission in &required {
            if !self.has_permission(permission) {
                return Err(anyhow::anyhow!("Permission denied: {:?}", permission));
            }
        }
        
        Ok(required)
    }
}
```

---

## 📊 Audit and Compliance

### **Audit Log Structure**

```json
{
  "id": "uuid",
  "timestamp": "2024-01-15T10:30:00Z",
  "entry_type": "ExecutionRequest",
  "user": "username",
  "session_id": "session_uuid",
  "data": {
    "command": "systemctl start nginx",
    "risk_level": "Medium",
    "permissions_required": ["ServiceManagement", "WriteSystem"],
    "user_approved": true,
    "execution_time": "2.3s",
    "exit_code": 0,
    "rollback_available": true
  }
}
```

### **Compliance Features**

- **📋 Complete Audit Trail** - Every operation logged
- **🔒 Tamper Protection** - Cryptographic integrity verification
- **📤 Export Capabilities** - JSON, CSV, HTML formats
- **🔍 Search and Filter** - Advanced log analysis
- **📊 Reporting** - Automated compliance reports
- **⏰ Retention Policies** - Configurable log retention

### **Audit Log Analysis**

```bash
# View recent operations
tuxpilot audit --limit 20

# Export for compliance
tuxpilot audit --export json > compliance-report.json

# Search for specific operations
tuxpilot audit --search "package install"

# Generate security report
tuxpilot audit --report security --period "last 30 days"
```

---

## 🚀 Execution Modes

### **Supervised Mode** (Default)
```rust
ExecutionMode::Supervised => {
    // Always ask for user approval
    let approved = self.request_user_approval(&request).await?;
    if !approved {
        return Ok(ExecutionResult::cancelled());
    }
    self.execute_with_monitoring(&request).await
}
```

### **Semi-Autonomous Mode**
```rust
ExecutionMode::SemiAuto => {
    match request.risk_level {
        RiskLevel::Safe | RiskLevel::Low => {
            // Execute automatically
            self.execute_with_monitoring(&request).await
        }
        _ => {
            // Ask for approval
            let approved = self.request_user_approval(&request).await?;
            if approved {
                self.execute_with_monitoring(&request).await
            } else {
                Ok(ExecutionResult::cancelled())
            }
        }
    }
}
```

### **Autonomous Mode**
```rust
ExecutionMode::Autonomous => {
    match request.risk_level {
        RiskLevel::Critical => {
            // Always ask for critical operations
            let approved = self.request_user_approval(&request).await?;
            if approved {
                self.execute_with_monitoring(&request).await
            } else {
                Ok(ExecutionResult::cancelled())
            }
        }
        _ => {
            // Execute with safety checks
            self.execute_with_monitoring(&request).await
        }
    }
}
```

---

## 🔄 Rollback and Recovery

### **Rollback Capabilities**

```rust
pub struct SideEffect {
    pub effect_type: SideEffectType,
    pub description: String,
    pub reversible: bool,
    pub rollback_command: Option<String>,
}

impl CommandExecutor {
    pub async fn rollback_execution(&mut self, execution_id: Uuid) -> Result<()> {
        let execution = self.audit_logger.get_execution(execution_id).await?;
        
        for side_effect in execution.result.side_effects {
            if side_effect.reversible {
                if let Some(rollback_cmd) = side_effect.rollback_command {
                    self.execute_rollback_command(rollback_cmd).await?;
                }
            }
        }
        
        Ok(())
    }
}
```

### **Automatic Rollback Scenarios**

- **Package Installation** → `pacman -R package_name`
- **Service Start** → `systemctl stop service_name`
- **File Creation** → `rm created_file`
- **Configuration Changes** → Restore from backup
- **Permission Changes** → Restore original permissions

---

## ⚙️ Security Configuration

### **Security Settings**

```toml
# ~/.config/tuxpilot/security.toml
[security]
# Execution mode
default_execution_mode = "supervised"

# Safety settings
enable_pattern_matching = true
enable_risk_assessment = true
require_approval_for_medium_risk = true
require_approval_for_high_risk = true
always_require_approval_for_critical = true

# Audit settings
enable_audit_logging = true
audit_log_encryption = true
audit_log_retention_days = 365
export_audit_logs = true

# Permission settings
auto_detect_permissions = true
require_explicit_permission_grants = true
enable_permission_elevation = true

[dangerous_commands]
# Additional dangerous commands
custom_blacklist = ["custom-dangerous-cmd"]

[safe_commands]
# Additional safe commands
custom_whitelist = ["custom-safe-cmd"]
```

---

## 🚨 Security Best Practices

### **For Users**

1. **🔒 Start with Supervised Mode**
   - Always begin with supervised mode to understand TuxPilot's behavior
   - Review all commands before approval

2. **📊 Regular Audit Reviews**
   - Check audit logs regularly: `tuxpilot audit --limit 50`
   - Look for unexpected operations or permission escalations

3. **🛡️ Principle of Least Privilege**
   - Only grant necessary permissions
   - Regularly review granted permissions: `tuxpilot permissions --detailed`

4. **💾 Backup Important Data**
   - Maintain regular backups before system modifications
   - Test rollback procedures in safe environments

5. **🔄 Gradual Automation**
   - Start with supervised mode
   - Move to semi-autonomous after gaining confidence
   - Use autonomous mode only in trusted environments

### **For Administrators**

1. **📋 Compliance Monitoring**
   - Implement audit log monitoring
   - Set up automated compliance reports
   - Regular security assessments

2. **🔐 Access Control**
   - Implement proper user access controls
   - Monitor permission escalations
   - Regular permission audits

3. **🚨 Incident Response**
   - Establish incident response procedures
   - Monitor for security violations
   - Implement automated alerting

---

## 🔍 Security Monitoring

### **Real-time Monitoring**

```bash
# Monitor security events
tuxpilot monitor --security

# Watch for permission violations
tuxpilot audit --follow --filter "permission_denied"

# Security dashboard
tuxpilot security --dashboard
```

### **Automated Alerts**

- **Permission Escalation** - Unusual permission requests
- **Safety Violations** - Blocked dangerous commands
- **Failed Operations** - Repeated execution failures
- **Audit Anomalies** - Unusual audit log patterns

---

**TuxPilot Security: Enabling safe autonomous operations through comprehensive security architecture** 🛡️🚀
