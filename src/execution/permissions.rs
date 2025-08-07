use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

use crate::config::Config;
use super::Permission;

#[derive(Debug, Clone)]
pub struct PermissionManager {
    granted_permissions: HashSet<Permission>,
    user_permissions: UserPermissions,
    system_permissions: SystemPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    pub is_root: bool,
    pub is_sudo_user: bool,
    pub groups: Vec<String>,
    pub uid: u32,
    pub gid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPermissions {
    pub can_install_packages: bool,
    pub can_manage_services: bool,
    pub can_modify_system_files: bool,
    pub can_create_users: bool,
    pub can_access_network: bool,
}

impl PermissionManager {
    pub async fn new(_config: &Config) -> Result<Self> {
        let user_permissions = Self::detect_user_permissions().await?;
        let system_permissions = Self::detect_system_permissions(&user_permissions).await?;
        let granted_permissions = Self::calculate_granted_permissions(&user_permissions, &system_permissions);

        Ok(Self {
            granted_permissions,
            user_permissions,
            system_permissions,
        })
    }

    async fn detect_user_permissions() -> Result<UserPermissions> {
        use std::os::unix::fs::MetadataExt;
        
        // Get current user info
        let uid = unsafe { libc::getuid() };
        let gid = unsafe { libc::getgid() };
        let is_root = uid == 0;

        // Check if user is in sudo group
        let groups_output = tokio::process::Command::new("groups")
            .output()
            .await
            .context("Failed to get user groups")?;
        
        let groups_str = String::from_utf8_lossy(&groups_output.stdout);
        let groups: Vec<String> = groups_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        let is_sudo_user = groups.contains(&"sudo".to_string()) || 
                          groups.contains(&"wheel".to_string()) ||
                          is_root;

        Ok(UserPermissions {
            is_root,
            is_sudo_user,
            groups,
            uid,
            gid,
        })
    }

    async fn detect_system_permissions(user_perms: &UserPermissions) -> Result<SystemPermissions> {
        let can_install_packages = user_perms.is_sudo_user || user_perms.is_root;
        let can_manage_services = user_perms.is_sudo_user || user_perms.is_root;
        let can_modify_system_files = user_perms.is_sudo_user || user_perms.is_root;
        let can_create_users = user_perms.is_root;
        let can_access_network = true; // Most users can access network

        Ok(SystemPermissions {
            can_install_packages,
            can_manage_services,
            can_modify_system_files,
            can_create_users,
            can_access_network,
        })
    }

    fn calculate_granted_permissions(
        user_perms: &UserPermissions,
        system_perms: &SystemPermissions,
    ) -> HashSet<Permission> {
        let mut permissions = HashSet::new();

        // Basic read permissions for all users
        permissions.insert(Permission::ReadSystem);
        permissions.insert(Permission::FileSystemRead);

        if system_perms.can_access_network {
            permissions.insert(Permission::NetworkAccess);
        }

        if user_perms.is_sudo_user || user_perms.is_root {
            permissions.insert(Permission::WriteSystem);
            permissions.insert(Permission::FileSystemWrite);
            permissions.insert(Permission::SystemConfiguration);
        }

        if system_perms.can_install_packages {
            permissions.insert(Permission::PackageManagement);
        }

        if system_perms.can_manage_services {
            permissions.insert(Permission::ServiceManagement);
        }

        if system_perms.can_create_users {
            permissions.insert(Permission::UserManagement);
        }

        permissions
    }

    pub fn check_permissions(&self, required: &[Permission]) -> Result<()> {
        for permission in required {
            if !self.granted_permissions.contains(permission) {
                return Err(anyhow::anyhow!(
                    "Permission denied: {:?}. Required permissions: {:?}",
                    permission,
                    required
                ));
            }
        }
        Ok(())
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.granted_permissions.contains(permission)
    }

    pub fn get_granted_permissions(&self) -> &HashSet<Permission> {
        &self.granted_permissions
    }

    pub fn can_execute_command(&self, command: &str, args: &[String]) -> Result<Vec<Permission>> {
        let mut required_permissions = Vec::new();

        match command {
            // Package managers
            "pacman" | "apt" | "dnf" | "zypper" | "emerge" => {
                required_permissions.push(Permission::ReadSystem);
                
                // Check if it's a write operation
                if args.iter().any(|arg| {
                    matches!(arg.as_str(), "-S" | "install" | "remove" | "upgrade" | "update")
                }) {
                    required_permissions.push(Permission::PackageManagement);
                    required_permissions.push(Permission::WriteSystem);
                }
            }

            // Service management
            "systemctl" | "service" | "rc-service" => {
                required_permissions.push(Permission::ReadSystem);
                
                if args.iter().any(|arg| {
                    matches!(arg.as_str(), "start" | "stop" | "restart" | "enable" | "disable")
                }) {
                    required_permissions.push(Permission::ServiceManagement);
                    required_permissions.push(Permission::WriteSystem);
                }
            }

            // File operations
            "cp" | "mv" | "rm" | "mkdir" | "rmdir" | "chmod" | "chown" => {
                required_permissions.push(Permission::FileSystemWrite);
                required_permissions.push(Permission::WriteSystem);
            }

            "cat" | "ls" | "find" | "grep" | "head" | "tail" => {
                required_permissions.push(Permission::FileSystemRead);
                required_permissions.push(Permission::ReadSystem);
            }

            // User management
            "useradd" | "userdel" | "usermod" | "passwd" => {
                required_permissions.push(Permission::UserManagement);
                required_permissions.push(Permission::WriteSystem);
            }

            // Network operations
            "wget" | "curl" | "ping" | "nslookup" | "dig" => {
                required_permissions.push(Permission::NetworkAccess);
                required_permissions.push(Permission::ReadSystem);
            }

            // System configuration
            "mount" | "umount" | "fdisk" | "parted" => {
                required_permissions.push(Permission::SystemConfiguration);
                required_permissions.push(Permission::WriteSystem);
            }

            // Default: require read access
            _ => {
                required_permissions.push(Permission::ReadSystem);
            }
        }

        // Check if we have the required permissions
        self.check_permissions(&required_permissions)?;
        
        Ok(required_permissions)
    }

    pub fn get_permission_summary(&self) -> PermissionSummary {
        PermissionSummary {
            user_info: self.user_permissions.clone(),
            can_install_packages: self.has_permission(&Permission::PackageManagement),
            can_manage_services: self.has_permission(&Permission::ServiceManagement),
            can_modify_files: self.has_permission(&Permission::FileSystemWrite),
            can_manage_users: self.has_permission(&Permission::UserManagement),
            can_configure_system: self.has_permission(&Permission::SystemConfiguration),
            total_permissions: self.granted_permissions.len(),
        }
    }

    pub async fn request_elevated_permissions(&mut self, permission: Permission) -> Result<bool> {
        use dialoguer::Confirm;

        if self.has_permission(&permission) {
            return Ok(true);
        }

        // Check if elevation is possible
        if !self.user_permissions.is_sudo_user && !self.user_permissions.is_root {
            return Err(anyhow::anyhow!(
                "Cannot elevate permissions: user is not in sudo group"
            ));
        }

        let permission_desc = match permission {
            Permission::PackageManagement => "install/remove packages",
            Permission::ServiceManagement => "manage system services",
            Permission::FileSystemWrite => "modify files and directories",
            Permission::UserManagement => "manage users and groups",
            Permission::SystemConfiguration => "modify system configuration",
            Permission::NetworkAccess => "access network resources",
            _ => "perform system operations",
        };

        let approved = Confirm::new()
            .with_prompt(&format!(
                "TuxPilot needs elevated permissions to {}. Grant permission?",
                permission_desc
            ))
            .default(false)
            .interact()?;

        if approved {
            // Test sudo access
            let sudo_test = tokio::process::Command::new("sudo")
                .args(&["-n", "true"])
                .output()
                .await?;

            if sudo_test.status.success() {
                self.granted_permissions.insert(permission);
                Ok(true)
            } else {
                Err(anyhow::anyhow!("Sudo authentication required"))
            }
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSummary {
    pub user_info: UserPermissions,
    pub can_install_packages: bool,
    pub can_manage_services: bool,
    pub can_modify_files: bool,
    pub can_manage_users: bool,
    pub can_configure_system: bool,
    pub total_permissions: usize,
}

impl std::fmt::Display for PermissionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🔐 TuxPilot Permission Summary")?;
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        writeln!(f, "👤 User: {} (UID: {})", 
                 if self.user_info.is_root { "root" } else { "user" }, 
                 self.user_info.uid)?;
        writeln!(f, "🛡️  Sudo Access: {}", 
                 if self.user_info.is_sudo_user { "✅ Yes" } else { "❌ No" })?;
        writeln!(f, "👥 Groups: {}", self.user_info.groups.join(", "))?;
        writeln!(f)?;
        writeln!(f, "📦 Package Management: {}", 
                 if self.can_install_packages { "✅ Allowed" } else { "❌ Denied" })?;
        writeln!(f, "⚙️  Service Management: {}", 
                 if self.can_manage_services { "✅ Allowed" } else { "❌ Denied" })?;
        writeln!(f, "📁 File Modification: {}", 
                 if self.can_modify_files { "✅ Allowed" } else { "❌ Denied" })?;
        writeln!(f, "👥 User Management: {}", 
                 if self.can_manage_users { "✅ Allowed" } else { "❌ Denied" })?;
        writeln!(f, "🔧 System Configuration: {}", 
                 if self.can_configure_system { "✅ Allowed" } else { "❌ Denied" })?;
        writeln!(f)?;
        writeln!(f, "📊 Total Permissions: {}", self.total_permissions)?;
        
        Ok(())
    }
}
