use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    pub system: SystemConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: AiProvider,
    pub openai: Option<OpenAiConfig>,
    pub anthropic: Option<AnthropicConfig>,
    pub local: Option<LocalAiConfig>,
    pub ollama: Option<OllamaConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AiProvider {
    OpenAI,
    Anthropic,
    Local,
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub api_key: String,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAiConfig {
    pub model_path: PathBuf,
    pub context_size: usize,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub base_url: String,
    pub model: String,
    pub temperature: f32,
    pub context_size: usize,
    pub timeout_seconds: u64,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub package_manager: PackageManager,
    pub log_paths: Vec<PathBuf>,
    pub service_manager: ServiceManager,
    pub execution_mode: ExecutionMode,
    pub require_confirmation: bool,
    pub command_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionMode {
    Supervised,
    SemiAuto,
    Autonomous,
    ReadOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    Pacman,
    Apt,
    Dnf,
    Zypper,
    Portage,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Pacman => write!(f, "Pacman"),
            PackageManager::Apt => write!(f, "Apt"),
            PackageManager::Dnf => write!(f, "Dnf"),
            PackageManager::Zypper => write!(f, "Zypper"),
            PackageManager::Portage => write!(f, "Portage"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceManager {
    Systemd,
    OpenRC,
    SysVInit,
}

impl std::fmt::Display for ServiceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceManager::Systemd => write!(f, "Systemd"),
            ServiceManager::OpenRC => write!(f, "OpenRC"),
            ServiceManager::SysVInit => write!(f, "SysVInit"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub show_tips: bool,
    pub auto_suggest: bool,
    pub web_port: u16,
    pub bind_address: String,
    pub ssl_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai: AiConfig {
                provider: AiProvider::Ollama, // Standard auf Ollama setzen
                openai: Some(OpenAiConfig {
                    api_key: String::new(),
                    model: "gpt-4".to_string(),
                    base_url: None,
                    temperature: Some(0.7),
                    max_tokens: Some(2048),
                }),
                anthropic: Some(AnthropicConfig {
                    api_key: String::new(),
                    model: "claude-3-sonnet-20240229".to_string(),
                    temperature: Some(0.7),
                    max_tokens: Some(2048),
                }),
                local: None,
                ollama: Some(OllamaConfig {
                    base_url: "http://localhost:11434".to_string(),
                    model: "gemma3:latest".to_string(),
                    temperature: 0.7,
                    context_size: 4096,
                    timeout_seconds: 30,
                    max_tokens: Some(2048),
                }),
            },
            system: SystemConfig {
                package_manager: PackageManager::Pacman, // Default to Arch Linux
                log_paths: vec![
                    PathBuf::from("/var/log/syslog"),
                    PathBuf::from("/var/log/messages"),
                    PathBuf::from("/var/log/kern.log"),
                    PathBuf::from("/var/log/auth.log"),
                    PathBuf::from("/var/log/pacman.log"),
                ],
                service_manager: ServiceManager::Systemd,
                execution_mode: ExecutionMode::Supervised,
                require_confirmation: true,
                command_timeout_seconds: 30,
            },
            ui: UiConfig {
                theme: "default".to_string(),
                show_tips: true,
                auto_suggest: true,
                web_port: 8082,
                bind_address: "127.0.0.1".to_string(),
                ssl_enabled: false,
            },
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let config_file = match config_path {
            Some(path) => path.to_path_buf(),
            None => Self::default_config_path()?,
        };

        if config_file.exists() {
            let content = fs::read_to_string(&config_file)
                .with_context(|| format!("Failed to read config file: {:?}", config_file))?;
            
            let config: Config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {:?}", config_file))?;
            
            Ok(config)
        } else {
            // Create default config
            let config = Self::default();
            config.save(&config_file)?;
            Ok(config)
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;
        
        Ok(())
    }

    pub fn default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?
            .join("tuxpilot");
        
        Ok(config_dir.join("config.toml"))
    }

    pub fn detect_system(&mut self) -> Result<()> {
        // Auto-detect package manager
        if Path::new("/usr/bin/pacman").exists() {
            self.system.package_manager = PackageManager::Pacman;
        } else if Path::new("/usr/bin/apt").exists() {
            self.system.package_manager = PackageManager::Apt;
        } else if Path::new("/usr/bin/dnf").exists() {
            self.system.package_manager = PackageManager::Dnf;
        } else if Path::new("/usr/bin/zypper").exists() {
            self.system.package_manager = PackageManager::Zypper;
        } else if Path::new("/usr/bin/emerge").exists() {
            self.system.package_manager = PackageManager::Portage;
        }

        // Auto-detect service manager
        if Path::new("/bin/systemctl").exists() || Path::new("/usr/bin/systemctl").exists() {
            self.system.service_manager = ServiceManager::Systemd;
        } else if Path::new("/sbin/rc-service").exists() {
            self.system.service_manager = ServiceManager::OpenRC;
        } else {
            self.system.service_manager = ServiceManager::SysVInit;
        }

        // Auto-detect Ollama installation
        if self.is_ollama_available() {
            self.ai.provider = AiProvider::Ollama;
            log::info!("Ollama detected, switching to local AI provider");
        }

        Ok(())
    }

    fn is_ollama_available(&self) -> bool {
        // Check if ollama command exists
        if let Ok(output) = std::process::Command::new("which")
            .arg("ollama")
            .output()
        {
            if output.status.success() {
                // Check if ollama service is running
                if let Ok(response) = std::process::Command::new("curl")
                    .arg("-s")
                    .arg("http://localhost:11434/api/tags")
                    .output()
                {
                    return response.status.success();
                }
            }
        }
        false
    }
}
