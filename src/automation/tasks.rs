// Task definitions and utilities for automation system

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Task execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatistics {
    pub task_id: String,
    pub total_executions: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
    pub average_duration_seconds: f32,
    pub last_execution_time: chrono::DateTime<chrono::Utc>,
    pub success_rate: f32,
}

/// Task dependency graph
#[derive(Debug, Clone)]
pub struct TaskDependencyGraph {
    pub dependencies: std::collections::HashMap<String, Vec<String>>,
}

impl TaskDependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: std::collections::HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, task_id: String, depends_on: String) {
        self.dependencies.entry(task_id).or_insert_with(Vec::new).push(depends_on);
    }

    pub fn get_execution_order(&self, tasks: &[String]) -> Result<Vec<String>> {
        // Simple topological sort implementation
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for task in tasks {
            if !visited.contains(task) {
                self.visit_task(task, &mut visited, &mut result)?;
            }
        }

        Ok(result)
    }

    fn visit_task(&self, task: &str, visited: &mut std::collections::HashSet<String>, result: &mut Vec<String>) -> Result<()> {
        visited.insert(task.to_string());

        if let Some(deps) = self.dependencies.get(task) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.visit_task(dep, visited, result)?;
                }
            }
        }

        result.push(task.to_string());
        Ok(())
    }
}
