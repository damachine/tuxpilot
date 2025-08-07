use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Permission;
use crate::config::Config;

/// Authentication manager for web interface
#[derive(Debug, Clone)]
pub struct AuthManager {
    users: HashMap<String, User>,
    api_keys: HashMap<String, ApiKey>,
    config: Config,
}

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub permissions: Vec<Permission>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub enabled: bool,
    pub role: UserRole,
}

/// User roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Operator,
    Viewer,
    Guest,
}

/// API key for programmatic access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key_id: String,
    pub key_hash: String,
    pub user_id: String,
    pub permissions: Vec<Permission>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub enabled: bool,
}

/// Authentication result
#[derive(Debug, Clone)]
pub enum AuthResult {
    Success(String), // user_id
    InvalidCredentials,
    AccountDisabled,
    PermissionDenied,
    TokenExpired,
}

impl AuthManager {
    pub async fn new(config: &Config) -> Result<Self> {
        let mut auth_manager = Self {
            users: HashMap::new(),
            api_keys: HashMap::new(),
            config: config.clone(),
        };

        // Create default admin user if none exists
        auth_manager.create_default_admin().await?;

        Ok(auth_manager)
    }

    async fn create_default_admin(&mut self) -> Result<()> {
        let admin_user = User {
            user_id: "admin".to_string(),
            username: "admin".to_string(),
            email: "admin@localhost".to_string(),
            password_hash: self.hash_password("admin123").await?, // Default password
            permissions: vec![
                Permission::ViewSystem,
                Permission::ExecuteCommands,
                Permission::ManageServices,
                Permission::ViewLogs,
                Permission::ManageUsers,
                Permission::SystemConfiguration,
                Permission::RemoteAccess,
            ],
            created_at: chrono::Utc::now(),
            last_login: None,
            enabled: true,
            role: UserRole::Admin,
        };

        self.users.insert("admin".to_string(), admin_user);
        println!("🔐 Created default admin user (username: admin, password: admin123)");
        println!("⚠️  Please change the default password immediately!");

        Ok(())
    }

    pub async fn authenticate_user(&mut self, username: &str, password: &str) -> Result<AuthResult> {
        if let Some(user) = self.users.get(username) {
            if !user.enabled {
                return Ok(AuthResult::AccountDisabled);
            }

            let password_hash = user.password_hash.clone();
            let user_id = user.user_id.clone();

            if self.verify_password(password, &password_hash).await? {
                if let Some(user) = self.users.get_mut(username) {
                    user.last_login = Some(chrono::Utc::now());
                }
                return Ok(AuthResult::Success(user_id));
            }
        }

        Ok(AuthResult::InvalidCredentials)
    }

    pub async fn authenticate_api_key(&mut self, api_key: &str) -> Result<AuthResult> {
        let key_hash = self.hash_api_key(api_key).await?;

        if let Some(key_info) = self.api_keys.get_mut(&key_hash) {
            if !key_info.enabled {
                return Ok(AuthResult::AccountDisabled);
            }

            if let Some(expires_at) = key_info.expires_at {
                if expires_at < chrono::Utc::now() {
                    return Ok(AuthResult::TokenExpired);
                }
            }

            key_info.last_used = Some(chrono::Utc::now());
            return Ok(AuthResult::Success(key_info.user_id.clone()));
        }

        Ok(AuthResult::InvalidCredentials)
    }

    pub async fn get_user_permissions(&self, user_id: &str) -> Result<Vec<Permission>> {
        if let Some(user) = self.users.get(user_id) {
            Ok(user.permissions.clone())
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn create_user(&mut self, username: String, email: String, password: String, role: UserRole) -> Result<String> {
        let user_id = uuid::Uuid::new_v4().to_string();
        let password_hash = self.hash_password(&password).await?;

        let permissions = match role {
            UserRole::Admin => vec![
                Permission::ViewSystem,
                Permission::ExecuteCommands,
                Permission::ManageServices,
                Permission::ViewLogs,
                Permission::ManageUsers,
                Permission::SystemConfiguration,
                Permission::RemoteAccess,
            ],
            UserRole::Operator => vec![
                Permission::ViewSystem,
                Permission::ExecuteCommands,
                Permission::ManageServices,
                Permission::ViewLogs,
            ],
            UserRole::Viewer => vec![
                Permission::ViewSystem,
                Permission::ViewLogs,
            ],
            UserRole::Guest => vec![
                Permission::ViewSystem,
            ],
        };

        let user = User {
            user_id: user_id.clone(),
            username: username.clone(),
            email,
            password_hash,
            permissions,
            created_at: chrono::Utc::now(),
            last_login: None,
            enabled: true,
            role,
        };

        self.users.insert(username, user);
        Ok(user_id)
    }

    pub async fn create_api_key(&mut self, user_id: String, expires_in_days: Option<u32>) -> Result<String> {
        let api_key = self.generate_api_key().await?;
        let key_hash = self.hash_api_key(&api_key).await?;
        let key_id = uuid::Uuid::new_v4().to_string();

        let expires_at = expires_in_days.map(|days| {
            chrono::Utc::now() + chrono::Duration::days(days as i64)
        });

        let permissions = self.get_user_permissions(&user_id).await?;

        let api_key_info = ApiKey {
            key_id,
            key_hash: key_hash.clone(),
            user_id,
            permissions,
            created_at: chrono::Utc::now(),
            expires_at,
            last_used: None,
            enabled: true,
        };

        self.api_keys.insert(key_hash, api_key_info);
        Ok(api_key)
    }

    pub async fn revoke_api_key(&mut self, key_id: &str) -> Result<()> {
        for (_, key_info) in self.api_keys.iter_mut() {
            if key_info.key_id == key_id {
                key_info.enabled = false;
                break;
            }
        }
        Ok(())
    }

    pub async fn change_password(&mut self, user_id: &str, old_password: &str, new_password: &str) -> Result<bool> {
        // First, find the user and get the password hash
        let old_hash = if let Some(user) = self.users.values().find(|u| u.user_id == user_id) {
            user.password_hash.clone()
        } else {
            return Ok(false);
        };

        // Verify the old password
        if self.verify_password(old_password, &old_hash).await? {
            // Hash the new password
            let new_hash = self.hash_password(new_password).await?;

            // Update the password
            if let Some(user) = self.users.values_mut().find(|u| u.user_id == user_id) {
                user.password_hash = new_hash;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub async fn list_api_keys(&self, user_id: &str) -> Vec<&ApiKey> {
        self.api_keys.values()
            .filter(|key| key.user_id == user_id)
            .collect()
    }

    async fn hash_password(&self, password: &str) -> Result<String> {
        // In a real implementation, use bcrypt or similar
        Ok(format!("hashed_{}", password))
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        // In a real implementation, use bcrypt verification
        Ok(hash == &format!("hashed_{}", password))
    }

    async fn generate_api_key(&self) -> Result<String> {
        Ok(format!("tuxpilot_{}", uuid::Uuid::new_v4().to_string().replace('-', "")))
    }

    async fn hash_api_key(&self, api_key: &str) -> Result<String> {
        // In a real implementation, use a proper hash function
        Ok(format!("key_hash_{}", api_key))
    }
}
