use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;
use uuid::Uuid;

pub mod permissions;
pub mod safety;
pub mod audit;

use crate::config::Config;

#[derive(Debug)]
pub struct CommandExecutor {
    config: Config,
    permission_manager: permissions::PermissionManager,
    safety_checker: safety::SafetyChecker,
    audit_logger: audit::AuditLogger,
    execution_mode: ExecutionMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    Supervised,     // Ask user before executing
    SemiAuto,      // Execute safe commands automatically, ask for dangerous ones
    Autonomous,    // Execute all commands automatically (with safety checks)
    ReadOnly,      // Only read operations, no modifications
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub id: Uuid,
    pub command: String,
    pub args: Vec<String>,
    pub description: String,
    pub risk_level: RiskLevel,
    pub required_permissions: Vec<Permission>,
    pub context: ExecutionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Safe,        // Read-only operations, system info
    Low,         // Package queries, service status
    Medium,      // Package installation, service restart
    High,        // System configuration changes
    Critical,    // Filesystem modifications, user management
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadSystem,
    WriteSystem,
    PackageManagement,
    ServiceManagement,
    FileSystemRead,
    FileSystemWrite,
    NetworkAccess,
    UserManagement,
    SystemConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub user_request: String,
    pub ai_reasoning: String,
    pub expected_outcome: String,
    pub rollback_plan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub id: Uuid,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: std::time::Duration,
    pub side_effects: Vec<SideEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect_type: SideEffectType,
    pub description: String,
    pub reversible: bool,
    pub rollback_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SideEffectType {
    FileCreated,
    FileModified,
    FileDeleted,
    PackageInstalled,
    PackageRemoved,
    ServiceStarted,
    ServiceStopped,
    ConfigurationChanged,
    UserCreated,
    UserModified,
}

impl CommandExecutor {
    pub async fn new(config: Config, execution_mode: ExecutionMode) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            permission_manager: permissions::PermissionManager::new(&config).await?,
            safety_checker: safety::SafetyChecker::new(&config).await?,
            audit_logger: audit::AuditLogger::new(&config).await?,
            execution_mode,
        })
    }

    pub async fn execute_request(&mut self, request: ExecutionRequest) -> Result<ExecutionResult> {
        // 1. Log the execution request
        self.audit_logger.log_request(&request).await?;

        // 2. Check permissions
        self.permission_manager.check_permissions(&request.required_permissions)?;

        // 3. Safety checks
        let safety_result = self.safety_checker.analyze_command(&request).await?;
        if !safety_result.is_safe {
            return Err(anyhow::anyhow!("Command failed safety check: {}", safety_result.reason));
        }

        // 4. Execution mode handling
        let should_execute = match self.execution_mode {
            ExecutionMode::ReadOnly => {
                if request.risk_level != RiskLevel::Safe {
                    return Err(anyhow::anyhow!("Read-only mode: command not allowed"));
                }
                true
            }
            ExecutionMode::Supervised => {
                self.request_user_approval(&request).await?
            }
            ExecutionMode::SemiAuto => {
                match request.risk_level {
                    RiskLevel::Safe | RiskLevel::Low => true,
                    _ => self.request_user_approval(&request).await?
                }
            }
            ExecutionMode::Autonomous => {
                match request.risk_level {
                    RiskLevel::Critical => self.request_user_approval(&request).await?,
                    _ => true
                }
            }
        };

        if !should_execute {
            return Ok(ExecutionResult {
                id: request.id,
                success: false,
                exit_code: None,
                stdout: "Execution cancelled by user".to_string(),
                stderr: String::new(),
                execution_time: std::time::Duration::from_secs(0),
                side_effects: vec![],
            });
        }

        // 5. Execute the command
        let result = self.execute_command_safely(&request).await?;

        // 6. Log the result
        self.audit_logger.log_result(&result).await?;

        Ok(result)
    }

    async fn execute_command_safely(&self, request: &ExecutionRequest) -> Result<ExecutionResult> {
        let start_time = std::time::Instant::now();

        // Create command with safety measures
        let mut cmd = Command::new(&request.command);
        cmd.args(&request.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null());

        // Set environment variables for safety
        cmd.env("DEBIAN_FRONTEND", "noninteractive");
        cmd.env("NEEDRESTART_MODE", "a");

        // Execute with timeout
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(300), // 5 minute timeout
            cmd.output()
        ).await
        .context("Command execution timed out")?
        .context("Failed to execute command")?;

        let execution_time = start_time.elapsed();

        // Analyze side effects
        let side_effects = self.analyze_side_effects(request, &output).await?;

        Ok(ExecutionResult {
            id: request.id,
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            execution_time,
            side_effects,
        })
    }

    async fn request_user_approval(&self, request: &ExecutionRequest) -> Result<bool> {
        use dialoguer::Confirm;

        println!("\n🤖 TuxPilot wants to execute a command:");
        println!("📝 Description: {}", request.description);
        println!("⚠️  Risk Level: {:?}", request.risk_level);
        println!("🔧 Command: {} {}", request.command, request.args.join(" "));
        println!("🎯 Expected: {}", request.context.expected_outcome);
        
        if let Some(rollback) = &request.context.rollback_plan {
            println!("🔄 Rollback: {}", rollback);
        }

        let approved = Confirm::new()
            .with_prompt("Do you want to execute this command?")
            .default(false)
            .interact()?;

        Ok(approved)
    }

    async fn analyze_side_effects(&self, request: &ExecutionRequest, _output: &std::process::Output) -> Result<Vec<SideEffect>> {
        let mut side_effects = Vec::new();

        // Analyze based on command type
        match request.command.as_str() {
            "pacman" | "apt" | "dnf" | "zypper" => {
                if request.args.contains(&"-S".to_string()) || request.args.contains(&"install".to_string()) {
                    side_effects.push(SideEffect {
                        effect_type: SideEffectType::PackageInstalled,
                        description: format!("Package installation: {}", request.args.join(" ")),
                        reversible: true,
                        rollback_command: Some(self.generate_package_removal_command(request)),
                    });
                }
            }
            "systemctl" => {
                if request.args.contains(&"start".to_string()) {
                    side_effects.push(SideEffect {
                        effect_type: SideEffectType::ServiceStarted,
                        description: format!("Service started: {}", request.args.join(" ")),
                        reversible: true,
                        rollback_command: Some(format!("systemctl stop {}", request.args.last().unwrap_or(&String::new()))),
                    });
                }
            }
            "cp" | "mv" | "mkdir" => {
                side_effects.push(SideEffect {
                    effect_type: SideEffectType::FileCreated,
                    description: format!("File operation: {}", request.args.join(" ")),
                    reversible: false, // Complex to determine
                    rollback_command: None,
                });
            }
            _ => {}
        }

        Ok(side_effects)
    }

    fn generate_package_removal_command(&self, request: &ExecutionRequest) -> String {
        match self.config.system.package_manager {
            crate::config::PackageManager::Pacman => {
                format!("pacman -R {}", request.args.last().unwrap_or(&String::new()))
            }
            crate::config::PackageManager::Apt => {
                format!("apt remove {}", request.args.last().unwrap_or(&String::new()))
            }
            crate::config::PackageManager::Dnf => {
                format!("dnf remove {}", request.args.last().unwrap_or(&String::new()))
            }
            _ => format!("# Manual removal required for {}", request.args.join(" ")),
        }
    }

    pub async fn rollback_execution(&mut self, execution_id: Uuid) -> Result<()> {
        // Implementation for rolling back executed commands
        let execution_record = self.audit_logger.get_execution(execution_id).await?;
        
        for side_effect in execution_record.result.side_effects {
            if side_effect.reversible {
                if let Some(rollback_cmd) = side_effect.rollback_command {
                    println!("🔄 Rolling back: {}", rollback_cmd);
                    // Execute rollback command
                    let parts: Vec<&str> = rollback_cmd.split_whitespace().collect();
                    if let Some((cmd, args)) = parts.split_first() {
                        let rollback_request = ExecutionRequest {
                            id: Uuid::new_v4(),
                            command: cmd.to_string(),
                            args: args.iter().map(|s| s.to_string()).collect(),
                            description: format!("Rollback: {}", side_effect.description),
                            risk_level: RiskLevel::Medium,
                            required_permissions: vec![Permission::WriteSystem],
                            context: ExecutionContext {
                                user_request: "Rollback operation".to_string(),
                                ai_reasoning: "Reversing previous command".to_string(),
                                expected_outcome: "System restored to previous state".to_string(),
                                rollback_plan: None,
                            },
                        };
                        
                        self.execute_request(rollback_request).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
}
