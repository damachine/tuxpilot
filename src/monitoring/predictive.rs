use anyhow::Result;
use super::{SystemMetrics, PredictiveAlert, AlertType, AlertSeverity};

/// Predictive analytics engine
#[derive(Debug, Clone)]
pub struct PredictiveEngine {
    models: Vec<PredictiveModel>,
    is_active: bool,
}

/// Predictive model for forecasting
#[derive(Debug, Clone)]
pub struct PredictiveModel {
    pub model_type: ModelType,
    pub target_metric: String,
    pub accuracy: f32,
    pub last_trained: chrono::DateTime<chrono::Utc>,
}

/// Types of predictive models
#[derive(Debug, Clone)]
pub enum ModelType {
    LinearRegression,
    MovingAverage,
    ExponentialSmoothing,
    ARIMA,
    NeuralNetwork,
}

impl PredictiveEngine {
    pub async fn new() -> Result<Self> {
        let models = vec![
            PredictiveModel {
                model_type: ModelType::LinearRegression,
                target_metric: "cpu_usage".to_string(),
                accuracy: 0.85,
                last_trained: chrono::Utc::now(),
            },
            PredictiveModel {
                model_type: ModelType::ExponentialSmoothing,
                target_metric: "memory_usage".to_string(),
                accuracy: 0.78,
                last_trained: chrono::Utc::now(),
            },
            PredictiveModel {
                model_type: ModelType::MovingAverage,
                target_metric: "disk_usage".to_string(),
                accuracy: 0.92,
                last_trained: chrono::Utc::now(),
            },
        ];

        Ok(Self {
            models,
            is_active: false,
        })
    }

    pub async fn start_analysis(&mut self) -> Result<()> {
        println!("🔮 Starting predictive analysis engine...");
        self.is_active = true;
        Ok(())
    }

    pub async fn generate_alerts(&self, metrics_history: &[SystemMetrics]) -> Result<Vec<PredictiveAlert>> {
        let mut alerts = Vec::new();

        // Predict CPU exhaustion
        if let Some(cpu_prediction) = self.predict_cpu_exhaustion(metrics_history).await? {
            alerts.push(cpu_prediction);
        }

        // Predict memory exhaustion
        if let Some(memory_prediction) = self.predict_memory_exhaustion(metrics_history).await? {
            alerts.push(memory_prediction);
        }

        // Predict disk full
        if let Some(disk_prediction) = self.predict_disk_full(metrics_history).await? {
            alerts.push(disk_prediction);
        }

        Ok(alerts)
    }

    async fn predict_cpu_exhaustion(&self, metrics: &[SystemMetrics]) -> Result<Option<PredictiveAlert>> {
        if metrics.len() < 10 {
            return Ok(None);
        }

        let recent_cpu: Vec<f32> = metrics.iter().rev().take(10).map(|m| m.cpu_usage).collect();
        let avg_cpu = recent_cpu.iter().sum::<f32>() / recent_cpu.len() as f32;

        if avg_cpu > 80.0 {
            return Ok(Some(PredictiveAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                alert_type: AlertType::ResourceExhaustion,
                severity: AlertSeverity::Warning,
                title: "CPU Exhaustion Predicted".to_string(),
                description: format!("CPU usage trending high at {:.1}%, may reach critical levels", avg_cpu),
                predicted_time: chrono::Utc::now() + chrono::Duration::hours(2),
                confidence: 0.75,
                affected_components: vec!["cpu".to_string()],
                preventive_actions: vec![
                    "Identify high CPU processes".to_string(),
                    "Consider process optimization".to_string(),
                ],
                created_at: chrono::Utc::now(),
            }));
        }

        Ok(None)
    }

    async fn predict_memory_exhaustion(&self, metrics: &[SystemMetrics]) -> Result<Option<PredictiveAlert>> {
        if metrics.len() < 10 {
            return Ok(None);
        }

        let recent_memory: Vec<f32> = metrics.iter().rev().take(10).map(|m| m.memory_usage).collect();
        let trend = recent_memory[0] - recent_memory[recent_memory.len() - 1];

        if trend > 5.0 && recent_memory[0] > 75.0 {
            return Ok(Some(PredictiveAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                alert_type: AlertType::ResourceExhaustion,
                severity: AlertSeverity::Warning,
                title: "Memory Exhaustion Predicted".to_string(),
                description: "Memory usage increasing rapidly, may reach critical levels".to_string(),
                predicted_time: chrono::Utc::now() + chrono::Duration::hours(4),
                confidence: 0.68,
                affected_components: vec!["memory".to_string()],
                preventive_actions: vec![
                    "Clear memory caches".to_string(),
                    "Restart memory-intensive services".to_string(),
                ],
                created_at: chrono::Utc::now(),
            }));
        }

        Ok(None)
    }

    async fn predict_disk_full(&self, metrics: &[SystemMetrics]) -> Result<Option<PredictiveAlert>> {
        if metrics.len() < 20 {
            return Ok(None);
        }

        let recent_disk: Vec<f32> = metrics.iter().rev().take(20).map(|m| m.disk_usage).collect();
        let growth_rate = (recent_disk[0] - recent_disk[recent_disk.len() - 1]) / recent_disk.len() as f32;

        if growth_rate > 0.1 && recent_disk[0] > 70.0 {
            let days_to_full = (100.0 - recent_disk[0]) / (growth_rate * 24.0);
            
            if days_to_full < 30.0 {
                return Ok(Some(PredictiveAlert {
                    alert_id: uuid::Uuid::new_v4().to_string(),
                    alert_type: AlertType::CapacityLimit,
                    severity: if days_to_full < 7.0 { AlertSeverity::Critical } else { AlertSeverity::Warning },
                    title: "Disk Space Exhaustion Predicted".to_string(),
                    description: format!("Disk may be full in approximately {:.1} days", days_to_full),
                    predicted_time: chrono::Utc::now() + chrono::Duration::days(days_to_full as i64),
                    confidence: 0.82,
                    affected_components: vec!["disk".to_string()],
                    preventive_actions: vec![
                        "Clean temporary files".to_string(),
                        "Archive old logs".to_string(),
                        "Remove unused packages".to_string(),
                    ],
                    created_at: chrono::Utc::now(),
                }));
            }
        }

        Ok(None)
    }

    pub async fn stop_analysis(&mut self) -> Result<()> {
        println!("🔄 Stopping predictive analysis...");
        self.is_active = false;
        Ok(())
    }
}
