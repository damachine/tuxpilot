use anyhow::{Context, Result};
use tokio::process::Command as AsyncCommand;

use crate::config::{Config, PackageManager, ServiceManager};

pub struct LinuxIntegration {
    pub config: Config,
}

impl LinuxIntegration {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
        })
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
