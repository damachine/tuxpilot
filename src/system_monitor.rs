use anyhow::{Context, Result};
use sysinfo::{System, Disks, Networks};
use std::time::Duration;
use tokio::time;

use crate::config::Config;

pub struct SystemMonitor {
    system: System,
    disks: Disks,
    networks: Networks,
    config: Config,
}

impl SystemMonitor {
    pub fn new(config: &Config) -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();

        let mut disks = Disks::new_with_refreshed_list();
        let mut networks = Networks::new_with_refreshed_list();

        Ok(Self {
            system,
            disks,
            networks,
            config: config.clone(),
        })
    }

    pub async fn get_system_status(&mut self) -> Result<String> {
        self.system.refresh_all();
        self.disks.refresh();
        self.networks.refresh();

        let mut status = Vec::new();

        // CPU Information
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        status.push(format!("CPU Usage: {:.1}%", cpu_usage));

        // Memory Information
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let memory_percent = (used_memory as f64 / total_memory as f64) * 100.0;
        status.push(format!(
            "Memory: {:.1}% ({} MB / {} MB)",
            memory_percent,
            used_memory / 1024 / 1024,
            total_memory / 1024 / 1024
        ));

        // Swap Information
        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();
        if total_swap > 0 {
            let swap_percent = (used_swap as f64 / total_swap as f64) * 100.0;
            status.push(format!(
                "Swap: {:.1}% ({} MB / {} MB)",
                swap_percent,
                used_swap / 1024 / 1024,
                total_swap / 1024 / 1024
            ));
        }

        // Disk Information
        for disk in &self.disks {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let usage_percent = (used_space as f64 / total_space as f64) * 100.0;

            status.push(format!(
                "Disk {}: {:.1}% ({} GB / {} GB)",
                disk.mount_point().display(),
                usage_percent,
                used_space / 1024 / 1024 / 1024,
                total_space / 1024 / 1024 / 1024
            ));
        }
        
        // Load Average (Linux specific)
        if let Ok(loadavg) = tokio::fs::read_to_string("/proc/loadavg").await {
            let load_parts: Vec<&str> = loadavg.split_whitespace().collect();
            if load_parts.len() >= 3 {
                status.push(format!(
                    "Load Average: {} {} {}",
                    load_parts[0], load_parts[1], load_parts[2]
                ));
            }
        }
        
        // Network Information
        let mut network_info = Vec::new();
        for (interface_name, network) in &self.networks {
            if network.received() > 0 || network.transmitted() > 0 {
                network_info.push(format!(
                    "{}: ↓{} MB ↑{} MB",
                    interface_name,
                    network.received() / 1024 / 1024,
                    network.transmitted() / 1024 / 1024
                ));
            }
        }
        if !network_info.is_empty() {
            status.push(format!("Network: {}", network_info.join(", ")));
        }
        
        // Top processes by CPU
        let mut processes: Vec<_> = self.system.processes().values().collect();
        processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
        
        let top_processes: Vec<String> = processes
            .iter()
            .take(5)
            .filter(|p| p.cpu_usage() > 0.1)
            .map(|p| format!("{} ({:.1}%)", p.name(), p.cpu_usage()))
            .collect();
        
        if !top_processes.is_empty() {
            status.push(format!("Top CPU: {}", top_processes.join(", ")));
        }
        
        Ok(status.join("\n"))
    }

    pub async fn start_continuous_monitoring(&mut self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            let status = self.get_system_status().await?;
            println!("\n=== System Status ===");
            println!("{}", status);
            
            // Check for alerts
            if let Some(alert) = self.check_system_alerts().await? {
                println!("\n⚠️  ALERT: {}", alert);
            }
        }
    }

    async fn check_system_alerts(&mut self) -> Result<Option<String>> {
        self.system.refresh_all();
        
        // CPU Alert
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        if cpu_usage > 90.0 {
            return Ok(Some(format!("High CPU usage: {:.1}%", cpu_usage)));
        }
        
        // Memory Alert
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let memory_percent = (used_memory as f64 / total_memory as f64) * 100.0;
        if memory_percent > 90.0 {
            return Ok(Some(format!("High memory usage: {:.1}%", memory_percent)));
        }
        
        // Disk Alert
        for disk in &self.disks {
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            let usage_percent = (used_space as f64 / total_space as f64) * 100.0;

            if usage_percent > 90.0 {
                return Ok(Some(format!(
                    "High disk usage on {}: {:.1}%",
                    disk.mount_point().display(),
                    usage_percent
                )));
            }
        }
        
        // Load Average Alert (Linux specific)
        if let Ok(loadavg) = tokio::fs::read_to_string("/proc/loadavg").await {
            let load_parts: Vec<&str> = loadavg.split_whitespace().collect();
            if let Ok(load1) = load_parts[0].parse::<f64>() {
                let cpu_count = self.system.cpus().len() as f64;
                if load1 > cpu_count * 2.0 {
                    return Ok(Some(format!("High load average: {:.2}", load1)));
                }
            }
        }
        
        Ok(None)
    }

    pub async fn get_process_info(&mut self, process_name: &str) -> Result<String> {
        self.system.refresh_processes();
        
        let mut matching_processes = Vec::new();
        
        for (pid, process) in self.system.processes() {
            if process.name().to_lowercase().contains(&process_name.to_lowercase()) {
                matching_processes.push(format!(
                    "PID: {} | Name: {} | CPU: {:.1}% | Memory: {} MB | Status: {:?}",
                    pid,
                    process.name(),
                    process.cpu_usage(),
                    process.memory() / 1024 / 1024,
                    process.status()
                ));
            }
        }
        
        if matching_processes.is_empty() {
            Ok(format!("No processes found matching '{}'", process_name))
        } else {
            Ok(matching_processes.join("\n"))
        }
    }

    pub async fn get_service_status(&self, service_name: &str) -> Result<String> {
        // This would integrate with systemctl or other service managers
        let output = tokio::process::Command::new("systemctl")
            .args(&["status", service_name])
            .output()
            .await
            .context("Failed to check service status")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!("Service '{}' not found or not accessible", service_name))
        }
    }

    pub async fn get_network_status(&mut self) -> Result<String> {
        self.networks.refresh();

        let mut network_status = Vec::new();

        for (interface_name, network) in &self.networks {
            network_status.push(format!(
                "Interface: {} | Received: {} MB | Transmitted: {} MB | Packets In: {} | Packets Out: {}",
                interface_name,
                network.received() / 1024 / 1024,
                network.transmitted() / 1024 / 1024,
                network.packets_received(),
                network.packets_transmitted()
            ));
        }
        
        // Add IP information
        if let Ok(ip_output) = tokio::process::Command::new("ip")
            .args(&["addr", "show"])
            .output()
            .await
        {
            if ip_output.status.success() {
                let ip_info = String::from_utf8_lossy(&ip_output.stdout);
                network_status.push(format!("\nIP Configuration:\n{}", ip_info));
            }
        }
        
        Ok(network_status.join("\n"))
    }

    pub async fn get_hardware_info(&mut self) -> Result<String> {
        self.system.refresh_all();
        
        let mut hardware_info = Vec::new();
        
        // CPU Information
        hardware_info.push(format!("CPU Cores: {}", self.system.cpus().len()));
        if let Some(cpu) = self.system.cpus().first() {
            hardware_info.push(format!("CPU Brand: {}", cpu.brand()));
            hardware_info.push(format!("CPU Frequency: {} MHz", cpu.frequency()));
        }
        
        // Memory Information
        hardware_info.push(format!(
            "Total Memory: {} GB",
            self.system.total_memory() / 1024 / 1024 / 1024
        ));
        
        // Disk Information
        for disk in &self.disks {
            hardware_info.push(format!(
                "Disk: {} | Type: {:?} | Total: {} GB | File System: {:?}",
                disk.name().to_string_lossy(),
                disk.kind(),
                disk.total_space() / 1024 / 1024 / 1024,
                disk.file_system()
            ));
        }
        
        Ok(hardware_info.join("\n"))
    }
}
