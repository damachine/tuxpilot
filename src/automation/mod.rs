use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub mod scheduler;
pub mod tasks;
pub mod backup;
pub mod updates;
pub mod maintenance;

use crate::config::Config;
use crate::linux_integration::LinuxIntegration;

/// Automation orchestrator for system maintenance
pub struct AutomationOrchestrator {
    config: Config,
    linux_integration: LinuxIntegration,
    scheduler: scheduler::TaskScheduler,
    task_registry: HashMap<String, Box<dyn AutomationTask>>,
    execution_history: Vec<TaskExecution>,
}

/// Automation task trait
#[async_trait::async_trait]
pub trait AutomationTask: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn task_type(&self) -> TaskType;
    fn priority(&self) -> TaskPriority;
    fn estimated_duration(&self) -> std::time::Duration;
    fn prerequisites(&self) -> Vec<String>;
    async fn can_execute(&self, context: &AutomationContext) -> Result<bool>;
    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult>;
    async fn rollback(&self, context: &AutomationContext) -> Result<()>;
    fn get_schedule(&self) -> Option<Schedule>;
}

/// Task types for automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Maintenance,
    Backup,
    Update,
    Security,
    Monitoring,
    Cleanup,
    Optimization,
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

/// Task execution schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub schedule_type: ScheduleType,
    pub interval: std::time::Duration,
    pub next_execution: DateTime<Utc>,
    pub max_executions: Option<u32>,
    pub enabled: bool,
}

/// Schedule types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleType {
    Once,
    Recurring,
    Cron(String),
    OnEvent(String),
}

/// Automation context
#[derive(Debug, Clone)]
pub struct AutomationContext {
    pub execution_id: String,
    pub user_id: Option<String>,
    pub system_state: SystemState,
    pub config: Config,
    pub linux_integration: LinuxIntegration,
    pub dry_run: bool,
}

/// System state for automation decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_active: bool,
    pub user_sessions: u32,
    pub system_load: f32,
    pub uptime_hours: u32,
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub execution_id: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub output: String,
    pub error_message: Option<String>,
    pub duration: std::time::Duration,
    pub resources_used: ResourceUsage,
    pub side_effects: Vec<SideEffect>,
}

/// Resource usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time: std::time::Duration,
    pub memory_peak: u64,
    pub disk_io: u64,
    pub network_io: u64,
}

/// Side effects of task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect_type: SideEffectType,
    pub description: String,
    pub reversible: bool,
    pub rollback_command: Option<String>,
}

/// Types of side effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SideEffectType {
    FileModified,
    ServiceRestarted,
    PackageInstalled,
    ConfigChanged,
    UserCreated,
    PermissionChanged,
}

/// Task execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecution {
    pub execution_id: String,
    pub task_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<TaskResult>,
    pub triggered_by: TriggerSource,
}

/// What triggered the task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerSource {
    Schedule,
    Manual,
    Event(String),
    Dependency(String),
}

impl AutomationOrchestrator {
    pub async fn new(config: Config, linux_integration: LinuxIntegration) -> Result<Self> {
        let scheduler = scheduler::TaskScheduler::new().await?;
        
        let mut orchestrator = Self {
            config: config.clone(),
            linux_integration: linux_integration.clone(),
            scheduler,
            task_registry: HashMap::new(),
            execution_history: Vec::new(),
        };

        // Register built-in automation tasks
        orchestrator.register_builtin_tasks().await?;

        Ok(orchestrator)
    }

    async fn register_builtin_tasks(&mut self) -> Result<()> {
        // System maintenance tasks
        self.register_task(Box::new(maintenance::SystemCleanupTask::new())).await?;
        self.register_task(Box::new(maintenance::LogRotationTask::new())).await?;
        self.register_task(Box::new(maintenance::TempCleanupTask::new())).await?;

        // Backup tasks
        self.register_task(Box::new(backup::ConfigBackupTask::new())).await?;
        self.register_task(Box::new(backup::UserDataBackupTask::new())).await?;

        // Update tasks
        self.register_task(Box::new(updates::SystemUpdateTask::new())).await?;
        self.register_task(Box::new(updates::SecurityUpdateTask::new())).await?;

        Ok(())
    }

    pub async fn register_task(&mut self, task: Box<dyn AutomationTask>) -> Result<()> {
        let task_id = task.id().to_string();
        
        // Register with scheduler if it has a schedule
        if let Some(schedule) = task.get_schedule() {
            self.scheduler.schedule_task(task_id.clone(), schedule).await?;
        }

        self.task_registry.insert(task_id.clone(), task);
        println!("🤖 Registered automation task: {}", task_id);
        Ok(())
    }

    pub async fn execute_task(&mut self, task_id: &str, trigger: TriggerSource) -> Result<TaskResult> {
        let execution_id = uuid::Uuid::new_v4().to_string();
        
        let execution = TaskExecution {
            execution_id: execution_id.clone(),
            task_id: task_id.to_string(),
            started_at: Utc::now(),
            completed_at: None,
            result: None,
            triggered_by: trigger,
        };

        self.execution_history.push(execution);

        if let Some(task) = self.task_registry.get(task_id) {
            let context = self.create_context(execution_id.clone()).await?;
            
            // Check prerequisites
            if !task.can_execute(&context).await? {
                return Err(anyhow::anyhow!("Task prerequisites not met"));
            }

            println!("🔄 Executing automation task: {}", task.name());
            
            let result = task.execute(&context).await?;
            
            // Update execution history
            if let Some(execution) = self.execution_history.iter_mut()
                .find(|e| e.execution_id == execution_id) {
                execution.completed_at = Some(Utc::now());
                execution.result = Some(result.clone());
            }

            Ok(result)
        } else {
            Err(anyhow::anyhow!("Task not found: {}", task_id))
        }
    }

    async fn create_context(&self, execution_id: String) -> Result<AutomationContext> {
        let system_state = self.get_system_state().await?;
        
        Ok(AutomationContext {
            execution_id,
            user_id: None,
            system_state,
            config: self.config.clone(),
            linux_integration: self.linux_integration.clone(),
            dry_run: false,
        })
    }

    async fn get_system_state(&self) -> Result<SystemState> {
        // Get current system metrics
        Ok(SystemState {
            cpu_usage: 25.0,  // Would be real metrics
            memory_usage: 60.0,
            disk_usage: 45.0,
            network_active: true,
            user_sessions: 1,
            system_load: 1.2,
            uptime_hours: 48,
        })
    }

    pub async fn run_scheduled_tasks(&mut self) -> Result<Vec<TaskResult>> {
        let due_tasks = self.scheduler.get_due_tasks().await?;
        let mut results = Vec::new();

        for task_id in due_tasks {
            match self.execute_task(&task_id, TriggerSource::Schedule).await {
                Ok(result) => {
                    results.push(result);
                    println!("✅ Scheduled task completed: {}", task_id);
                }
                Err(e) => {
                    eprintln!("❌ Scheduled task failed: {}: {}", task_id, e);
                }
            }
        }

        Ok(results)
    }

    pub async fn get_task_status(&self, task_id: &str) -> Result<TaskStatus> {
        if let Some(task) = self.task_registry.get(task_id) {
            let recent_executions: Vec<&TaskExecution> = self.execution_history
                .iter()
                .filter(|e| e.task_id == task_id)
                .collect();

            let last_execution = recent_executions.last().cloned();
            let success_rate = if recent_executions.is_empty() {
                0.0
            } else {
                let successful = recent_executions.iter()
                    .filter(|e| e.result.as_ref().map_or(false, |r| r.success))
                    .count();
                successful as f32 / recent_executions.len() as f32
            };

            Ok(TaskStatus {
                task_id: task_id.to_string(),
                task_name: task.name().to_string(),
                task_type: task.task_type(),
                enabled: true,
                last_execution: last_execution.cloned(),
                next_execution: task.get_schedule().map(|s| s.next_execution),
                success_rate,
                total_executions: recent_executions.len(),
            })
        } else {
            Err(anyhow::anyhow!("Task not found: {}", task_id))
        }
    }

    pub async fn get_system_status(&self) -> Result<AutomationSystemStatus> {
        let total_tasks = self.task_registry.len();
        let active_tasks = self.task_registry.values()
            .filter(|task| task.get_schedule().map_or(false, |s| s.enabled))
            .count();

        let recent_executions = self.execution_history.iter()
            .filter(|e| e.started_at > Utc::now() - chrono::Duration::hours(24))
            .count();

        Ok(AutomationSystemStatus {
            total_tasks,
            active_tasks,
            recent_executions,
            system_health: SystemHealth::Good,
            last_maintenance: Utc::now() - chrono::Duration::hours(6),
        })
    }
}

/// Task status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub task_id: String,
    pub task_name: String,
    pub task_type: TaskType,
    pub enabled: bool,
    pub last_execution: Option<TaskExecution>,
    pub next_execution: Option<DateTime<Utc>>,
    pub success_rate: f32,
    pub total_executions: usize,
}

/// Overall automation system status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationSystemStatus {
    pub total_tasks: usize,
    pub active_tasks: usize,
    pub recent_executions: usize,
    pub system_health: SystemHealth,
    pub last_maintenance: DateTime<Utc>,
}

/// System health indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemHealth {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}
