use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub mod scanner;
pub mod compliance;
pub mod vulnerability;
pub mod hardening;
pub mod audit;

use crate::config::Config;
use crate::linux_integration::LinuxIntegration;

/// Security and compliance framework
#[derive(Debug, Clone)]
pub struct SecurityFramework {
    config: Config,
    linux_integration: LinuxIntegration,
    scanner: scanner::SecurityScanner,
    compliance_checker: compliance::ComplianceChecker,
    vulnerability_assessor: vulnerability::VulnerabilityAssessor,
    hardening_engine: hardening::HardeningEngine,
    audit_system: audit::SecurityAuditSystem,
}

/// Security assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAssessment {
    pub assessment_id: String,
    pub timestamp: DateTime<Utc>,
    pub overall_score: f32,
    pub security_level: SecurityLevel,
    pub findings: Vec<SecurityFinding>,
    pub compliance_status: HashMap<String, ComplianceStatus>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub hardening_recommendations: Vec<HardeningRecommendation>,
    pub risk_summary: RiskSummary,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Excellent,  // 90-100%
    Good,       // 70-89%
    Moderate,   // 50-69%
    Poor,       // 30-49%
    Critical,   // 0-29%
}

/// Security finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub finding_id: String,
    pub category: SecurityCategory,
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub affected_components: Vec<String>,
    pub risk_score: f32,
    pub remediation: Vec<RemediationStep>,
    pub references: Vec<String>,
    pub detected_at: DateTime<Utc>,
}

/// Security categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCategory {
    Authentication,
    Authorization,
    NetworkSecurity,
    FileSystemSecurity,
    ProcessSecurity,
    ServiceConfiguration,
    SystemHardening,
    Cryptography,
    Logging,
    Updates,
}

/// Severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub framework: String,
    pub version: String,
    pub overall_compliance: f32,
    pub passed_controls: u32,
    pub failed_controls: u32,
    pub not_applicable: u32,
    pub control_results: HashMap<String, ControlResult>,
}

/// Control result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlResult {
    pub control_id: String,
    pub status: ControlStatus,
    pub evidence: Vec<String>,
    pub remediation_required: bool,
}

/// Control status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlStatus {
    Pass,
    Fail,
    NotApplicable,
    Manual,
}

/// Vulnerability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub vulnerability_id: String,
    pub cve_id: Option<String>,
    pub title: String,
    pub description: String,
    pub severity: VulnerabilitySeverity,
    pub cvss_score: Option<f32>,
    pub affected_packages: Vec<String>,
    pub fixed_version: Option<String>,
    pub exploit_available: bool,
    pub patch_available: bool,
    pub discovered_at: DateTime<Utc>,
}

/// Vulnerability severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Hardening recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardeningRecommendation {
    pub recommendation_id: String,
    pub category: HardeningCategory,
    pub priority: HardeningPriority,
    pub title: String,
    pub description: String,
    pub implementation_steps: Vec<String>,
    pub verification_commands: Vec<String>,
    pub impact_assessment: ImpactAssessment,
    pub compliance_frameworks: Vec<String>,
}

/// Hardening categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardeningCategory {
    KernelSecurity,
    NetworkSecurity,
    FileSystemSecurity,
    ServiceHardening,
    UserManagement,
    AccessControl,
    Logging,
    Monitoring,
}

/// Hardening priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardeningPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub security_improvement: f32,
    pub performance_impact: f32,
    pub usability_impact: f32,
    pub compatibility_risk: f32,
    pub implementation_complexity: f32,
}

/// Risk summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub total_risks: u32,
    pub critical_risks: u32,
    pub high_risks: u32,
    pub medium_risks: u32,
    pub low_risks: u32,
    pub risk_score: f32,
    pub top_risks: Vec<TopRisk>,
}

/// Top risk item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopRisk {
    pub risk_id: String,
    pub title: String,
    pub risk_score: f32,
    pub category: SecurityCategory,
    pub immediate_action_required: bool,
}

/// Remediation step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationStep {
    pub step_number: u32,
    pub description: String,
    pub command: Option<String>,
    pub verification: Option<String>,
    pub risk_level: Severity,
    pub estimated_time_minutes: u32,
}

impl SecurityFramework {
    pub async fn new(config: Config, linux_integration: LinuxIntegration) -> Result<Self> {
        let scanner = scanner::SecurityScanner::new(&config).await?;
        let compliance_checker = compliance::ComplianceChecker::new().await?;
        let vulnerability_assessor = vulnerability::VulnerabilityAssessor::new().await?;
        let hardening_engine = hardening::HardeningEngine::new().await?;
        let audit_system = audit::SecurityAuditSystem::new().await?;

        Ok(Self {
            config,
            linux_integration,
            scanner,
            compliance_checker,
            vulnerability_assessor,
            hardening_engine,
            audit_system,
        })
    }

    pub async fn perform_comprehensive_assessment(&self) -> Result<SecurityAssessment> {
        println!("🔒 Starting comprehensive security assessment...");

        let assessment_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        // Perform security scan
        let findings = self.scanner.perform_security_scan().await?;

        // Check compliance
        let compliance_status = self.compliance_checker.check_compliance().await?;

        // Assess vulnerabilities
        let vulnerabilities = self.vulnerability_assessor.assess_vulnerabilities().await?;

        // Generate hardening recommendations
        let hardening_recommendations = self.hardening_engine.generate_recommendations(&findings).await?;

        // Calculate overall security score
        let overall_score = self.calculate_security_score(&findings, &vulnerabilities);
        let security_level = self.determine_security_level(overall_score);

        // Generate risk summary
        let risk_summary = self.generate_risk_summary(&findings, &vulnerabilities);

        let assessment = SecurityAssessment {
            assessment_id,
            timestamp,
            overall_score,
            security_level,
            findings,
            compliance_status,
            vulnerabilities,
            hardening_recommendations,
            risk_summary,
        };

        // Log assessment
        self.audit_system.log_assessment(&assessment).await?;

        println!("✅ Security assessment completed (Score: {:.1}/100)", overall_score);
        Ok(assessment)
    }

    pub async fn quick_security_scan(&self) -> Result<Vec<SecurityFinding>> {
        println!("⚡ Performing quick security scan...");
        self.scanner.quick_scan().await
    }

    pub async fn check_specific_compliance(&self, framework: &str) -> Result<ComplianceStatus> {
        self.compliance_checker.check_framework(framework).await
    }

    pub async fn scan_vulnerabilities(&self) -> Result<Vec<Vulnerability>> {
        self.vulnerability_assessor.scan_system().await
    }

    pub async fn apply_hardening_recommendation(&self, recommendation_id: &str) -> Result<()> {
        self.hardening_engine.apply_recommendation(recommendation_id).await
    }

    fn calculate_security_score(&self, findings: &[SecurityFinding], vulnerabilities: &[Vulnerability]) -> f32 {
        let mut score: f32 = 100.0;

        // Deduct points for findings
        for finding in findings {
            let deduction = match finding.severity {
                Severity::Critical => 20.0,
                Severity::High => 10.0,
                Severity::Medium => 5.0,
                Severity::Low => 2.0,
                Severity::Info => 0.5,
            };
            score -= deduction;
        }

        // Deduct points for vulnerabilities
        for vulnerability in vulnerabilities {
            let deduction = match vulnerability.severity {
                VulnerabilitySeverity::Critical => 15.0,
                VulnerabilitySeverity::High => 8.0,
                VulnerabilitySeverity::Medium => 4.0,
                VulnerabilitySeverity::Low => 1.0,
                VulnerabilitySeverity::None => 0.0,
            };
            score -= deduction;
        }

        score.max(0.0).min(100.0)
    }

    fn determine_security_level(&self, score: f32) -> SecurityLevel {
        match score {
            s if s >= 90.0 => SecurityLevel::Excellent,
            s if s >= 70.0 => SecurityLevel::Good,
            s if s >= 50.0 => SecurityLevel::Moderate,
            s if s >= 30.0 => SecurityLevel::Poor,
            _ => SecurityLevel::Critical,
        }
    }

    fn generate_risk_summary(&self, findings: &[SecurityFinding], vulnerabilities: &[Vulnerability]) -> RiskSummary {
        let mut critical_risks = 0;
        let mut high_risks = 0;
        let mut medium_risks = 0;
        let mut low_risks = 0;

        // Count finding risks
        for finding in findings {
            match finding.severity {
                Severity::Critical => critical_risks += 1,
                Severity::High => high_risks += 1,
                Severity::Medium => medium_risks += 1,
                Severity::Low => low_risks += 1,
                Severity::Info => {}
            }
        }

        // Count vulnerability risks
        for vulnerability in vulnerabilities {
            match vulnerability.severity {
                VulnerabilitySeverity::Critical => critical_risks += 1,
                VulnerabilitySeverity::High => high_risks += 1,
                VulnerabilitySeverity::Medium => medium_risks += 1,
                VulnerabilitySeverity::Low => low_risks += 1,
                VulnerabilitySeverity::None => {}
            }
        }

        let total_risks = critical_risks + high_risks + medium_risks + low_risks;
        let risk_score = (critical_risks as f32 * 4.0 + high_risks as f32 * 3.0 + 
                         medium_risks as f32 * 2.0 + low_risks as f32) / 
                         (total_risks as f32).max(1.0);

        // Generate top risks
        let mut top_risks = Vec::new();
        for finding in findings.iter().take(5) {
            if matches!(finding.severity, Severity::Critical | Severity::High) {
                top_risks.push(TopRisk {
                    risk_id: finding.finding_id.clone(),
                    title: finding.title.clone(),
                    risk_score: finding.risk_score,
                    category: finding.category.clone(),
                    immediate_action_required: matches!(finding.severity, Severity::Critical),
                });
            }
        }

        RiskSummary {
            total_risks,
            critical_risks,
            high_risks,
            medium_risks,
            low_risks,
            risk_score,
            top_risks,
        }
    }

    pub async fn generate_security_report(&self, assessment: &SecurityAssessment) -> Result<String> {
        let mut report = String::new();
        
        report.push_str(&format!("# Security Assessment Report\n"));
        report.push_str(&format!("**Assessment ID:** {}\n", assessment.assessment_id));
        report.push_str(&format!("**Timestamp:** {}\n", assessment.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**Overall Score:** {:.1}/100\n", assessment.overall_score));
        report.push_str(&format!("**Security Level:** {:?}\n\n", assessment.security_level));

        report.push_str("## Risk Summary\n");
        report.push_str(&format!("- **Total Risks:** {}\n", assessment.risk_summary.total_risks));
        report.push_str(&format!("- **Critical:** {}\n", assessment.risk_summary.critical_risks));
        report.push_str(&format!("- **High:** {}\n", assessment.risk_summary.high_risks));
        report.push_str(&format!("- **Medium:** {}\n", assessment.risk_summary.medium_risks));
        report.push_str(&format!("- **Low:** {}\n\n", assessment.risk_summary.low_risks));

        report.push_str("## Security Findings\n");
        for finding in &assessment.findings {
            report.push_str(&format!("### {} ({:?})\n", finding.title, finding.severity));
            report.push_str(&format!("{}\n\n", finding.description));
        }

        report.push_str("## Vulnerabilities\n");
        for vulnerability in &assessment.vulnerabilities {
            report.push_str(&format!("### {} ({:?})\n", vulnerability.title, vulnerability.severity));
            if let Some(cve) = &vulnerability.cve_id {
                report.push_str(&format!("**CVE:** {}\n", cve));
            }
            report.push_str(&format!("{}\n\n", vulnerability.description));
        }

        Ok(report)
    }
}
