use anyhow::Result;

use super::{MCPResource, MCPContext};

/// System logs resource
pub struct SystemLogsResource;

impl SystemLogsResource {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPResource for SystemLogsResource {
    fn uri(&self) -> &str {
        "tuxpilot://logs/system"
    }

    fn name(&self) -> &str {
        "System Logs"
    }

    fn description(&self) -> &str {
        "Recent system logs from journalctl"
    }

    fn mime_type(&self) -> &str {
        "text/plain"
    }

    async fn read(&self, _context: &MCPContext) -> Result<String> {
        let output = tokio::process::Command::new("journalctl")
            .args(&["-n", "100", "--no-pager"])
            .output()
            .await?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// Configuration files resource
pub struct ConfigFilesResource;

impl ConfigFilesResource {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPResource for ConfigFilesResource {
    fn uri(&self) -> &str {
        "tuxpilot://config/files"
    }

    fn name(&self) -> &str {
        "Configuration Files"
    }

    fn description(&self) -> &str {
        "Important system configuration files"
    }

    fn mime_type(&self) -> &str {
        "application/json"
    }

    async fn read(&self, _context: &MCPContext) -> Result<String> {
        let config_files = vec![
            "/etc/os-release",
            "/etc/hostname",
            "/etc/hosts",
            "/etc/fstab",
            "/etc/passwd",
            "/etc/group",
        ];

        let mut configs = serde_json::Map::new();
        
        for file_path in config_files {
            if let Ok(content) = tokio::fs::read_to_string(file_path).await {
                configs.insert(
                    file_path.to_string(),
                    serde_json::Value::String(content)
                );
            }
        }

        Ok(serde_json::to_string_pretty(&configs)?)
    }
}

/// Process list resource
pub struct ProcessListResource;

impl ProcessListResource {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPResource for ProcessListResource {
    fn uri(&self) -> &str {
        "tuxpilot://processes/list"
    }

    fn name(&self) -> &str {
        "Process List"
    }

    fn description(&self) -> &str {
        "Current running processes"
    }

    fn mime_type(&self) -> &str {
        "text/plain"
    }

    async fn read(&self, _context: &MCPContext) -> Result<String> {
        let output = tokio::process::Command::new("ps")
            .args(&["aux", "--sort=-pcpu"])
            .output()
            .await?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// System status resource
pub struct SystemStatusResource;

impl SystemStatusResource {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl MCPResource for SystemStatusResource {
    fn uri(&self) -> &str {
        "tuxpilot://system/status"
    }

    fn name(&self) -> &str {
        "System Status"
    }

    fn description(&self) -> &str {
        "Overall system status and health"
    }

    fn mime_type(&self) -> &str {
        "application/json"
    }

    async fn read(&self, context: &MCPContext) -> Result<String> {
        let system_info = context.linux_integration.get_system_info().await?;
        
        // Get additional status information
        let uptime_output = tokio::process::Command::new("uptime")
            .output()
            .await?;
        let uptime = String::from_utf8_lossy(&uptime_output.stdout);

        let df_output = tokio::process::Command::new("df")
            .args(&["-h"])
            .output()
            .await?;
        let disk_usage = String::from_utf8_lossy(&df_output.stdout);

        let free_output = tokio::process::Command::new("free")
            .args(&["-h"])
            .output()
            .await?;
        let memory_usage = String::from_utf8_lossy(&free_output.stdout);

        let status = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "system_info": system_info,
            "uptime": uptime.trim(),
            "disk_usage": disk_usage,
            "memory_usage": memory_usage,
            "distribution": context.linux_integration.distribution_info
        });

        Ok(serde_json::to_string_pretty(&status)?)
    }
}
