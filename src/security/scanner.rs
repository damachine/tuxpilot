use anyhow::Result;
use super::{SecurityFinding, SecurityCategory, Severity, RemediationStep};

/// Security scanner for system vulnerabilities
#[derive(Debug, Clone)]
pub struct SecurityScanner {
    scan_rules: Vec<ScanRule>,
}

/// Security scan rule
#[derive(Debug, Clone)]
pub struct ScanRule {
    pub rule_id: String,
    pub category: SecurityCategory,
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub check_command: String,
    pub remediation_steps: Vec<RemediationStep>,
}

impl SecurityScanner {
    pub async fn new(_config: &crate::config::Config) -> Result<Self> {
        let scan_rules = Self::initialize_scan_rules();
        Ok(Self { scan_rules })
    }

    fn initialize_scan_rules() -> Vec<ScanRule> {
        vec![
            ScanRule {
                rule_id: "SSH_ROOT_LOGIN".to_string(),
                category: SecurityCategory::Authentication,
                severity: Severity::High,
                title: "SSH Root Login Enabled".to_string(),
                description: "SSH allows direct root login, which is a security risk".to_string(),
                check_command: "grep -E '^PermitRootLogin\\s+yes' /etc/ssh/sshd_config".to_string(),
                remediation_steps: vec![
                    RemediationStep {
                        step_number: 1,
                        description: "Disable root login in SSH configuration".to_string(),
                        command: Some("sed -i 's/^PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config".to_string()),
                        verification: Some("grep '^PermitRootLogin no' /etc/ssh/sshd_config".to_string()),
                        risk_level: Severity::Low,
                        estimated_time_minutes: 2,
                    },
                    RemediationStep {
                        step_number: 2,
                        description: "Restart SSH service".to_string(),
                        command: Some("systemctl restart sshd".to_string()),
                        verification: Some("systemctl status sshd".to_string()),
                        risk_level: Severity::Medium,
                        estimated_time_minutes: 1,
                    },
                ],
            },
            ScanRule {
                rule_id: "WEAK_PASSWORDS".to_string(),
                category: SecurityCategory::Authentication,
                severity: Severity::Medium,
                title: "Weak Password Policy".to_string(),
                description: "System may allow weak passwords".to_string(),
                check_command: "test -f /etc/security/pwquality.conf".to_string(),
                remediation_steps: vec![
                    RemediationStep {
                        step_number: 1,
                        description: "Configure password quality requirements".to_string(),
                        command: Some("echo 'minlen = 12' >> /etc/security/pwquality.conf".to_string()),
                        verification: Some("grep 'minlen = 12' /etc/security/pwquality.conf".to_string()),
                        risk_level: Severity::Low,
                        estimated_time_minutes: 5,
                    },
                ],
            },
            ScanRule {
                rule_id: "UNENCRYPTED_SERVICES".to_string(),
                category: SecurityCategory::NetworkSecurity,
                severity: Severity::High,
                title: "Unencrypted Network Services".to_string(),
                description: "Services running without encryption detected".to_string(),
                check_command: "netstat -tlnp | grep -E ':(21|23|80|110|143|993|995)\\s'".to_string(),
                remediation_steps: vec![
                    RemediationStep {
                        step_number: 1,
                        description: "Review and secure unencrypted services".to_string(),
                        command: Some("netstat -tlnp".to_string()),
                        verification: None,
                        risk_level: Severity::Medium,
                        estimated_time_minutes: 15,
                    },
                ],
            },
            ScanRule {
                rule_id: "WORLD_WRITABLE_FILES".to_string(),
                category: SecurityCategory::FileSystemSecurity,
                severity: Severity::Medium,
                title: "World-Writable Files".to_string(),
                description: "Files with world-write permissions found".to_string(),
                check_command: "find /etc /usr /var -type f -perm -002 2>/dev/null | head -10".to_string(),
                remediation_steps: vec![
                    RemediationStep {
                        step_number: 1,
                        description: "Review and fix file permissions".to_string(),
                        command: Some("find /etc /usr /var -type f -perm -002 -exec chmod o-w {} \\;".to_string()),
                        verification: Some("find /etc /usr /var -type f -perm -002 2>/dev/null | wc -l".to_string()),
                        risk_level: Severity::Low,
                        estimated_time_minutes: 10,
                    },
                ],
            },
            ScanRule {
                rule_id: "SUID_SGID_FILES".to_string(),
                category: SecurityCategory::FileSystemSecurity,
                severity: Severity::Medium,
                title: "Excessive SUID/SGID Files".to_string(),
                description: "Unusual SUID/SGID files detected".to_string(),
                check_command: "find /usr /bin /sbin -type f \\( -perm -4000 -o -perm -2000 \\) 2>/dev/null".to_string(),
                remediation_steps: vec![
                    RemediationStep {
                        step_number: 1,
                        description: "Review SUID/SGID files for necessity".to_string(),
                        command: Some("find /usr /bin /sbin -type f \\( -perm -4000 -o -perm -2000 \\) -ls".to_string()),
                        verification: None,
                        risk_level: Severity::Low,
                        estimated_time_minutes: 20,
                    },
                ],
            },
        ]
    }

    pub async fn perform_security_scan(&self) -> Result<Vec<SecurityFinding>> {
        println!("🔍 Performing comprehensive security scan...");
        let mut findings = Vec::new();

        for rule in &self.scan_rules {
            if let Ok(finding) = self.execute_scan_rule(rule).await {
                if let Some(f) = finding {
                    findings.push(f);
                }
            }
        }

        println!("🔍 Security scan completed: {} findings", findings.len());
        Ok(findings)
    }

    pub async fn quick_scan(&self) -> Result<Vec<SecurityFinding>> {
        println!("⚡ Performing quick security scan...");
        let mut findings = Vec::new();

        // Only run high-priority rules for quick scan
        let high_priority_rules: Vec<&ScanRule> = self.scan_rules
            .iter()
            .filter(|rule| matches!(rule.severity, Severity::Critical | Severity::High))
            .collect();

        for rule in high_priority_rules {
            if let Ok(finding) = self.execute_scan_rule(rule).await {
                if let Some(f) = finding {
                    findings.push(f);
                }
            }
        }

        println!("⚡ Quick scan completed: {} findings", findings.len());
        Ok(findings)
    }

    async fn execute_scan_rule(&self, rule: &ScanRule) -> Result<Option<SecurityFinding>> {
        // Execute the check command
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&rule.check_command)
            .output()
            .await?;

        // If command succeeds (exit code 0), it means the issue was found
        if output.status.success() && !output.stdout.is_empty() {
            let finding = SecurityFinding {
                finding_id: uuid::Uuid::new_v4().to_string(),
                category: rule.category.clone(),
                severity: rule.severity.clone(),
                title: rule.title.clone(),
                description: rule.description.clone(),
                affected_components: vec!["system".to_string()],
                risk_score: self.calculate_risk_score(&rule.severity),
                remediation: rule.remediation_steps.clone(),
                references: vec![
                    "https://www.cisecurity.org/".to_string(),
                    "https://nvd.nist.gov/".to_string(),
                ],
                detected_at: chrono::Utc::now(),
            };

            return Ok(Some(finding));
        }

        Ok(None)
    }

    fn calculate_risk_score(&self, severity: &Severity) -> f32 {
        match severity {
            Severity::Critical => 9.0,
            Severity::High => 7.0,
            Severity::Medium => 5.0,
            Severity::Low => 3.0,
            Severity::Info => 1.0,
        }
    }

    pub async fn scan_network_services(&self) -> Result<Vec<SecurityFinding>> {
        println!("🌐 Scanning network services...");
        let mut findings = Vec::new();

        // Check for open ports
        let output = tokio::process::Command::new("netstat")
            .args(&["-tlnp"])
            .output()
            .await?;

        let netstat_output = String::from_utf8_lossy(&output.stdout);
        
        // Look for potentially risky services
        let risky_ports = vec![
            (21, "FTP"),
            (23, "Telnet"),
            (53, "DNS"),
            (80, "HTTP"),
            (110, "POP3"),
            (143, "IMAP"),
            (993, "IMAPS"),
            (995, "POP3S"),
        ];

        for (port, service) in risky_ports {
            if netstat_output.contains(&format!(":{}", port)) {
                findings.push(SecurityFinding {
                    finding_id: uuid::Uuid::new_v4().to_string(),
                    category: SecurityCategory::NetworkSecurity,
                    severity: if port == 21 || port == 23 { Severity::High } else { Severity::Medium },
                    title: format!("{} Service Running", service),
                    description: format!("{} service detected on port {}", service, port),
                    affected_components: vec![format!("port_{}", port)],
                    risk_score: if port == 21 || port == 23 { 7.0 } else { 4.0 },
                    remediation: vec![
                        RemediationStep {
                            step_number: 1,
                            description: format!("Review {} service necessity", service),
                            command: Some(format!("systemctl status $(lsof -ti:{}) 2>/dev/null || echo 'Service check'", port)),
                            verification: None,
                            risk_level: Severity::Low,
                            estimated_time_minutes: 5,
                        },
                    ],
                    references: vec!["https://www.iana.org/assignments/service-names-port-numbers/".to_string()],
                    detected_at: chrono::Utc::now(),
                });
            }
        }

        Ok(findings)
    }

    pub async fn scan_file_permissions(&self) -> Result<Vec<SecurityFinding>> {
        println!("📁 Scanning file permissions...");
        let mut findings = Vec::new();

        // Check for world-writable files in critical directories
        let output = tokio::process::Command::new("find")
            .args(&["/etc", "/usr", "/var", "-type", "f", "-perm", "-002", "2>/dev/null"])
            .output()
            .await?;

        if output.status.success() && !output.stdout.is_empty() {
            let files = String::from_utf8_lossy(&output.stdout);
            let file_count = files.lines().count();

            if file_count > 0 {
                findings.push(SecurityFinding {
                    finding_id: uuid::Uuid::new_v4().to_string(),
                    category: SecurityCategory::FileSystemSecurity,
                    severity: Severity::Medium,
                    title: "World-Writable Files Found".to_string(),
                    description: format!("Found {} world-writable files in critical directories", file_count),
                    affected_components: files.lines().take(10).map(|s| s.to_string()).collect(),
                    risk_score: 5.0,
                    remediation: vec![
                        RemediationStep {
                            step_number: 1,
                            description: "Remove world-write permissions".to_string(),
                            command: Some("find /etc /usr /var -type f -perm -002 -exec chmod o-w {} \\;".to_string()),
                            verification: Some("find /etc /usr /var -type f -perm -002 2>/dev/null | wc -l".to_string()),
                            risk_level: Severity::Low,
                            estimated_time_minutes: 10,
                        },
                    ],
                    references: vec!["https://www.cyberciti.biz/tips/understanding-linux-unix-umask-value-usage.html".to_string()],
                    detected_at: chrono::Utc::now(),
                });
            }
        }

        Ok(findings)
    }
}
