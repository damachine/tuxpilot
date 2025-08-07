use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod loader;
pub mod registry;
pub mod api;
pub mod builtin;

use crate::config::Config;
use crate::linux_integration::LinuxIntegration;

/// Plugin system manager
pub struct PluginSystem {
    config: Config,
    registry: registry::PluginRegistry,
    loader: loader::PluginLoader,
    loaded_plugins: HashMap<String, Box<dyn Plugin>>,
    plugin_api: api::PluginApi,
}

/// Plugin trait that all plugins must implement
#[async_trait::async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;
    
    /// Initialize the plugin
    async fn initialize(&mut self, api: &api::PluginApi) -> Result<()>;
    
    /// Execute plugin functionality
    async fn execute(&self, request: PluginRequest) -> Result<PluginResponse>;
    
    /// Get plugin capabilities
    fn capabilities(&self) -> Vec<PluginCapability>;
    
    /// Check if plugin can handle a specific request
    fn can_handle(&self, request: &PluginRequest) -> bool;
    
    /// Shutdown the plugin
    async fn shutdown(&mut self) -> Result<()>;
    
    /// Get plugin status
    async fn status(&self) -> Result<PluginStatus>;
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub supported_platforms: Vec<String>,
    pub min_tuxpilot_version: String,
    pub max_tuxpilot_version: Option<String>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version_requirement: String,
    pub optional: bool,
}

/// Plugin capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
    CommandExecution,
    SystemMonitoring,
    FileManagement,
    NetworkOperations,
    ServiceManagement,
    PackageManagement,
    SecurityScanning,
    LogAnalysis,
    BackupOperations,
    ConfigurationManagement,
    UserInterface,
    ApiExtension,
}

/// Plugin request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRequest {
    pub request_id: String,
    pub plugin_id: String,
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub context: PluginContext,
}

/// Plugin execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub system_info: SystemInfo,
    pub permissions: Vec<String>,
}

/// System information for plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub distribution: String,
    pub kernel_version: String,
    pub architecture: String,
    pub hostname: String,
    pub uptime_seconds: u64,
}

/// Plugin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResponse {
    pub request_id: String,
    pub success: bool,
    pub data: serde_json::Value,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
    pub side_effects: Vec<SideEffect>,
}

/// Side effects of plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect_type: String,
    pub description: String,
    pub reversible: bool,
    pub rollback_data: Option<serde_json::Value>,
}

/// Plugin status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStatus {
    pub plugin_id: String,
    pub state: PluginState,
    pub health: PluginHealth,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub error_count: u32,
    pub execution_count: u64,
    pub average_execution_time_ms: f64,
}

/// Plugin states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginState {
    Unloaded,
    Loading,
    Loaded,
    Active,
    Error,
    Disabled,
}

/// Plugin health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginHealth {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub auto_load: bool,
    pub settings: HashMap<String, serde_json::Value>,
    pub permissions: Vec<String>,
    pub resource_limits: ResourceLimits,
}

/// Resource limits for plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: Option<u64>,
    pub max_cpu_percent: Option<f32>,
    pub max_execution_time_ms: Option<u64>,
    pub max_file_operations: Option<u32>,
    pub max_network_requests: Option<u32>,
}

impl PluginSystem {
    pub async fn new(config: Config, linux_integration: LinuxIntegration) -> Result<Self> {
        let registry = registry::PluginRegistry::new().await?;
        let loader = loader::PluginLoader::new().await?;
        let plugin_api = api::PluginApi::new(config.clone(), linux_integration).await?;

        let mut system = Self {
            config,
            registry,
            loader,
            loaded_plugins: HashMap::new(),
            plugin_api,
        };

        // Load built-in plugins
        system.load_builtin_plugins().await?;

        Ok(system)
    }

    async fn load_builtin_plugins(&mut self) -> Result<()> {
        // Load built-in plugins
        let docker_plugin = Box::new(builtin::DockerPlugin::new());
        self.register_plugin(docker_plugin).await?;

        let git_plugin = Box::new(builtin::GitPlugin::new());
        self.register_plugin(git_plugin).await?;

        let monitoring_plugin = Box::new(builtin::MonitoringPlugin::new());
        self.register_plugin(monitoring_plugin).await?;

        Ok(())
    }

    pub async fn register_plugin(&mut self, mut plugin: Box<dyn Plugin>) -> Result<()> {
        let plugin_id = plugin.metadata().id.clone();
        
        // Initialize plugin
        plugin.initialize(&self.plugin_api).await?;
        
        // Register in registry
        self.registry.register_plugin(plugin.metadata().clone()).await?;
        
        // Store loaded plugin
        self.loaded_plugins.insert(plugin_id.clone(), plugin);
        
        println!("🔌 Registered plugin: {}", plugin_id);
        Ok(())
    }

    pub async fn load_plugin_from_file(&mut self, plugin_path: PathBuf) -> Result<()> {
        let plugin = self.loader.load_plugin(plugin_path).await?;
        self.register_plugin(plugin).await?;
        Ok(())
    }

    pub async fn execute_plugin_request(&self, request: PluginRequest) -> Result<PluginResponse> {
        if let Some(plugin) = self.loaded_plugins.get(&request.plugin_id) {
            if plugin.can_handle(&request) {
                return plugin.execute(request).await;
            }
        }

        Err(anyhow::anyhow!("Plugin not found or cannot handle request: {}", request.plugin_id))
    }

    pub async fn list_plugins(&self) -> Vec<&PluginMetadata> {
        self.loaded_plugins.values()
            .map(|plugin| plugin.metadata())
            .collect()
    }

    pub async fn get_plugin_status(&self, plugin_id: &str) -> Result<PluginStatus> {
        if let Some(plugin) = self.loaded_plugins.get(plugin_id) {
            plugin.status().await
        } else {
            Err(anyhow::anyhow!("Plugin not found: {}", plugin_id))
        }
    }

    pub async fn enable_plugin(&mut self, plugin_id: &str) -> Result<()> {
        // Implementation would enable a disabled plugin
        println!("✅ Enabled plugin: {}", plugin_id);
        Ok(())
    }

    pub async fn disable_plugin(&mut self, plugin_id: &str) -> Result<()> {
        // Implementation would disable an active plugin
        println!("❌ Disabled plugin: {}", plugin_id);
        Ok(())
    }

    pub async fn unload_plugin(&mut self, plugin_id: &str) -> Result<()> {
        if let Some(mut plugin) = self.loaded_plugins.remove(plugin_id) {
            plugin.shutdown().await?;
            self.registry.unregister_plugin(plugin_id).await?;
            println!("🔌 Unloaded plugin: {}", plugin_id);
        }
        Ok(())
    }

    pub async fn reload_plugin(&mut self, plugin_id: &str) -> Result<()> {
        // Get plugin path from registry
        if let Some(plugin_info) = self.registry.get_plugin_info(plugin_id).await? {
            // Unload current plugin
            self.unload_plugin(plugin_id).await?;
            
            // Reload from file
            if let Some(path) = plugin_info.file_path {
                self.load_plugin_from_file(path).await?;
                println!("🔄 Reloaded plugin: {}", plugin_id);
            }
        }
        Ok(())
    }

    pub async fn get_plugins_by_capability(&self, capability: PluginCapability) -> Vec<&str> {
        self.loaded_plugins.values()
            .filter(|plugin| plugin.capabilities().contains(&capability))
            .map(|plugin| plugin.metadata().id.as_str())
            .collect()
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        println!("🔄 Shutting down plugin system...");
        
        for (plugin_id, plugin) in &mut self.loaded_plugins {
            println!("🔄 Shutting down plugin: {}", plugin_id);
            if let Err(e) = plugin.shutdown().await {
                eprintln!("⚠️ Error shutting down plugin {}: {}", plugin_id, e);
            }
        }

        self.loaded_plugins.clear();
        println!("✅ Plugin system shutdown complete");
        Ok(())
    }
}
