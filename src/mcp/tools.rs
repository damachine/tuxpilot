use anyhow::Result;
use serde_json::json;

use super::{MCPTool, MCPContext, MCPToolResult};

/// System information tool
pub struct SystemInfoTool;

impl SystemInfoTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for SystemInfoTool {
    fn name(&self) -> &str {
        "system_info"
    }

    fn description(&self) -> &str {
        "Get comprehensive system information including OS, hardware, and performance metrics"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "category": {
                    "type": "string",
                    "enum": ["all", "os", "hardware", "performance", "network"],
                    "description": "Category of system information to retrieve"
                }
            }
        })
    }

    async fn execute(&self, params: serde_json::Value, context: &MCPContext) -> Result<MCPToolResult> {
        let category = params.get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("all");

        let system_info = context.linux_integration.get_system_info().await?;
        
        let result = match category {
            "all" => json!({
                "system_info": system_info,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
            "os" => json!({
                "os_info": system_info,
                "distribution": context.linux_integration.distribution_info
            }),
            _ => json!({
                "system_info": system_info
            })
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}

/// Package manager tool
pub struct PackageManagerTool;

impl PackageManagerTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for PackageManagerTool {
    fn name(&self) -> &str {
        "package_manager"
    }

    fn description(&self) -> &str {
        "Manage system packages - search, install, remove, update packages"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["search", "install", "remove", "update", "list", "info"],
                    "description": "Package management action to perform"
                },
                "package": {
                    "type": "string",
                    "description": "Package name (required for most actions)"
                },
                "options": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Additional options for the package manager"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: serde_json::Value, context: &MCPContext) -> Result<MCPToolResult> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Action is required"))?;

        let package = params.get("package")
            .and_then(|v| v.as_str());

        let result = match action {
            "search" => {
                if let Some(pkg) = package {
                    let suggestion = context.linux_integration.get_package_suggestion("search", Some(pkg)).await?;
                    json!({
                        "action": "search",
                        "package": pkg,
                        "suggestion": suggestion
                    })
                } else {
                    return Ok(MCPToolResult {
                        success: false,
                        content: json!({}),
                        error: Some("Package name required for search".to_string()),
                    });
                }
            },
            "list" => {
                let suggestion = context.linux_integration.get_package_suggestion("list", None).await?;
                json!({
                    "action": "list",
                    "suggestion": suggestion
                })
            },
            _ => {
                json!({
                    "action": action,
                    "package": package,
                    "note": "This action would require execution permissions"
                })
            }
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}

/// Service manager tool
pub struct ServiceManagerTool;

impl ServiceManagerTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for ServiceManagerTool {
    fn name(&self) -> &str {
        "service_manager"
    }

    fn description(&self) -> &str {
        "Manage system services - start, stop, restart, enable, disable services"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["status", "start", "stop", "restart", "enable", "disable", "list"],
                    "description": "Service management action"
                },
                "service": {
                    "type": "string",
                    "description": "Service name (required for most actions)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: serde_json::Value, context: &MCPContext) -> Result<MCPToolResult> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Action is required"))?;

        let service = params.get("service")
            .and_then(|v| v.as_str());

        let result = match action {
            "status" => {
                if let Some(svc) = service {
                    let status = context.linux_integration.get_service_info(svc).await?;
                    json!({
                        "action": "status",
                        "service": svc,
                        "status": status
                    })
                } else {
                    return Ok(MCPToolResult {
                        success: false,
                        content: json!({}),
                        error: Some("Service name required for status".to_string()),
                    });
                }
            },
            "list" => {
                // This would list all services
                json!({
                    "action": "list",
                    "note": "Service listing would be implemented here"
                })
            },
            _ => {
                json!({
                    "action": action,
                    "service": service,
                    "note": "This action would require execution permissions"
                })
            }
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}

/// File system tool
pub struct FileSystemTool;

impl FileSystemTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for FileSystemTool {
    fn name(&self) -> &str {
        "filesystem"
    }

    fn description(&self) -> &str {
        "File system operations - list, read, write, permissions"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "read", "stat", "permissions"],
                    "description": "File system action"
                },
                "path": {
                    "type": "string",
                    "description": "File or directory path"
                }
            },
            "required": ["action", "path"]
        })
    }

    async fn execute(&self, params: serde_json::Value, _context: &MCPContext) -> Result<MCPToolResult> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Action is required"))?;

        let path = params.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Path is required"))?;

        let result = match action {
            "list" => {
                // Safe directory listing
                if let Ok(entries) = tokio::fs::read_dir(path).await {
                    let mut files = Vec::new();
                    let mut entries = entries;
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        if let Ok(metadata) = entry.metadata().await {
                            files.push(json!({
                                "name": entry.file_name().to_string_lossy(),
                                "is_dir": metadata.is_dir(),
                                "size": metadata.len(),
                                "modified": metadata.modified().ok()
                                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                    .map(|d| d.as_secs())
                            }));
                        }
                    }
                    json!({
                        "action": "list",
                        "path": path,
                        "entries": files
                    })
                } else {
                    return Ok(MCPToolResult {
                        success: false,
                        content: json!({}),
                        error: Some(format!("Cannot access path: {}", path)),
                    });
                }
            },
            "read" => {
                // Safe file reading (with size limits)
                match tokio::fs::metadata(path).await {
                    Ok(metadata) => {
                        if metadata.len() > 1024 * 1024 { // 1MB limit
                            return Ok(MCPToolResult {
                                success: false,
                                content: json!({}),
                                error: Some("File too large to read".to_string()),
                            });
                        }
                        
                        match tokio::fs::read_to_string(path).await {
                            Ok(content) => json!({
                                "action": "read",
                                "path": path,
                                "content": content,
                                "size": metadata.len()
                            }),
                            Err(e) => {
                                return Ok(MCPToolResult {
                                    success: false,
                                    content: json!({}),
                                    error: Some(format!("Cannot read file: {}", e)),
                                });
                            }
                        }
                    },
                    Err(e) => {
                        return Ok(MCPToolResult {
                            success: false,
                            content: json!({}),
                            error: Some(format!("Cannot access file: {}", e)),
                        });
                    }
                }
            },
            _ => {
                json!({
                    "action": action,
                    "path": path,
                    "note": "Action not yet implemented"
                })
            }
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}

/// Process manager tool
pub struct ProcessManagerTool;

impl ProcessManagerTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for ProcessManagerTool {
    fn name(&self) -> &str {
        "process_manager"
    }

    fn description(&self) -> &str {
        "Process management - list, monitor, analyze running processes"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "info", "top", "search"],
                    "description": "Process management action"
                },
                "process": {
                    "type": "string",
                    "description": "Process name or PID"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: serde_json::Value, _context: &MCPContext) -> Result<MCPToolResult> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Action is required"))?;

        let result = match action {
            "list" => {
                // Use ps command to list processes
                let output = tokio::process::Command::new("ps")
                    .args(&["aux", "--sort=-pcpu"])
                    .output()
                    .await?;

                let ps_output = String::from_utf8_lossy(&output.stdout);
                json!({
                    "action": "list",
                    "processes": ps_output.lines().take(20).collect::<Vec<_>>()
                })
            },
            _ => {
                json!({
                    "action": action,
                    "note": "Action not yet implemented"
                })
            }
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}

/// Network tool
pub struct NetworkTool;

impl NetworkTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for NetworkTool {
    fn name(&self) -> &str {
        "network"
    }

    fn description(&self) -> &str {
        "Network diagnostics and information"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["interfaces", "connections", "ping", "dns"],
                    "description": "Network action"
                },
                "target": {
                    "type": "string",
                    "description": "Target for ping/dns operations"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: serde_json::Value, _context: &MCPContext) -> Result<MCPToolResult> {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Action is required"))?;

        let result = match action {
            "interfaces" => {
                let output = tokio::process::Command::new("ip")
                    .args(&["addr", "show"])
                    .output()
                    .await?;

                let interfaces = String::from_utf8_lossy(&output.stdout);
                json!({
                    "action": "interfaces",
                    "interfaces": interfaces
                })
            },
            _ => {
                json!({
                    "action": action,
                    "note": "Action not yet implemented"
                })
            }
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}

/// Log analyzer tool
pub struct LogAnalyzerTool;

impl LogAnalyzerTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPTool for LogAnalyzerTool {
    fn name(&self) -> &str {
        "log_analyzer"
    }

    fn description(&self) -> &str {
        "Analyze system logs for errors, warnings, and patterns"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "log_type": {
                    "type": "string",
                    "enum": ["syslog", "auth", "kernel", "journal"],
                    "description": "Type of log to analyze"
                },
                "lines": {
                    "type": "integer",
                    "description": "Number of recent lines to analyze",
                    "default": 100
                }
            },
            "required": ["log_type"]
        })
    }

    async fn execute(&self, params: serde_json::Value, _context: &MCPContext) -> Result<MCPToolResult> {
        let log_type = params.get("log_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Log type is required"))?;

        let lines = params.get("lines")
            .and_then(|v| v.as_u64())
            .unwrap_or(100);

        let result = match log_type {
            "journal" => {
                let output = tokio::process::Command::new("journalctl")
                    .args(&["-n", &lines.to_string(), "--no-pager"])
                    .output()
                    .await?;

                let logs = String::from_utf8_lossy(&output.stdout);
                json!({
                    "log_type": "journal",
                    "lines": lines,
                    "content": logs
                })
            },
            _ => {
                json!({
                    "log_type": log_type,
                    "note": "Log type not yet implemented"
                })
            }
        };

        Ok(MCPToolResult {
            success: true,
            content: result,
            error: None,
        })
    }
}
