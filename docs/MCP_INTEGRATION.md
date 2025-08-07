# 🔌 Model Context Protocol (MCP) Integration

**TuxPilot MCP Integration für erweiterte AI-Tool-Kommunikation**

## 🎯 **Überblick**

Das Model Context Protocol (MCP) ermöglicht es TuxPilot, mit verschiedenen AI-Modellen und Tools zu kommunizieren, Kontext zu teilen und komplexe Workflows zu orchestrieren.

## 🏗️ **Architektur**

### **MCP Server Implementation**

```rust
// src/mcp/server.rs
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPServer {
    pub name: String,
    pub version: String,
    pub capabilities: ServerCapabilities,
    pub tools: Vec<MCPTool>,
    pub resources: Vec<MCPResource>,
    pub prompts: Vec<MCPPrompt>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: Option<ToolCapabilities>,
    pub resources: Option<ResourceCapabilities>,
    pub prompts: Option<PromptCapabilities>,
    pub logging: Option<LoggingCapabilities>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub handler: ToolHandler,
}

// Tool-Definitionen für Linux-Operationen
pub enum ToolHandler {
    SystemInfo,
    PackageManager(PackageOperation),
    ServiceManager(ServiceOperation),
    FileSystem(FileSystemOperation),
    NetworkDiagnostics,
    LogAnalysis,
    ProcessManager,
    UserManager,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageOperation {
    Install { package: String },
    Remove { package: String },
    Update { package: Option<String> },
    Search { query: String },
    Info { package: String },
    ListInstalled,
    ListUpgradable,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceOperation {
    Start { service: String },
    Stop { service: String },
    Restart { service: String },
    Status { service: String },
    Enable { service: String },
    Disable { service: String },
    List,
}
```

### **MCP Client Implementation**

```rust
// src/mcp/client.rs
use reqwest::Client;
use serde_json::Value;

pub struct MCPClient {
    client: Client,
    server_url: String,
    session_id: Uuid,
}

impl MCPClient {
    pub async fn new(server_url: String) -> Result<Self> {
        let client = Client::new();
        let session_id = Uuid::new_v4();
        
        // Initialize session with server
        let init_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": {},
                    "prompts": {}
                },
                "clientInfo": {
                    "name": "TuxPilot",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        });
        
        // Send initialization request
        let response = client
            .post(&server_url)
            .json(&init_request)
            .send()
            .await?;
            
        Ok(Self {
            client,
            server_url,
            session_id,
        })
    }
    
    pub async fn call_tool(&self, tool_name: &str, arguments: Value) -> Result<Value> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4(),
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        });
        
        let response = self.client
            .post(&self.server_url)
            .json(&request)
            .send()
            .await?;
            
        let result: Value = response.json().await?;
        Ok(result["result"].clone())
    }
}
```

## 🛠️ **Tool-Implementierungen**

### **System Information Tool**

```rust
// src/mcp/tools/system_info.rs
use crate::system_monitor::SystemMonitor;

pub struct SystemInfoTool {
    monitor: SystemMonitor,
}

impl SystemInfoTool {
    pub async fn execute(&mut self, _args: Value) -> Result<Value> {
        let system_status = self.monitor.get_system_status().await?;
        let hardware_info = self.monitor.get_hardware_info().await?;
        
        Ok(json!({
            "system_status": system_status,
            "hardware_info": hardware_info,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}
```

### **Package Manager Tool**

```rust
// src/mcp/tools/package_manager.rs
use crate::linux_integration::LinuxIntegration;

pub struct PackageManagerTool {
    integration: LinuxIntegration,
}

impl PackageManagerTool {
    pub async fn execute(&self, operation: PackageOperation) -> Result<Value> {
        match operation {
            PackageOperation::Install { package } => {
                let suggestion = self.integration
                    .get_package_suggestion("install", Some(&package))
                    .await?;
                    
                Ok(json!({
                    "operation": "install",
                    "package": package,
                    "suggested_command": suggestion,
                    "safe_to_execute": true
                }))
            }
            
            PackageOperation::Search { query } => {
                // Implement package search
                let results = self.search_packages(&query).await?;
                
                Ok(json!({
                    "operation": "search",
                    "query": query,
                    "results": results
                }))
            }
            
            // ... weitere Operationen
        }
    }
    
    async fn search_packages(&self, query: &str) -> Result<Vec<PackageInfo>> {
        // Implementation für Paket-Suche
        // Je nach Paket-Manager (pacman, apt, etc.)
        todo!()
    }
}
```

## 🔄 **Workflow Integration**

### **Multi-Step Workflows**

```rust
// src/mcp/workflows.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub context: WorkflowContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub tool: String,
    pub arguments: Value,
    pub depends_on: Vec<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowContext {
    pub variables: HashMap<String, Value>,
    pub shared_state: Value,
}

// Beispiel: System-Update Workflow
pub fn create_system_update_workflow() -> Workflow {
    Workflow {
        id: Uuid::new_v4(),
        name: "System Update".to_string(),
        description: "Vollständiges System-Update mit Sicherheitschecks".to_string(),
        steps: vec![
            WorkflowStep {
                id: "backup_check".to_string(),
                tool: "backup_manager".to_string(),
                arguments: json!({"action": "verify_recent_backup"}),
                depends_on: vec![],
                condition: None,
            },
            WorkflowStep {
                id: "update_package_lists".to_string(),
                tool: "package_manager".to_string(),
                arguments: json!({"operation": "update_lists"}),
                depends_on: vec!["backup_check".to_string()],
                condition: Some("backup_check.success == true".to_string()),
            },
            WorkflowStep {
                id: "list_upgrades".to_string(),
                tool: "package_manager".to_string(),
                arguments: json!({"operation": "list_upgradable"}),
                depends_on: vec!["update_package_lists".to_string()],
                condition: None,
            },
            WorkflowStep {
                id: "security_scan".to_string(),
                tool: "security_scanner".to_string(),
                arguments: json!({"scan_type": "pre_update"}),
                depends_on: vec!["list_upgrades".to_string()],
                condition: None,
            },
            WorkflowStep {
                id: "perform_update".to_string(),
                tool: "package_manager".to_string(),
                arguments: json!({"operation": "upgrade_all"}),
                depends_on: vec!["security_scan".to_string()],
                condition: Some("security_scan.critical_issues == 0".to_string()),
            },
            WorkflowStep {
                id: "post_update_check".to_string(),
                tool: "system_health".to_string(),
                arguments: json!({"check_type": "post_update"}),
                depends_on: vec!["perform_update".to_string()],
                condition: None,
            },
        ],
        context: WorkflowContext {
            variables: HashMap::new(),
            shared_state: json!({}),
        },
    }
}
```

## 🔗 **Integration mit AI-Modellen**

### **Context Sharing**

```rust
// src/mcp/context.rs
use std::collections::HashMap;

pub struct ContextManager {
    contexts: HashMap<String, Context>,
    global_context: Context,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub system_state: SystemState,
    pub user_preferences: UserPreferences,
    pub recent_actions: Vec<Action>,
    pub error_history: Vec<ErrorEvent>,
    pub performance_metrics: PerformanceMetrics,
}

impl ContextManager {
    pub fn create_ai_context(&self, session_id: &str) -> AIContext {
        let context = self.contexts.get(session_id)
            .unwrap_or(&self.global_context);
            
        AIContext {
            system_info: context.system_state.to_summary(),
            recent_errors: context.error_history.iter()
                .take(5)
                .cloned()
                .collect(),
            user_expertise_level: context.user_preferences.expertise_level,
            preferred_language: context.user_preferences.language.clone(),
            active_services: context.system_state.active_services.clone(),
            resource_usage: context.performance_metrics.current_usage(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AIContext {
    pub system_info: String,
    pub recent_errors: Vec<ErrorEvent>,
    pub user_expertise_level: ExpertiseLevel,
    pub preferred_language: String,
    pub active_services: Vec<String>,
    pub resource_usage: ResourceUsage,
}
```

## 📡 **MCP Server Configuration**

### **Server Setup**

```toml
# mcp-server.toml
[server]
name = "TuxPilot MCP Server"
version = "0.1.0"
host = "localhost"
port = 8080
protocol_version = "2024-11-05"

[capabilities]
tools = true
resources = true
prompts = true
logging = true

[tools]
# System Tools
system_info = { enabled = true, permissions = ["read"] }
package_manager = { enabled = true, permissions = ["read", "write"] }
service_manager = { enabled = true, permissions = ["read", "write"] }
file_system = { enabled = true, permissions = ["read", "write"] }
network_diagnostics = { enabled = true, permissions = ["read"] }
log_analysis = { enabled = true, permissions = ["read"] }

[security]
require_authentication = true
allowed_clients = ["tuxpilot-cli", "tuxpilot-web"]
rate_limiting = { requests_per_minute = 100 }

[logging]
level = "info"
file = "/var/log/tuxpilot-mcp.log"
```

## 🚀 **Verwendung**

### **CLI Integration**

```bash
# MCP Server starten
tuxpilot mcp server --config mcp-server.toml

# MCP Client verwenden
tuxpilot mcp call system_info
tuxpilot mcp call package_manager --operation install --package firefox
tuxpilot mcp workflow run system_update

# MCP Tools auflisten
tuxpilot mcp tools list
tuxpilot mcp tools describe package_manager
```

### **Programmatische Verwendung**

```rust
// Beispiel: MCP in TuxPilot AI Client
impl AiClient {
    pub async fn process_with_mcp(&self, query: &str) -> Result<String> {
        // 1. Kontext aus MCP abrufen
        let context = self.mcp_client.get_context().await?;
        
        // 2. Verfügbare Tools ermitteln
        let tools = self.mcp_client.list_tools().await?;
        
        // 3. AI-Modell mit Kontext und Tools aufrufen
        let enhanced_prompt = format!(
            "System Context: {}\nAvailable Tools: {}\nUser Query: {}",
            serde_json::to_string(&context)?,
            serde_json::to_string(&tools)?,
            query
        );
        
        // 4. AI-Antwort mit Tool-Aufrufen verarbeiten
        let response = self.send_ollama_request(&enhanced_prompt).await?;
        
        // 5. Tool-Aufrufe extrahieren und ausführen
        if let Some(tool_calls) = self.extract_tool_calls(&response)? {
            for tool_call in tool_calls {
                let result = self.mcp_client
                    .call_tool(&tool_call.name, tool_call.arguments)
                    .await?;
                    
                // Tool-Ergebnis in Antwort integrieren
            }
        }
        
        Ok(response)
    }
}
```

## 🎯 **Vorteile der MCP-Integration**

✅ **Standardisierte Tool-Kommunikation**
✅ **Erweiterte Kontext-Verwaltung**
✅ **Multi-Model Unterstützung**
✅ **Workflow-Orchestrierung**
✅ **Plugin-Ecosystem Vorbereitung**
✅ **Enterprise-Integration**

**MCP macht TuxPilot zu einer echten AI-Plattform! 🚀**
