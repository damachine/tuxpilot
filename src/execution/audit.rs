use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::config::Config;
use super::{ExecutionRequest, ExecutionResult};

#[derive(Debug, Clone)]
pub struct AuditLogger {
    log_file: PathBuf,
    config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub entry_type: AuditEntryType,
    pub user: String,
    pub session_id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEntryType {
    ExecutionRequest,
    ExecutionResult,
    PermissionRequest,
    PermissionGranted,
    PermissionDenied,
    SafetyViolation,
    SystemChange,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub request: ExecutionRequest,
    pub result: ExecutionResult,
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub session_id: String,
}

impl AuditLogger {
    pub async fn new(config: &Config) -> Result<Self> {
        let log_dir = dirs::data_dir()
            .context("Failed to get data directory")?
            .join("tuxpilot")
            .join("audit");
        
        tokio::fs::create_dir_all(&log_dir).await
            .context("Failed to create audit log directory")?;
        
        let log_file = log_dir.join("audit.jsonl");
        
        Ok(Self {
            log_file,
            config: config.clone(),
        })
    }

    pub async fn log_request(&self, request: &ExecutionRequest) -> Result<()> {
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            entry_type: AuditEntryType::ExecutionRequest,
            user: self.get_current_user(),
            session_id: self.get_session_id(),
            data: serde_json::to_value(request)?,
        };

        self.write_audit_entry(&entry).await
    }

    pub async fn log_result(&self, result: &ExecutionResult) -> Result<()> {
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            entry_type: AuditEntryType::ExecutionResult,
            user: self.get_current_user(),
            session_id: self.get_session_id(),
            data: serde_json::to_value(result)?,
        };

        self.write_audit_entry(&entry).await
    }

    pub async fn log_permission_request(&self, permission: &str, granted: bool) -> Result<()> {
        let entry_type = if granted {
            AuditEntryType::PermissionGranted
        } else {
            AuditEntryType::PermissionDenied
        };

        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            entry_type,
            user: self.get_current_user(),
            session_id: self.get_session_id(),
            data: serde_json::json!({
                "permission": permission,
                "granted": granted
            }),
        };

        self.write_audit_entry(&entry).await
    }

    pub async fn log_safety_violation(&self, command: &str, reason: &str) -> Result<()> {
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            entry_type: AuditEntryType::SafetyViolation,
            user: self.get_current_user(),
            session_id: self.get_session_id(),
            data: serde_json::json!({
                "command": command,
                "reason": reason,
                "severity": "high"
            }),
        };

        self.write_audit_entry(&entry).await
    }

    pub async fn log_system_change(&self, change_type: &str, description: &str, reversible: bool) -> Result<()> {
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            entry_type: AuditEntryType::SystemChange,
            user: self.get_current_user(),
            session_id: self.get_session_id(),
            data: serde_json::json!({
                "change_type": change_type,
                "description": description,
                "reversible": reversible,
                "timestamp": Utc::now()
            }),
        };

        self.write_audit_entry(&entry).await
    }

    pub async fn log_error(&self, error: &str, context: Option<&str>) -> Result<()> {
        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            entry_type: AuditEntryType::Error,
            user: self.get_current_user(),
            session_id: self.get_session_id(),
            data: serde_json::json!({
                "error": error,
                "context": context,
                "severity": "error"
            }),
        };

        self.write_audit_entry(&entry).await
    }

    async fn write_audit_entry(&self, entry: &AuditEntry) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
            .await
            .context("Failed to open audit log file")?;

        let json_line = serde_json::to_string(entry)?;
        file.write_all(format!("{}\n", json_line).as_bytes()).await
            .context("Failed to write audit entry")?;

        file.flush().await
            .context("Failed to flush audit log")?;

        Ok(())
    }

    pub async fn get_execution(&self, execution_id: Uuid) -> Result<ExecutionRecord> {
        let content = tokio::fs::read_to_string(&self.log_file).await
            .context("Failed to read audit log")?;

        let mut request: Option<ExecutionRequest> = None;
        let mut result: Option<ExecutionResult> = None;

        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
                match entry.entry_type {
                    AuditEntryType::ExecutionRequest => {
                        if let Ok(req) = serde_json::from_value::<ExecutionRequest>(entry.data) {
                            if req.id == execution_id {
                                request = Some(req);
                            }
                        }
                    }
                    AuditEntryType::ExecutionResult => {
                        if let Ok(res) = serde_json::from_value::<ExecutionResult>(entry.data) {
                            if res.id == execution_id {
                                result = Some(res);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        match (request, result) {
            (Some(req), Some(res)) => Ok(ExecutionRecord {
                request: req,
                result: res,
                timestamp: Utc::now(),
                user: self.get_current_user(),
                session_id: self.get_session_id(),
            }),
            _ => Err(anyhow::anyhow!("Execution record not found: {}", execution_id)),
        }
    }

    pub async fn get_recent_executions(&self, limit: usize) -> Result<Vec<ExecutionRecord>> {
        let content = tokio::fs::read_to_string(&self.log_file).await
            .context("Failed to read audit log")?;

        let mut executions = std::collections::HashMap::new();
        let mut requests = std::collections::HashMap::new();
        let mut results = std::collections::HashMap::new();

        // Parse all entries
        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
                match entry.entry_type {
                    AuditEntryType::ExecutionRequest => {
                        if let Ok(req) = serde_json::from_value::<ExecutionRequest>(entry.data) {
                            requests.insert(req.id, (req, entry.timestamp, entry.user, entry.session_id));
                        }
                    }
                    AuditEntryType::ExecutionResult => {
                        if let Ok(res) = serde_json::from_value::<ExecutionResult>(entry.data) {
                            results.insert(res.id, res);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Combine requests and results
        for (id, (request, timestamp, user, session_id)) in requests {
            if let Some(result) = results.get(&id) {
                executions.insert(id, ExecutionRecord {
                    request,
                    result: result.clone(),
                    timestamp,
                    user,
                    session_id,
                });
            }
        }

        // Sort by timestamp and take the most recent
        let mut execution_list: Vec<_> = executions.into_values().collect();
        execution_list.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        execution_list.truncate(limit);

        Ok(execution_list)
    }

    pub async fn get_audit_summary(&self) -> Result<AuditSummary> {
        let content = tokio::fs::read_to_string(&self.log_file).await
            .context("Failed to read audit log")?;

        let mut summary = AuditSummary::default();

        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
                match entry.entry_type {
                    AuditEntryType::ExecutionRequest => summary.total_executions += 1,
                    AuditEntryType::PermissionDenied => summary.permission_denials += 1,
                    AuditEntryType::SafetyViolation => summary.safety_violations += 1,
                    AuditEntryType::Error => summary.errors += 1,
                    _ => {}
                }

                // Track recent activity (last 24 hours)
                if entry.timestamp > Utc::now() - chrono::Duration::hours(24) {
                    summary.recent_activity += 1;
                }
            }
        }

        Ok(summary)
    }

    pub async fn export_audit_log(&self, format: ExportFormat) -> Result<String> {
        let content = tokio::fs::read_to_string(&self.log_file).await
            .context("Failed to read audit log")?;

        match format {
            ExportFormat::Json => Ok(content),
            ExportFormat::Csv => self.convert_to_csv(&content),
            ExportFormat::Html => self.convert_to_html(&content),
        }
    }

    fn convert_to_csv(&self, content: &str) -> Result<String> {
        let mut csv = String::from("timestamp,type,user,session_id,data\n");
        
        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
                csv.push_str(&format!(
                    "{},{:?},{},{},{}\n",
                    entry.timestamp,
                    entry.entry_type,
                    entry.user,
                    entry.session_id,
                    entry.data.to_string().replace(',', ";")
                ));
            }
        }
        
        Ok(csv)
    }

    fn convert_to_html(&self, content: &str) -> Result<String> {
        let mut html = String::from(r#"
<!DOCTYPE html>
<html>
<head>
    <title>TuxPilot Audit Log</title>
    <style>
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        .error { background-color: #ffebee; }
        .violation { background-color: #fff3e0; }
    </style>
</head>
<body>
    <h1>TuxPilot Audit Log</h1>
    <table>
        <tr>
            <th>Timestamp</th>
            <th>Type</th>
            <th>User</th>
            <th>Session</th>
            <th>Data</th>
        </tr>
"#);

        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
                let class = match entry.entry_type {
                    AuditEntryType::Error => "error",
                    AuditEntryType::SafetyViolation => "violation",
                    _ => "",
                };

                html.push_str(&format!(
                    r#"        <tr class="{}">
            <td>{}</td>
            <td>{:?}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
        </tr>
"#,
                    class,
                    entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    entry.entry_type,
                    entry.user,
                    entry.session_id,
                    html_escape::encode_text(&entry.data.to_string())
                ));
            }
        }

        html.push_str(r#"    </table>
</body>
</html>"#);

        Ok(html)
    }

    fn get_current_user(&self) -> String {
        std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    }

    fn get_session_id(&self) -> String {
        // Simple session ID based on process ID and start time
        format!("{}_{}", std::process::id(), Utc::now().timestamp())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_executions: u64,
    pub permission_denials: u64,
    pub safety_violations: u64,
    pub errors: u64,
    pub recent_activity: u64,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
    Html,
}

impl std::fmt::Display for AuditSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📊 TuxPilot Audit Summary")?;
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        writeln!(f, "🔧 Total Executions: {}", self.total_executions)?;
        writeln!(f, "🚫 Permission Denials: {}", self.permission_denials)?;
        writeln!(f, "⚠️  Safety Violations: {}", self.safety_violations)?;
        writeln!(f, "❌ Errors: {}", self.errors)?;
        writeln!(f, "📈 Recent Activity (24h): {}", self.recent_activity)?;
        Ok(())
    }
}
