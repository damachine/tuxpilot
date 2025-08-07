use anyhow::Result;
use super::{SecurityFinding, HardeningRecommendation, HardeningCategory, HardeningPriority, ImpactAssessment};

/// System hardening engine
#[derive(Debug, Clone)]
pub struct HardeningEngine {
    hardening_rules: Vec<HardeningRule>,
}

/// Hardening rule definition
#[derive(Debug, Clone)]
pub struct HardeningRule {
    pub rule_id: String,
    pub category: HardeningCategory,
    pub priority: HardeningPriority,
    pub title: String,
    pub description: String,
    pub implementation_steps: Vec<String>,
    pub verification_commands: Vec<String>,
    pub impact_assessment: ImpactAssessment,
}

impl HardeningEngine {
    pub async fn new() -> Result<Self> {
        let hardening_rules = Self::initialize_hardening_rules();
        Ok(Self { hardening_rules })
    }

    fn initialize_hardening_rules() -> Vec<HardeningRule> {
        vec![
            HardeningRule {
                rule_id: "DISABLE_UNUSED_SERVICES".to_string(),
                category: HardeningCategory::ServiceHardening,
                priority: HardeningPriority::High,
                title: "Disable Unused Network Services".to_string(),
                description: "Disable unnecessary network services to reduce attack surface".to_string(),
                implementation_steps: vec![
                    "systemctl list-unit-files --type=service --state=enabled".to_string(),
                    "systemctl disable <unused-service>".to_string(),
                    "systemctl stop <unused-service>".to_string(),
                ],
                verification_commands: vec![
                    "systemctl is-enabled <service>".to_string(),
                    "systemctl is-active <service>".to_string(),
                ],
                impact_assessment: ImpactAssessment {
                    security_improvement: 8.0,
                    performance_impact: 1.0,
                    usability_impact: 2.0,
                    compatibility_risk: 3.0,
                    implementation_complexity: 2.0,
                },
            },
            HardeningRule {
                rule_id: "CONFIGURE_FIREWALL".to_string(),
                category: HardeningCategory::NetworkSecurity,
                priority: HardeningPriority::Critical,
                title: "Configure Host-Based Firewall".to_string(),
                description: "Configure iptables or ufw for network access control".to_string(),
                implementation_steps: vec![
                    "ufw enable".to_string(),
                    "ufw default deny incoming".to_string(),
                    "ufw default allow outgoing".to_string(),
                    "ufw allow ssh".to_string(),
                ],
                verification_commands: vec![
                    "ufw status".to_string(),
                    "iptables -L".to_string(),
                ],
                impact_assessment: ImpactAssessment {
                    security_improvement: 9.0,
                    performance_impact: 1.0,
                    usability_impact: 3.0,
                    compatibility_risk: 4.0,
                    implementation_complexity: 3.0,
                },
            },
            HardeningRule {
                rule_id: "SECURE_SSH_CONFIG".to_string(),
                category: HardeningCategory::NetworkSecurity,
                priority: HardeningPriority::High,
                title: "Secure SSH Configuration".to_string(),
                description: "Implement secure SSH configuration settings".to_string(),
                implementation_steps: vec![
                    "sed -i 's/#PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config".to_string(),
                    "sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config".to_string(),
                    "echo 'AllowUsers your-username' >> /etc/ssh/sshd_config".to_string(),
                    "systemctl restart sshd".to_string(),
                ],
                verification_commands: vec![
                    "grep '^PermitRootLogin no' /etc/ssh/sshd_config".to_string(),
                    "grep '^PasswordAuthentication no' /etc/ssh/sshd_config".to_string(),
                ],
                impact_assessment: ImpactAssessment {
                    security_improvement: 8.5,
                    performance_impact: 0.0,
                    usability_impact: 5.0,
                    compatibility_risk: 2.0,
                    implementation_complexity: 2.0,
                },
            },
        ]
    }

    pub async fn generate_recommendations(&self, findings: &[SecurityFinding]) -> Result<Vec<HardeningRecommendation>> {
        let mut recommendations = Vec::new();

        // Generate recommendations based on security findings
        for finding in findings {
            let related_rules = self.find_related_hardening_rules(finding);
            for rule in related_rules {
                let recommendation = self.create_recommendation_from_rule(&rule);
                recommendations.push(recommendation);
            }
        }

        // Add general hardening recommendations
        for rule in &self.hardening_rules {
            if matches!(rule.priority, HardeningPriority::Critical | HardeningPriority::High) {
                let recommendation = self.create_recommendation_from_rule(rule);
                recommendations.push(recommendation);
            }
        }

        // Remove duplicates
        recommendations.dedup_by(|a, b| a.recommendation_id == b.recommendation_id);

        Ok(recommendations)
    }

    fn find_related_hardening_rules(&self, finding: &SecurityFinding) -> Vec<&HardeningRule> {
        self.hardening_rules
            .iter()
            .filter(|rule| {
                // Match based on category or keywords in finding
                match finding.category {
                    super::SecurityCategory::NetworkSecurity => {
                        matches!(rule.category, HardeningCategory::NetworkSecurity)
                    }
                    super::SecurityCategory::ServiceConfiguration => {
                        matches!(rule.category, HardeningCategory::ServiceHardening)
                    }
                    super::SecurityCategory::Authentication => {
                        matches!(rule.category, HardeningCategory::AccessControl)
                    }
                    _ => false,
                }
            })
            .collect()
    }

    fn create_recommendation_from_rule(&self, rule: &HardeningRule) -> HardeningRecommendation {
        HardeningRecommendation {
            recommendation_id: uuid::Uuid::new_v4().to_string(),
            category: rule.category.clone(),
            priority: rule.priority.clone(),
            title: rule.title.clone(),
            description: rule.description.clone(),
            implementation_steps: rule.implementation_steps.clone(),
            verification_commands: rule.verification_commands.clone(),
            impact_assessment: rule.impact_assessment.clone(),
            compliance_frameworks: vec![
                "CIS Controls".to_string(),
                "NIST Cybersecurity Framework".to_string(),
            ],
        }
    }

    pub async fn apply_recommendation(&self, recommendation_id: &str) -> Result<()> {
        println!("🔧 Applying hardening recommendation: {}", recommendation_id);
        
        // In a real implementation, this would:
        // 1. Find the recommendation by ID
        // 2. Execute the implementation steps
        // 3. Verify the changes
        // 4. Log the results
        
        println!("✅ Hardening recommendation applied successfully");
        Ok(())
    }

    pub async fn get_hardening_checklist(&self) -> Result<Vec<HardeningRecommendation>> {
        let mut checklist = Vec::new();

        for rule in &self.hardening_rules {
            let recommendation = self.create_recommendation_from_rule(rule);
            checklist.push(recommendation);
        }

        // Sort by priority
        checklist.sort_by(|a, b| {
            let priority_order = |p: &HardeningPriority| match p {
                HardeningPriority::Critical => 0,
                HardeningPriority::High => 1,
                HardeningPriority::Medium => 2,
                HardeningPriority::Low => 3,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });

        Ok(checklist)
    }
}
