use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod system_agent;
pub mod security_agent;
pub mod package_agent;
pub mod network_agent;
pub mod performance_agent;
pub mod orchestrator;

use crate::config::Config;
use crate::linux_integration::LinuxIntegration;
use crate::ai::AiClient;

/// Agent capability types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentCapability {
    SystemManagement,
    SecurityAnalysis,
    PackageManagement,
    NetworkDiagnostics,
    PerformanceOptimization,
    LogAnalysis,
    ServiceManagement,
    FileSystemOperations,
    UserManagement,
    ProcessManagement,
}

/// Agent specialization levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecializationLevel {
    Beginner,
    Intermediate,
    Expert,
    Master,
}

/// Agent execution context
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub session_id: String,
    pub user_request: String,
    pub system_state: SystemState,
    pub config: Config,
    pub linux_integration: LinuxIntegration,
}

/// Current system state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_active: bool,
    pub services_running: Vec<String>,
    pub recent_errors: Vec<String>,
}

/// Agent task result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub agent_id: String,
    pub task_id: String,
    pub success: bool,
    pub confidence: f32,
    pub recommendations: Vec<String>,
    pub actions_taken: Vec<String>,
    pub warnings: Vec<String>,
    pub next_steps: Vec<String>,
    pub execution_time_ms: u64,
}

/// Agent task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: u64,
    pub retry_count: u32,
}

/// Task types for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Analyze,
    Diagnose,
    Optimize,
    Monitor,
    Execute,
    Report,
    Recommend,
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

/// Base trait for all specialized agents
#[async_trait::async_trait]
pub trait Agent: Send + Sync {
    /// Get agent identifier
    fn id(&self) -> &str;
    
    /// Get agent name
    fn name(&self) -> &str;
    
    /// Get agent description
    fn description(&self) -> &str;
    
    /// Get agent capabilities
    fn capabilities(&self) -> &[AgentCapability];
    
    /// Get specialization level
    fn specialization_level(&self) -> SpecializationLevel;
    
    /// Check if agent can handle a specific task
    fn can_handle_task(&self, task: &AgentTask) -> bool;
    
    /// Execute a task
    async fn execute_task(&self, task: &AgentTask, context: &AgentContext) -> Result<AgentResult>;
    
    /// Get agent status and health
    async fn get_status(&self) -> Result<AgentStatus>;
    
    /// Initialize agent with configuration
    async fn initialize(&mut self, config: &Config) -> Result<()>;
    
    /// Shutdown agent gracefully
    async fn shutdown(&mut self) -> Result<()>;
}

/// Agent status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub agent_id: String,
    pub is_healthy: bool,
    pub is_busy: bool,
    pub current_task: Option<String>,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub average_execution_time_ms: f64,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Multi-agent system coordinator
pub struct AgentSystem {
    agents: HashMap<String, Box<dyn Agent>>,
    orchestrator: orchestrator::AgentOrchestrator,
    config: Config,
    active_tasks: HashMap<String, AgentTask>,
    task_history: Vec<AgentResult>,
}

impl AgentSystem {
    pub async fn new(config: Config, linux_integration: LinuxIntegration, ai_client: AiClient) -> Result<Self> {
        let mut system = Self {
            agents: HashMap::new(),
            orchestrator: orchestrator::AgentOrchestrator::new(config.clone(), linux_integration, ai_client).await?,
            config: config.clone(),
            active_tasks: HashMap::new(),
            task_history: Vec::new(),
        };

        // Initialize specialized agents
        system.register_default_agents().await?;

        Ok(system)
    }

    async fn register_default_agents(&mut self) -> Result<()> {
        // System management agent
        let system_agent = Box::new(system_agent::SystemAgent::new(&self.config).await?);
        self.register_agent(system_agent).await?;

        // Security analysis agent
        let security_agent = Box::new(security_agent::SecurityAgent::new(&self.config).await?);
        self.register_agent(security_agent).await?;

        // Package management agent
        let package_agent = Box::new(package_agent::PackageAgent::new(&self.config).await?);
        self.register_agent(package_agent).await?;

        // Network diagnostics agent
        let network_agent = Box::new(network_agent::NetworkAgent::new(&self.config).await?);
        self.register_agent(network_agent).await?;

        // Performance optimization agent
        let performance_agent = Box::new(performance_agent::PerformanceAgent::new(&self.config).await?);
        self.register_agent(performance_agent).await?;

        Ok(())
    }

    pub async fn register_agent(&mut self, agent: Box<dyn Agent>) -> Result<()> {
        let agent_id = agent.id().to_string();
        self.agents.insert(agent_id.clone(), agent);
        println!("🤖 Registered agent: {}", agent_id);
        Ok(())
    }

    pub async fn execute_user_request(&mut self, request: &str, context: &AgentContext) -> Result<Vec<AgentResult>> {
        // Use orchestrator to analyze request and delegate to appropriate agents
        let task_plan = self.orchestrator.analyze_request(request, context).await?;
        
        let mut results = Vec::new();
        
        for task in task_plan.tasks {
            if let Some(result) = self.execute_task(&task, context).await? {
                results.push(result);
            }
        }

        // Store results in history
        self.task_history.extend(results.clone());

        Ok(results)
    }

    async fn execute_task(&mut self, task: &AgentTask, context: &AgentContext) -> Result<Option<AgentResult>> {
        // Find the best agent for this task
        let best_agent_id = self.find_best_agent_for_task(task)?;
        
        if let Some(agent) = self.agents.get(&best_agent_id) {
            println!("🔄 Executing task '{}' with agent '{}'", task.description, best_agent_id);
            
            // Track active task
            self.active_tasks.insert(task.id.clone(), task.clone());
            
            // Execute task
            let result = agent.execute_task(task, context).await?;
            
            // Remove from active tasks
            self.active_tasks.remove(&task.id);
            
            Ok(Some(result))
        } else {
            println!("⚠️ No suitable agent found for task: {}", task.description);
            Ok(None)
        }
    }

    fn find_best_agent_for_task(&self, task: &AgentTask) -> Result<String> {
        let mut best_agent_id = None;
        let mut best_score = 0.0;

        for (agent_id, agent) in &self.agents {
            if agent.can_handle_task(task) {
                let score = self.calculate_agent_score(agent.as_ref(), task);
                if score > best_score {
                    best_score = score;
                    best_agent_id = Some(agent_id.clone());
                }
            }
        }

        best_agent_id.ok_or_else(|| anyhow::anyhow!("No suitable agent found for task"))
    }

    fn calculate_agent_score(&self, agent: &dyn Agent, task: &AgentTask) -> f64 {
        let mut score = 0.0;

        // Base score for capability match
        if agent.can_handle_task(task) {
            score += 50.0;
        }

        // Bonus for specialization level
        match agent.specialization_level() {
            SpecializationLevel::Master => score += 40.0,
            SpecializationLevel::Expert => score += 30.0,
            SpecializationLevel::Intermediate => score += 20.0,
            SpecializationLevel::Beginner => score += 10.0,
        }

        // Bonus for task priority match
        match task.priority {
            TaskPriority::Emergency => score += 20.0,
            TaskPriority::Critical => score += 15.0,
            TaskPriority::High => score += 10.0,
            TaskPriority::Normal => score += 5.0,
            TaskPriority::Low => score += 1.0,
        }

        score
    }

    pub async fn get_system_status(&self) -> Result<SystemStatus> {
        let mut agent_statuses = HashMap::new();
        
        for (agent_id, agent) in &self.agents {
            let status = agent.get_status().await?;
            agent_statuses.insert(agent_id.clone(), status);
        }

        Ok(SystemStatus {
            total_agents: self.agents.len(),
            active_tasks: self.active_tasks.len(),
            completed_tasks: self.task_history.len(),
            agent_statuses,
            orchestrator_status: self.orchestrator.get_status().await?,
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        println!("🔄 Shutting down agent system...");
        
        for (agent_id, agent) in &mut self.agents {
            println!("🔄 Shutting down agent: {}", agent_id);
            if let Err(e) = agent.shutdown().await {
                eprintln!("⚠️ Error shutting down agent {}: {}", agent_id, e);
            }
        }

        self.orchestrator.shutdown().await?;
        
        println!("✅ Agent system shutdown complete");
        Ok(())
    }
}

/// Overall system status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub total_agents: usize,
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub agent_statuses: HashMap<String, AgentStatus>,
    pub orchestrator_status: orchestrator::OrchestratorStatus,
}
