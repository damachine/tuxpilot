# 📚 TuxPilot API Reference

**Complete reference for TuxPilot's command-line interface and internal APIs**

## 🖥️ Command Line Interface

### **Core Commands**

#### **`tuxpilot execute`**
Execute commands using natural language with AI assistance.

```bash
tuxpilot execute <description> [OPTIONS]
```

**Arguments:**
- `<description>` - Natural language description of what to do

**Options:**
- `--mode <MODE>` - Execution mode: `supervised`, `semi-auto`, `autonomous`, `read-only`
- `--dry-run` - Show what would be executed without running
- `--verbose` - Show detailed execution information

**Examples:**
```bash
tuxpilot execute "install docker and start the service"
tuxpilot execute "optimize system performance" --mode autonomous
tuxpilot execute "check disk usage" --mode read-only
```

#### **`tuxpilot chat`**
Start interactive chat session with command execution.

```bash
tuxpilot chat [OPTIONS]
```

**Options:**
- `--execute-mode <MODE>` - Execution mode for commands
- `--no-execute` - Chat only, no command execution
- `--context <FILE>` - Load conversation context from file

**Examples:**
```bash
tuxpilot chat --execute-mode supervised
tuxpilot chat --execute-mode autonomous
tuxpilot chat --no-execute
```

#### **`tuxpilot permissions`**
Display and manage execution permissions.

```bash
tuxpilot permissions [OPTIONS]
```

**Options:**
- `--detailed` - Show detailed permission analysis
- `--check <COMMAND>` - Check permissions for specific command
- `--grant <PERMISSION>` - Request permission elevation

**Examples:**
```bash
tuxpilot permissions
tuxpilot permissions --detailed
tuxpilot permissions --check "sudo systemctl restart nginx"
```

#### **`tuxpilot audit`**
View and manage audit logs.

```bash
tuxpilot audit [OPTIONS]
```

**Options:**
- `--limit <N>` - Number of recent entries to show (default: 10)
- `--export <FORMAT>` - Export format: `json`, `csv`, `html`
- `--filter <PATTERN>` - Filter entries by pattern
- `--since <DATE>` - Show entries since date
- `--rollback <ID>` - Rollback execution by ID

**Examples:**
```bash
tuxpilot audit --limit 20
tuxpilot audit --export json > audit.json
tuxpilot audit --filter "package install"
tuxpilot audit --since "2024-01-01"
```

### **Traditional Commands**

#### **`tuxpilot diagnose`**
System error diagnosis and troubleshooting.

```bash
tuxpilot diagnose [OPTIONS]
```

**Options:**
- `--auto` - Automatic error detection
- `--fix` - Attempt to fix detected issues
- `--input <TEXT>` - Analyze specific error message
- `--logs <PATH>` - Analyze specific log file

#### **`tuxpilot package`**
Package management assistance.

```bash
tuxpilot package <ACTION> [PACKAGE] [OPTIONS]
```

**Actions:**
- `install <package>` - Get installation help
- `remove <package>` - Get removal help
- `search <query>` - Search for packages
- `update` - System update guidance

#### **`tuxpilot service`**
Service management assistance.

```bash
tuxpilot service <SERVICE> <ACTION> [OPTIONS]
```

**Actions:**
- `status` - Check service status
- `start` - Start service guidance
- `stop` - Stop service guidance
- `restart` - Restart service guidance

#### **`tuxpilot monitor`**
System monitoring and health checks.

```bash
tuxpilot monitor [OPTIONS]
```

**Options:**
- `--continuous` - Continuous monitoring
- `--interval <SECONDS>` - Monitoring interval
- `--alerts` - Enable alert notifications

#### **`tuxpilot config`**
Configuration management.

```bash
tuxpilot config [OPTIONS]
```

**Options:**
- `--show` - Display current configuration
- `--set <KEY=VALUE>` - Set configuration value
- `--reset` - Reset to default configuration

## 🔧 Internal APIs

### **Execution System**

#### **CommandExecutor**
Core command execution engine with safety checks.

```rust
pub struct CommandExecutor {
    config: Config,
    permission_manager: PermissionManager,
    safety_checker: SafetyChecker,
    audit_logger: AuditLogger,
    execution_mode: ExecutionMode,
}

impl CommandExecutor {
    pub async fn new(config: Config, execution_mode: ExecutionMode) -> Result<Self>;
    pub async fn execute_request(&mut self, request: ExecutionRequest) -> Result<ExecutionResult>;
    pub async fn rollback_execution(&mut self, execution_id: Uuid) -> Result<()>;
}
```

#### **ExecutionRequest**
Request structure for command execution.

```rust
pub struct ExecutionRequest {
    pub id: Uuid,
    pub command: String,
    pub args: Vec<String>,
    pub description: String,
    pub risk_level: RiskLevel,
    pub required_permissions: Vec<Permission>,
    pub context: ExecutionContext,
}
```

#### **ExecutionResult**
Result structure from command execution.

```rust
pub struct ExecutionResult {
    pub id: Uuid,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: Duration,
    pub side_effects: Vec<SideEffect>,
}
```

### **Safety System**

#### **SafetyChecker**
Multi-layer safety analysis for commands.

```rust
pub struct SafetyChecker {
    dangerous_commands: HashSet<String>,
    dangerous_patterns: Vec<Regex>,
    safe_commands: HashSet<String>,
}

impl SafetyChecker {
    pub async fn analyze_command(&self, request: &ExecutionRequest) -> Result<SafetyResult>;
    pub fn get_safety_recommendations(&self, command: &str) -> Vec<String>;
}
```

#### **RiskLevel**
Command risk assessment levels.

```rust
pub enum RiskLevel {
    Safe,        // Read-only operations, system info
    Low,         // Package queries, service status
    Medium,      // Package installation, service restart
    High,        // System configuration changes
    Critical,    // Filesystem modifications, user management
}
```

### **Permission System**

#### **PermissionManager**
Granular permission control and verification.

```rust
pub struct PermissionManager {
    granted_permissions: HashSet<Permission>,
    user_permissions: UserPermissions,
    system_permissions: SystemPermissions,
}

impl PermissionManager {
    pub async fn new(config: &Config) -> Result<Self>;
    pub fn check_permissions(&self, required: &[Permission]) -> Result<()>;
    pub fn can_execute_command(&self, command: &str, args: &[String]) -> Result<Vec<Permission>>;
}
```

#### **Permission**
Available permission categories.

```rust
pub enum Permission {
    ReadSystem,           // System information access
    WriteSystem,          // System modifications
    PackageManagement,    // Package install/remove
    ServiceManagement,    // Service control
    FileSystemRead,       // File reading
    FileSystemWrite,      // File modification
    NetworkAccess,        // Network operations
    UserManagement,       // User/group management
    SystemConfiguration, // System config changes
}
```

### **Audit System**

#### **AuditLogger**
Complete operation logging and audit trails.

```rust
pub struct AuditLogger {
    log_file: PathBuf,
    config: Config,
}

impl AuditLogger {
    pub async fn new(config: &Config) -> Result<Self>;
    pub async fn log_request(&self, request: &ExecutionRequest) -> Result<()>;
    pub async fn log_result(&self, result: &ExecutionResult) -> Result<()>;
    pub async fn get_recent_executions(&self, limit: usize) -> Result<Vec<ExecutionRecord>>;
}
```

### **Distribution Detection**

#### **LinuxIntegration**
Linux distribution detection and command generation.

```rust
pub struct LinuxIntegration {
    pub config: Config,
    pub distribution_info: Option<DistributionInfo>,
}

impl LinuxIntegration {
    pub async fn new(config: &Config) -> Result<Self>;
    pub async fn detect_distribution(&self) -> Result<DistributionInfo>;
    pub async fn generate_commands_for_request(&self, user_request: &str, ai_client: &AiClient) -> Result<Vec<String>>;
}
```

#### **DistributionInfo**
Detected Linux distribution information.

```rust
pub struct DistributionInfo {
    pub name: String,
    pub version: String,
    pub id: String,
    pub id_like: Vec<String>,
    pub package_manager: PackageManager,
    pub service_manager: ServiceManager,
    pub init_system: String,
    pub shell: String,
    pub architecture: String,
}
```

### **MCP Integration**

#### **MCPServer**
Model Context Protocol server for AI tool communication.

```rust
pub struct MCPServer {
    config: Config,
    linux_integration: LinuxIntegration,
    tools: HashMap<String, Box<dyn MCPTool>>,
    resources: HashMap<String, Box<dyn MCPResource>>,
}

impl MCPServer {
    pub async fn new(config: Config, linux_integration: LinuxIntegration) -> Result<Self>;
    pub async fn start(&mut self, port: u16) -> Result<()>;
}
```

#### **MCPTool**
Tool interface for MCP integration.

```rust
#[async_trait]
pub trait MCPTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value, context: &MCPContext) -> Result<MCPToolResult>;
}
```

## 🔌 MCP Tools

### **Available Tools**

#### **system_info**
Get comprehensive system information.

**Parameters:**
```json
{
  "category": "all|os|hardware|performance|network"
}
```

#### **package_manager**
Manage system packages.

**Parameters:**
```json
{
  "action": "search|install|remove|update|list|info",
  "package": "package_name",
  "options": ["additional", "options"]
}
```

#### **service_manager**
Manage system services.

**Parameters:**
```json
{
  "action": "status|start|stop|restart|enable|disable|list",
  "service": "service_name"
}
```

#### **filesystem**
File system operations.

**Parameters:**
```json
{
  "action": "list|read|stat|permissions",
  "path": "/path/to/file/or/directory"
}
```

#### **process_manager**
Process management and monitoring.

**Parameters:**
```json
{
  "action": "list|info|top|search",
  "process": "process_name_or_pid"
}
```

#### **network**
Network diagnostics and information.

**Parameters:**
```json
{
  "action": "interfaces|connections|ping|dns",
  "target": "hostname_or_ip"
}
```

#### **log_analyzer**
Analyze system logs.

**Parameters:**
```json
{
  "log_type": "syslog|auth|kernel|journal",
  "lines": 100
}
```

## 🔗 MCP Resources

### **Available Resources**

#### **tuxpilot://logs/system**
Recent system logs from journalctl.

#### **tuxpilot://config/files**
Important system configuration files.

#### **tuxpilot://processes/list**
Current running processes.

#### **tuxpilot://system/status**
Overall system status and health.

## 📊 Configuration Schema

### **Main Configuration**
```toml
[ai]
provider = "Ollama"  # "Ollama" | "OpenAI" | "Anthropic"

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"
timeout = 30

[ai.openai]
api_key = "your-api-key"
model = "gpt-4"
timeout = 30

[ai.anthropic]
api_key = "your-api-key"
model = "claude-3-sonnet"
timeout = 30

[execution]
default_mode = "supervised"  # "supervised" | "semi-auto" | "autonomous" | "read-only"
enable_audit_logging = true
require_approval_for_high_risk = true
max_execution_time = 300

[system]
package_manager = "Pacman"  # Auto-detected
service_manager = "Systemd"
log_paths = ["/var/log/syslog", "/var/log/messages"]

[security]
enable_pattern_matching = true
enable_risk_assessment = true
audit_log_retention_days = 365
```

## 🚨 Error Codes

### **Execution Errors**
- `E001` - Permission denied
- `E002` - Safety check failed
- `E003` - Command not found
- `E004` - Execution timeout
- `E005` - Invalid parameters

### **Configuration Errors**
- `C001` - Invalid configuration file
- `C002` - Missing required configuration
- `C003` - AI provider not available

### **System Errors**
- `S001` - Distribution detection failed
- `S002` - Package manager not found
- `S003` - Service manager not available

## 🔄 Return Codes

### **Success Codes**
- `0` - Success
- `1` - General error
- `2` - Misuse of shell command
- `126` - Command cannot execute
- `127` - Command not found
- `128` - Invalid argument to exit
- `130` - Script terminated by Control-C

### **TuxPilot Specific Codes**
- `64` - Permission denied by user
- `65` - Safety check failed
- `66` - AI provider unavailable
- `67` - Configuration error
- `68` - Audit log error

---

**For more information, see the [Getting Started Guide](GETTING_STARTED.md) and [Security Documentation](../SECURITY.md).**
