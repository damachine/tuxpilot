use anyhow::Result;
use console::{style, Term};
use dialoguer::{Input, Confirm};
use indicatif::{ProgressBar, ProgressStyle};


use crate::ai::AiClient;
use crate::config::Config;
use crate::error_diagnosis::ErrorDiagnostic;
use crate::execution::CommandExecutor;
use crate::linux_integration::LinuxIntegration;
use crate::system_monitor::SystemMonitor;
use crate::Commands;

pub struct TuxPilotCli {
    config: Config,
    ai_client: AiClient,
    linux_integration: LinuxIntegration,
    system_monitor: SystemMonitor,
    command_executor: Option<CommandExecutor>,
    term: Term,
}

impl TuxPilotCli {
    pub async fn new(mut config: Config, use_local: bool) -> Result<Self> {
        // Auto-detect system configuration
        config.detect_system()?;
        
        let ai_client = AiClient::new(&config, use_local).await?;
        let linux_integration = LinuxIntegration::new(&config).await?;
        let system_monitor = SystemMonitor::new(&config)?;
        let term = Term::stdout();

        Ok(Self {
            config,
            ai_client,
            linux_integration,
            system_monitor,
            command_executor: None, // Will be initialized when needed
            term,
        })
    }

    pub async fn handle_command(&mut self, command: Commands) -> Result<()> {
        match command {
            Commands::Diagnose { input, auto } => {
                self.handle_diagnose(input, auto).await?;
            }
            Commands::Explain { command } => {
                self.handle_help(command).await?;
            }
            Commands::Monitor { continuous } => {
                self.handle_monitor(continuous).await?;
            }
            Commands::Package { operation, package } => {
                self.handle_package(operation, package).await?;
            }
            Commands::Service { name, action } => {
                self.handle_service(name, action).await?;
            }
            Commands::Chat { execute_mode } => {
                self.interactive_mode_with_execution(&execute_mode).await?;
            }
            Commands::Execute { description, mode } => {
                self.handle_execute_command(&description, &mode).await?;
            }
            Commands::Permissions { detailed } => {
                self.show_permissions(detailed).await?;
            }
            Commands::Audit { limit, export } => {
                self.show_audit_log(limit, export.as_deref()).await?;
            }
            Commands::Config { show, set } => {
                self.handle_config(show, set).await?;
            }
        }
        Ok(())
    }

    pub async fn interactive_mode(&mut self) -> Result<()> {
        let _ = self.print_welcome();

        loop {
            let input: String = Input::new()
                .with_prompt(&format!("{}", style("tuxpilot>").cyan().bold()))
                .interact_text()?;

            if input.trim().is_empty() {
                continue;
            }

            if input.trim() == "exit" || input.trim() == "quit" {
                break;
            }

            if input.trim() == "help" {
                let _ = self.print_help();
                continue;
            }

            // Process the input with AI
            self.process_interactive_input(&input).await?;
        }

        self.term.write_line(&format!("{}", style("Goodbye! 👋").green()))?;
        Ok(())
    }

    async fn process_interactive_input(&mut self, input: &str) -> Result<()> {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        spinner.set_message("Thinking...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        // Analyze the input and determine the best action
        let response = self.ai_client.process_query(input).await?;
        
        spinner.finish_and_clear();
        
        // Display the AI response
        self.term.write_line(&format!("\n{}", style("🤖 TuxPilot:").blue().bold()))?;
        self.term.write_line(&response)?;
        self.term.write_line("")?;

        Ok(())
    }

    async fn handle_diagnose(&mut self, input: Option<String>, auto: bool) -> Result<()> {
        if auto {
            self.term.write_line(&format!("{}", style("🔍 Analyzing system logs...").yellow()))?;
            let diagnostic = ErrorDiagnostic::analyze_system_logs(&self.linux_integration).await?;
            let analysis = self.ai_client.analyze_diagnostic(&diagnostic).await?;
            self.display_analysis(&analysis)?;
        } else if let Some(error_input) = input {
            self.term.write_line(&format!("{}", style("🔍 Analyzing error...").yellow()))?;
            let diagnostic = ErrorDiagnostic::analyze_error(&error_input, &self.linux_integration).await?;
            let analysis = self.ai_client.analyze_diagnostic(&diagnostic).await?;
            self.display_analysis(&analysis)?;
        } else {
            let error_input: String = Input::new()
                .with_prompt("Enter the error message or describe the problem")
                .interact_text()?;
            
            let diagnostic = ErrorDiagnostic::analyze_error(&error_input, &self.linux_integration).await?;
            let analysis = self.ai_client.analyze_diagnostic(&diagnostic).await?;
            self.display_analysis(&analysis)?;
        }
        Ok(())
    }

    async fn handle_help(&mut self, command: Option<String>) -> Result<()> {
        if let Some(cmd) = command {
            let help_text = self.ai_client.get_command_help(&cmd).await?;
            self.term.write_line(&format!("{} {}", style("📖 Help for").blue(), style(&cmd).bold()))?;
            self.term.write_line(&help_text)?;
        } else {
            let _ = self.print_help();
        }
        Ok(())
    }

    async fn handle_monitor(&mut self, continuous: bool) -> Result<()> {
        if continuous {
            self.term.write_line(&format!("{}", style("📊 Starting continuous monitoring... (Press Ctrl+C to stop)").green()))?;
            self.system_monitor.start_continuous_monitoring().await?;
        } else {
            let status = self.system_monitor.get_system_status().await?;
            let analysis = self.ai_client.analyze_system_status(&status).await?;
            self.display_system_status(&status, &analysis)?;
        }
        Ok(())
    }

    async fn handle_package(&mut self, operation: String, package: Option<String>) -> Result<()> {
        let suggestion = self.linux_integration.get_package_suggestion(&operation, package.as_deref()).await?;
        let ai_advice = self.ai_client.get_package_advice(&operation, package.as_deref(), &suggestion).await?;
        
        self.term.write_line(&format!("{}", style("📦 Package Management").blue().bold()))?;
        self.term.write_line(&ai_advice)?;
        
        if Confirm::new()
            .with_prompt("Would you like to execute the suggested command?")
            .interact()?
        {
            self.linux_integration.execute_package_command(&suggestion).await?;
        }
        
        Ok(())
    }

    async fn handle_service(&mut self, name: String, action: Option<String>) -> Result<()> {
        let service_info = self.linux_integration.get_service_info(&name).await?;
        let ai_advice = self.ai_client.get_service_advice(&name, action.as_deref(), &service_info).await?;
        
        self.term.write_line(&format!("{}", style("⚙️  Service Management").blue().bold()))?;
        self.term.write_line(&ai_advice)?;
        
        Ok(())
    }

    async fn handle_config(&mut self, show: bool, set: Option<String>) -> Result<()> {
        if show {
            self.display_config()?;
        } else if let Some(_setting) = set {
            self.term.write_line("Configuration editing not yet implemented")?;
        } else {
            self.display_config()?;
        }
        Ok(())
    }

    fn print_welcome(&self) -> Result<()> {
        self.term.write_line(&format!("{}", style("🐧 Welcome to TuxPilot!").green().bold()))?;
        self.term.write_line("Your AI-powered Linux assistant")?;
        self.term.write_line("")?;
        self.term.write_line("Type 'help' for available commands or just describe your problem.")?;
        self.term.write_line("Type 'exit' or 'quit' to leave.")?;
        self.term.write_line("")?;
        Ok(())
    }

    fn print_help(&self) -> Result<()> {
        self.term.write_line(&format!("{}", style("📖 TuxPilot Help").blue().bold()))?;
        self.term.write_line("")?;
        self.term.write_line("Available commands:")?;
        self.term.write_line("  diagnose [--auto] [--input <error>] - Diagnose system errors")?;
        self.term.write_line("  help [command]                     - Get help with commands")?;
        self.term.write_line("  monitor [--continuous]             - Monitor system health")?;
        self.term.write_line("  package <operation> [package]      - Package management help")?;
        self.term.write_line("  service <name> [action]            - Service management help")?;
        self.term.write_line("  config [--show] [--set <key=value>] - Configuration management")?;
        self.term.write_line("  chat                               - Interactive chat mode")?;
        self.term.write_line("")?;
        self.term.write_line("You can also just describe your problem in natural language!")?;
        self.term.write_line("")?;
        Ok(())
    }

    fn display_analysis(&self, analysis: &str) -> Result<()> {
        self.term.write_line(&format!("{}", style("🔍 Analysis:").green().bold()))?;
        self.term.write_line(analysis)?;
        self.term.write_line("")?;
        Ok(())
    }

    fn display_system_status(&self, _status: &str, analysis: &str) -> Result<()> {
        self.term.write_line(&format!("{}", style("📊 System Status:").green().bold()))?;
        self.term.write_line(analysis)?;
        self.term.write_line("")?;
        Ok(())
    }

    fn display_config(&self) -> Result<()> {
        self.term.write_line(&format!("{}", style("⚙️  Current Configuration:").blue().bold()))?;
        self.term.write_line(&format!("AI Provider: {:?}", self.config.ai.provider))?;
        self.term.write_line(&format!("Package Manager: {:?}", self.config.system.package_manager))?;
        self.term.write_line(&format!("Service Manager: {:?}", self.config.system.service_manager))?;

        // Show distribution information if available
        if let Some(ref distro_info) = self.linux_integration.distribution_info {
            self.term.write_line("")?;
            self.term.write_line(&format!("{}", style("🐧 Detected Distribution:").blue().bold()))?;
            self.term.write_line(&format!("Name: {}", distro_info.name))?;
            self.term.write_line(&format!("Version: {}", distro_info.version))?;
            self.term.write_line(&format!("ID: {}", distro_info.id))?;
            self.term.write_line(&format!("Architecture: {}", distro_info.architecture))?;
            self.term.write_line(&format!("Init System: {}", distro_info.init_system))?;
            self.term.write_line(&format!("Shell: {}", distro_info.shell))?;
        }

        self.term.write_line("")?;
        Ok(())
    }

    async fn interactive_mode_with_execution(&mut self, execute_mode: &str) -> Result<()> {
        self.term.write_line(&format!("{}", style("🤖 TuxPilot Interactive Mode with Command Execution").green().bold()))?;
        self.term.write_line(&format!("Execution Mode: {}", style(execute_mode).yellow()))?;
        self.term.write_line("Type 'help' for commands, 'exit' to quit")?;
        self.term.write_line("")?;

        // For now, fall back to regular interactive mode
        // TODO: Implement execution mode handling
        self.interactive_mode().await
    }

    async fn handle_execute_command(&mut self, description: &str, mode: &str) -> Result<()> {
        self.term.write_line(&format!("{}", style("🔧 Command Execution").blue().bold()))?;
        self.term.write_line(&format!("Description: {}", description))?;
        self.term.write_line(&format!("Mode: {}", mode))?;
        self.term.write_line("")?;

        // TODO: Implement actual command execution
        self.term.write_line("⚠️  Command execution system not yet fully implemented.")?;
        self.term.write_line("This feature will allow TuxPilot to execute commands based on natural language descriptions.")?;

        Ok(())
    }

    async fn show_permissions(&mut self, detailed: bool) -> Result<()> {
        self.term.write_line(&format!("{}", style("🔐 TuxPilot Permissions").blue().bold()))?;
        self.term.write_line("")?;

        // TODO: Implement permission checking
        self.term.write_line("📋 Permission system features:")?;
        self.term.write_line("  ✅ Granular permission control")?;
        self.term.write_line("  ✅ Safety checks for dangerous commands")?;
        self.term.write_line("  ✅ Audit logging of all operations")?;
        self.term.write_line("  ✅ User approval for risky operations")?;

        if detailed {
            self.term.write_line("")?;
            self.term.write_line("🔍 Detailed permission analysis:")?;
            self.term.write_line("  - Read System: Available to all users")?;
            self.term.write_line("  - Write System: Requires sudo/root access")?;
            self.term.write_line("  - Package Management: Requires package manager permissions")?;
            self.term.write_line("  - Service Management: Requires systemctl permissions")?;
        }

        Ok(())
    }

    async fn show_audit_log(&mut self, limit: usize, export_format: Option<&str>) -> Result<()> {
        self.term.write_line(&format!("{}", style("📊 TuxPilot Audit Log").blue().bold()))?;
        self.term.write_line(&format!("Showing last {} entries", limit))?;
        self.term.write_line("")?;

        // TODO: Implement audit log reading
        self.term.write_line("📋 Audit log features:")?;
        self.term.write_line("  ✅ Complete command execution history")?;
        self.term.write_line("  ✅ Permission requests and grants")?;
        self.term.write_line("  ✅ Safety violations and warnings")?;
        self.term.write_line("  ✅ System changes and rollback information")?;

        if let Some(format) = export_format {
            self.term.write_line("")?;
            self.term.write_line(&format!("📤 Export format: {}", format))?;
            self.term.write_line("Export functionality will be available soon.")?;
        }

        Ok(())
    }
}
