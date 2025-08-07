use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod server;
pub mod api;
pub mod auth;
pub mod websocket;

use crate::config::Config;
use crate::linux_integration::LinuxIntegration;
use crate::execution::CommandExecutor;
use crate::agents::AgentSystem;

/// Web interface server for TuxPilot
#[derive(Clone)]
pub struct WebServer {
    config: Config,
    linux_integration: LinuxIntegration,
    command_executor: Arc<RwLock<CommandExecutor>>,
    agent_system: Arc<RwLock<AgentSystem>>,
    active_sessions: Arc<RwLock<HashMap<String, WebSession>>>,
    auth_manager: auth::AuthManager,
}

/// Web session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSession {
    pub session_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub permissions: Vec<Permission>,
    pub ip_address: String,
    pub user_agent: String,
}

/// Web interface permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    ViewSystem,
    ExecuteCommands,
    ManageServices,
    ViewLogs,
    ManageUsers,
    SystemConfiguration,
    RemoteAccess,
}

/// API request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub endpoint: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub session_id: Option<String>,
}

/// API response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
    pub execution_time_ms: u64,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    SystemStatus {
        cpu_usage: f32,
        memory_usage: f32,
        disk_usage: f32,
        network_active: bool,
    },
    CommandOutput {
        execution_id: String,
        output: String,
        is_complete: bool,
    },
    AgentUpdate {
        agent_id: String,
        status: String,
        message: String,
    },
    LogEntry {
        timestamp: chrono::DateTime<chrono::Utc>,
        level: String,
        source: String,
        message: String,
    },
    Error {
        error_code: String,
        message: String,
    },
}

/// Dashboard data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub system_overview: SystemOverview,
    pub recent_activities: Vec<ActivityEntry>,
    pub agent_status: Vec<AgentStatusEntry>,
    pub system_health: SystemHealthMetrics,
    pub alerts: Vec<AlertEntry>,
}

/// System overview for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOverview {
    pub hostname: String,
    pub distribution: String,
    pub kernel_version: String,
    pub uptime_hours: u32,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_interfaces: u32,
    pub running_services: u32,
}

/// Activity entry for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub activity_type: ActivityType,
    pub description: String,
    pub user: Option<String>,
    pub success: bool,
}

/// Activity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    CommandExecution,
    ServiceManagement,
    PackageOperation,
    SystemConfiguration,
    UserLogin,
    AgentAction,
}

/// Agent status for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatusEntry {
    pub agent_id: String,
    pub agent_name: String,
    pub status: AgentStatus,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub tasks_completed: u32,
    pub success_rate: f32,
}

/// Agent status types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Active,
    Idle,
    Busy,
    Error,
    Offline,
}

/// System health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthMetrics {
    pub overall_health: HealthStatus,
    pub cpu_health: HealthStatus,
    pub memory_health: HealthStatus,
    pub disk_health: HealthStatus,
    pub network_health: HealthStatus,
    pub service_health: HealthStatus,
}

/// Health status levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Excellent,
    Good,
    Warning,
    Critical,
    Unknown,
}

/// Alert entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEntry {
    pub id: String,
    pub severity: AlertSeverity,
    pub title: String,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub acknowledged: bool,
    pub source: String,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl WebServer {
    pub async fn new(
        config: Config,
        linux_integration: LinuxIntegration,
        command_executor: CommandExecutor,
        agent_system: AgentSystem,
    ) -> Result<Self> {
        let auth_manager = auth::AuthManager::new(&config).await?;

        Ok(Self {
            config,
            linux_integration,
            command_executor: Arc::new(RwLock::new(command_executor)),
            agent_system: Arc::new(RwLock::new(agent_system)),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            auth_manager,
        })
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        println!("🌐 Starting TuxPilot Web Interface on port {}", port);

        // Start HTTP server
        let server = server::HttpServer::new(self.clone()).await?;
        server.start(port).await?;

        Ok(())
    }

    pub async fn create_session(&self, user_id: String, ip_address: String, user_agent: String) -> Result<WebSession> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let permissions = self.auth_manager.get_user_permissions(&user_id).await?;

        let session = WebSession {
            session_id: session_id.clone(),
            user_id,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            permissions,
            ip_address,
            user_agent,
        };

        self.active_sessions.write().await.insert(session_id.clone(), session.clone());
        Ok(session)
    }

    pub async fn get_session(&self, session_id: &str) -> Option<WebSession> {
        self.active_sessions.read().await.get(session_id).cloned()
    }

    pub async fn update_session_activity(&self, session_id: &str) -> Result<()> {
        if let Some(session) = self.active_sessions.write().await.get_mut(session_id) {
            session.last_activity = chrono::Utc::now();
        }
        Ok(())
    }

    pub async fn get_dashboard_data(&self, _session_id: &str) -> Result<DashboardData> {
        // Get system overview
        let system_overview = SystemOverview {
            hostname: "tuxpilot-server".to_string(),
            distribution: self.linux_integration.distribution_info
                .as_ref()
                .map(|d| d.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: "6.1.0".to_string(),
            uptime_hours: 48,
            cpu_usage: 25.5,
            memory_usage: 68.2,
            disk_usage: 45.8,
            network_interfaces: 2,
            running_services: 45,
        };

        // Get recent activities (mock data)
        let recent_activities = vec![
            ActivityEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(5),
                activity_type: ActivityType::CommandExecution,
                description: "System status check completed".to_string(),
                user: Some("admin".to_string()),
                success: true,
            },
            ActivityEntry {
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(15),
                activity_type: ActivityType::AgentAction,
                description: "Security scan completed".to_string(),
                user: None,
                success: true,
            },
        ];

        // Get agent status
        let agent_status = vec![
            AgentStatusEntry {
                agent_id: "system-agent".to_string(),
                agent_name: "System Management Agent".to_string(),
                status: AgentStatus::Active,
                last_activity: chrono::Utc::now() - chrono::Duration::minutes(2),
                tasks_completed: 156,
                success_rate: 0.98,
            },
            AgentStatusEntry {
                agent_id: "security-agent".to_string(),
                agent_name: "Security Analysis Agent".to_string(),
                status: AgentStatus::Idle,
                last_activity: chrono::Utc::now() - chrono::Duration::minutes(30),
                tasks_completed: 42,
                success_rate: 0.95,
            },
        ];

        // System health metrics
        let system_health = SystemHealthMetrics {
            overall_health: HealthStatus::Good,
            cpu_health: HealthStatus::Good,
            memory_health: HealthStatus::Warning,
            disk_health: HealthStatus::Good,
            network_health: HealthStatus::Excellent,
            service_health: HealthStatus::Good,
        };

        // Alerts
        let alerts = vec![
            AlertEntry {
                id: uuid::Uuid::new_v4().to_string(),
                severity: AlertSeverity::Warning,
                title: "High Memory Usage".to_string(),
                description: "Memory usage is above 65%".to_string(),
                timestamp: chrono::Utc::now() - chrono::Duration::minutes(10),
                acknowledged: false,
                source: "system-monitor".to_string(),
            },
        ];

        Ok(DashboardData {
            system_overview,
            recent_activities,
            agent_status,
            system_health,
            alerts,
        })
    }

    pub async fn execute_command_via_web(&self, session_id: &str, command: &str) -> Result<String> {
        // Verify session and permissions
        let session = self.get_session(session_id).await
            .ok_or_else(|| anyhow::anyhow!("Invalid session"))?;

        if !session.permissions.contains(&Permission::ExecuteCommands) {
            return Err(anyhow::anyhow!("Insufficient permissions"));
        }

        // Execute command through command executor
        let execution_id = uuid::Uuid::new_v4().to_string();
        
        // This would integrate with the actual command execution system
        Ok(format!("Command '{}' queued for execution with ID: {}", command, execution_id))
    }

    pub async fn get_system_logs(&self, session_id: &str, lines: usize) -> Result<Vec<String>> {
        // Verify session and permissions
        let session = self.get_session(session_id).await
            .ok_or_else(|| anyhow::anyhow!("Invalid session"))?;

        if !session.permissions.contains(&Permission::ViewLogs) {
            return Err(anyhow::anyhow!("Insufficient permissions"));
        }

        // Get system logs
        let output = tokio::process::Command::new("journalctl")
            .args(&["-n", &lines.to_string(), "--no-pager"])
            .output()
            .await?;

        let logs = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(logs)
    }
}
