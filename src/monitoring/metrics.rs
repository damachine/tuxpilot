use anyhow::Result;
use super::SystemMetrics;

/// Metrics collection system
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    collection_interval: u64,
    is_collecting: bool,
    metrics_history: Vec<SystemMetrics>,
}

impl MetricsCollector {
    pub async fn new(_config: &crate::config::Config) -> Result<Self> {
        Ok(Self {
            collection_interval: 60,
            is_collecting: false,
            metrics_history: Vec::new(),
        })
    }

    pub async fn start_collection(&mut self, interval_seconds: u64) -> Result<()> {
        self.collection_interval = interval_seconds;
        self.is_collecting = true;
        println!("📊 Started metrics collection (interval: {}s)", interval_seconds);
        Ok(())
    }

    pub async fn stop_collection(&mut self) -> Result<()> {
        self.is_collecting = false;
        println!("📊 Stopped metrics collection");
        Ok(())
    }

    pub async fn get_historical_data(&self, hours: u32) -> Result<Vec<SystemMetrics>> {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(hours as i64);
        let filtered: Vec<SystemMetrics> = self.metrics_history
            .iter()
            .filter(|m| m.timestamp > cutoff_time)
            .cloned()
            .collect();
        Ok(filtered)
    }
}
