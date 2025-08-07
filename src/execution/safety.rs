use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::config::Config;
use super::{ExecutionRequest, RiskLevel};

#[derive(Debug, Clone)]
pub struct SafetyChecker {
    dangerous_commands: HashSet<String>,
    dangerous_patterns: Vec<Regex>,
    safe_commands: HashSet<String>,
    config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyResult {
    pub is_safe: bool,
    pub risk_level: RiskLevel,
    pub reason: String,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

impl SafetyChecker {
    pub async fn new(config: &Config) -> Result<Self> {
        let dangerous_commands = Self::load_dangerous_commands();
        let dangerous_patterns = Self::load_dangerous_patterns()?;
        let safe_commands = Self::load_safe_commands();

        Ok(Self {
            dangerous_commands,
            dangerous_patterns,
            safe_commands,
            config: config.clone(),
        })
    }

    fn load_dangerous_commands() -> HashSet<String> {
        let mut commands = HashSet::new();
        
        // Destructive commands
        commands.insert("rm".to_string());
        commands.insert("rmdir".to_string());
        commands.insert("dd".to_string());
        commands.insert("mkfs".to_string());
        commands.insert("fdisk".to_string());
        commands.insert("parted".to_string());
        commands.insert("shred".to_string());
        commands.insert("wipefs".to_string());
        
        // System modification
        commands.insert("chmod".to_string());
        commands.insert("chown".to_string());
        commands.insert("mount".to_string());
        commands.insert("umount".to_string());
        
        // User management
        commands.insert("userdel".to_string());
        commands.insert("passwd".to_string());
        
        // Network/Security
        commands.insert("iptables".to_string());
        commands.insert("ufw".to_string());
        commands.insert("firewall-cmd".to_string());
        
        // Package management (potentially dangerous)
        commands.insert("dpkg".to_string());
        
        commands
    }

    fn load_dangerous_patterns() -> Result<Vec<Regex>> {
        let patterns = vec![
            // Recursive deletion
            r"rm\s+.*-r.*\s*/",
            r"rm\s+.*--recursive.*\s*/",
            
            // Force operations
            r"rm\s+.*-f",
            r".*--force",
            
            // System directories
            r".*/etc/.*",
            r".*/boot/.*",
            r".*/sys/.*",
            r".*/proc/.*",
            r".*/dev/.*",
            
            // Wildcards in dangerous contexts
            r"rm\s+.*\*",
            r"chmod\s+.*\*",
            r"chown\s+.*\*",
            
            // Pipe to shell
            r".*\|\s*sh",
            r".*\|\s*bash",
            
            // Download and execute
            r"curl.*\|\s*sh",
            r"wget.*\|\s*sh",
            
            // Format operations
            r"mkfs\.",
            r"dd\s+.*of=/dev/",
        ];

        patterns.into_iter()
            .map(|p| Regex::new(p).context("Failed to compile regex pattern"))
            .collect()
    }

    fn load_safe_commands() -> HashSet<String> {
        let mut commands = HashSet::new();
        
        // Read-only commands
        commands.insert("ls".to_string());
        commands.insert("cat".to_string());
        commands.insert("head".to_string());
        commands.insert("tail".to_string());
        commands.insert("grep".to_string());
        commands.insert("find".to_string());
        commands.insert("locate".to_string());
        commands.insert("which".to_string());
        commands.insert("whereis".to_string());
        
        // System information
        commands.insert("ps".to_string());
        commands.insert("top".to_string());
        commands.insert("htop".to_string());
        commands.insert("free".to_string());
        commands.insert("df".to_string());
        commands.insert("du".to_string());
        commands.insert("lscpu".to_string());
        commands.insert("lsblk".to_string());
        commands.insert("lsusb".to_string());
        commands.insert("lspci".to_string());
        commands.insert("uname".to_string());
        commands.insert("uptime".to_string());
        commands.insert("whoami".to_string());
        commands.insert("id".to_string());
        commands.insert("groups".to_string());
        
        // Network information
        commands.insert("ping".to_string());
        commands.insert("traceroute".to_string());
        commands.insert("nslookup".to_string());
        commands.insert("dig".to_string());
        commands.insert("ip".to_string());
        commands.insert("ss".to_string());
        commands.insert("netstat".to_string());
        
        // Package queries
        commands.insert("pacman".to_string()); // With safe args only
        commands.insert("apt".to_string());
        commands.insert("dnf".to_string());
        commands.insert("zypper".to_string());
        
        commands
    }

    pub async fn analyze_command(&self, request: &ExecutionRequest) -> Result<SafetyResult> {
        let full_command = format!("{} {}", request.command, request.args.join(" "));
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        
        // Check if command is explicitly dangerous
        if self.dangerous_commands.contains(&request.command) {
            return Ok(SafetyResult {
                is_safe: false,
                risk_level: RiskLevel::High,
                reason: format!("Command '{}' is potentially dangerous", request.command),
                warnings: vec![format!("The command '{}' can cause system damage", request.command)],
                suggestions: vec!["Consider using safer alternatives or review the command carefully".to_string()],
            });
        }

        // Check dangerous patterns
        for pattern in &self.dangerous_patterns {
            if pattern.is_match(&full_command) {
                warnings.push(format!("Command matches dangerous pattern: {}", pattern.as_str()));
            }
        }

        // Analyze specific command safety
        let (is_safe, risk_level, reason) = self.analyze_specific_command(request)?;
        
        if !is_safe {
            return Ok(SafetyResult {
                is_safe: false,
                risk_level,
                reason,
                warnings,
                suggestions,
            });
        }

        // Check for common safety issues
        self.check_common_safety_issues(request, &mut warnings, &mut suggestions);

        // Determine final safety
        let final_risk_level = if warnings.is_empty() {
            RiskLevel::Safe
        } else if warnings.len() <= 2 {
            RiskLevel::Low
        } else {
            RiskLevel::Medium
        };

        Ok(SafetyResult {
            is_safe: true,
            risk_level: final_risk_level,
            reason: "Command passed safety checks".to_string(),
            warnings,
            suggestions,
        })
    }

    fn analyze_specific_command(&self, request: &ExecutionRequest) -> Result<(bool, RiskLevel, String)> {
        match request.command.as_str() {
            "rm" => self.analyze_rm_command(&request.args),
            "chmod" | "chown" => self.analyze_permission_command(&request.args),
            "pacman" | "apt" | "dnf" | "zypper" => self.analyze_package_command(&request.args),
            "systemctl" | "service" => self.analyze_service_command(&request.args),
            "dd" => self.analyze_dd_command(&request.args),
            "mount" | "umount" => self.analyze_mount_command(&request.args),
            _ => {
                if self.safe_commands.contains(&request.command) {
                    Ok((true, RiskLevel::Safe, "Command is safe".to_string()))
                } else {
                    Ok((true, RiskLevel::Low, "Command not in known dangerous list".to_string()))
                }
            }
        }
    }

    fn analyze_rm_command(&self, args: &[String]) -> Result<(bool, RiskLevel, String)> {
        let _args_str = args.join(" ");
        
        // Check for recursive deletion of system directories
        if args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string()) {
            for arg in args {
                if arg.starts_with('/') && (
                    arg.starts_with("/etc") ||
                    arg.starts_with("/boot") ||
                    arg.starts_with("/sys") ||
                    arg.starts_with("/proc") ||
                    arg.starts_with("/dev") ||
                    arg == "/"
                ) {
                    return Ok((false, RiskLevel::Critical, 
                              format!("Recursive deletion of system directory: {}", arg)));
                }
            }
        }

        // Check for force flag with wildcards
        if (args.contains(&"-f".to_string()) || args.contains(&"--force".to_string())) &&
           args.iter().any(|arg| arg.contains('*')) {
            return Ok((false, RiskLevel::High, 
                      "Force deletion with wildcards is dangerous".to_string()));
        }

        Ok((true, RiskLevel::Medium, "rm command requires caution".to_string()))
    }

    fn analyze_permission_command(&self, args: &[String]) -> Result<(bool, RiskLevel, String)> {
        // Check for recursive permission changes on system directories
        if args.contains(&"-R".to_string()) || args.contains(&"--recursive".to_string()) {
            for arg in args {
                if arg.starts_with('/') && (
                    arg.starts_with("/etc") ||
                    arg.starts_with("/boot") ||
                    arg.starts_with("/sys") ||
                    arg == "/"
                ) {
                    return Ok((false, RiskLevel::Critical, 
                              format!("Recursive permission change on system directory: {}", arg)));
                }
            }
        }

        Ok((true, RiskLevel::Medium, "Permission changes require caution".to_string()))
    }

    fn analyze_package_command(&self, args: &[String]) -> Result<(bool, RiskLevel, String)> {
        // Check for safe package operations
        let safe_operations = ["search", "info", "list", "show", "-Q", "-Ss", "-Si"];
        
        if args.iter().any(|arg| safe_operations.contains(&arg.as_str())) {
            return Ok((true, RiskLevel::Safe, "Package query operation".to_string()));
        }

        // Check for installation/removal operations
        let modify_operations = ["install", "remove", "upgrade", "update", "-S", "-R", "-Syu"];
        
        if args.iter().any(|arg| modify_operations.contains(&arg.as_str())) {
            return Ok((true, RiskLevel::Medium, "Package modification operation".to_string()));
        }

        Ok((true, RiskLevel::Low, "Package operation".to_string()))
    }

    fn analyze_service_command(&self, args: &[String]) -> Result<(bool, RiskLevel, String)> {
        let safe_operations = ["status", "is-active", "is-enabled", "list-units"];
        
        if args.iter().any(|arg| safe_operations.contains(&arg.as_str())) {
            return Ok((true, RiskLevel::Safe, "Service query operation".to_string()));
        }

        let modify_operations = ["start", "stop", "restart", "enable", "disable"];
        
        if args.iter().any(|arg| modify_operations.contains(&arg.as_str())) {
            return Ok((true, RiskLevel::Medium, "Service modification operation".to_string()));
        }

        Ok((true, RiskLevel::Low, "Service operation".to_string()))
    }

    fn analyze_dd_command(&self, args: &[String]) -> Result<(bool, RiskLevel, String)> {
        // dd is always dangerous
        for arg in args {
            if arg.starts_with("of=/dev/") {
                return Ok((false, RiskLevel::Critical, 
                          "dd command writing to device is extremely dangerous".to_string()));
            }
        }

        Ok((false, RiskLevel::High, "dd command is potentially dangerous".to_string()))
    }

    fn analyze_mount_command(&self, _args: &[String]) -> Result<(bool, RiskLevel, String)> {
        // Mount operations can be risky
        Ok((true, RiskLevel::Medium, "Mount operations require caution".to_string()))
    }

    fn check_common_safety_issues(&self, request: &ExecutionRequest, warnings: &mut Vec<String>, suggestions: &mut Vec<String>) {
        let full_command = format!("{} {}", request.command, request.args.join(" "));

        // Check for sudo usage
        if request.command == "sudo" {
            warnings.push("Command uses sudo - elevated privileges".to_string());
            suggestions.push("Ensure you trust the command being executed with sudo".to_string());
        }

        // Check for pipe to shell
        if full_command.contains("| sh") || full_command.contains("| bash") {
            warnings.push("Command pipes output to shell - potential security risk".to_string());
            suggestions.push("Review the command output before piping to shell".to_string());
        }

        // Check for network downloads
        if (request.command == "curl" || request.command == "wget") && 
           request.args.iter().any(|arg| arg.starts_with("http")) {
            warnings.push("Command downloads content from internet".to_string());
            suggestions.push("Verify the source is trustworthy".to_string());
        }

        // Check for wildcards in potentially dangerous contexts
        if request.args.iter().any(|arg| arg.contains('*')) &&
           ["rm", "chmod", "chown", "mv"].contains(&request.command.as_str()) {
            warnings.push("Command uses wildcards - may affect more files than intended".to_string());
            suggestions.push("Consider using find with -exec for more precise control".to_string());
        }
    }

    pub fn get_safety_recommendations(&self, command: &str) -> Vec<String> {
        match command {
            "rm" => vec![
                "Use 'ls' first to see what files will be affected".to_string(),
                "Consider using 'trash' command instead of 'rm' for safety".to_string(),
                "Always double-check paths before using -r flag".to_string(),
            ],
            "chmod" | "chown" => vec![
                "Test permission changes on a single file first".to_string(),
                "Use 'ls -l' to verify current permissions".to_string(),
                "Be careful with recursive (-R) permission changes".to_string(),
            ],
            "dd" => vec![
                "ALWAYS verify input and output devices".to_string(),
                "Use 'lsblk' to confirm device names".to_string(),
                "Consider using 'cp' for file copying instead".to_string(),
            ],
            _ => vec![
                "Review command documentation if unsure".to_string(),
                "Test commands in a safe environment first".to_string(),
                "Keep backups of important data".to_string(),
            ],
        }
    }
}
