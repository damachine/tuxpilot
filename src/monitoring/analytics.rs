use anyhow::Result;
use std::collections::HashMap;

use super::{SystemMetrics, PerformanceTrend, TrendDirection, SystemHealthAssessment, HealthStatus, HealthIssue, IssueSeverity, HealthRecommendation, RecommendationPriority, RecommendationCategory, EffortLevel};

/// Analytics engine for performance analysis
#[derive(Debug, Clone)]
pub struct AnalyticsEngine {
    trend_analyzers: HashMap<String, TrendAnalyzer>,
    health_assessors: HashMap<String, HealthAssessor>,
    recommendation_engine: RecommendationEngine,
}

/// Trend analyzer for specific metrics
#[derive(Debug, Clone)]
pub struct TrendAnalyzer {
    metric_name: String,
    window_size: usize,
    sensitivity: f32,
}

/// Health assessor for system components
#[derive(Debug, Clone)]
pub struct HealthAssessor {
    component_name: String,
    thresholds: ComponentThresholds,
    weight: f32,
}

/// Component thresholds for health assessment
#[derive(Debug, Clone)]
pub struct ComponentThresholds {
    pub excellent: f32,
    pub good: f32,
    pub warning: f32,
    pub critical: f32,
}

/// Recommendation engine
#[derive(Debug, Clone)]
pub struct RecommendationEngine {
    rule_base: Vec<RecommendationRule>,
}

/// Recommendation rule
#[derive(Debug, Clone)]
pub struct RecommendationRule {
    pub condition: String,
    pub recommendation: HealthRecommendation,
}

impl AnalyticsEngine {
    pub async fn new() -> Result<Self> {
        let mut trend_analyzers = HashMap::new();
        let mut health_assessors = HashMap::new();

        // Initialize trend analyzers
        trend_analyzers.insert("cpu_usage".to_string(), TrendAnalyzer {
            metric_name: "cpu_usage".to_string(),
            window_size: 60, // 1 hour of data points
            sensitivity: 0.1,
        });

        trend_analyzers.insert("memory_usage".to_string(), TrendAnalyzer {
            metric_name: "memory_usage".to_string(),
            window_size: 60,
            sensitivity: 0.05,
        });

        trend_analyzers.insert("disk_usage".to_string(), TrendAnalyzer {
            metric_name: "disk_usage".to_string(),
            window_size: 1440, // 24 hours
            sensitivity: 0.02,
        });

        // Initialize health assessors
        health_assessors.insert("cpu".to_string(), HealthAssessor {
            component_name: "cpu".to_string(),
            thresholds: ComponentThresholds {
                excellent: 20.0,
                good: 50.0,
                warning: 80.0,
                critical: 95.0,
            },
            weight: 0.3,
        });

        health_assessors.insert("memory".to_string(), HealthAssessor {
            component_name: "memory".to_string(),
            thresholds: ComponentThresholds {
                excellent: 30.0,
                good: 60.0,
                warning: 85.0,
                critical: 95.0,
            },
            weight: 0.25,
        });

        health_assessors.insert("disk".to_string(), HealthAssessor {
            component_name: "disk".to_string(),
            thresholds: ComponentThresholds {
                excellent: 40.0,
                good: 70.0,
                warning: 90.0,
                critical: 98.0,
            },
            weight: 0.2,
        });

        let recommendation_engine = RecommendationEngine::new().await?;

        Ok(Self {
            trend_analyzers,
            health_assessors,
            recommendation_engine,
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        println!("🔍 Initializing analytics engine...");
        Ok(())
    }

    pub async fn analyze_trends(&self, historical_data: &[SystemMetrics]) -> Result<Vec<PerformanceTrend>> {
        let mut trends = Vec::new();

        // Analyze CPU usage trend
        if let Some(analyzer) = self.trend_analyzers.get("cpu_usage") {
            let cpu_values: Vec<f32> = historical_data.iter().map(|m| m.cpu_usage).collect();
            let trend = analyzer.analyze_trend(&cpu_values)?;
            trends.push(trend);
        }

        // Analyze memory usage trend
        if let Some(analyzer) = self.trend_analyzers.get("memory_usage") {
            let memory_values: Vec<f32> = historical_data.iter().map(|m| m.memory_usage).collect();
            let trend = analyzer.analyze_trend(&memory_values)?;
            trends.push(trend);
        }

        // Analyze disk usage trend
        if let Some(analyzer) = self.trend_analyzers.get("disk_usage") {
            let disk_values: Vec<f32> = historical_data.iter().map(|m| m.disk_usage).collect();
            let trend = analyzer.analyze_trend(&disk_values)?;
            trends.push(trend);
        }

        Ok(trends)
    }

    pub async fn assess_health(&self, current_metrics: &SystemMetrics, trends: &[PerformanceTrend]) -> Result<SystemHealthAssessment> {
        let mut component_scores = HashMap::new();
        let mut critical_issues = Vec::new();

        // Assess CPU health
        if let Some(assessor) = self.health_assessors.get("cpu") {
            let score = assessor.assess_component_health(current_metrics.cpu_usage);
            component_scores.insert("cpu".to_string(), score);

            if current_metrics.cpu_usage > assessor.thresholds.critical {
                critical_issues.push(HealthIssue {
                    issue_id: uuid::Uuid::new_v4().to_string(),
                    severity: IssueSeverity::Critical,
                    component: "cpu".to_string(),
                    description: format!("CPU usage is critically high: {:.1}%", current_metrics.cpu_usage),
                    impact_score: 0.9,
                    detected_at: chrono::Utc::now(),
                    auto_fixable: true,
                    fix_commands: vec![
                        "top -b -n1 | head -20".to_string(),
                        "ps aux --sort=-%cpu | head -10".to_string(),
                    ],
                });
            }
        }

        // Assess memory health
        if let Some(assessor) = self.health_assessors.get("memory") {
            let score = assessor.assess_component_health(current_metrics.memory_usage);
            component_scores.insert("memory".to_string(), score);

            if current_metrics.memory_usage > assessor.thresholds.warning {
                critical_issues.push(HealthIssue {
                    issue_id: uuid::Uuid::new_v4().to_string(),
                    severity: if current_metrics.memory_usage > assessor.thresholds.critical {
                        IssueSeverity::Critical
                    } else {
                        IssueSeverity::High
                    },
                    component: "memory".to_string(),
                    description: format!("Memory usage is high: {:.1}%", current_metrics.memory_usage),
                    impact_score: 0.7,
                    detected_at: chrono::Utc::now(),
                    auto_fixable: true,
                    fix_commands: vec![
                        "free -h".to_string(),
                        "ps aux --sort=-%mem | head -10".to_string(),
                        "sync && echo 3 > /proc/sys/vm/drop_caches".to_string(),
                    ],
                });
            }
        }

        // Assess disk health
        if let Some(assessor) = self.health_assessors.get("disk") {
            let score = assessor.assess_component_health(current_metrics.disk_usage);
            component_scores.insert("disk".to_string(), score);

            if current_metrics.disk_usage > assessor.thresholds.warning {
                critical_issues.push(HealthIssue {
                    issue_id: uuid::Uuid::new_v4().to_string(),
                    severity: if current_metrics.disk_usage > assessor.thresholds.critical {
                        IssueSeverity::Critical
                    } else {
                        IssueSeverity::Medium
                    },
                    component: "disk".to_string(),
                    description: format!("Disk usage is high: {:.1}%", current_metrics.disk_usage),
                    impact_score: 0.6,
                    detected_at: chrono::Utc::now(),
                    auto_fixable: true,
                    fix_commands: vec![
                        "df -h".to_string(),
                        "du -sh /* 2>/dev/null | sort -hr | head -10".to_string(),
                        "find /tmp -type f -atime +7 -delete".to_string(),
                    ],
                });
            }
        }

        // Calculate overall health score
        let overall_score = self.calculate_overall_health_score(&component_scores);
        let health_status = self.determine_health_status(overall_score);

        let recommendations = self.generate_recommendations(&SystemHealthAssessment {
            overall_score,
            component_scores: component_scores.clone(),
            health_status: health_status.clone(),
            critical_issues: critical_issues.clone(),
            recommendations: Vec::new(),
            assessment_time: chrono::Utc::now(),
        }, trends).await?;

        Ok(SystemHealthAssessment {
            overall_score,
            component_scores,
            health_status,
            critical_issues,
            recommendations,
            assessment_time: chrono::Utc::now(),
        })
    }

    pub async fn generate_recommendations(&self, health_assessment: &SystemHealthAssessment, trends: &[PerformanceTrend]) -> Result<Vec<HealthRecommendation>> {
        self.recommendation_engine.generate_recommendations(health_assessment, trends).await
    }

    fn calculate_overall_health_score(&self, component_scores: &HashMap<String, f32>) -> f32 {
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (component, score) in component_scores {
            if let Some(assessor) = self.health_assessors.get(component) {
                weighted_sum += score * assessor.weight;
                total_weight += assessor.weight;
            }
        }

        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    }

    fn determine_health_status(&self, score: f32) -> HealthStatus {
        match score {
            s if s >= 90.0 => HealthStatus::Excellent,
            s if s >= 70.0 => HealthStatus::Good,
            s if s >= 50.0 => HealthStatus::Warning,
            s if s >= 30.0 => HealthStatus::Critical,
            _ => HealthStatus::Emergency,
        }
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        println!("🔄 Shutting down analytics engine...");
        Ok(())
    }
}

impl TrendAnalyzer {
    pub fn analyze_trend(&self, values: &[f32]) -> Result<PerformanceTrend> {
        if values.len() < 2 {
            return Ok(PerformanceTrend {
                metric_name: self.metric_name.clone(),
                trend_direction: TrendDirection::Unknown,
                trend_strength: 0.0,
                prediction_confidence: 0.0,
                time_window_hours: 1,
                current_value: values.last().copied().unwrap_or(0.0),
                predicted_value: values.last().copied().unwrap_or(0.0),
                threshold_breach_probability: 0.0,
            });
        }

        let current_value = values[values.len() - 1];
        let previous_value = values[values.len() - 2];
        
        let change = current_value - previous_value;
        let change_percent = if previous_value != 0.0 {
            (change / previous_value).abs()
        } else {
            0.0
        };

        let trend_direction = if change > self.sensitivity {
            TrendDirection::Increasing
        } else if change < -self.sensitivity {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        // Simple linear prediction
        let predicted_value = current_value + change;
        let prediction_confidence = (1.0 - change_percent).max(0.1).min(0.9);

        Ok(PerformanceTrend {
            metric_name: self.metric_name.clone(),
            trend_direction,
            trend_strength: change_percent,
            prediction_confidence,
            time_window_hours: 1,
            current_value,
            predicted_value,
            threshold_breach_probability: if predicted_value > 90.0 { 0.8 } else { 0.1 },
        })
    }
}

impl HealthAssessor {
    pub fn assess_component_health(&self, current_value: f32) -> f32 {
        if current_value <= self.thresholds.excellent {
            100.0
        } else if current_value <= self.thresholds.good {
            90.0 - (current_value - self.thresholds.excellent) / (self.thresholds.good - self.thresholds.excellent) * 20.0
        } else if current_value <= self.thresholds.warning {
            70.0 - (current_value - self.thresholds.good) / (self.thresholds.warning - self.thresholds.good) * 20.0
        } else if current_value <= self.thresholds.critical {
            50.0 - (current_value - self.thresholds.warning) / (self.thresholds.critical - self.thresholds.warning) * 20.0
        } else {
            30.0 - ((current_value - self.thresholds.critical) / 5.0).min(30.0)
        }
    }
}

impl RecommendationEngine {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            rule_base: Vec::new(),
        })
    }

    pub async fn generate_recommendations(&self, health_assessment: &SystemHealthAssessment, _trends: &[PerformanceTrend]) -> Result<Vec<HealthRecommendation>> {
        let mut recommendations = Vec::new();

        // Generate recommendations based on health issues
        for issue in &health_assessment.critical_issues {
            match issue.component.as_str() {
                "cpu" => {
                    recommendations.push(HealthRecommendation {
                        recommendation_id: uuid::Uuid::new_v4().to_string(),
                        priority: RecommendationPriority::High,
                        category: RecommendationCategory::Performance,
                        title: "Optimize CPU Usage".to_string(),
                        description: "Identify and optimize high CPU usage processes".to_string(),
                        expected_impact: 0.7,
                        implementation_effort: EffortLevel::Low,
                        commands: vec![
                            "top -b -n1 | head -20".to_string(),
                            "ps aux --sort=-%cpu | head -10".to_string(),
                        ],
                        estimated_time_minutes: 10,
                    });
                }
                "memory" => {
                    recommendations.push(HealthRecommendation {
                        recommendation_id: uuid::Uuid::new_v4().to_string(),
                        priority: RecommendationPriority::Medium,
                        category: RecommendationCategory::Performance,
                        title: "Free Memory".to_string(),
                        description: "Clear memory caches and identify memory-intensive processes".to_string(),
                        expected_impact: 0.6,
                        implementation_effort: EffortLevel::Minimal,
                        commands: vec![
                            "sync && echo 3 > /proc/sys/vm/drop_caches".to_string(),
                            "ps aux --sort=-%mem | head -10".to_string(),
                        ],
                        estimated_time_minutes: 5,
                    });
                }
                "disk" => {
                    recommendations.push(HealthRecommendation {
                        recommendation_id: uuid::Uuid::new_v4().to_string(),
                        priority: RecommendationPriority::Medium,
                        category: RecommendationCategory::Maintenance,
                        title: "Clean Disk Space".to_string(),
                        description: "Remove temporary files and clean package cache".to_string(),
                        expected_impact: 0.5,
                        implementation_effort: EffortLevel::Low,
                        commands: vec![
                            "find /tmp -type f -atime +7 -delete".to_string(),
                            "journalctl --vacuum-time=7d".to_string(),
                        ],
                        estimated_time_minutes: 15,
                    });
                }
                _ => {}
            }
        }

        Ok(recommendations)
    }
}
