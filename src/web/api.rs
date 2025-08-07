// API endpoint implementations for web interface

use anyhow::Result;
use super::{ApiRequest, ApiResponse, WebServer};

impl WebServer {
    pub async fn handle_api_request(&self, request: ApiRequest) -> Result<ApiResponse> {
        let start_time = std::time::Instant::now();
        
        let response_body = match request.endpoint.as_str() {
            "/api/dashboard" => self.handle_dashboard_request(&request).await?,
            "/api/system/status" => self.handle_system_status_request(&request).await?,
            "/api/commands/execute" => self.handle_command_execution_request(&request).await?,
            "/api/logs" => self.handle_logs_request(&request).await?,
            _ => serde_json::json!({"error": "Endpoint not found"}),
        };

        Ok(ApiResponse {
            status: 200,
            headers: std::collections::HashMap::new(),
            body: response_body,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }

    async fn handle_dashboard_request(&self, request: &ApiRequest) -> Result<serde_json::Value> {
        if let Some(session_id) = &request.session_id {
            let dashboard_data = self.get_dashboard_data(session_id).await?;
            Ok(serde_json::to_value(dashboard_data)?)
        } else {
            Ok(serde_json::json!({"error": "Authentication required"}))
        }
    }

    async fn handle_system_status_request(&self, _request: &ApiRequest) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "status": "online",
            "uptime": "48h 32m",
            "version": "0.1.0"
        }))
    }

    async fn handle_command_execution_request(&self, request: &ApiRequest) -> Result<serde_json::Value> {
        if let Some(session_id) = &request.session_id {
            if let Some(body) = &request.body {
                if let Some(command) = body.get("command").and_then(|c| c.as_str()) {
                    let result = self.execute_command_via_web(session_id, command).await?;
                    return Ok(serde_json::json!({"result": result}));
                }
            }
        }
        Ok(serde_json::json!({"error": "Invalid request"}))
    }

    async fn handle_logs_request(&self, request: &ApiRequest) -> Result<serde_json::Value> {
        if let Some(session_id) = &request.session_id {
            let logs = self.get_system_logs(session_id, 100).await?;
            Ok(serde_json::json!({"logs": logs}))
        } else {
            Ok(serde_json::json!({"error": "Authentication required"}))
        }
    }
}
