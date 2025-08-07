use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::PluginMetadata;

/// Plugin registry for managing plugin information
#[derive(Debug)]
pub struct PluginRegistry {
    plugins: HashMap<String, PluginInfo>,
    plugin_configs: HashMap<String, super::PluginConfig>,
}

/// Plugin information stored in registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub metadata: PluginMetadata,
    pub file_path: Option<PathBuf>,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub checksum: Option<String>,
    pub enabled: bool,
}

impl PluginRegistry {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            plugins: HashMap::new(),
            plugin_configs: HashMap::new(),
        })
    }

    pub async fn register_plugin(&mut self, metadata: PluginMetadata) -> Result<()> {
        let plugin_info = PluginInfo {
            metadata: metadata.clone(),
            file_path: None,
            installed_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            checksum: None,
            enabled: true,
        };

        self.plugins.insert(metadata.id.clone(), plugin_info);
        
        // Create default config
        let default_config = super::PluginConfig {
            enabled: true,
            auto_load: true,
            settings: HashMap::new(),
            permissions: Vec::new(),
            resource_limits: super::ResourceLimits {
                max_memory_mb: Some(100),
                max_cpu_percent: Some(10.0),
                max_execution_time_ms: Some(30000),
                max_file_operations: Some(100),
                max_network_requests: Some(50),
            },
        };

        self.plugin_configs.insert(metadata.id, default_config);
        Ok(())
    }

    pub async fn unregister_plugin(&mut self, plugin_id: &str) -> Result<()> {
        self.plugins.remove(plugin_id);
        self.plugin_configs.remove(plugin_id);
        Ok(())
    }

    pub async fn get_plugin_info(&self, plugin_id: &str) -> Result<Option<PluginInfo>> {
        Ok(self.plugins.get(plugin_id).cloned())
    }

    pub async fn get_plugin_config(&self, plugin_id: &str) -> Result<Option<super::PluginConfig>> {
        Ok(self.plugin_configs.get(plugin_id).cloned())
    }

    pub async fn update_plugin_config(&mut self, plugin_id: &str, config: super::PluginConfig) -> Result<()> {
        self.plugin_configs.insert(plugin_id.to_string(), config);
        Ok(())
    }

    pub async fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    pub async fn find_plugins_by_capability(&self, _capability: &super::PluginCapability) -> Vec<&PluginInfo> {
        // This would need to be implemented with capability checking
        self.plugins.values().collect()
    }
}
