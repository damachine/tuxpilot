use anyhow::Result;
use std::collections::HashMap;
use super::{ComplianceStatus, ControlResult, ControlStatus};

/// Compliance checker for security frameworks
#[derive(Debug, Clone)]
pub struct ComplianceChecker {
    frameworks: HashMap<String, ComplianceFramework>,
}

/// Compliance framework definition
#[derive(Debug, Clone)]
pub struct ComplianceFramework {
    pub name: String,
    pub version: String,
    pub controls: Vec<ComplianceControl>,
}

/// Individual compliance control
#[derive(Debug, Clone)]
pub struct ComplianceControl {
    pub control_id: String,
    pub title: String,
    pub description: String,
    pub check_command: String,
    pub expected_result: String,
}

impl ComplianceChecker {
    pub async fn new() -> Result<Self> {
        let mut frameworks = HashMap::new();
        
        // Add CIS Controls
        frameworks.insert("CIS".to_string(), Self::create_cis_framework());
        
        // Add NIST framework
        frameworks.insert("NIST".to_string(), Self::create_nist_framework());

        Ok(Self { frameworks })
    }

    fn create_cis_framework() -> ComplianceFramework {
        ComplianceFramework {
            name: "CIS Controls".to_string(),
            version: "8.0".to_string(),
            controls: vec![
                ComplianceControl {
                    control_id: "CIS-1.1".to_string(),
                    title: "Maintain Inventory of Authorized Software".to_string(),
                    description: "Maintain an inventory of authorized software".to_string(),
                    check_command: "dpkg -l | wc -l".to_string(),
                    expected_result: "numeric".to_string(),
                },
                ComplianceControl {
                    control_id: "CIS-4.1".to_string(),
                    title: "Secure Configuration of Network Infrastructure".to_string(),
                    description: "Establish secure configurations for network devices".to_string(),
                    check_command: "iptables -L | grep -c Chain".to_string(),
                    expected_result: "numeric".to_string(),
                },
            ],
        }
    }

    fn create_nist_framework() -> ComplianceFramework {
        ComplianceFramework {
            name: "NIST Cybersecurity Framework".to_string(),
            version: "1.1".to_string(),
            controls: vec![
                ComplianceControl {
                    control_id: "NIST-ID.AM-1".to_string(),
                    title: "Physical devices and systems are inventoried".to_string(),
                    description: "Maintain inventory of physical devices".to_string(),
                    check_command: "lscpu | grep 'CPU(s):' | head -1".to_string(),
                    expected_result: "numeric".to_string(),
                },
            ],
        }
    }

    pub async fn check_compliance(&self) -> Result<HashMap<String, ComplianceStatus>> {
        let mut compliance_results = HashMap::new();

        for (framework_name, framework) in &self.frameworks {
            let status = self.check_framework_compliance(framework).await?;
            compliance_results.insert(framework_name.clone(), status);
        }

        Ok(compliance_results)
    }

    pub async fn check_framework(&self, framework_name: &str) -> Result<ComplianceStatus> {
        if let Some(framework) = self.frameworks.get(framework_name) {
            self.check_framework_compliance(framework).await
        } else {
            Err(anyhow::anyhow!("Framework not found: {}", framework_name))
        }
    }

    async fn check_framework_compliance(&self, framework: &ComplianceFramework) -> Result<ComplianceStatus> {
        let mut control_results = HashMap::new();
        let mut passed_controls = 0;
        let mut failed_controls = 0;
        let mut not_applicable = 0;

        for control in &framework.controls {
            let result = self.check_control(control).await?;
            
            match result.status {
                ControlStatus::Pass => passed_controls += 1,
                ControlStatus::Fail => failed_controls += 1,
                ControlStatus::NotApplicable => not_applicable += 1,
                ControlStatus::Manual => {} // Don't count manual checks
            }

            control_results.insert(control.control_id.clone(), result);
        }

        let total_applicable = passed_controls + failed_controls;
        let overall_compliance = if total_applicable > 0 {
            (passed_controls as f32 / total_applicable as f32) * 100.0
        } else {
            0.0
        };

        Ok(ComplianceStatus {
            framework: framework.name.clone(),
            version: framework.version.clone(),
            overall_compliance,
            passed_controls,
            failed_controls,
            not_applicable,
            control_results,
        })
    }

    async fn check_control(&self, control: &ComplianceControl) -> Result<ControlResult> {
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&control.check_command)
            .output()
            .await?;

        let status = if output.status.success() {
            ControlStatus::Pass
        } else {
            ControlStatus::Fail
        };

        let evidence = vec![
            format!("Command: {}", control.check_command),
            format!("Output: {}", String::from_utf8_lossy(&output.stdout)),
        ];

        let remediation_required = matches!(status, ControlStatus::Fail);

        Ok(ControlResult {
            control_id: control.control_id.clone(),
            status,
            evidence,
            remediation_required,
        })
    }
}
