use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod agents;
mod ai;
mod automation;
mod cli;
mod config;
mod containers;
mod error_diagnosis;
mod execution;
mod linux_integration;
mod mcp;
mod monitoring;
mod nlp;
mod performance;
mod plugins;
mod security;
mod system_monitor;
mod web;

use crate::cli::TuxPilotCli;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "tuxpilot")]
#[command(about = "An AI-powered copilot for Linux systems")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Use local AI model instead of cloud API
    #[arg(long)]
    local: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Diagnose system errors and provide solutions
    Diagnose {
        /// Error message or log file to analyze
        #[arg(short, long)]
        input: Option<String>,
        
        /// Analyze system logs automatically
        #[arg(long)]
        auto: bool,
    },
    
    /// Get help with Linux commands
    Explain {
        /// Command to get help with
        command: Option<String>,
    },
    
    /// Monitor system health and performance
    Monitor {
        /// Continuous monitoring mode
        #[arg(short, long)]
        continuous: bool,
    },
    
    /// Package management assistance
    Package {
        /// Package operation (install, remove, update, search)
        operation: String,
        /// Package name
        package: Option<String>,
    },
    
    /// Service management help
    Service {
        /// Service name
        name: String,
        /// Action (start, stop, restart, status)
        action: Option<String>,
    },
    
    /// Interactive chat mode
    Chat {
        /// Execution mode for commands
        #[arg(long, default_value = "supervised")]
        execute_mode: String,
    },

    /// Execute a command with AI assistance
    Execute {
        /// Natural language description of what to do
        description: String,
        /// Execution mode (supervised, semi-auto, autonomous, read-only)
        #[arg(long, default_value = "supervised")]
        mode: String,
    },

    /// Show execution permissions and capabilities
    Permissions {
        /// Show detailed permission information
        #[arg(long)]
        detailed: bool,
    },

    /// View audit log of executed commands
    Audit {
        /// Number of recent entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
        /// Export format (json, csv, html)
        #[arg(long)]
        export: Option<String>,
    },

    /// Configure TuxPilot settings
    Config {
        /// Show current configuration
        #[arg(long)]
        show: bool,
        /// Set configuration value
        #[arg(long)]
        set: Option<String>,
    },

    /// Start web interface server
    Web {
        /// Port to bind the web server to
        #[arg(short, long, default_value = "8080")]
        port: u16,
        /// Bind address for the web server
        #[arg(long, default_value = "127.0.0.1")]
        bind: String,
        /// Enable SSL/TLS
        #[arg(long)]
        ssl: bool,
        /// SSL certificate file path
        #[arg(long)]
        ssl_cert: Option<PathBuf>,
        /// SSL private key file path
        #[arg(long)]
        ssl_key: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    // Load configuration
    let config = Config::load(args.config.as_deref())?;
    
    // Initialize CLI
    let mut cli = TuxPilotCli::new(config, args.local).await?;
    
    match args.command {
        Some(command) => cli.handle_command(command).await?,
        None => cli.interactive_mode().await?,
    }
    
    Ok(())
}
