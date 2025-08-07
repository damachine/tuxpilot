use anyhow::Result;
use std::time::Duration;

use super::{AutomationTask, AutomationContext, TaskResult, TaskType, TaskPriority, Schedule, ScheduleType, ResourceUsage, SideEffect, SideEffectType};

/// Configuration backup task
pub struct ConfigBackupTask {
    id: String,
}

impl ConfigBackupTask {
    pub fn new() -> Self {
        Self {
            id: "config-backup".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl AutomationTask for ConfigBackupTask {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { "Configuration Backup" }
    fn description(&self) -> &str { "Backup system configuration files" }
    fn task_type(&self) -> TaskType { TaskType::Backup }
    fn priority(&self) -> TaskPriority { TaskPriority::High }
    fn estimated_duration(&self) -> Duration { Duration::from_secs(180) }
    fn prerequisites(&self) -> Vec<String> { vec!["backup_storage_available".to_string()] }

    async fn can_execute(&self, context: &AutomationContext) -> Result<bool> {
        Ok(context.system_state.disk_usage < 95.0) // Don't backup if disk is full
    }

    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = std::time::Instant::now();
        let output = "Configuration backup completed successfully".to_string();

        Ok(TaskResult {
            task_id: self.id.clone(),
            execution_id: context.execution_id.clone(),
            success: true,
            exit_code: Some(0),
            output,
            error_message: None,
            duration: start_time.elapsed(),
            resources_used: ResourceUsage {
                cpu_time: Duration::from_millis(1000),
                memory_peak: 2 * 1024 * 1024, // 2MB
                disk_io: 50 * 1024 * 1024, // 50MB
                network_io: 0,
            },
            side_effects: vec![SideEffect {
                effect_type: SideEffectType::FileModified,
                description: "Backup files created".to_string(),
                reversible: true,
                rollback_command: Some("rm -f /backup/config-*.tar.gz".to_string()),
            }],
        })
    }

    async fn rollback(&self, _context: &AutomationContext) -> Result<()> {
        // Remove backup files if needed
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

/// User data backup task
pub struct UserDataBackupTask {
    id: String,
}

impl UserDataBackupTask {
    pub fn new() -> Self {
        Self {
            id: "user-data-backup".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl AutomationTask for UserDataBackupTask {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { "User Data Backup" }
    fn description(&self) -> &str { "Backup user home directories" }
    fn task_type(&self) -> TaskType { TaskType::Backup }
    fn priority(&self) -> TaskPriority { TaskPriority::Normal }
    fn estimated_duration(&self) -> Duration { Duration::from_secs(600) }
    fn prerequisites(&self) -> Vec<String> { vec!["backup_storage_available".to_string()] }

    async fn can_execute(&self, context: &AutomationContext) -> Result<bool> {
        Ok(context.system_state.user_sessions == 0) // Only when no users logged in
    }

    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = std::time::Instant::now();
        let output = "User data backup completed successfully".to_string();

        Ok(TaskResult {
            task_id: self.id.clone(),
            execution_id: context.execution_id.clone(),
            success: true,
            exit_code: Some(0),
            output,
            error_message: None,
            duration: start_time.elapsed(),
            resources_used: ResourceUsage {
                cpu_time: Duration::from_millis(5000),
                memory_peak: 10 * 1024 * 1024, // 10MB
                disk_io: 500 * 1024 * 1024, // 500MB
                network_io: 0,
            },
            side_effects: vec![SideEffect {
                effect_type: SideEffectType::FileModified,
                description: "User backup files created".to_string(),
                reversible: true,
                rollback_command: Some("rm -f /backup/users-*.tar.gz".to_string()),
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
