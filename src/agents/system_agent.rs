use anyhow::Result;
use std::time::Instant;

use super::{Agent, AgentTask, AgentContext, AgentResult, AgentStatus, AgentCapability, SpecializationLevel, TaskType};
use crate::config::Config;

/// Specialized agent for system management tasks
pub struct SystemAgent {
    id: String,
    config: Config,
    tasks_completed: u64,
    tasks_failed: u64,
    total_execution_time_ms: u64,
    is_busy: bool,
    current_task: Option<String>,
}

impl SystemAgent {
    pub async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            id: "system-agent".to_string(),
            config: config.clone(),
            tasks_completed: 0,
            tasks_failed: 0,
            total_execution_time_ms: 0,
            is_busy: false,
            current_task: None,
        })
    }

    async fn analyze_system_status(&self, context: &AgentContext) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // CPU usage analysis
        if context.system_state.cpu_usage > 80.0 {
            recommendations.push("High CPU usage detected. Consider identifying resource-intensive processes.".to_string());
        }

        // Memory usage analysis
        if context.system_state.memory_usage > 85.0 {
            recommendations.push("High memory usage detected. Consider freeing up memory or adding more RAM.".to_string());
        }

        // Disk usage analysis
        if context.system_state.disk_usage > 90.0 {
            recommendations.push("Critical disk usage detected. Immediate cleanup required.".to_string());
        } else if context.system_state.disk_usage > 80.0 {
            recommendations.push("High disk usage detected. Consider cleaning up unnecessary files.".to_string());
        }

        // Service analysis
        let critical_services = vec!["sshd", "systemd", "networkd"];
        for service in critical_services {
            if !context.system_state.services_running.contains(&service.to_string()) {
                recommendations.push(format!("Critical service '{}' is not running.", service));
            }
        }

        // Error analysis
        if !context.system_state.recent_errors.is_empty() {
            recommendations.push(format!("Found {} recent system errors that need attention.", 
                                       context.system_state.recent_errors.len()));
        }

        Ok(recommendations)
    }

    async fn get_system_information(&self, _context: &AgentContext) -> Result<Vec<String>> {
        let mut info = Vec::new();

        // Get system uptime
        if let Ok(output) = tokio::process::Command::new("uptime")
            .output()
            .await
        {
            let uptime = String::from_utf8_lossy(&output.stdout);
            info.push(format!("System uptime: {}", uptime.trim()));
        }

        // Get kernel version
        if let Ok(output) = tokio::process::Command::new("uname")
            .args(&["-r"])
            .output()
            .await
        {
            let kernel = String::from_utf8_lossy(&output.stdout);
            info.push(format!("Kernel version: {}", kernel.trim()));
        }

        // Get load average
        if let Ok(output) = tokio::process::Command::new("cat")
            .args(&["/proc/loadavg"])
            .output()
            .await
        {
            let loadavg = String::from_utf8_lossy(&output.stdout);
            info.push(format!("Load average: {}", loadavg.trim()));
        }

        // Get memory information
        if let Ok(output) = tokio::process::Command::new("free")
            .args(&["-h"])
            .output()
            .await
        {
            let memory = String::from_utf8_lossy(&output.stdout);
            info.push(format!("Memory usage:\n{}", memory));
        }

        Ok(info)
    }

    async fn optimize_system(&self, _context: &AgentContext) -> Result<Vec<String>> {
        let mut actions = Vec::new();

        // Clear package cache (safe operation)
        if self.config.system.package_manager.to_string().contains("Pacman") {
            actions.push("Suggested: sudo pacman -Sc (clear package cache)".to_string());
        } else if self.config.system.package_manager.to_string().contains("Apt") {
            actions.push("Suggested: sudo apt autoremove && sudo apt autoclean".to_string());
        }

        // Clear temporary files
        actions.push("Suggested: sudo find /tmp -type f -atime +7 -delete".to_string());

        // Update package database
        actions.push("Suggested: Update package database".to_string());

        // Check for system updates
        actions.push("Suggested: Check for available system updates".to_string());

        Ok(actions)
    }

    async fn monitor_system(&self, context: &AgentContext) -> Result<Vec<String>> {
        let mut monitoring_data = Vec::new();

        monitoring_data.push(format!("CPU Usage: {:.1}%", context.system_state.cpu_usage));
        monitoring_data.push(format!("Memory Usage: {:.1}%", context.system_state.memory_usage));
        monitoring_data.push(format!("Disk Usage: {:.1}%", context.system_state.disk_usage));
        monitoring_data.push(format!("Network Active: {}", context.system_state.network_active));
        monitoring_data.push(format!("Running Services: {}", context.system_state.services_running.len()));

        if !context.system_state.recent_errors.is_empty() {
            monitoring_data.push(format!("Recent Errors: {}", context.system_state.recent_errors.len()));
        }

        Ok(monitoring_data)
    }
}

#[async_trait::async_trait]
impl Agent for SystemAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        "System Management Agent"
    }

    fn description(&self) -> &str {
        "Specialized agent for system status analysis, optimization, and monitoring"
    }

    fn capabilities(&self) -> &[AgentCapability] {
        &[
            AgentCapability::SystemManagement,
            AgentCapability::ServiceManagement,
            AgentCapability::ProcessManagement,
            AgentCapability::PerformanceOptimization,
        ]
    }

    fn specialization_level(&self) -> SpecializationLevel {
        SpecializationLevel::Expert
    }

    fn can_handle_task(&self, task: &AgentTask) -> bool {
        matches!(task.task_type, 
            TaskType::Analyze | 
            TaskType::Monitor | 
            TaskType::Optimize | 
            TaskType::Report
        )
    }

    async fn execute_task(&self, task: &AgentTask, context: &AgentContext) -> Result<AgentResult> {
        let start_time = Instant::now();
        let mut result = AgentResult {
            agent_id: self.id.clone(),
            task_id: task.id.clone(),
            success: false,
            confidence: 0.0,
            recommendations: Vec::new(),
            actions_taken: Vec::new(),
            warnings: Vec::new(),
            next_steps: Vec::new(),
            execution_time_ms: 0,
        };

        match task.task_type {
            TaskType::Analyze => {
                match self.analyze_system_status(context).await {
                    Ok(recommendations) => {
                        result.recommendations = recommendations;
                        result.success = true;
                        result.confidence = 0.9;
                        result.next_steps.push("Review recommendations and take appropriate action".to_string());
                    }
                    Err(e) => {
                        result.warnings.push(format!("Analysis failed: {}", e));
                    }
                }
            }

            TaskType::Monitor => {
                match self.monitor_system(context).await {
                    Ok(monitoring_data) => {
                        result.recommendations = monitoring_data;
                        result.success = true;
                        result.confidence = 0.95;
                        result.actions_taken.push("System monitoring completed".to_string());
                    }
                    Err(e) => {
                        result.warnings.push(format!("Monitoring failed: {}", e));
                    }
                }
            }

            TaskType::Optimize => {
                match self.optimize_system(context).await {
                    Ok(actions) => {
                        result.recommendations = actions;
                        result.success = true;
                        result.confidence = 0.8;
                        result.next_steps.push("Execute optimization commands with appropriate permissions".to_string());
                        result.warnings.push("Review all commands before execution".to_string());
                    }
                    Err(e) => {
                        result.warnings.push(format!("Optimization planning failed: {}", e));
                    }
                }
            }

            TaskType::Report => {
                match self.get_system_information(context).await {
                    Ok(info) => {
                        result.recommendations = info;
                        result.success = true;
                        result.confidence = 0.95;
                        result.actions_taken.push("System information report generated".to_string());
                    }
                    Err(e) => {
                        result.warnings.push(format!("Report generation failed: {}", e));
                    }
                }
            }

            _ => {
                result.warnings.push("Task type not supported by System Agent".to_string());
            }
        }

        result.execution_time_ms = start_time.elapsed().as_millis() as u64;
        Ok(result)
    }

    async fn get_status(&self) -> Result<AgentStatus> {
        let average_execution_time = if self.tasks_completed > 0 {
            self.total_execution_time_ms as f64 / self.tasks_completed as f64
        } else {
            0.0
        };

        Ok(AgentStatus {
            agent_id: self.id.clone(),
            is_healthy: true,
            is_busy: self.is_busy,
            current_task: self.current_task.clone(),
            tasks_completed: self.tasks_completed,
            tasks_failed: self.tasks_failed,
            average_execution_time_ms: average_execution_time,
            last_activity: chrono::Utc::now(),
        })
    }

    async fn initialize(&mut self, _config: &Config) -> Result<()> {
        println!("🔧 Initializing System Agent");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        println!("🔄 Shutting down System Agent");
        Ok(())
    }
}
