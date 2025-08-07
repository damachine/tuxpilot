use anyhow::Result;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use super::Schedule;

/// Task scheduler for automation
#[derive(Debug, Clone)]
pub struct TaskScheduler {
    scheduled_tasks: HashMap<String, Schedule>,
    execution_queue: Vec<ScheduledExecution>,
}

/// Scheduled execution entry
#[derive(Debug, Clone)]
pub struct ScheduledExecution {
    pub task_id: String,
    pub scheduled_time: DateTime<Utc>,
    pub priority: u8,
}

impl TaskScheduler {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            scheduled_tasks: HashMap::new(),
            execution_queue: Vec::new(),
        })
    }

    pub async fn schedule_task(&mut self, task_id: String, schedule: Schedule) -> Result<()> {
        self.scheduled_tasks.insert(task_id.clone(), schedule);
        self.update_execution_queue(&task_id).await?;
        Ok(())
    }

    pub async fn get_due_tasks(&mut self) -> Result<Vec<String>> {
        let now = Utc::now();
        let mut due_tasks = Vec::new();

        // Find tasks that are due
        for execution in &self.execution_queue {
            if execution.scheduled_time <= now {
                due_tasks.push(execution.task_id.clone());
            }
        }

        // Remove executed tasks and reschedule recurring ones
        self.execution_queue.retain(|e| e.scheduled_time > now);
        
        for task_id in &due_tasks {
            self.update_execution_queue(task_id).await?;
        }

        Ok(due_tasks)
    }

    async fn update_execution_queue(&mut self, task_id: &str) -> Result<()> {
        // First, get the schedule and calculate next time
        let next_time = if let Some(schedule) = self.scheduled_tasks.get(task_id) {
            if schedule.enabled {
                self.calculate_next_execution(schedule)?
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        // Then update the schedule
        if let Some(schedule) = self.scheduled_tasks.get_mut(task_id) {
            schedule.next_execution = next_time;
        }

        self.execution_queue.push(ScheduledExecution {
            task_id: task_id.to_string(),
            scheduled_time: next_time,
            priority: 5, // Default priority
        });

        // Sort by scheduled time
        self.execution_queue.sort_by(|a, b| a.scheduled_time.cmp(&b.scheduled_time));
        Ok(())
    }

    fn calculate_next_execution(&self, schedule: &Schedule) -> Result<DateTime<Utc>> {
        match schedule.schedule_type {
            super::ScheduleType::Once => Ok(schedule.next_execution),
            super::ScheduleType::Recurring => {
                Ok(Utc::now() + chrono::Duration::from_std(schedule.interval)?)
            }
            super::ScheduleType::Cron(_) => {
                // Simplified cron implementation
                Ok(Utc::now() + chrono::Duration::hours(1))
            }
            super::ScheduleType::OnEvent(_) => {
                // Event-based scheduling
                Ok(Utc::now() + chrono::Duration::minutes(5))
            }
        }
    }
}
