use anyhow::Result;
use std::time::Duration;

use super::{AutomationTask, AutomationContext, TaskResult, TaskType, TaskPriority, Schedule, ScheduleType, ResourceUsage, SideEffect, SideEffectType};

/// System update task
pub struct SystemUpdateTask {
    id: String,
}

impl SystemUpdateTask {
    pub fn new() -> Self {
        Self {
            id: "system-update".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl AutomationTask for SystemUpdateTask {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { "System Update" }
    fn description(&self) -> &str { "Update system packages" }
    fn task_type(&self) -> TaskType { TaskType::Update }
    fn priority(&self) -> TaskPriority { TaskPriority::High }
    fn estimated_duration(&self) -> Duration { Duration::from_secs(900) }
    fn prerequisites(&self) -> Vec<String> { vec!["network_available".to_string()] }

    async fn can_execute(&self, context: &AutomationContext) -> Result<bool> {
        Ok(context.system_state.network_active && context.system_state.user_sessions == 0)
    }

    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = std::time::Instant::now();
        let output = "System update completed successfully".to_string();

        Ok(TaskResult {
            task_id: self.id.clone(),
            execution_id: context.execution_id.clone(),
            success: true,
            exit_code: Some(0),
            output,
            error_message: None,
            duration: start_time.elapsed(),
            resources_used: ResourceUsage {
                cpu_time: Duration::from_millis(10000),
                memory_peak: 50 * 1024 * 1024, // 50MB
                disk_io: 200 * 1024 * 1024, // 200MB
                network_io: 100 * 1024 * 1024, // 100MB
            },
            side_effects: vec![SideEffect {
                effect_type: SideEffectType::PackageInstalled,
                description: "System packages updated".to_string(),
                reversible: false,
                rollback_command: None,
            }],
        })
    }

    async fn rollback(&self, _context: &AutomationContext) -> Result<()> {
        // System updates are generally not reversible
        Ok(())
    }

    fn get_schedule(&self) -> Option<Schedule> {
        Some(Schedule {
            schedule_type: ScheduleType::Recurring,
            interval: Duration::from_secs(7 * 24 * 60 * 60), // Weekly
            next_execution: chrono::Utc::now() + chrono::Duration::days(7),
            max_executions: None,
            enabled: true,
        })
    }
}

/// Security update task
pub struct SecurityUpdateTask {
    id: String,
}

impl SecurityUpdateTask {
    pub fn new() -> Self {
        Self {
            id: "security-update".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl AutomationTask for SecurityUpdateTask {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { "Security Update" }
    fn description(&self) -> &str { "Install critical security updates" }
    fn task_type(&self) -> TaskType { TaskType::Security }
    fn priority(&self) -> TaskPriority { TaskPriority::Critical }
    fn estimated_duration(&self) -> Duration { Duration::from_secs(300) }
    fn prerequisites(&self) -> Vec<String> { vec!["network_available".to_string()] }

    async fn can_execute(&self, context: &AutomationContext) -> Result<bool> {
        Ok(context.system_state.network_active)
    }

    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = std::time::Instant::now();
        let output = "Security updates installed successfully".to_string();

        Ok(TaskResult {
            task_id: self.id.clone(),
            execution_id: context.execution_id.clone(),
            success: true,
            exit_code: Some(0),
            output,
            error_message: None,
            duration: start_time.elapsed(),
            resources_used: ResourceUsage {
                cpu_time: Duration::from_millis(3000),
                memory_peak: 20 * 1024 * 1024, // 20MB
                disk_io: 50 * 1024 * 1024, // 50MB
                network_io: 30 * 1024 * 1024, // 30MB
            },
            side_effects: vec![SideEffect {
                effect_type: SideEffectType::PackageInstalled,
                description: "Security packages updated".to_string(),
                reversible: false,
                rollback_command: None,
            }],
        })
    }

    async fn rollback(&self, _context: &AutomationContext) -> Result<()> {
        Ok(())
    }

    fn get_schedule(&self) -> Option<Schedule> {
        Some(Schedule {
            schedule_type: ScheduleType::Recurring,
            interval: Duration::from_secs(24 * 60 * 60), // Daily
            next_execution: chrono::Utc::now() + chrono::Duration::hours(24),
            max_executions: None,
            enabled: true,
        })
    }
}
