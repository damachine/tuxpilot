use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};

use crate::linux_integration::LinuxIntegration;

pub mod advanced;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorDiagnostic {
    pub error_message: String,
    pub context: String,
    pub system_info: String,
    pub relevant_logs: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorCategory {
    System,
    Network,
    Package,
    Service,
    Hardware,
    Permission,
    Unknown,
}

impl ErrorDiagnostic {
    pub async fn analyze_error(error_input: &str, linux_integration: &LinuxIntegration) -> Result<Self> {
        let category = Self::categorize_error(error_input);
        let severity = Self::assess_severity(error_input, &category);
        
        let system_info = linux_integration.get_system_info().await?;
        let relevant_logs = Self::extract_relevant_logs(error_input, linux_integration).await?;
        
        Ok(Self {
            error_message: error_input.to_string(),
            context: Self::build_context(error_input, &system_info),
            system_info,
            relevant_logs,
            timestamp: Utc::now(),
            severity,
            category,
        })
    }

    pub async fn analyze_system_logs(linux_integration: &LinuxIntegration) -> Result<Self> {
        let recent_errors = Self::scan_recent_errors(linux_integration).await?;
        let system_info = linux_integration.get_system_info().await?;
        
        let error_message = if recent_errors.is_empty() {
            "No recent errors found in system logs".to_string()
        } else {
            format!("Found {} recent errors", recent_errors.len())
        };

        Ok(Self {
            error_message: error_message.clone(),
            context: format!("Automatic system log analysis - {}", error_message),
            system_info,
            relevant_logs: recent_errors,
            timestamp: Utc::now(),
            severity: ErrorSeverity::Medium,
            category: ErrorCategory::System,
        })
    }

    fn categorize_error(error_message: &str) -> ErrorCategory {
        let error_lower = error_message.to_lowercase();
        
        if error_lower.contains("network") || error_lower.contains("connection") || 
           error_lower.contains("dns") || error_lower.contains("timeout") {
            ErrorCategory::Network
        } else if error_lower.contains("package") || error_lower.contains("pacman") || 
                  error_lower.contains("apt") || error_lower.contains("dependency") {
            ErrorCategory::Package
        } else if error_lower.contains("service") || error_lower.contains("systemd") || 
                  error_lower.contains("daemon") {
            ErrorCategory::Service
        } else if error_lower.contains("permission") || error_lower.contains("denied") || 
                  error_lower.contains("unauthorized") {
            ErrorCategory::Permission
        } else if error_lower.contains("hardware") || error_lower.contains("device") || 
                  error_lower.contains("driver") {
            ErrorCategory::Hardware
        } else if error_lower.contains("kernel") || error_lower.contains("system") || 
                  error_lower.contains("boot") {
            ErrorCategory::System
        } else {
            ErrorCategory::Unknown
        }
    }

    fn assess_severity(error_message: &str, category: &ErrorCategory) -> ErrorSeverity {
        let error_lower = error_message.to_lowercase();
        
        if error_lower.contains("critical") || error_lower.contains("fatal") || 
           error_lower.contains("panic") || error_lower.contains("segfault") {
            ErrorSeverity::Critical
        } else if error_lower.contains("error") || error_lower.contains("failed") || 
                  error_lower.contains("cannot") {
            match category {
                ErrorCategory::System | ErrorCategory::Hardware => ErrorSeverity::High,
                ErrorCategory::Service | ErrorCategory::Package => ErrorSeverity::Medium,
                _ => ErrorSeverity::Medium,
            }
        } else if error_lower.contains("warning") || error_lower.contains("deprecated") {
            ErrorSeverity::Low
        } else {
            ErrorSeverity::Medium
        }
    }

    fn build_context(error_message: &str, system_info: &str) -> String {
        format!(
            "Error occurred on system: {}\nError details: {}",
            system_info.lines().take(3).collect::<Vec<_>>().join(" | "),
            error_message
        )
    }

    async fn extract_relevant_logs(error_input: &str, linux_integration: &LinuxIntegration) -> Result<Vec<String>> {
        let mut relevant_logs = Vec::new();
        
        // Extract keywords from error message for log searching
        let keywords = Self::extract_keywords(error_input);
        
        // Search through system logs
        for log_path in &linux_integration.config.system.log_paths {
            if let Ok(logs) = Self::search_logs(log_path, &keywords).await {
                relevant_logs.extend(logs);
            }
        }

        // Limit to most recent 10 entries
        relevant_logs.truncate(10);
        Ok(relevant_logs)
    }

    async fn scan_recent_errors(linux_integration: &LinuxIntegration) -> Result<Vec<String>> {
        let mut errors = Vec::new();
        
        // Scan journalctl for recent errors
        if let Ok(output) = linux_integration.execute_command("journalctl", &["-p", "err", "-n", "20", "--no-pager"]).await {
            let lines: Vec<String> = output.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|s| s.to_string())
                .collect();
            errors.extend(lines);
        }

        // Scan dmesg for kernel errors
        if let Ok(output) = linux_integration.execute_command("dmesg", &["-l", "err", "-T"]).await {
            let lines: Vec<String> = output.lines()
                .take(10)
                .map(|s| s.to_string())
                .collect();
            errors.extend(lines);
        }

        Ok(errors)
    }

    fn extract_keywords(error_message: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Extract quoted strings
        let quote_regex = Regex::new(r#""([^"]+)""#).unwrap();
        for cap in quote_regex.captures_iter(error_message) {
            if let Some(quoted) = cap.get(1) {
                keywords.push(quoted.as_str().to_string());
            }
        }
        
        // Extract common error patterns
        let patterns = [
            r"\b[A-Z][a-z]+Error\b",
            r"\b[A-Z][a-z]+Exception\b",
            r"\bfailed\s+to\s+\w+\b",
            r"\bcannot\s+\w+\b",
            r"\b\w+\s+not\s+found\b",
        ];
        
        for pattern in &patterns {
            let regex = Regex::new(pattern).unwrap();
            for mat in regex.find_iter(error_message) {
                keywords.push(mat.as_str().to_string());
            }
        }
        
        // Extract service/package names
        let word_regex = Regex::new(r"\b[a-z][a-z0-9-]+\.(service|timer|socket)\b").unwrap();
        for mat in word_regex.find_iter(error_message) {
            keywords.push(mat.as_str().to_string());
        }
        
        keywords.dedup();
        keywords.truncate(5); // Limit keywords
        keywords
    }

    async fn search_logs(log_path: &Path, keywords: &[String]) -> Result<Vec<String>> {
        if !log_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(log_path)
            .context(format!("Failed to read log file: {:?}", log_path))?;

        let mut matching_lines = Vec::new();
        
        for line in content.lines().rev().take(1000) { // Check last 1000 lines
            for keyword in keywords {
                if line.to_lowercase().contains(&keyword.to_lowercase()) {
                    matching_lines.push(format!("{}: {}", log_path.display(), line));
                    break;
                }
            }
            
            if matching_lines.len() >= 5 {
                break;
            }
        }
        
        Ok(matching_lines)
    }
}
