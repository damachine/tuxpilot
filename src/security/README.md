# TuxPilot Security Framework

The TuxPilot security framework provides comprehensive security scanning, hardening, compliance checking, and threat detection for Linux systems. It implements industry-standard security practices and automated security management.

## 🏗️ Security Architecture

```
Security Framework Architecture:
├── 🛡️ Security Scanner (Vulnerability detection)
├── 🔒 System Hardening (Security configuration)
├── 📋 Compliance Checker (Standards compliance)
├── 🚨 Threat Detection (Real-time monitoring)
├── 🔐 Access Control (Permission management)
└── 📊 Security Reporting (Audit and reports)
```

## 🛡️ Security Scanner

**File**: `scanner.rs`

Comprehensive vulnerability scanning and security assessment system.

### Scanner Implementation

```rust
pub struct SecurityScanner {
    config: SecurityConfig,
    linux_integration: LinuxIntegration,
    vulnerability_db: VulnerabilityDatabase,
    scan_history: Vec<ScanResult>,
}

pub struct ScanResult {
    pub scan_id: String,
    pub timestamp: DateTime<Utc>,
    pub scan_type: ScanType,
    pub vulnerabilities: Vec<Vulnerability>,
    pub security_score: f64,
    pub recommendations: Vec<SecurityRecommendation>,
    pub execution_time: Duration,
}

#[derive(Debug, Clone)]
pub enum ScanType {
    Quick,           // Basic security check
    Comprehensive,   // Full system scan
    Targeted(Vec<String>), // Specific components
    Compliance(ComplianceFramework), // Standards-based scan
}
```

### Vulnerability Detection

```rust
#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub id: String,
    pub cve_id: Option<String>,
    pub severity: VulnerabilitySeverity,
    pub title: String,
    pub description: String,
    pub affected_component: String,
    pub affected_version: Option<String>,
    pub fixed_version: Option<String>,
    pub cvss_score: Option<f64>,
    pub exploit_available: bool,
    pub patch_available: bool,
    pub remediation: Vec<RemediationStep>,
}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum VulnerabilitySeverity {
    Critical = 0,
    High = 1,
    Medium = 2,
    Low = 3,
    Info = 4,
}

#[derive(Debug, Clone)]
pub struct RemediationStep {
    pub step_type: RemediationType,
    pub description: String,
    pub command: Option<String>,
    pub estimated_time: Duration,
    pub risk_level: RiskLevel,
}
```

### Scanning Capabilities

```rust
impl SecurityScanner {
    pub async fn scan_system(&self, scan_type: ScanType) -> Result<ScanResult> {
        let start_time = Instant::now();
        let scan_id = Uuid::new_v4().to_string();
        
        let mut vulnerabilities = Vec::new();
        
        // Package vulnerabilities
        vulnerabilities.extend(self.scan_packages().await?);
        
        // Configuration vulnerabilities
        vulnerabilities.extend(self.scan_configurations().await?);
        
        // Service vulnerabilities
        vulnerabilities.extend(self.scan_services().await?);
        
        // Network vulnerabilities
        vulnerabilities.extend(self.scan_network().await?);
        
        // File system vulnerabilities
        vulnerabilities.extend(self.scan_filesystem().await?);
        
        // Calculate security score
        let security_score = self.calculate_security_score(&vulnerabilities);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&vulnerabilities).await?;
        
        Ok(ScanResult {
            scan_id,
            timestamp: Utc::now(),
            scan_type,
            vulnerabilities,
            security_score,
            recommendations,
            execution_time: start_time.elapsed(),
        })
    }
    
    async fn scan_packages(&self) -> Result<Vec<Vulnerability>> {
        let installed_packages = self.linux_integration.get_installed_packages().await?;
        let mut vulnerabilities = Vec::new();
        
        for package in installed_packages {
            if let Some(vulns) = self.vulnerability_db.get_vulnerabilities(&package).await? {
                vulnerabilities.extend(vulns);
            }
        }
        
        Ok(vulnerabilities)
    }
    
    async fn scan_configurations(&self) -> Result<Vec<Vulnerability>> {
        let mut vulnerabilities = Vec::new();
        
        // SSH configuration
        vulnerabilities.extend(self.scan_ssh_config().await?);
        
        // Firewall configuration
        vulnerabilities.extend(self.scan_firewall_config().await?);
        
        // User accounts and permissions
        vulnerabilities.extend(self.scan_user_accounts().await?);
        
        // File permissions
        vulnerabilities.extend(self.scan_file_permissions().await?);
        
        Ok(vulnerabilities)
    }
}
```

## 🔒 System Hardening

**File**: `hardening.rs`

Automated system hardening based on security best practices and compliance standards.

### Hardening Framework

```rust
pub struct SystemHardening {
    config: HardeningConfig,
    linux_integration: LinuxIntegration,
    applied_hardenings: Vec<HardeningRule>,
}

#[derive(Debug, Clone)]
pub struct HardeningRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: HardeningCategory,
    pub severity: HardeningSeverity,
    pub compliance_frameworks: Vec<ComplianceFramework>,
    pub prerequisites: Vec<String>,
    pub implementation: HardeningImplementation,
    pub rollback: Option<RollbackProcedure>,
}

#[derive(Debug, Clone)]
pub enum HardeningCategory {
    Authentication,
    Authorization,
    NetworkSecurity,
    FileSystemSecurity,
    ServiceConfiguration,
    KernelSecurity,
    Logging,
    Encryption,
}

#[derive(Debug, Clone)]
pub enum HardeningImplementation {
    ConfigFile {
        path: PathBuf,
        changes: Vec<ConfigChange>,
    },
    Command {
        command: String,
        args: Vec<String>,
    },
    Service {
        name: String,
        action: ServiceAction,
    },
    FilePermissions {
        path: PathBuf,
        permissions: String,
        owner: Option<String>,
        group: Option<String>,
    },
}
```

### Built-in Hardening Rules

```rust
impl SystemHardening {
    pub fn get_default_rules() -> Vec<HardeningRule> {
        vec![
            // SSH Hardening
            HardeningRule {
                id: "ssh-disable-root-login".to_string(),
                name: "Disable SSH Root Login".to_string(),
                description: "Prevent direct root login via SSH".to_string(),
                category: HardeningCategory::Authentication,
                severity: HardeningSeverity::High,
                compliance_frameworks: vec![
                    ComplianceFramework::CIS,
                    ComplianceFramework::NIST,
                ],
                prerequisites: vec![],
                implementation: HardeningImplementation::ConfigFile {
                    path: PathBuf::from("/etc/ssh/sshd_config"),
                    changes: vec![
                        ConfigChange::Set("PermitRootLogin".to_string(), "no".to_string()),
                    ],
                },
                rollback: Some(RollbackProcedure {
                    description: "Re-enable root login".to_string(),
                    steps: vec![
                        RollbackStep::ConfigChange {
                            path: PathBuf::from("/etc/ssh/sshd_config"),
                            change: ConfigChange::Set("PermitRootLogin".to_string(), "yes".to_string()),
                        },
                    ],
                }),
            },
            
            // Firewall Configuration
            HardeningRule {
                id: "firewall-enable-ufw".to_string(),
                name: "Enable UFW Firewall".to_string(),
                description: "Enable and configure UFW firewall with default deny policy".to_string(),
                category: HardeningCategory::NetworkSecurity,
                severity: HardeningSeverity::High,
                compliance_frameworks: vec![ComplianceFramework::CIS],
                prerequisites: vec![],
                implementation: HardeningImplementation::Command {
                    command: "ufw".to_string(),
                    args: vec!["--force", "enable"].iter().map(|s| s.to_string()).collect(),
                },
                rollback: Some(RollbackProcedure {
                    description: "Disable UFW firewall".to_string(),
                    steps: vec![
                        RollbackStep::Command {
                            command: "ufw".to_string(),
                            args: vec!["--force", "disable"].iter().map(|s| s.to_string()).collect(),
                        },
                    ],
                }),
            },
            
            // File Permissions
            HardeningRule {
                id: "secure-tmp-permissions".to_string(),
                name: "Secure /tmp Directory Permissions".to_string(),
                description: "Set secure permissions on /tmp directory".to_string(),
                category: HardeningCategory::FileSystemSecurity,
                severity: HardeningSeverity::Medium,
                compliance_frameworks: vec![ComplianceFramework::CIS],
                prerequisites: vec![],
                implementation: HardeningImplementation::FilePermissions {
                    path: PathBuf::from("/tmp"),
                    permissions: "1777".to_string(),
                    owner: Some("root".to_string()),
                    group: Some("root".to_string()),
                },
                rollback: None, // Generally safe, no rollback needed
            },
        ]
    }
    
    pub async fn apply_hardening(&mut self, rule_ids: &[String]) -> Result<HardeningResult> {
        let mut results = Vec::new();
        
        for rule_id in rule_ids {
            if let Some(rule) = self.get_rule(rule_id) {
                let result = self.apply_rule(rule).await?;
                results.push(result);
            }
        }
        
        Ok(HardeningResult {
            applied_rules: results,
            overall_success: results.iter().all(|r| r.success),
            security_improvement: self.calculate_security_improvement(&results),
        })
    }
}
```

## 📋 Compliance Checker

**File**: `compliance.rs`

Automated compliance checking against industry standards and frameworks.

### Compliance Frameworks

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceFramework {
    CIS,           // Center for Internet Security
    NIST,          // NIST Cybersecurity Framework
    PCI_DSS,       // Payment Card Industry Data Security Standard
    HIPAA,         // Health Insurance Portability and Accountability Act
    SOX,           // Sarbanes-Oxley Act
    ISO27001,      // ISO/IEC 27001
    Custom(String), // Custom compliance framework
}

pub struct ComplianceChecker {
    frameworks: HashMap<ComplianceFramework, ComplianceRuleset>,
    linux_integration: LinuxIntegration,
    check_history: Vec<ComplianceReport>,
}

#[derive(Debug, Clone)]
pub struct ComplianceRuleset {
    pub framework: ComplianceFramework,
    pub version: String,
    pub rules: Vec<ComplianceRule>,
    pub categories: Vec<ComplianceCategory>,
}

#[derive(Debug, Clone)]
pub struct ComplianceRule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub severity: ComplianceSeverity,
    pub check: ComplianceCheck,
    pub remediation: Option<String>,
}
```

### Compliance Checking

```rust
impl ComplianceChecker {
    pub async fn check_compliance(
        &self, 
        framework: ComplianceFramework
    ) -> Result<ComplianceReport> {
        let ruleset = self.frameworks.get(&framework)
            .ok_or_else(|| Error::UnsupportedFramework(framework.clone()))?;
            
        let mut results = Vec::new();
        
        for rule in &ruleset.rules {
            let result = self.check_rule(rule).await?;
            results.push(result);
        }
        
        let compliance_score = self.calculate_compliance_score(&results);
        
        Ok(ComplianceReport {
            framework: framework.clone(),
            timestamp: Utc::now(),
            overall_score: compliance_score,
            rule_results: results,
            recommendations: self.generate_compliance_recommendations(&results),
        })
    }
    
    async fn check_rule(&self, rule: &ComplianceRule) -> Result<ComplianceRuleResult> {
        match &rule.check {
            ComplianceCheck::FileExists(path) => {
                let exists = path.exists();
                Ok(ComplianceRuleResult {
                    rule_id: rule.id.clone(),
                    status: if exists { ComplianceStatus::Pass } else { ComplianceStatus::Fail },
                    details: format!("File {} {}", path.display(), 
                        if exists { "exists" } else { "does not exist" }),
                    evidence: None,
                })
            }
            ComplianceCheck::FilePermissions(path, expected_perms) => {
                let actual_perms = self.linux_integration.get_file_permissions(path).await?;
                let compliant = actual_perms == *expected_perms;
                
                Ok(ComplianceRuleResult {
                    rule_id: rule.id.clone(),
                    status: if compliant { ComplianceStatus::Pass } else { ComplianceStatus::Fail },
                    details: format!("File {} permissions: expected {}, actual {}", 
                        path.display(), expected_perms, actual_perms),
                    evidence: Some(format!("ls -la {}", path.display())),
                })
            }
            ComplianceCheck::ServiceRunning(service_name) => {
                let running = self.linux_integration.is_service_running(service_name).await?;
                
                Ok(ComplianceRuleResult {
                    rule_id: rule.id.clone(),
                    status: if running { ComplianceStatus::Pass } else { ComplianceStatus::Fail },
                    details: format!("Service {} is {}", service_name, 
                        if running { "running" } else { "not running" }),
                    evidence: Some(format!("systemctl status {}", service_name)),
                })
            }
            ComplianceCheck::ConfigValue(file_path, key, expected_value) => {
                let actual_value = self.linux_integration
                    .get_config_value(file_path, key).await?;
                let compliant = actual_value.as_deref() == Some(expected_value);
                
                Ok(ComplianceRuleResult {
                    rule_id: rule.id.clone(),
                    status: if compliant { ComplianceStatus::Pass } else { ComplianceStatus::Fail },
                    details: format!("Config {}:{} expected '{}', actual '{}'", 
                        file_path.display(), key, expected_value, 
                        actual_value.unwrap_or_else(|| "not set".to_string())),
                    evidence: Some(format!("grep {} {}", key, file_path.display())),
                })
            }
        }
    }
}
```

## 🚨 Threat Detection

**File**: `monitoring.rs`

Real-time threat detection and security monitoring system.

### Threat Detection Engine

```rust
pub struct ThreatDetectionEngine {
    detectors: Vec<Box<dyn ThreatDetector>>,
    alert_manager: AlertManager,
    config: ThreatDetectionConfig,
    linux_integration: LinuxIntegration,
}

pub trait ThreatDetector: Send + Sync {
    fn name(&self) -> &str;
    fn threat_types(&self) -> &[ThreatType];
    
    async fn detect(&self, context: &DetectionContext) -> Result<Vec<ThreatAlert>>;
    fn confidence_threshold(&self) -> f64;
}

#[derive(Debug, Clone)]
pub enum ThreatType {
    Intrusion,
    Malware,
    UnauthorizedAccess,
    PrivilegeEscalation,
    DataExfiltration,
    DenialOfService,
    SuspiciousActivity,
}

#[derive(Debug, Clone)]
pub struct ThreatAlert {
    pub id: String,
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub confidence: f64,
    pub description: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub indicators: Vec<ThreatIndicator>,
    pub recommended_actions: Vec<String>,
}
```

### Built-in Threat Detectors

```rust
pub struct LoginAnomalyDetector {
    baseline: LoginBaseline,
}

impl ThreatDetector for LoginAnomalyDetector {
    fn name(&self) -> &str { "Login Anomaly Detector" }
    
    fn threat_types(&self) -> &[ThreatType] {
        &[ThreatType::UnauthorizedAccess, ThreatType::Intrusion]
    }
    
    async fn detect(&self, context: &DetectionContext) -> Result<Vec<ThreatAlert>> {
        let recent_logins = context.linux_integration.get_recent_logins().await?;
        let mut alerts = Vec::new();
        
        for login in recent_logins {
            // Check for unusual login times
            if self.is_unusual_time(&login) {
                alerts.push(ThreatAlert {
                    id: Uuid::new_v4().to_string(),
                    threat_type: ThreatType::SuspiciousActivity,
                    severity: ThreatSeverity::Medium,
                    confidence: 0.7,
                    description: format!("Unusual login time for user {}", login.username),
                    source: "login_logs".to_string(),
                    timestamp: Utc::now(),
                    indicators: vec![
                        ThreatIndicator::UnusualLoginTime(login.timestamp),
                        ThreatIndicator::Username(login.username.clone()),
                    ],
                    recommended_actions: vec![
                        "Verify user activity".to_string(),
                        "Check for unauthorized access".to_string(),
                    ],
                });
            }
            
            // Check for unusual source IPs
            if self.is_unusual_source(&login) {
                alerts.push(ThreatAlert {
                    id: Uuid::new_v4().to_string(),
                    threat_type: ThreatType::UnauthorizedAccess,
                    severity: ThreatSeverity::High,
                    confidence: 0.8,
                    description: format!("Login from unusual IP: {}", login.source_ip),
                    source: "login_logs".to_string(),
                    timestamp: Utc::now(),
                    indicators: vec![
                        ThreatIndicator::SourceIP(login.source_ip),
                        ThreatIndicator::Username(login.username.clone()),
                    ],
                    recommended_actions: vec![
                        "Block suspicious IP".to_string(),
                        "Force password reset".to_string(),
                        "Enable 2FA".to_string(),
                    ],
                });
            }
        }
        
        Ok(alerts)
    }
}

pub struct ProcessAnomalyDetector {
    baseline: ProcessBaseline,
}

impl ThreatDetector for ProcessAnomalyDetector {
    fn name(&self) -> &str { "Process Anomaly Detector" }
    
    fn threat_types(&self) -> &[ThreatType] {
        &[ThreatType::Malware, ThreatType::PrivilegeEscalation]
    }
    
    async fn detect(&self, context: &DetectionContext) -> Result<Vec<ThreatAlert>> {
        let running_processes = context.linux_integration.get_processes().await?;
        let mut alerts = Vec::new();
        
        for process in running_processes {
            // Check for suspicious process names
            if self.is_suspicious_process_name(&process.name) {
                alerts.push(ThreatAlert {
                    id: Uuid::new_v4().to_string(),
                    threat_type: ThreatType::Malware,
                    severity: ThreatSeverity::High,
                    confidence: 0.9,
                    description: format!("Suspicious process detected: {}", process.name),
                    source: "process_monitor".to_string(),
                    timestamp: Utc::now(),
                    indicators: vec![
                        ThreatIndicator::ProcessName(process.name.clone()),
                        ThreatIndicator::ProcessPID(process.pid),
                    ],
                    recommended_actions: vec![
                        "Terminate suspicious process".to_string(),
                        "Scan for malware".to_string(),
                        "Isolate system".to_string(),
                    ],
                });
            }
            
            // Check for privilege escalation attempts
            if self.is_privilege_escalation(&process) {
                alerts.push(ThreatAlert {
                    id: Uuid::new_v4().to_string(),
                    threat_type: ThreatType::PrivilegeEscalation,
                    severity: ThreatSeverity::Critical,
                    confidence: 0.85,
                    description: format!("Potential privilege escalation: {}", process.name),
                    source: "process_monitor".to_string(),
                    timestamp: Utc::now(),
                    indicators: vec![
                        ThreatIndicator::ProcessName(process.name.clone()),
                        ThreatIndicator::UserID(process.uid),
                        ThreatIndicator::ProcessPID(process.pid),
                    ],
                    recommended_actions: vec![
                        "Investigate process activity".to_string(),
                        "Check user permissions".to_string(),
                        "Review audit logs".to_string(),
                    ],
                });
            }
        }
        
        Ok(alerts)
    }
}
```

## 🔧 Configuration

### Security Configuration

```toml
[security]
enabled = true
scan_interval = "1h"
auto_hardening = false
compliance_frameworks = ["cis", "nist"]

[security.scanning]
auto_scan = true
scan_schedule = "daily:02:00"
scan_types = ["packages", "configurations", "services"]
vulnerability_db_update = "daily"

[security.hardening]
auto_apply = false
backup_before_hardening = true
rollback_on_failure = true
hardening_categories = ["authentication", "network", "filesystem"]

[security.compliance]
auto_check = true
check_schedule = "weekly:sunday:03:00"
frameworks = ["cis", "nist"]
generate_reports = true

[security.monitoring]
real_time_detection = true
alert_threshold = "medium"
log_retention_days = 90
enable_threat_intelligence = true

[security.alerts]
email_notifications = true
syslog_integration = true
webhook_url = ""
alert_cooldown = "5m"
```

## 🚀 Usage Examples

### Security Scanning

```bash
# Quick security scan
tuxpilot security scan --type quick

# Comprehensive security scan
tuxpilot security scan --type comprehensive

# Scan specific components
tuxpilot security scan --target packages,services

# Compliance scan
tuxpilot security scan --compliance cis
```

### System Hardening

```bash
# Apply CIS hardening rules
tuxpilot security harden --framework cis

# Apply specific hardening rules
tuxpilot security harden --rules ssh-disable-root,firewall-enable

# Preview hardening changes
tuxpilot security harden --dry-run --framework nist

# Rollback hardening
tuxpilot security rollback --rule ssh-disable-root
```

### Compliance Checking

```bash
# Check CIS compliance
tuxpilot security compliance --framework cis

# Generate compliance report
tuxpilot security compliance --framework nist --report

# Check specific compliance rules
tuxpilot security compliance --rules auth-001,net-002
```
