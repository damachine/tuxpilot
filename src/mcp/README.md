# TuxPilot MCP (Model Context Protocol) Integration

The Model Context Protocol (MCP) integration in TuxPilot provides a standardized way for AI models to interact with external tools and resources, enabling more sophisticated and context-aware system administration capabilities.

## 🔗 What is MCP?

Model Context Protocol (MCP) is an open standard that enables AI models to securely connect to external data sources and tools. In TuxPilot, MCP allows AI agents to:

- **Access system resources** in a controlled and secure manner
- **Execute tools** with proper permission management
- **Maintain context** across complex multi-step operations
- **Integrate with external services** and APIs
- **Provide real-time data** to AI models

## 🏗️ MCP Architecture in TuxPilot

```
MCP Integration Architecture:
├── 🔌 MCP Server (TuxPilot as MCP Server)
├── 🛠️ Tools Registry (Available system tools)
├── 📚 Resources Manager (System resources access)
├── 🔒 Security Layer (Permission and validation)
└── 🤖 AI Client Integration (Model communication)
```

## 🛠️ Tools System

**File**: `tools.rs`

The tools system provides AI models with access to system administration capabilities through standardized MCP tool interfaces.

### Available Tools

#### System Information Tools

```rust
pub struct SystemInfoTool;

impl McpTool for SystemInfoTool {
    fn name(&self) -> &str { "system_info" }
    
    fn description(&self) -> &str {
        "Get comprehensive system information including hardware, OS, and performance metrics"
    }
    
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "category": {
                    "type": "string",
                    "enum": ["hardware", "os", "performance", "network", "all"],
                    "description": "Category of system information to retrieve"
                }
            }
        })
    }
}
```

#### Package Management Tools

```rust
pub struct PackageManagerTool;

impl McpTool for PackageManagerTool {
    fn name(&self) -> &str { "package_manager" }
    
    fn description(&self) -> &str {
        "Manage system packages: install, remove, update, search"
    }
    
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["install", "remove", "update", "search", "info"],
                    "description": "Package management action to perform"
                },
                "packages": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "List of package names"
                },
                "options": {
                    "type": "object",
                    "description": "Additional options for the operation"
                }
            },
            "required": ["action"]
        })
    }
}
```

#### Service Management Tools

```rust
pub struct ServiceManagerTool;

impl McpTool for ServiceManagerTool {
    fn name(&self) -> &str { "service_manager" }
    
    fn description(&self) -> &str {
        "Manage system services: start, stop, restart, enable, disable, status"
    }
    
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["start", "stop", "restart", "enable", "disable", "status"],
                    "description": "Service management action"
                },
                "service": {
                    "type": "string",
                    "description": "Name of the service"
                }
            },
            "required": ["action", "service"]
        })
    }
}
```

#### File System Tools

```rust
pub struct FileSystemTool;

impl McpTool for FileSystemTool {
    fn name(&self) -> &str { "filesystem" }
    
    fn description(&self) -> &str {
        "File system operations: read, write, list, permissions, ownership"
    }
    
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["read", "write", "list", "chmod", "chown", "mkdir", "rm"],
                    "description": "File system operation to perform"
                },
                "path": {
                    "type": "string",
                    "description": "File or directory path"
                },
                "content": {
                    "type": "string",
                    "description": "Content for write operations"
                },
                "permissions": {
                    "type": "string",
                    "description": "Permissions for chmod operations"
                }
            },
            "required": ["operation", "path"]
        })
    }
}
```

### Tool Execution Framework

```rust
pub trait McpTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;
    
    async fn execute(
        &self,
        parameters: serde_json::Value,
        context: &McpContext
    ) -> Result<McpToolResult>;
}

pub struct McpToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}
```

## 📚 Resources System

**File**: `resources.rs`

The resources system provides AI models with access to system resources and data through MCP resource interfaces.

### Resource Types

#### System Configuration Resources

```rust
pub struct ConfigurationResource;

impl McpResource for ConfigurationResource {
    fn uri(&self) -> &str { "config://tuxpilot" }
    
    fn name(&self) -> &str { "TuxPilot Configuration" }
    
    fn description(&self) -> &str {
        "Access to TuxPilot configuration settings and system preferences"
    }
    
    fn mime_type(&self) -> &str { "application/toml" }
    
    async fn read(&self, context: &McpContext) -> Result<String> {
        // Return current configuration as TOML
        let config = context.get_config();
        Ok(toml::to_string(&config)?)
    }
}
```

#### Log File Resources

```rust
pub struct LogFileResource {
    log_path: PathBuf,
}

impl McpResource for LogFileResource {
    fn uri(&self) -> &str { "logs://system" }
    
    fn name(&self) -> &str { "System Logs" }
    
    fn description(&self) -> &str {
        "Access to system log files for analysis and troubleshooting"
    }
    
    async fn read(&self, context: &McpContext) -> Result<String> {
        // Read and return log content with proper permissions
        if context.has_permission(Permission::ReadLogs) {
            fs::read_to_string(&self.log_path).await
        } else {
            Err(McpError::PermissionDenied)
        }
    }
}
```

#### Process Information Resources

```rust
pub struct ProcessResource;

impl McpResource for ProcessResource {
    fn uri(&self) -> &str { "proc://processes" }
    
    fn name(&self) -> &str { "Process Information" }
    
    fn description(&self) -> &str {
        "Real-time process information and system resource usage"
    }
    
    async fn read(&self, context: &McpContext) -> Result<String> {
        let processes = context.linux_integration.get_processes().await?;
        Ok(serde_json::to_string_pretty(&processes)?)
    }
}
```

### Resource Management

```rust
pub trait McpResource: Send + Sync {
    fn uri(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn mime_type(&self) -> &str;
    
    async fn read(&self, context: &McpContext) -> Result<String>;
    async fn write(&self, content: &str, context: &McpContext) -> Result<()> {
        Err(McpError::ReadOnly)
    }
}
```

## 🔒 Security and Permissions

### MCP Security Model

The MCP integration includes comprehensive security controls:

```rust
pub struct McpSecurityManager {
    permissions: PermissionManager,
    audit_logger: AuditLogger,
    rate_limiter: RateLimiter,
}

impl McpSecurityManager {
    pub async fn validate_tool_execution(
        &self,
        tool_name: &str,
        parameters: &serde_json::Value,
        context: &McpContext
    ) -> Result<()> {
        // Check permissions
        self.check_tool_permissions(tool_name, context)?;
        
        // Validate parameters
        self.validate_parameters(tool_name, parameters)?;
        
        // Rate limiting
        self.check_rate_limits(context)?;
        
        // Audit logging
        self.log_tool_request(tool_name, parameters, context).await?;
        
        Ok(())
    }
}
```

### Permission Categories

```rust
pub enum McpPermission {
    // System Information
    ReadSystemInfo,
    ReadProcessInfo,
    ReadNetworkInfo,
    
    // File System
    ReadFiles(PathBuf),
    WriteFiles(PathBuf),
    ExecuteFiles(PathBuf),
    
    // Package Management
    InstallPackages,
    RemovePackages,
    UpdatePackages,
    
    // Service Management
    StartServices,
    StopServices,
    ConfigureServices,
    
    // Network Operations
    ConfigureNetwork,
    ManageFirewall,
    
    // System Configuration
    ModifyConfig,
    ManageUsers,
    ModifyKernel,
}
```

## 🔌 MCP Server Implementation

### Server Configuration

```rust
pub struct McpServer {
    tools: HashMap<String, Box<dyn McpTool>>,
    resources: HashMap<String, Box<dyn McpResource>>,
    security_manager: McpSecurityManager,
    config: McpConfig,
}

pub struct McpConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub port: u16,
    pub max_connections: usize,
    pub request_timeout: Duration,
    pub enable_tls: bool,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
}
```

### Server Initialization

```rust
impl McpServer {
    pub async fn new(config: McpConfig, linux_integration: LinuxIntegration) -> Result<Self> {
        let mut server = McpServer {
            tools: HashMap::new(),
            resources: HashMap::new(),
            security_manager: McpSecurityManager::new(&config),
            config,
        };
        
        // Register built-in tools
        server.register_tool(Box::new(SystemInfoTool::new()));
        server.register_tool(Box::new(PackageManagerTool::new()));
        server.register_tool(Box::new(ServiceManagerTool::new()));
        server.register_tool(Box::new(FileSystemTool::new()));
        
        // Register built-in resources
        server.register_resource(Box::new(ConfigurationResource::new()));
        server.register_resource(Box::new(LogFileResource::new()));
        server.register_resource(Box::new(ProcessResource::new()));
        
        Ok(server)
    }
    
    pub async fn start(&self) -> Result<()> {
        // Start MCP server
        let listener = TcpListener::bind(&format!("{}:{}", 
            self.config.bind_address, 
            self.config.port
        )).await?;
        
        loop {
            let (stream, addr) = listener.accept().await?;
            let server = self.clone();
            
            tokio::spawn(async move {
                if let Err(e) = server.handle_connection(stream, addr).await {
                    eprintln!("MCP connection error: {}", e);
                }
            });
        }
    }
}
```

## 🤖 AI Model Integration

### MCP Client for AI Models

```rust
pub struct McpClient {
    connection: McpConnection,
    available_tools: Vec<McpToolInfo>,
    available_resources: Vec<McpResourceInfo>,
}

impl McpClient {
    pub async fn connect(server_url: &str) -> Result<Self> {
        let connection = McpConnection::connect(server_url).await?;
        
        // Discover available tools and resources
        let tools = connection.list_tools().await?;
        let resources = connection.list_resources().await?;
        
        Ok(McpClient {
            connection,
            available_tools: tools,
            available_resources: resources,
        })
    }
    
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        parameters: serde_json::Value
    ) -> Result<McpToolResult> {
        self.connection.call_tool(tool_name, parameters).await
    }
    
    pub async fn read_resource(&self, uri: &str) -> Result<String> {
        self.connection.read_resource(uri).await
    }
}
```

### Integration with AI Providers

```rust
// Example: Integrating MCP with Ollama
pub struct OllamaMcpIntegration {
    ollama_client: OllamaClient,
    mcp_client: McpClient,
}

impl OllamaMcpIntegration {
    pub async fn process_request(&self, user_input: &str) -> Result<String> {
        // Prepare context with available tools and resources
        let context = self.prepare_mcp_context().await?;
        
        // Send request to Ollama with MCP context
        let response = self.ollama_client.chat_with_tools(
            user_input,
            &context.tools,
            &context.resources
        ).await?;
        
        // Execute any tool calls requested by the model
        if let Some(tool_calls) = response.tool_calls {
            for tool_call in tool_calls {
                let result = self.mcp_client.execute_tool(
                    &tool_call.name,
                    tool_call.parameters
                ).await?;
                
                // Send tool result back to model
                self.ollama_client.submit_tool_result(
                    &tool_call.id,
                    &result
                ).await?;
            }
        }
        
        Ok(response.content)
    }
}
```

## 📊 Monitoring and Analytics

### MCP Usage Metrics

```rust
pub struct McpMetrics {
    pub tool_calls: HashMap<String, u64>,
    pub resource_reads: HashMap<String, u64>,
    pub error_counts: HashMap<String, u64>,
    pub average_response_time: Duration,
    pub active_connections: u32,
}
```

### Performance Monitoring

```rust
impl McpServer {
    async fn handle_tool_call(&self, request: ToolCallRequest) -> Result<ToolCallResponse> {
        let start_time = Instant::now();
        
        // Execute tool
        let result = self.execute_tool(&request.tool_name, &request.parameters).await;
        
        // Record metrics
        let execution_time = start_time.elapsed();
        self.metrics.record_tool_call(&request.tool_name, execution_time, &result);
        
        result
    }
}
```

## 🔧 Configuration

### MCP Configuration

```toml
[mcp]
enabled = true
bind_address = "127.0.0.1"
port = 3000
max_connections = 100
request_timeout = "30s"
enable_tls = false

[mcp.security]
require_authentication = true
rate_limit_requests = 60  # per minute
audit_all_requests = true

[mcp.tools]
enabled = [
    "system_info",
    "package_manager", 
    "service_manager",
    "filesystem"
]

[mcp.resources]
enabled = [
    "config://tuxpilot",
    "logs://system",
    "proc://processes"
]
```

## 🚀 Usage Examples

### Basic Tool Execution

```bash
# Using MCP client to get system information
curl -X POST http://localhost:3000/tools/call \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "system_info",
    "parameters": {
      "category": "performance"
    }
  }'
```

### Resource Access

```bash
# Reading system configuration
curl -X GET http://localhost:3000/resources/config%3A%2F%2Ftuxpilot
```

### Integration with AI Models

```python
# Python example using MCP client
import asyncio
from mcp_client import McpClient

async def main():
    client = await McpClient.connect("http://localhost:3000")
    
    # Execute system information tool
    result = await client.execute_tool("system_info", {
        "category": "all"
    })
    
    print(f"System info: {result.output}")

asyncio.run(main())
```
