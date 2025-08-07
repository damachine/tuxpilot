# TuxPilot Automation Framework

The TuxPilot automation framework provides intelligent task scheduling, execution, and management for Linux system administration. It enables automated maintenance, updates, backups, and system optimization tasks.

## 🏗️ Architecture Overview

```
Automation Framework Architecture:
├── 🎯 Automation Orchestrator (Central Controller)
├── ⏰ Task Scheduler (Cron-like scheduling)
├── 📋 Task Management (Task lifecycle)
├── 🔄 Built-in Tasks (System maintenance)
├── 🔗 Dependency Management (Task dependencies)
└── 📊 Execution Monitoring (Performance tracking)
```

## 🎯 Automation Orchestrator

**File**: `mod.rs`

The central controller that manages all automation tasks and their execution.

### Core Structure

```rust
pub struct AutomationOrchestrator {
    tasks: HashMap<String, Box<dyn AutomationTask>>,
    scheduler: TaskScheduler,
    dependency_graph: TaskDependencyGraph,
    execution_history: Vec<TaskExecution>,
    config: Config,
    linux_integration: LinuxIntegration,
}

pub struct AutomationContext {
    pub execution_id: String,
    pub user_id: Option<String>,
    pub trigger_source: TriggerSource,
    pub system_state: SystemState,
    pub linux_integration: LinuxIntegration,
}
```

### Task Management

```rust
pub trait AutomationTask: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn task_type(&self) -> TaskType;
    fn priority(&self) -> TaskPriority;
    fn estimated_duration(&self) -> std::time::Duration;
    fn prerequisites(&self) -> Vec<String>;
    
    async fn can_execute(&self, context: &AutomationContext) -> Result<bool>;
    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult>;
    async fn rollback(&self, context: &AutomationContext) -> Result<()>;
    
    fn get_schedule(&self) -> Option<Schedule>;
}
```

### Task Types and Priorities

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Maintenance,
    Backup,
    Update,
    Security,
    Optimization,
    Monitoring,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum TaskPriority {
    Critical = 0,
    High = 1,
    Medium = 2,
    Low = 3,
    Background = 4,
}

#[derive(Debug, Clone)]
pub enum TriggerSource {
    Scheduled,
    Manual,
    Event(String),
    Condition(String),
}
```

### Execution Results

```rust
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: Duration,
    pub resources_used: ResourceUsage,
    pub changes_made: Vec<SystemChange>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
    Skipped,
}
```

## ⏰ Task Scheduler

**File**: `scheduler.rs`

Advanced scheduling system supporting cron-like expressions and complex timing patterns.

### Scheduler Implementation

```rust
pub struct TaskScheduler {
    scheduled_tasks: HashMap<String, ScheduledExecution>,
    execution_queue: VecDeque<String>,
}

#[derive(Debug, Clone)]
pub struct ScheduledExecution {
    pub task_id: String,
    pub schedule: Schedule,
    pub next_execution: DateTime<Utc>,
    pub last_execution: Option<DateTime<Utc>>,
    pub enabled: bool,
}
```

### Schedule Types

```rust
#[derive(Debug, Clone)]
pub enum Schedule {
    // Cron-like expressions
    Cron(String),
    
    // Simple intervals
    Interval(Duration),
    
    // Specific times
    Daily(NaiveTime),
    Weekly(Weekday, NaiveTime),
    Monthly(u32, NaiveTime), // day of month
    
    // Event-based
    OnBoot,
    OnShutdown,
    OnUserLogin,
    OnSystemLoad(f64), // trigger when load average exceeds threshold
    OnDiskUsage(f64),  // trigger when disk usage exceeds percentage
    
    // Conditional
    When(Condition),
}

#[derive(Debug, Clone)]
pub enum Condition {
    SystemLoad(f64),
    DiskUsage(String, f64), // path, percentage
    MemoryUsage(f64),
    ProcessExists(String),
    FileExists(String),
    ServiceRunning(String),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}
```

### Usage Examples

```rust
// Schedule daily backup at 2 AM
let backup_schedule = Schedule::Daily(NaiveTime::from_hms(2, 0, 0));

// Schedule system cleanup every 6 hours
let cleanup_schedule = Schedule::Interval(Duration::from_secs(6 * 3600));

// Schedule security scan on high system load
let security_schedule = Schedule::OnSystemLoad(0.8);

// Complex cron expression
let complex_schedule = Schedule::Cron("0 2 * * 1-5".to_string()); // 2 AM on weekdays
```

## 📋 Task Dependencies

**File**: `tasks.rs`

Manages task dependencies and execution order.

### Dependency Graph

```rust
pub struct TaskDependencyGraph {
    dependencies: HashMap<String, Vec<String>>,
}

impl TaskDependencyGraph {
    pub fn add_dependency(&mut self, task_id: String, depends_on: String) {
        self.dependencies.entry(task_id)
            .or_insert_with(Vec::new)
            .push(depends_on);
    }
    
    pub fn get_execution_order(&self, tasks: &[String]) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        
        for task in tasks {
            self.visit_task(task, &mut visited, &mut result)?;
        }
        
        Ok(result)
    }
}
```

### Dependency Examples

```rust
// System update must run before security scan
dependency_graph.add_dependency("security-scan".to_string(), "system-update".to_string());

// Backup must run before cleanup
dependency_graph.add_dependency("cleanup".to_string(), "backup".to_string());

// Multiple dependencies
dependency_graph.add_dependency("final-report".to_string(), "backup".to_string());
dependency_graph.add_dependency("final-report".to_string(), "security-scan".to_string());
dependency_graph.add_dependency("final-report".to_string(), "cleanup".to_string());
```

## 🔄 Built-in Automation Tasks

### System Backup Tasks

**File**: `backup.rs`

Automated backup tasks for system configuration and user data.

```rust
pub struct ConfigBackupTask {
    id: String,
    schedule: Schedule,
    backup_paths: Vec<PathBuf>,
    destination: PathBuf,
    compression: CompressionType,
    retention_days: u32,
}

impl AutomationTask for ConfigBackupTask {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { "Configuration Backup" }
    fn description(&self) -> &str {
        "Backup system configuration files and TuxPilot settings"
    }
    
    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = Instant::now();
        
        // Create backup directory
        let backup_dir = self.destination.join(format!(
            "config-backup-{}", 
            Utc::now().format("%Y%m%d-%H%M%S")
        ));
        
        fs::create_dir_all(&backup_dir).await?;
        
        // Backup each configured path
        let mut backed_up_files = Vec::new();
        for path in &self.backup_paths {
            if path.exists() {
                let dest = backup_dir.join(path.file_name().unwrap());
                fs::copy(path, &dest).await?;
                backed_up_files.push(dest);
            }
        }
        
        // Compress if enabled
        if self.compression != CompressionType::None {
            self.compress_backup(&backup_dir).await?;
        }
        
        // Clean old backups
        self.cleanup_old_backups().await?;
        
        Ok(TaskResult {
            task_id: self.id.clone(),
            status: TaskStatus::Success,
            output: format!("Backed up {} files to {}", 
                backed_up_files.len(), 
                backup_dir.display()
            ),
            error: None,
            execution_time: start_time.elapsed(),
            resources_used: ResourceUsage::default(),
            changes_made: vec![SystemChange::FileCreated(backup_dir)],
        })
    }
}
```

### System Update Tasks

**File**: `updates.rs`

Automated system and package updates with safety checks.

```rust
pub struct SystemUpdateTask {
    id: String,
    update_type: UpdateType,
    auto_reboot: bool,
    backup_before_update: bool,
    test_mode: bool,
}

#[derive(Debug, Clone)]
pub enum UpdateType {
    Security,      // Security updates only
    All,          // All available updates
    Packages(Vec<String>), // Specific packages
}

impl AutomationTask for SystemUpdateTask {
    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = Instant::now();
        
        // Create backup if enabled
        if self.backup_before_update {
            self.create_system_backup(context).await?;
        }
        
        // Check for available updates
        let available_updates = context.linux_integration
            .get_available_updates().await?;
            
        if available_updates.is_empty() {
            return Ok(TaskResult {
                task_id: self.id.clone(),
                status: TaskStatus::Success,
                output: "No updates available".to_string(),
                error: None,
                execution_time: start_time.elapsed(),
                resources_used: ResourceUsage::default(),
                changes_made: vec![],
            });
        }
        
        // Filter updates based on type
        let updates_to_install = self.filter_updates(&available_updates);
        
        // Install updates
        let mut installed_updates = Vec::new();
        for update in updates_to_install {
            if self.test_mode {
                // Simulate installation
                installed_updates.push(update.clone());
            } else {
                // Actually install
                context.linux_integration.install_update(&update).await?;
                installed_updates.push(update);
            }
        }
        
        // Reboot if required and enabled
        if self.auto_reboot && self.requires_reboot(&installed_updates) {
            context.linux_integration.schedule_reboot(Duration::from_secs(60)).await?;
        }
        
        Ok(TaskResult {
            task_id: self.id.clone(),
            status: TaskStatus::Success,
            output: format!("Installed {} updates", installed_updates.len()),
            error: None,
            execution_time: start_time.elapsed(),
            resources_used: ResourceUsage::default(),
            changes_made: installed_updates.into_iter()
                .map(|u| SystemChange::PackageUpdated(u.name))
                .collect(),
        })
    }
}
```

### System Maintenance Tasks

**File**: `maintenance.rs`

Automated system cleanup and maintenance tasks.

```rust
pub struct SystemCleanupTask {
    id: String,
    cleanup_types: Vec<CleanupType>,
    dry_run: bool,
}

#[derive(Debug, Clone)]
pub enum CleanupType {
    PackageCache,
    TempFiles,
    LogFiles,
    OrphanedPackages,
    BrokenSymlinks,
    EmptyDirectories,
    OldKernels,
    UserCache,
}

impl AutomationTask for SystemCleanupTask {
    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        let start_time = Instant::now();
        let mut cleaned_items = Vec::new();
        let mut space_freed = 0u64;
        
        for cleanup_type in &self.cleanup_types {
            match cleanup_type {
                CleanupType::PackageCache => {
                    let result = self.cleanup_package_cache(context).await?;
                    space_freed += result.space_freed;
                    cleaned_items.extend(result.items);
                }
                CleanupType::TempFiles => {
                    let result = self.cleanup_temp_files(context).await?;
                    space_freed += result.space_freed;
                    cleaned_items.extend(result.items);
                }
                CleanupType::LogFiles => {
                    let result = self.cleanup_old_logs(context).await?;
                    space_freed += result.space_freed;
                    cleaned_items.extend(result.items);
                }
                // ... other cleanup types
            }
        }
        
        Ok(TaskResult {
            task_id: self.id.clone(),
            status: TaskStatus::Success,
            output: format!(
                "Cleaned {} items, freed {} MB", 
                cleaned_items.len(),
                space_freed / 1024 / 1024
            ),
            error: None,
            execution_time: start_time.elapsed(),
            resources_used: ResourceUsage::default(),
            changes_made: cleaned_items.into_iter()
                .map(|item| SystemChange::FileDeleted(item))
                .collect(),
        })
    }
}
```

## 🔧 Configuration

### Automation Configuration

```toml
[automation]
enabled = true
max_concurrent_tasks = 3
task_timeout = "1h"
retry_failed_tasks = true
max_retries = 3

[automation.scheduling]
check_interval = "1m"
max_queued_tasks = 100
priority_scheduling = true

[automation.backup]
enabled = true
default_retention_days = 30
compression = "gzip"
backup_location = "/var/backups/tuxpilot"

[automation.updates]
auto_security_updates = true
auto_reboot = false
backup_before_update = true
update_window = "02:00-04:00"

[automation.maintenance]
auto_cleanup = true
cleanup_schedule = "weekly"
cleanup_types = ["package_cache", "temp_files", "log_files"]
```

### Task Configuration Examples

```toml
# Daily backup task
[[automation.tasks]]
id = "daily-backup"
type = "backup"
schedule = "daily:02:00"
enabled = true

[automation.tasks.daily-backup.config]
paths = ["/etc", "/home", "~/.config/tuxpilot"]
destination = "/var/backups"
compression = "gzip"
retention_days = 7

# Weekly system update
[[automation.tasks]]
id = "weekly-updates"
type = "update"
schedule = "weekly:sunday:03:00"
enabled = true

[automation.tasks.weekly-updates.config]
update_type = "security"
auto_reboot = false
backup_before_update = true

# System cleanup on high disk usage
[[automation.tasks]]
id = "emergency-cleanup"
type = "maintenance"
schedule = "condition:disk_usage:/var:90"
enabled = true

[automation.tasks.emergency-cleanup.config]
cleanup_types = ["package_cache", "temp_files", "log_files"]
aggressive_cleanup = true
```

## 📊 Monitoring and Reporting

### Execution Monitoring

```rust
pub struct TaskExecution {
    pub execution_id: String,
    pub task_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub trigger_source: TriggerSource,
    pub result: Option<TaskResult>,
    pub resource_usage: ResourceUsage,
}

pub struct AutomationMetrics {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_execution_time: Duration,
    pub tasks_by_type: HashMap<TaskType, u64>,
    pub space_freed: u64,
    pub files_backed_up: u64,
    pub updates_installed: u64,
}
```

### Reporting

```rust
impl AutomationOrchestrator {
    pub async fn generate_report(&self, period: ReportPeriod) -> Result<AutomationReport> {
        let executions = self.get_executions_in_period(period).await?;
        
        let report = AutomationReport {
            period,
            total_tasks: executions.len(),
            successful_tasks: executions.iter().filter(|e| e.status == TaskStatus::Success).count(),
            failed_tasks: executions.iter().filter(|e| e.status == TaskStatus::Failed).count(),
            total_execution_time: executions.iter()
                .map(|e| e.result.as_ref().map(|r| r.execution_time).unwrap_or_default())
                .sum(),
            space_freed: executions.iter()
                .filter_map(|e| e.result.as_ref())
                .map(|r| r.resources_used.disk_space_freed)
                .sum(),
            task_breakdown: self.calculate_task_breakdown(&executions),
        };
        
        Ok(report)
    }
}
```

## 🚀 Usage Examples

### Creating Custom Tasks

```rust
use crate::automation::{AutomationTask, AutomationContext, TaskResult, TaskStatus};

pub struct CustomMaintenanceTask {
    id: String,
    config: CustomConfig,
}

impl AutomationTask for CustomMaintenanceTask {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { "Custom Maintenance" }
    
    async fn execute(&self, context: &AutomationContext) -> Result<TaskResult> {
        // Custom implementation
        Ok(TaskResult {
            task_id: self.id.clone(),
            status: TaskStatus::Success,
            output: "Custom task completed".to_string(),
            error: None,
            execution_time: Duration::from_secs(1),
            resources_used: ResourceUsage::default(),
            changes_made: vec![],
        })
    }
}

// Register the custom task
let mut orchestrator = AutomationOrchestrator::new(config, linux_integration).await?;
orchestrator.register_task(Box::new(CustomMaintenanceTask::new())).await?;
```

### Manual Task Execution

```bash
# Execute specific task
tuxpilot automation run daily-backup

# Execute task with custom parameters
tuxpilot automation run system-update --type security --backup

# List scheduled tasks
tuxpilot automation list

# Show task status
tuxpilot automation status daily-backup

# Generate automation report
tuxpilot automation report --period last-week
```
