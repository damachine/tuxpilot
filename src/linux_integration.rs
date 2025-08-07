use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command as AsyncCommand;

use crate::config::{Config, PackageManager, ServiceManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionInfo {
    pub name: String,
    pub version: String,
    pub id: String,
    pub id_like: Vec<String>,
    pub package_manager: PackageManager,
    pub service_manager: ServiceManager,
    pub init_system: String,
    pub shell: String,
    pub architecture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandContext {
    pub distribution: DistributionInfo,
    pub user_request: String,
    pub operation_type: OperationType,
    pub safety_level: SafetyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    PackageManagement,
    ServiceManagement,
    FileOperations,
    SystemConfiguration,
    NetworkOperations,
    UserManagement,
    SystemInformation,
    Troubleshooting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    Moderate,
    Risky,
    Dangerous,
}

#[derive(Debug, Clone)]
pub struct LinuxIntegration {
    pub config: Config,
    pub distribution_info: Option<DistributionInfo>,
}

impl LinuxIntegration {
    pub async fn new(config: &Config) -> Result<Self> {
        let mut integration = Self {
            config: config.clone(),
            distribution_info: None,
        };

        // Detect distribution on initialization
        integration.distribution_info = Some(integration.detect_distribution().await?);

        Ok(integration)
    }

    /// Comprehensive Linux distribution detection
    pub async fn detect_distribution(&self) -> Result<DistributionInfo> {
        let mut distro_info = HashMap::new();

        // Read /etc/os-release (standard)
        if let Ok(content) = tokio::fs::read_to_string("/etc/os-release").await {
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    let value = value.trim_matches('"');
                    distro_info.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Fallback: try other detection methods
        if distro_info.is_empty() {
            distro_info = self.detect_distribution_fallback().await?;
        }

        let name = distro_info.get("PRETTY_NAME")
            .or_else(|| distro_info.get("NAME"))
            .unwrap_or(&"Unknown Linux".to_string())
            .clone();

        let version = distro_info.get("VERSION")
            .or_else(|| distro_info.get("VERSION_ID"))
            .unwrap_or(&"Unknown".to_string())
            .clone();

        let id = distro_info.get("ID")
            .unwrap_or(&"unknown".to_string())
            .clone();

        let id_like = distro_info.get("ID_LIKE")
            .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
            .unwrap_or_else(Vec::new);

        // Detect package manager based on distribution
        let package_manager = self.detect_package_manager(&id, &id_like).await?;

        // Detect service manager
        let service_manager = self.detect_service_manager().await?;

        // Detect init system
        let init_system = self.detect_init_system().await?;

        // Detect shell
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

        // Detect architecture
        let architecture = self.execute_command("uname", &["-m"]).await
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string();

        Ok(DistributionInfo {
            name,
            version,
            id,
            id_like,
            package_manager,
            service_manager,
            init_system,
            shell,
            architecture,
        })
    }

    async fn detect_distribution_fallback(&self) -> Result<HashMap<String, String>> {
        let mut info = HashMap::new();

        // Try different distribution-specific files
        let detection_files = vec![
            ("/etc/arch-release", "arch", "Arch Linux"),
            ("/etc/debian_version", "debian", "Debian"),
            ("/etc/ubuntu-release", "ubuntu", "Ubuntu"),
            ("/etc/fedora-release", "fedora", "Fedora"),
            ("/etc/centos-release", "centos", "CentOS"),
            ("/etc/redhat-release", "rhel", "Red Hat Enterprise Linux"),
            ("/etc/opensuse-release", "opensuse", "openSUSE"),
            ("/etc/gentoo-release", "gentoo", "Gentoo"),
        ];

        for (file_path, id, name) in detection_files {
            if tokio::fs::metadata(file_path).await.is_ok() {
                info.insert("ID".to_string(), id.to_string());
                info.insert("NAME".to_string(), name.to_string());
                info.insert("PRETTY_NAME".to_string(), name.to_string());

                // Try to read version from file
                if let Ok(content) = tokio::fs::read_to_string(file_path).await {
                    let version = content.lines().next().unwrap_or("Unknown").trim();
                    info.insert("VERSION".to_string(), version.to_string());
                }
                break;
            }
        }

        // Last resort: use uname
        if info.is_empty() {
            if let Ok(uname_output) = self.execute_command("uname", &["-a"]).await {
                info.insert("NAME".to_string(), "Generic Linux".to_string());
                info.insert("ID".to_string(), "linux".to_string());
                info.insert("VERSION".to_string(), uname_output.trim().to_string());
            }
        }

        Ok(info)
    }

    async fn detect_package_manager(&self, id: &str, id_like: &[String]) -> Result<PackageManager> {
        // Check for package manager binaries and distribution
        let package_managers = vec![
            ("pacman", PackageManager::Pacman),
            ("apt", PackageManager::Apt),
            ("dnf", PackageManager::Dnf),
            ("zypper", PackageManager::Zypper),
            ("emerge", PackageManager::Portage),
        ];

        // First, try distribution-based detection
        match id {
            "arch" | "manjaro" | "endeavouros" => return Ok(PackageManager::Pacman),
            "ubuntu" | "debian" | "linuxmint" | "pop" => return Ok(PackageManager::Apt),
            "fedora" | "centos" | "rhel" | "almalinux" | "rocky" => return Ok(PackageManager::Dnf),
            "opensuse" | "sles" => return Ok(PackageManager::Zypper),
            "gentoo" => return Ok(PackageManager::Portage),
            _ => {}
        }

        // Check ID_LIKE for family-based detection
        for like in id_like {
            match like.as_str() {
                "arch" => return Ok(PackageManager::Pacman),
                "debian" => return Ok(PackageManager::Apt),
                "fedora" | "rhel" => return Ok(PackageManager::Dnf),
                "suse" => return Ok(PackageManager::Zypper),
                _ => {}
            }
        }

        // Fallback: check for available package manager binaries
        for (cmd, pm) in package_managers {
            if self.command_exists(cmd).await {
                return Ok(pm);
            }
        }

        // Default fallback
        Ok(PackageManager::Apt)
    }

    async fn detect_service_manager(&self) -> Result<ServiceManager> {
        // Check for systemd first (most common)
        if self.command_exists("systemctl").await {
            return Ok(ServiceManager::Systemd);
        }

        // Check for other service managers
        if self.command_exists("rc-service").await {
            return Ok(ServiceManager::OpenRC);
        }

        if self.command_exists("service").await {
            return Ok(ServiceManager::SysVInit);
        }

        // Default to systemd
        Ok(ServiceManager::Systemd)
    }

    async fn detect_init_system(&self) -> Result<String> {
        // Check if systemd is running
        if tokio::fs::metadata("/run/systemd/system").await.is_ok() {
            return Ok("systemd".to_string());
        }

        // Check for other init systems
        if let Ok(output) = self.execute_command("ps", &["-p", "1", "-o", "comm="]).await {
            let init_name = output.trim();
            return Ok(init_name.to_string());
        }

        Ok("unknown".to_string())
    }

    async fn command_exists(&self, command: &str) -> bool {
        self.execute_command("which", &[command]).await.is_ok()
    }

    /// Generate commands using LLM with distribution-specific context
    pub async fn generate_commands_for_request(&self, user_request: &str, ai_client: &crate::ai::AiClient) -> Result<Vec<String>> {
        let distro = self.distribution_info.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Distribution info not available"))?;

        let operation_type = self.classify_operation(user_request);
        let safety_level = self.assess_safety_level(user_request);

        let context = CommandContext {
            distribution: distro.clone(),
            user_request: user_request.to_string(),
            operation_type,
            safety_level,
        };

        let prompt = self.build_command_generation_prompt(&context);

        // Get response from LLM
        let response = ai_client.process_query(&prompt).await?;

        // Parse commands from response
        let commands = self.parse_commands_from_response(&response)?;

        Ok(commands)
    }

    fn classify_operation(&self, request: &str) -> OperationType {
        let request_lower = request.to_lowercase();

        if request_lower.contains("install") || request_lower.contains("remove") ||
           request_lower.contains("update") || request_lower.contains("upgrade") ||
           request_lower.contains("package") {
            OperationType::PackageManagement
        } else if request_lower.contains("service") || request_lower.contains("start") ||
                  request_lower.contains("stop") || request_lower.contains("restart") ||
                  request_lower.contains("enable") || request_lower.contains("disable") {
            OperationType::ServiceManagement
        } else if request_lower.contains("file") || request_lower.contains("directory") ||
                  request_lower.contains("copy") || request_lower.contains("move") ||
                  request_lower.contains("delete") || request_lower.contains("permission") {
            OperationType::FileOperations
        } else if request_lower.contains("network") || request_lower.contains("firewall") ||
                  request_lower.contains("port") || request_lower.contains("connection") {
            OperationType::NetworkOperations
        } else if request_lower.contains("user") || request_lower.contains("group") ||
                  request_lower.contains("password") || request_lower.contains("sudo") {
            OperationType::UserManagement
        } else if request_lower.contains("config") || request_lower.contains("setting") ||
                  request_lower.contains("kernel") || request_lower.contains("mount") {
            OperationType::SystemConfiguration
        } else if request_lower.contains("info") || request_lower.contains("status") ||
                  request_lower.contains("show") || request_lower.contains("list") ||
                  request_lower.contains("check") {
            OperationType::SystemInformation
        } else {
            OperationType::Troubleshooting
        }
    }

    fn assess_safety_level(&self, request: &str) -> SafetyLevel {
        let request_lower = request.to_lowercase();

        // Dangerous operations
        if request_lower.contains("delete") || request_lower.contains("remove") ||
           request_lower.contains("format") || request_lower.contains("wipe") ||
           request_lower.contains("destroy") {
            SafetyLevel::Dangerous
        }
        // Risky operations
        else if request_lower.contains("modify") || request_lower.contains("change") ||
                request_lower.contains("edit") || request_lower.contains("configure") ||
                request_lower.contains("install") || request_lower.contains("uninstall") {
            SafetyLevel::Risky
        }
        // Moderate operations
        else if request_lower.contains("start") || request_lower.contains("stop") ||
                request_lower.contains("restart") || request_lower.contains("enable") ||
                request_lower.contains("disable") {
            SafetyLevel::Moderate
        }
        // Safe operations
        else {
            SafetyLevel::Safe
        }
    }

    fn build_command_generation_prompt(&self, context: &CommandContext) -> String {
        let distro = &context.distribution;

        format!(r#"You are an expert Linux system administrator. Generate the appropriate commands for the following request.

SYSTEM INFORMATION:
- Distribution: {} ({})
- Version: {}
- Package Manager: {:?}
- Service Manager: {:?}
- Init System: {}
- Architecture: {}
- Shell: {}

USER REQUEST: {}

OPERATION TYPE: {:?}
SAFETY LEVEL: {:?}

INSTRUCTIONS:
1. Generate ONLY the specific commands needed for this distribution
2. Use the correct package manager: {}
3. Use the correct service manager: {}
4. Consider the safety level and provide appropriate warnings
5. Return commands in this format:
   COMMAND: <command>
   DESCRIPTION: <what this command does>
   RISK: <Safe|Moderate|Risky|Dangerous>

6. For {} distribution, remember:
   - Package manager: {}
   - Service commands: {}
   - Configuration paths: {}

EXAMPLE FORMAT:
COMMAND: sudo pacman -S nginx
DESCRIPTION: Install nginx web server using pacman
RISK: Moderate

COMMAND: sudo systemctl enable --now nginx
DESCRIPTION: Enable and start nginx service
RISK: Moderate

Generate the commands now:"#,
            distro.name, distro.id,
            distro.version,
            distro.package_manager,
            distro.service_manager,
            distro.init_system,
            distro.architecture,
            distro.shell,
            context.user_request,
            context.operation_type,
            context.safety_level,
            self.get_package_manager_name(&distro.package_manager),
            self.get_service_manager_name(&distro.service_manager),
            distro.name,
            self.get_package_manager_name(&distro.package_manager),
            self.get_service_commands(&distro.service_manager),
            self.get_config_paths(&distro.id)
        )
    }

    fn get_package_manager_name(&self, pm: &PackageManager) -> &str {
        match pm {
            PackageManager::Pacman => "pacman",
            PackageManager::Apt => "apt",
            PackageManager::Dnf => "dnf",
            PackageManager::Zypper => "zypper",
            PackageManager::Portage => "emerge",
        }
    }

    fn get_service_manager_name(&self, sm: &ServiceManager) -> &str {
        match sm {
            ServiceManager::Systemd => "systemctl",
            ServiceManager::OpenRC => "rc-service",
            ServiceManager::SysVInit => "service",
        }
    }

    fn get_service_commands(&self, sm: &ServiceManager) -> &str {
        match sm {
            ServiceManager::Systemd => "systemctl start/stop/restart/enable/disable",
            ServiceManager::OpenRC => "rc-service start/stop/restart",
            ServiceManager::SysVInit => "service start/stop/restart",
        }
    }

    fn get_config_paths(&self, distro_id: &str) -> &str {
        match distro_id {
            "arch" | "manjaro" => "/etc, /usr/share, ~/.config",
            "ubuntu" | "debian" => "/etc, /usr/share, ~/.config",
            "fedora" | "centos" | "rhel" => "/etc, /usr/share, ~/.config",
            "opensuse" => "/etc, /usr/share, ~/.config",
            _ => "/etc, /usr/share, ~/.config",
        }
    }

    fn parse_commands_from_response(&self, response: &str) -> Result<Vec<String>> {
        let mut commands = Vec::new();

        for line in response.lines() {
            if line.starts_with("COMMAND:") {
                let command = line.replace("COMMAND:", "").trim().to_string();
                if !command.is_empty() {
                    commands.push(command);
                }
            }
        }

        // Fallback: if no COMMAND: format found, try to extract commands differently
        if commands.is_empty() {
            // Look for common command patterns
            for line in response.lines() {
                let line = line.trim();
                if line.starts_with("sudo ") || line.starts_with("pacman ") ||
                   line.starts_with("apt ") || line.starts_with("dnf ") ||
                   line.starts_with("systemctl ") || line.starts_with("service ") {
                    commands.push(line.to_string());
                }
            }
        }

        Ok(commands)
    }

    pub async fn get_system_info(&self) -> Result<String> {
        let mut info = Vec::new();
        
        // OS Information
        if let Ok(os_info) = self.execute_command("uname", &["-a"]).await {
            info.push(format!("System: {}", os_info.trim()));
        }
        
        // Distribution info
        if let Ok(distro) = tokio::fs::read_to_string("/etc/os-release").await {
            if let Some(name) = distro.lines().find(|line| line.starts_with("PRETTY_NAME=")) {
                let name = name.replace("PRETTY_NAME=", "").trim_matches('"').to_string();
                info.push(format!("Distribution: {}", name));
            }
        }
        
        // Uptime
        if let Ok(uptime) = self.execute_command("uptime", &["-p"]).await {
            info.push(format!("Uptime: {}", uptime.trim()));
        }
        
        // Memory info
        if let Ok(memory) = self.execute_command("free", &["-h"]).await {
            if let Some(mem_line) = memory.lines().nth(1) {
                info.push(format!("Memory: {}", mem_line));
            }
        }
        
        // Disk usage
        if let Ok(disk) = self.execute_command("df", &["-h", "/"]).await {
            if let Some(disk_line) = disk.lines().nth(1) {
                info.push(format!("Root disk: {}", disk_line));
            }
        }
        
        Ok(info.join("\n"))
    }

    pub async fn get_package_suggestion(&self, operation: &str, package: Option<&str>) -> Result<String> {
        match self.config.system.package_manager {
            PackageManager::Pacman => self.get_pacman_suggestion(operation, package),
            PackageManager::Apt => self.get_apt_suggestion(operation, package),
            PackageManager::Dnf => self.get_dnf_suggestion(operation, package),
            PackageManager::Zypper => self.get_zypper_suggestion(operation, package),
            PackageManager::Portage => self.get_portage_suggestion(operation, package),
        }
    }

    pub async fn execute_package_command(&self, command: &str) -> Result<String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("Empty command"));
        }
        
        let (cmd, args) = parts.split_first().unwrap();
        self.execute_command_with_sudo(cmd, args).await
    }

    pub async fn get_service_info(&self, service_name: &str) -> Result<String> {
        match self.config.system.service_manager {
            ServiceManager::Systemd => {
                let status = self.execute_command("systemctl", &["status", service_name]).await
                    .unwrap_or_else(|_| "Service not found or not accessible".to_string());
                Ok(status)
            }
            ServiceManager::OpenRC => {
                let status = self.execute_command("rc-service", &[service_name, "status"]).await
                    .unwrap_or_else(|_| "Service not found or not accessible".to_string());
                Ok(status)
            }
            ServiceManager::SysVInit => {
                let status = self.execute_command("service", &[service_name, "status"]).await
                    .unwrap_or_else(|_| "Service not found or not accessible".to_string());
                Ok(status)
            }
        }
    }

    pub async fn execute_command(&self, command: &str, args: &[&str]) -> Result<String> {
        let output = AsyncCommand::new(command)
            .args(args)
            .output()
            .await
            .context(format!("Failed to execute command: {} {:?}", command, args))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Command failed: {}", stderr))
        }
    }

    async fn execute_command_with_sudo(&self, command: &str, args: &[&str]) -> Result<String> {
        let mut sudo_args = vec!["sudo", command];
        sudo_args.extend(args);
        
        let output = AsyncCommand::new("sudo")
            .args(&sudo_args[1..])
            .output()
            .await
            .context(format!("Failed to execute sudo command: {} {:?}", command, args))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Sudo command failed: {}", stderr))
        }
    }

    fn get_pacman_suggestion(&self, operation: &str, package: Option<&str>) -> Result<String> {
        let cmd = match operation.to_lowercase().as_str() {
            "install" | "add" => {
                if let Some(pkg) = package {
                    format!("sudo pacman -S {}", pkg)
                } else {
                    "sudo pacman -S <package_name>".to_string()
                }
            }
            "remove" | "uninstall" => {
                if let Some(pkg) = package {
                    format!("sudo pacman -R {}", pkg)
                } else {
                    "sudo pacman -R <package_name>".to_string()
                }
            }
            "update" | "upgrade" => "sudo pacman -Syu".to_string(),
            "search" => {
                if let Some(pkg) = package {
                    format!("pacman -Ss {}", pkg)
                } else {
                    "pacman -Ss <search_term>".to_string()
                }
            }
            "info" => {
                if let Some(pkg) = package {
                    format!("pacman -Si {}", pkg)
                } else {
                    "pacman -Si <package_name>".to_string()
                }
            }
            "list" => "pacman -Q".to_string(),
            "clean" => "sudo pacman -Sc".to_string(),
            _ => format!("Unknown operation: {}. Try: install, remove, update, search, info, list, clean", operation),
        };
        Ok(cmd)
    }

    fn get_apt_suggestion(&self, operation: &str, package: Option<&str>) -> Result<String> {
        let cmd = match operation.to_lowercase().as_str() {
            "install" | "add" => {
                if let Some(pkg) = package {
                    format!("sudo apt install {}", pkg)
                } else {
                    "sudo apt install <package_name>".to_string()
                }
            }
            "remove" | "uninstall" => {
                if let Some(pkg) = package {
                    format!("sudo apt remove {}", pkg)
                } else {
                    "sudo apt remove <package_name>".to_string()
                }
            }
            "update" => "sudo apt update".to_string(),
            "upgrade" => "sudo apt update && sudo apt upgrade".to_string(),
            "search" => {
                if let Some(pkg) = package {
                    format!("apt search {}", pkg)
                } else {
                    "apt search <search_term>".to_string()
                }
            }
            "info" => {
                if let Some(pkg) = package {
                    format!("apt show {}", pkg)
                } else {
                    "apt show <package_name>".to_string()
                }
            }
            "list" => "apt list --installed".to_string(),
            "clean" => "sudo apt autoremove && sudo apt autoclean".to_string(),
            _ => format!("Unknown operation: {}. Try: install, remove, update, upgrade, search, info, list, clean", operation),
        };
        Ok(cmd)
    }

    fn get_dnf_suggestion(&self, operation: &str, package: Option<&str>) -> Result<String> {
        let cmd = match operation.to_lowercase().as_str() {
            "install" | "add" => {
                if let Some(pkg) = package {
                    format!("sudo dnf install {}", pkg)
                } else {
                    "sudo dnf install <package_name>".to_string()
                }
            }
            "remove" | "uninstall" => {
                if let Some(pkg) = package {
                    format!("sudo dnf remove {}", pkg)
                } else {
                    "sudo dnf remove <package_name>".to_string()
                }
            }
            "update" | "upgrade" => "sudo dnf upgrade".to_string(),
            "search" => {
                if let Some(pkg) = package {
                    format!("dnf search {}", pkg)
                } else {
                    "dnf search <search_term>".to_string()
                }
            }
            "info" => {
                if let Some(pkg) = package {
                    format!("dnf info {}", pkg)
                } else {
                    "dnf info <package_name>".to_string()
                }
            }
            "list" => "dnf list installed".to_string(),
            "clean" => "sudo dnf autoremove && sudo dnf clean all".to_string(),
            _ => format!("Unknown operation: {}. Try: install, remove, update, search, info, list, clean", operation),
        };
        Ok(cmd)
    }

    fn get_zypper_suggestion(&self, operation: &str, package: Option<&str>) -> Result<String> {
        let cmd = match operation.to_lowercase().as_str() {
            "install" | "add" => {
                if let Some(pkg) = package {
                    format!("sudo zypper install {}", pkg)
                } else {
                    "sudo zypper install <package_name>".to_string()
                }
            }
            "remove" | "uninstall" => {
                if let Some(pkg) = package {
                    format!("sudo zypper remove {}", pkg)
                } else {
                    "sudo zypper remove <package_name>".to_string()
                }
            }
            "update" | "upgrade" => "sudo zypper update".to_string(),
            "search" => {
                if let Some(pkg) = package {
                    format!("zypper search {}", pkg)
                } else {
                    "zypper search <search_term>".to_string()
                }
            }
            "info" => {
                if let Some(pkg) = package {
                    format!("zypper info {}", pkg)
                } else {
                    "zypper info <package_name>".to_string()
                }
            }
            "list" => "zypper search --installed-only".to_string(),
            "clean" => "sudo zypper clean".to_string(),
            _ => format!("Unknown operation: {}. Try: install, remove, update, search, info, list, clean", operation),
        };
        Ok(cmd)
    }

    fn get_portage_suggestion(&self, operation: &str, package: Option<&str>) -> Result<String> {
        let cmd = match operation.to_lowercase().as_str() {
            "install" | "add" => {
                if let Some(pkg) = package {
                    format!("sudo emerge {}", pkg)
                } else {
                    "sudo emerge <package_name>".to_string()
                }
            }
            "remove" | "uninstall" => {
                if let Some(pkg) = package {
                    format!("sudo emerge --unmerge {}", pkg)
                } else {
                    "sudo emerge --unmerge <package_name>".to_string()
                }
            }
            "update" | "upgrade" => "sudo emerge --sync && sudo emerge -uDN @world".to_string(),
            "search" => {
                if let Some(pkg) = package {
                    format!("emerge --search {}", pkg)
                } else {
                    "emerge --search <search_term>".to_string()
                }
            }
            "info" => {
                if let Some(pkg) = package {
                    format!("emerge --info {}", pkg)
                } else {
                    "emerge --info <package_name>".to_string()
                }
            }
            "list" => "qlist -I".to_string(),
            "clean" => "sudo emerge --depclean".to_string(),
            _ => format!("Unknown operation: {}. Try: install, remove, update, search, info, list, clean", operation),
        };
        Ok(cmd)
    }
}
