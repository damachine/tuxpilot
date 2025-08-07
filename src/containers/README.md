# TuxPilot Container Management

The TuxPilot container management module provides comprehensive Docker and container orchestration capabilities with AI-powered optimization and management.

## 🏗️ Architecture Overview

```
Container Management Architecture:
├── 🐳 Docker Integration (Docker API client)
├── 🎯 Container Orchestrator (Multi-container management)
├── 📊 Container Monitoring (Resource tracking)
├── 🔧 Container Optimization (Performance tuning)
├── 🔒 Security Scanner (Container security)
└── 📋 Compose Management (Docker Compose integration)
```

## 🐳 Docker Integration

**File**: `docker.rs`

Core Docker API integration for container lifecycle management.

### Docker Client

```rust
pub struct DockerClient {
    client: Docker,
    config: ContainerConfig,
    event_stream: Option<EventStream>,
}

impl DockerClient {
    pub async fn new() -> Result<Self> {
        let client = Docker::connect_with_local_defaults()?;
        
        Ok(DockerClient {
            client,
            config: ContainerConfig::default(),
            event_stream: None,
        })
    }
    
    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerInfo>> {
        let options = ListContainersOptions::<String> {
            all,
            ..Default::default()
        };
        
        let containers = self.client.list_containers(Some(options)).await?;
        
        Ok(containers.into_iter().map(|c| ContainerInfo {
            id: c.id.unwrap_or_default(),
            name: c.names.unwrap_or_default().first().unwrap_or(&String::new()).clone(),
            image: c.image.unwrap_or_default(),
            status: c.status.unwrap_or_default(),
            state: c.state.unwrap_or_default(),
            ports: c.ports.unwrap_or_default(),
            created: c.created.unwrap_or_default(),
        }).collect())
    }
    
    pub async fn create_container(
        &self,
        config: &ContainerCreateConfig
    ) -> Result<String> {
        let container_config = Config {
            image: Some(config.image.clone()),
            env: config.environment.clone(),
            cmd: config.command.clone(),
            exposed_ports: config.exposed_ports.clone(),
            host_config: Some(HostConfig {
                port_bindings: config.port_bindings.clone(),
                binds: config.volumes.clone(),
                memory: config.memory_limit,
                cpu_shares: config.cpu_shares,
                restart_policy: config.restart_policy.clone(),
                ..Default::default()
            }),
            ..Default::default()
        };
        
        let response = self.client.create_container(
            Some(CreateContainerOptions {
                name: config.name.clone(),
                ..Default::default()
            }),
            container_config
        ).await?;
        
        Ok(response.id)
    }
    
    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.client.start_container(id, None::<StartContainerOptions<String>>).await?;
        Ok(())
    }
    
    pub async fn stop_container(&self, id: &str, timeout: Option<Duration>) -> Result<()> {
        let options = StopContainerOptions {
            t: timeout.map(|d| d.as_secs() as i64).unwrap_or(10),
        };
        
        self.client.stop_container(id, Some(options)).await?;
        Ok(())
    }
    
    pub async fn remove_container(&self, id: &str, force: bool) -> Result<()> {
        let options = RemoveContainerOptions {
            force,
            v: true, // Remove volumes
            ..Default::default()
        };
        
        self.client.remove_container(id, Some(options)).await?;
        Ok(())
    }
}
```

### Container Information

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub state: String,
    pub ports: Vec<Port>,
    pub created: i64,
    pub labels: HashMap<String, String>,
    pub mounts: Vec<MountPoint>,
    pub network_settings: NetworkSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerCreateConfig {
    pub name: String,
    pub image: String,
    pub command: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub exposed_ports: Option<HashMap<String, HashMap<(), ()>>>,
    pub port_bindings: Option<HashMap<String, Option<Vec<PortBinding>>>>,
    pub volumes: Option<Vec<String>>,
    pub memory_limit: Option<i64>,
    pub cpu_shares: Option<i64>,
    pub restart_policy: Option<RestartPolicy>,
    pub labels: Option<HashMap<String, String>>,
}
```

## 🎯 Container Orchestrator

**File**: `orchestrator.rs`

High-level container orchestration and management.

### Orchestrator Implementation

```rust
pub struct ContainerOrchestrator {
    docker_client: DockerClient,
    compose_manager: ComposeManager,
    monitoring: ContainerMonitoring,
    security_scanner: ContainerSecurityScanner,
    config: OrchestratorConfig,
}

impl ContainerOrchestrator {
    pub async fn deploy_application(
        &self,
        deployment: &ApplicationDeployment
    ) -> Result<DeploymentResult> {
        let mut deployed_containers = Vec::new();
        
        // Create network if specified
        if let Some(network_config) = &deployment.network {
            self.create_network(network_config).await?;
        }
        
        // Deploy containers in dependency order
        let deployment_order = self.calculate_deployment_order(&deployment.containers)?;
        
        for container_name in deployment_order {
            let container_config = deployment.containers.get(&container_name)
                .ok_or_else(|| Error::ContainerNotFound(container_name.clone()))?;
                
            let container_id = self.deploy_container(container_config).await?;
            deployed_containers.push(DeployedContainer {
                name: container_name,
                id: container_id,
                status: ContainerStatus::Running,
            });
            
            // Wait for health check if configured
            if let Some(health_check) = &container_config.health_check {
                self.wait_for_health_check(&container_id, health_check).await?;
            }
        }
        
        Ok(DeploymentResult {
            deployment_id: Uuid::new_v4().to_string(),
            containers: deployed_containers,
            network: deployment.network.clone(),
            status: DeploymentStatus::Success,
        })
    }
    
    pub async fn scale_service(
        &self,
        service_name: &str,
        replicas: u32
    ) -> Result<ScalingResult> {
        let current_containers = self.get_service_containers(service_name).await?;
        let current_count = current_containers.len() as u32;
        
        if replicas > current_count {
            // Scale up
            let scale_up_count = replicas - current_count;
            let mut new_containers = Vec::new();
            
            for i in 0..scale_up_count {
                let container_name = format!("{}-{}", service_name, current_count + i + 1);
                let container_config = self.get_service_template(service_name)?;
                
                let container_id = self.deploy_container(&container_config).await?;
                new_containers.push(container_id);
            }
            
            Ok(ScalingResult {
                service: service_name.to_string(),
                previous_replicas: current_count,
                new_replicas: replicas,
                scaled_containers: new_containers,
            })
        } else if replicas < current_count {
            // Scale down
            let scale_down_count = current_count - replicas;
            let mut removed_containers = Vec::new();
            
            for container in current_containers.iter().take(scale_down_count as usize) {
                self.docker_client.stop_container(&container.id, Some(Duration::from_secs(30))).await?;
                self.docker_client.remove_container(&container.id, false).await?;
                removed_containers.push(container.id.clone());
            }
            
            Ok(ScalingResult {
                service: service_name.to_string(),
                previous_replicas: current_count,
                new_replicas: replicas,
                scaled_containers: removed_containers,
            })
        } else {
            // No scaling needed
            Ok(ScalingResult {
                service: service_name.to_string(),
                previous_replicas: current_count,
                new_replicas: replicas,
                scaled_containers: vec![],
            })
        }
    }
}
```

### Application Deployment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationDeployment {
    pub name: String,
    pub version: String,
    pub containers: HashMap<String, ContainerDeploymentConfig>,
    pub network: Option<NetworkConfig>,
    pub volumes: Option<HashMap<String, VolumeConfig>>,
    pub secrets: Option<HashMap<String, SecretConfig>>,
    pub environment: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDeploymentConfig {
    pub image: String,
    pub tag: Option<String>,
    pub replicas: Option<u32>,
    pub resources: Option<ResourceLimits>,
    pub environment: Option<HashMap<String, String>>,
    pub ports: Option<Vec<PortMapping>>,
    pub volumes: Option<Vec<VolumeMount>>,
    pub depends_on: Option<Vec<String>>,
    pub health_check: Option<HealthCheck>,
    pub restart_policy: Option<RestartPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub memory: Option<String>,
    pub cpu: Option<String>,
    pub memory_reservation: Option<String>,
    pub cpu_reservation: Option<String>,
}
```

## 📊 Container Monitoring

**File**: `monitoring.rs`

Real-time container monitoring and metrics collection.

### Monitoring System

```rust
pub struct ContainerMonitoring {
    docker_client: DockerClient,
    metrics_collector: MetricsCollector,
    alert_manager: AlertManager,
    config: MonitoringConfig,
}

impl ContainerMonitoring {
    pub async fn collect_container_metrics(
        &self,
        container_id: &str
    ) -> Result<ContainerMetrics> {
        let stats = self.docker_client.client.stats(container_id, Some(StatsOptions {
            stream: false,
            one_shot: true,
        })).try_next().await?;
        
        if let Some(stats) = stats {
            Ok(ContainerMetrics {
                container_id: container_id.to_string(),
                timestamp: Utc::now(),
                cpu_usage: self.calculate_cpu_usage(&stats)?,
                memory_usage: self.calculate_memory_usage(&stats)?,
                network_io: self.calculate_network_io(&stats)?,
                disk_io: self.calculate_disk_io(&stats)?,
                pids: stats.pids_stats.map(|p| p.current).unwrap_or(0),
            })
        } else {
            Err(Error::MetricsNotAvailable)
        }
    }
    
    pub async fn monitor_containers(&self) -> Result<Vec<ContainerMetrics>> {
        let containers = self.docker_client.list_containers(false).await?;
        let mut metrics = Vec::new();
        
        for container in containers {
            if let Ok(container_metrics) = self.collect_container_metrics(&container.id).await {
                metrics.push(container_metrics);
                
                // Check for alerts
                self.check_alerts(&container_metrics).await?;
            }
        }
        
        Ok(metrics)
    }
    
    async fn check_alerts(&self, metrics: &ContainerMetrics) -> Result<()> {
        // CPU usage alert
        if metrics.cpu_usage > self.config.cpu_alert_threshold {
            self.alert_manager.send_alert(Alert {
                alert_type: AlertType::HighCpuUsage,
                container_id: metrics.container_id.clone(),
                message: format!("High CPU usage: {:.2}%", metrics.cpu_usage),
                severity: AlertSeverity::Warning,
                timestamp: Utc::now(),
            }).await?;
        }
        
        // Memory usage alert
        if metrics.memory_usage.percentage > self.config.memory_alert_threshold {
            self.alert_manager.send_alert(Alert {
                alert_type: AlertType::HighMemoryUsage,
                container_id: metrics.container_id.clone(),
                message: format!("High memory usage: {:.2}%", metrics.memory_usage.percentage),
                severity: AlertSeverity::Warning,
                timestamp: Utc::now(),
            }).await?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetrics {
    pub container_id: String,
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: MemoryUsage,
    pub network_io: NetworkIO,
    pub disk_io: DiskIO,
    pub pids: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub used: u64,
    pub limit: u64,
    pub percentage: f64,
}
```

## 🔒 Container Security

**File**: `security.rs`

Container security scanning and vulnerability assessment.

### Security Scanner

```rust
pub struct ContainerSecurityScanner {
    vulnerability_db: VulnerabilityDatabase,
    config: SecurityConfig,
}

impl ContainerSecurityScanner {
    pub async fn scan_image(&self, image: &str) -> Result<ImageScanResult> {
        let mut vulnerabilities = Vec::new();
        
        // Scan for known vulnerabilities
        vulnerabilities.extend(self.scan_image_vulnerabilities(image).await?);
        
        // Check image configuration
        vulnerabilities.extend(self.scan_image_configuration(image).await?);
        
        // Check for secrets in image
        vulnerabilities.extend(self.scan_for_secrets(image).await?);
        
        // Calculate security score
        let security_score = self.calculate_security_score(&vulnerabilities);
        
        Ok(ImageScanResult {
            image: image.to_string(),
            scan_time: Utc::now(),
            vulnerabilities,
            security_score,
            recommendations: self.generate_recommendations(&vulnerabilities),
        })
    }
    
    pub async fn scan_container_runtime(
        &self,
        container_id: &str
    ) -> Result<RuntimeScanResult> {
        let mut security_issues = Vec::new();
        
        // Check container privileges
        security_issues.extend(self.check_container_privileges(container_id).await?);
        
        // Check network security
        security_issues.extend(self.check_network_security(container_id).await?);
        
        // Check file system security
        security_issues.extend(self.check_filesystem_security(container_id).await?);
        
        // Check resource limits
        security_issues.extend(self.check_resource_limits(container_id).await?);
        
        Ok(RuntimeScanResult {
            container_id: container_id.to_string(),
            scan_time: Utc::now(),
            security_issues,
            compliance_status: self.check_compliance(&security_issues),
        })
    }
    
    async fn check_container_privileges(
        &self,
        container_id: &str
    ) -> Result<Vec<SecurityIssue>> {
        let mut issues = Vec::new();
        
        // Check if running as root
        if self.is_running_as_root(container_id).await? {
            issues.push(SecurityIssue {
                issue_type: SecurityIssueType::PrivilegedExecution,
                severity: SecuritySeverity::High,
                description: "Container is running as root user".to_string(),
                recommendation: "Use a non-root user in the container".to_string(),
            });
        }
        
        // Check for privileged mode
        if self.is_privileged_mode(container_id).await? {
            issues.push(SecurityIssue {
                issue_type: SecurityIssueType::PrivilegedMode,
                severity: SecuritySeverity::Critical,
                description: "Container is running in privileged mode".to_string(),
                recommendation: "Remove --privileged flag and use specific capabilities".to_string(),
            });
        }
        
        Ok(issues)
    }
}
```

## 📋 Docker Compose Integration

**File**: `compose.rs`

Docker Compose file management and orchestration.

### Compose Manager

```rust
pub struct ComposeManager {
    docker_client: DockerClient,
    compose_files: HashMap<String, ComposeFile>,
}

impl ComposeManager {
    pub async fn deploy_compose(
        &self,
        compose_file_path: &Path,
        project_name: Option<String>
    ) -> Result<ComposeDeployment> {
        let compose_content = fs::read_to_string(compose_file_path).await?;
        let compose_file: ComposeFile = serde_yaml::from_str(&compose_content)?;
        
        let project = project_name.unwrap_or_else(|| {
            compose_file_path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("default")
                .to_string()
        });
        
        // Create networks
        for (network_name, network_config) in &compose_file.networks {
            self.create_compose_network(&project, network_name, network_config).await?;
        }
        
        // Create volumes
        for (volume_name, volume_config) in &compose_file.volumes {
            self.create_compose_volume(&project, volume_name, volume_config).await?;
        }
        
        // Deploy services
        let mut deployed_services = Vec::new();
        for (service_name, service_config) in &compose_file.services {
            let container_id = self.deploy_compose_service(
                &project,
                service_name,
                service_config
            ).await?;
            
            deployed_services.push(DeployedService {
                name: service_name.clone(),
                container_id,
                status: ServiceStatus::Running,
            });
        }
        
        Ok(ComposeDeployment {
            project_name: project,
            services: deployed_services,
            networks: compose_file.networks.keys().cloned().collect(),
            volumes: compose_file.volumes.keys().cloned().collect(),
        })
    }
    
    pub async fn scale_compose_service(
        &self,
        project_name: &str,
        service_name: &str,
        replicas: u32
    ) -> Result<()> {
        // Implementation for scaling compose services
        Ok(())
    }
    
    pub async fn stop_compose_project(&self, project_name: &str) -> Result<()> {
        let containers = self.get_project_containers(project_name).await?;
        
        for container in containers {
            self.docker_client.stop_container(&container.id, Some(Duration::from_secs(10))).await?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, ComposeService>,
    pub networks: HashMap<String, ComposeNetwork>,
    pub volumes: HashMap<String, ComposeVolume>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeService {
    pub image: Option<String>,
    pub build: Option<ComposeBuild>,
    pub ports: Option<Vec<String>>,
    pub environment: Option<HashMap<String, String>>,
    pub volumes: Option<Vec<String>>,
    pub depends_on: Option<Vec<String>>,
    pub networks: Option<Vec<String>>,
    pub restart: Option<String>,
    pub deploy: Option<ComposeDeploy>,
}
```

## 🔧 Configuration

### Container Configuration

```toml
[containers]
enabled = true
docker_socket = "/var/run/docker.sock"
default_network = "tuxpilot"
auto_cleanup = true

[containers.monitoring]
enabled = true
interval = "30s"
cpu_alert_threshold = 80.0
memory_alert_threshold = 85.0
disk_alert_threshold = 90.0

[containers.security]
scan_images = true
scan_on_pull = true
vulnerability_db_update = "daily"
compliance_frameworks = ["cis", "nist"]

[containers.orchestration]
max_concurrent_deployments = 5
deployment_timeout = "10m"
health_check_timeout = "2m"
auto_restart_failed = true

[containers.compose]
default_project_name = "tuxpilot"
auto_pull_images = true
remove_orphans = true
```

## 🚀 Usage Examples

### Container Management

```bash
# List containers
tuxpilot containers list

# Deploy application
tuxpilot containers deploy --file app.yaml

# Scale service
tuxpilot containers scale web-service --replicas 3

# Monitor containers
tuxpilot containers monitor --real-time

# Security scan
tuxpilot containers scan --image nginx:latest
```

### Docker Compose Integration

```bash
# Deploy compose application
tuxpilot compose up --file docker-compose.yml

# Scale compose service
tuxpilot compose scale web=3

# View compose logs
tuxpilot compose logs --follow

# Stop compose application
tuxpilot compose down --project myapp
```

### AI-Powered Container Management

```bash
tuxpilot chat
> "Deploy a web application with nginx and redis"
> "Scale my web service to handle more traffic"
> "Check the security of my running containers"
> "Optimize container resource usage"
> "Set up monitoring for my containers"
```
