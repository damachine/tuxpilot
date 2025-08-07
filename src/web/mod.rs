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
    chat_sessions: Arc<RwLock<HashMap<String, ChatSession>>>,
    chat_history: Arc<RwLock<HashMap<String, Vec<ChatMessage>>>>,
    config_backups: Arc<RwLock<HashMap<String, ConfigBackup>>>,
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
    ChatAccess,
    ConfigurationManagement,
}

/// Chat message sender types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatSender {
    User,
    Agent { agent_id: String, agent_name: String },
    System,
}

/// Chat message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatMessageType {
    Text,
    Command,
    SystemInfo,
    Error,
    Suggestion,
    CodeBlock,
}

/// Chat session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub chat_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub execution_mode: String,
    pub active_agent: Option<String>,
    pub message_count: usize,
}

/// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: String,
    pub chat_id: String,
    pub sender: ChatSender,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message_type: ChatMessageType,
    pub metadata: Option<serde_json::Value>,
}

/// Chat request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub chat_id: Option<String>,
    pub message: String,
    pub execution_mode: Option<String>,
    pub context: Option<serde_json::Value>,
}

/// Configuration management structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSchema {
    pub sections: HashMap<String, ConfigSection>,
    pub version: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSection {
    pub name: String,
    pub description: String,
    pub fields: HashMap<String, ConfigField>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    pub field_type: ConfigFieldType,
    pub description: String,
    pub default_value: Option<serde_json::Value>,
    pub validation: Option<ConfigValidation>,
    pub sensitive: bool,
    pub requires_restart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigFieldType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
    Enum { options: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub pattern: Option<String>,
    pub custom_validator: Option<String>,
}

/// Configuration update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdateRequest {
    pub section: String,
    pub updates: HashMap<String, serde_json::Value>,
    pub validate_only: bool,
}

/// Configuration backup structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBackup {
    pub backup_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub config_data: serde_json::Value,
    pub version: String,
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
    ChatMessage {
        chat_id: String,
        message_id: String,
        sender: ChatSender,
        content: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        message_type: ChatMessageType,
    },
    ChatTyping {
        chat_id: String,
        sender: ChatSender,
        is_typing: bool,
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
            chat_sessions: Arc::new(RwLock::new(HashMap::new())),
            chat_history: Arc::new(RwLock::new(HashMap::new())),
            config_backups: Arc::new(RwLock::new(HashMap::new())),
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

    // Chat functionality
    pub async fn create_chat_session(&self, user_id: String, execution_mode: String) -> Result<ChatSession> {
        let chat_id = uuid::Uuid::new_v4().to_string();
        let session = ChatSession {
            chat_id: chat_id.clone(),
            user_id,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            execution_mode,
            active_agent: None,
            message_count: 0,
        };

        self.chat_sessions.write().await.insert(chat_id.clone(), session.clone());
        self.chat_history.write().await.insert(chat_id.clone(), Vec::new());

        Ok(session)
    }

    pub async fn send_chat_message(&self, chat_request: ChatRequest, user_id: String) -> Result<ChatMessage> {
        let chat_id = chat_request.chat_id.clone().unwrap_or_else(|| {
            // Create new chat session if none provided
            uuid::Uuid::new_v4().to_string()
        });

        // Create user message
        let user_message = ChatMessage {
            message_id: uuid::Uuid::new_v4().to_string(),
            chat_id: chat_id.clone(),
            sender: ChatSender::User,
            content: chat_request.message.clone(),
            timestamp: chrono::Utc::now(),
            message_type: ChatMessageType::Text,
            metadata: chat_request.context.clone(),
        };

        // Add to history
        self.chat_history.write().await
            .entry(chat_id.clone())
            .or_insert_with(Vec::new)
            .push(user_message.clone());

        // Update session activity
        if let Some(session) = self.chat_sessions.write().await.get_mut(&chat_id) {
            session.last_activity = chrono::Utc::now();
            session.message_count += 1;
        }

        // Process message with AI agents
        let ai_response = self.process_chat_message(&chat_request, &user_id).await?;

        // Add AI response to history
        self.chat_history.write().await
            .get_mut(&chat_id)
            .unwrap()
            .push(ai_response.clone());

        Ok(ai_response)
    }

    async fn process_chat_message(&self, request: &ChatRequest, user_id: &str) -> Result<ChatMessage> {
        use crate::agents::AgentContext;

        // Create agent context
        let context = AgentContext {
            session_id: user_id.to_string(),
            user_request: request.message.clone(),
            system_state: crate::agents::SystemState {
                cpu_usage: 25.0,
                memory_usage: 60.0,
                disk_usage: 40.0,
                network_active: true,
                services_running: vec!["nginx".to_string(), "ssh".to_string()],
                recent_errors: vec![],
            },
            config: self.config.clone(),
            linux_integration: self.linux_integration.clone(),
        };

        // Get AI response from agent system
        let mut agent_system = self.agent_system.write().await;
        let responses = agent_system.execute_user_request(&request.message, &context).await?;

        let response_content = if responses.is_empty() {
            "I understand your request. How can I help you with your Linux system?".to_string()
        } else {
            responses.into_iter()
                .map(|r| {
                    let mut content = Vec::new();
                    if !r.recommendations.is_empty() {
                        content.push(format!("Recommendations: {}", r.recommendations.join(", ")));
                    }
                    if !r.actions_taken.is_empty() {
                        content.push(format!("Actions taken: {}", r.actions_taken.join(", ")));
                    }
                    if !r.next_steps.is_empty() {
                        content.push(format!("Next steps: {}", r.next_steps.join(", ")));
                    }
                    if content.is_empty() {
                        format!("Agent {} completed task successfully", r.agent_id)
                    } else {
                        content.join("\n")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        };

        Ok(ChatMessage {
            message_id: uuid::Uuid::new_v4().to_string(),
            chat_id: request.chat_id.clone().unwrap_or_default(),
            sender: ChatSender::Agent {
                agent_id: "tuxpilot-ai".to_string(),
                agent_name: "TuxPilot AI".to_string(),
            },
            content: response_content,
            timestamp: chrono::Utc::now(),
            message_type: ChatMessageType::Text,
            metadata: None,
        })
    }

    pub async fn get_chat_history(&self, chat_id: &str, limit: Option<usize>) -> Result<Vec<ChatMessage>> {
        let history = self.chat_history.read().await;
        if let Some(messages) = history.get(chat_id) {
            let messages = if let Some(limit) = limit {
                messages.iter().rev().take(limit).rev().cloned().collect()
            } else {
                messages.clone()
            };
            Ok(messages)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_chat_sessions(&self, user_id: &str) -> Result<Vec<ChatSession>> {
        let sessions = self.chat_sessions.read().await;
        Ok(sessions.values()
            .filter(|s| s.user_id == user_id)
            .cloned()
            .collect())
    }
}
