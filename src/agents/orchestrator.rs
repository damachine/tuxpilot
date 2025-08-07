use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{AgentTask, AgentContext, TaskType, TaskPriority, AgentCapability};
use crate::config::Config;
use crate::linux_integration::LinuxIntegration;
use crate::ai::AiClient;

/// Task execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    pub id: String,
    pub description: String,
    pub tasks: Vec<AgentTask>,
    pub estimated_duration_seconds: u64,
    pub complexity_score: f32,
}

/// Orchestrator status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorStatus {
    pub is_active: bool,
    pub plans_created: u64,
    pub plans_executed: u64,
    pub average_plan_duration_seconds: f64,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Agent orchestrator for coordinating multi-agent tasks
pub struct AgentOrchestrator {
    config: Config,
    linux_integration: LinuxIntegration,
    ai_client: AiClient,
    plans_created: u64,
    plans_executed: u64,
    plan_history: Vec<TaskPlan>,
}

impl AgentOrchestrator {
    pub async fn new(config: Config, linux_integration: LinuxIntegration, ai_client: AiClient) -> Result<Self> {
        Ok(Self {
            config,
            linux_integration,
            ai_client,
            plans_created: 0,
            plans_executed: 0,
            plan_history: Vec::new(),
        })
    }

    /// Analyze user request and create execution plan
    pub async fn analyze_request(&mut self, request: &str, context: &AgentContext) -> Result<TaskPlan> {
        let plan_id = uuid::Uuid::new_v4().to_string();
        
        // Classify the request type
        let request_type = self.classify_request(request).await?;
        
        // Generate tasks based on request type
        let tasks = self.generate_tasks_for_request(&request_type, request, context).await?;
        
        // Calculate complexity and duration
        let complexity_score = self.calculate_complexity(&tasks);
        let estimated_duration = self.estimate_duration(&tasks);
        
        let plan = TaskPlan {
            id: plan_id,
            description: request.to_string(),
            tasks,
            estimated_duration_seconds: estimated_duration,
            complexity_score,
        };

        self.plans_created += 1;
        self.plan_history.push(plan.clone());

        println!("📋 Created execution plan: {} tasks, complexity: {:.2}, duration: {}s", 
                 plan.tasks.len(), plan.complexity_score, plan.estimated_duration_seconds);

        Ok(plan)
    }

    async fn classify_request(&self, request: &str) -> Result<RequestType> {
        let request_lower = request.to_lowercase();

        // System management requests
        if request_lower.contains("system") || request_lower.contains("status") || request_lower.contains("health") {
            return Ok(RequestType::SystemManagement);
        }

        // Security requests
        if request_lower.contains("security") || request_lower.contains("vulnerability") || request_lower.contains("scan") {
            return Ok(RequestType::Security);
        }

        // Package management requests
        if request_lower.contains("install") || request_lower.contains("package") || request_lower.contains("update") {
            return Ok(RequestType::PackageManagement);
        }

        // Network requests
        if request_lower.contains("network") || request_lower.contains("connection") || request_lower.contains("dns") {
            return Ok(RequestType::Network);
        }

        // Performance requests
        if request_lower.contains("performance") || request_lower.contains("optimize") || request_lower.contains("slow") {
            return Ok(RequestType::Performance);
        }

        // Service management requests
        if request_lower.contains("service") || request_lower.contains("daemon") || request_lower.contains("restart") {
            return Ok(RequestType::ServiceManagement);
        }

        // Log analysis requests
        if request_lower.contains("log") || request_lower.contains("error") || request_lower.contains("debug") {
            return Ok(RequestType::LogAnalysis);
        }

        // Default to general system management
        Ok(RequestType::General)
    }

    async fn generate_tasks_for_request(&self, request_type: &RequestType, request: &str, _context: &AgentContext) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        match request_type {
            RequestType::SystemManagement => {
                tasks.push(self.create_task(
                    TaskType::Analyze,
                    TaskPriority::Normal,
                    "Analyze system status",
                    vec![AgentCapability::SystemManagement],
                    30,
                ));
                
                if request.to_lowercase().contains("optimize") {
                    tasks.push(self.create_task(
                        TaskType::Optimize,
                        TaskPriority::Normal,
                        "Optimize system performance",
                        vec![AgentCapability::PerformanceOptimization],
                        60,
                    ));
                }
            },

            RequestType::Security => {
                tasks.push(self.create_task(
                    TaskType::Analyze,
                    TaskPriority::High,
                    "Security vulnerability scan",
                    vec![AgentCapability::SecurityAnalysis],
                    120,
                ));
                
                tasks.push(self.create_task(
                    TaskType::Report,
                    TaskPriority::High,
                    "Generate security report",
                    vec![AgentCapability::SecurityAnalysis],
                    30,
                ));
            },

            RequestType::PackageManagement => {
                tasks.push(self.create_task(
                    TaskType::Analyze,
                    TaskPriority::Normal,
                    "Analyze package requirements",
                    vec![AgentCapability::PackageManagement],
                    15,
                ));
                
                if request.to_lowercase().contains("install") {
                    tasks.push(self.create_task(
                        TaskType::Execute,
                        TaskPriority::Normal,
                        "Install packages",
                        vec![AgentCapability::PackageManagement],
                        180,
                    ));
                }
            },

            RequestType::Network => {
                tasks.push(self.create_task(
                    TaskType::Diagnose,
                    TaskPriority::Normal,
                    "Network connectivity diagnosis",
                    vec![AgentCapability::NetworkDiagnostics],
                    45,
                ));
                
                tasks.push(self.create_task(
                    TaskType::Report,
                    TaskPriority::Normal,
                    "Network status report",
                    vec![AgentCapability::NetworkDiagnostics],
                    15,
                ));
            },

            RequestType::Performance => {
                tasks.push(self.create_task(
                    TaskType::Monitor,
                    TaskPriority::Normal,
                    "Performance monitoring",
                    vec![AgentCapability::PerformanceOptimization],
                    60,
                ));
                
                tasks.push(self.create_task(
                    TaskType::Optimize,
                    TaskPriority::Normal,
                    "Performance optimization",
                    vec![AgentCapability::PerformanceOptimization],
                    120,
                ));
            },

            RequestType::ServiceManagement => {
                tasks.push(self.create_task(
                    TaskType::Analyze,
                    TaskPriority::Normal,
                    "Service status analysis",
                    vec![AgentCapability::ServiceManagement],
                    30,
                ));
            },

            RequestType::LogAnalysis => {
                tasks.push(self.create_task(
                    TaskType::Analyze,
                    TaskPriority::Normal,
                    "Log file analysis",
                    vec![AgentCapability::LogAnalysis],
                    90,
                ));
            },

            RequestType::General => {
                tasks.push(self.create_task(
                    TaskType::Analyze,
                    TaskPriority::Normal,
                    "General system analysis",
                    vec![AgentCapability::SystemManagement],
                    45,
                ));
            },
        }

        Ok(tasks)
    }

    fn create_task(&self, task_type: TaskType, priority: TaskPriority, description: &str, _capabilities: Vec<AgentCapability>, timeout: u64) -> AgentTask {
        AgentTask {
            id: uuid::Uuid::new_v4().to_string(),
            task_type,
            priority,
            description: description.to_string(),
            parameters: HashMap::new(),
            timeout_seconds: timeout,
            retry_count: 3,
        }
    }

    fn calculate_complexity(&self, tasks: &[AgentTask]) -> f32 {
        let mut complexity = 0.0;

        for task in tasks {
            complexity += match task.task_type {
                TaskType::Analyze => 1.0,
                TaskType::Diagnose => 1.5,
                TaskType::Monitor => 1.2,
                TaskType::Report => 0.8,
                TaskType::Recommend => 1.0,
                TaskType::Optimize => 2.5,
                TaskType::Execute => 3.0,
            };

            complexity += match task.priority {
                TaskPriority::Emergency => 2.0,
                TaskPriority::Critical => 1.5,
                TaskPriority::High => 1.2,
                TaskPriority::Normal => 1.0,
                TaskPriority::Low => 0.8,
            };
        }

        complexity / tasks.len() as f32
    }

    fn estimate_duration(&self, tasks: &[AgentTask]) -> u64 {
        tasks.iter().map(|task| task.timeout_seconds).sum()
    }

    pub async fn get_status(&self) -> Result<OrchestratorStatus> {
        let average_duration = if self.plans_executed > 0 {
            self.plan_history.iter()
                .map(|plan| plan.estimated_duration_seconds as f64)
                .sum::<f64>() / self.plans_executed as f64
        } else {
            0.0
        };

        Ok(OrchestratorStatus {
            is_active: true,
            plans_created: self.plans_created,
            plans_executed: self.plans_executed,
            average_plan_duration_seconds: average_duration,
            last_activity: chrono::Utc::now(),
        })
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        println!("🔄 Shutting down orchestrator...");
        Ok(())
    }
}

/// Request classification types
#[derive(Debug, Clone, Serialize, Deserialize)]
enum RequestType {
    SystemManagement,
    Security,
    PackageManagement,
    Network,
    Performance,
    ServiceManagement,
    LogAnalysis,
    General,
}
