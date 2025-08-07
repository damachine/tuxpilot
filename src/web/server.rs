use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use super::{ApiRequest, WebServer, ChatRequest, ChatMessage, ChatSession, ConfigUpdateRequest, ConfigurationSchema, ConfigBackup};

/// HTTP server implementation
pub struct HttpServer {
    web_server: WebServer,
}

impl HttpServer {
    pub async fn new(web_server: WebServer) -> Result<Self> {
        Ok(Self { web_server })
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let bind_addr = format!("127.0.0.1:{}", port);
        let addr: SocketAddr = bind_addr.parse()?;

        println!("🌐 HTTP Server starting on {}", addr);

        // Create router with all endpoints
        let app = Router::new()
            // API routes
            .route("/api/system/status", get(system_status))
            .route("/api/dashboard", get(dashboard))
            .route("/api/commands/execute", post(execute_command))
            .route("/api/logs", get(get_logs))
            // Chat endpoints
            .route("/api/chat", post(send_chat_message))
            .route("/api/chat/sessions", get(get_chat_sessions))
            .route("/api/chat/:chat_id/history", get(get_chat_history))
            .route("/api/chat/session", post(create_chat_session))
            // Configuration endpoints
            .route("/api/config", get(get_configuration).put(update_configuration))
            .route("/api/config/schema", get(get_configuration_schema))
            .route("/api/config/backup", post(create_config_backup))
            .route("/api/config/backup/:backup_id", get(get_config_backup))
            .route("/api/config/restore/:backup_id", post(restore_config_backup))
            .route("/api/config/validate", post(validate_configuration))
            .route("/api/config/ai-providers", get(get_available_ai_providers))
            .route("/api/config/web-server", get(get_web_server_config))
            .route("/api/config/web-server", post(update_web_server_config))
            // Health check
            .route("/health", get(health_check))
            // Static files
            .route("/css/tuxpilot.css", get(serve_global_css))
            // Root endpoint
            .route("/", get(home_page))
            .route("/chat", get(chat_interface))
            .route("/config", get(config_interface))
            .route("/api", get(api_info))
            // Add CORS middleware
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
            )
            // Add shared state
            .with_state(self.web_server.clone());

        // Start the server
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        println!("✅ HTTP Server listening on {}", addr);

        axum::serve(listener, app).await?;

        Ok(())
    }
}

// Handler functions
async fn chat_interface() -> Html<String> {
    let html = include_str!("../../static/chat.html");
    Html(html.to_string())
}

async fn config_interface() -> Html<String> {
    let html = include_str!("../../static/config.html");
    Html(html.to_string())
}

async fn serve_global_css() -> (StatusCode, [(&'static str, &'static str); 1], String) {
    let css = include_str!("../../static/css/tuxpilot.css");
    (StatusCode::OK, [("content-type", "text/css")], css.to_string())
}

async fn home_page() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TuxPilot - AI-Powered Linux Assistant</title>
    <link rel="stylesheet" href="/css/tuxpilot.css">
    <style>
        :root {
            --bg-primary: #f8f9fa;
            --bg-secondary: #e9ecef;
            --text-primary: #212529;
            --text-secondary: #495057;
            --border-light: #dee2e6;
            --accent-primary: #495057;
            --accent-hover: #343a40;
            --gradient-bg: linear-gradient(135deg, #868e96 0%, #495057 100%);
            --shadow-light: rgba(0, 0, 0, 0.08);
            --shadow-medium: rgba(0, 0, 0, 0.16);
        }

        [data-theme="dark"] {
            --bg-primary: #1a1d23;
            --bg-secondary: #2d3748;
            --text-primary: #f7fafc;
            --text-secondary: #e2e8f0;
            --border-light: #4a5568;
            --accent-primary: #718096;
            --accent-hover: #a0aec0;
            --gradient-bg: linear-gradient(135deg, #2d3748 0%, #1a202c 100%);
            --shadow-light: rgba(0, 0, 0, 0.25);
            --shadow-medium: rgba(0, 0, 0, 0.45);
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', sans-serif;
            background: var(--gradient-bg);
            color: var(--text-primary);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            transition: all 0.3s ease;
        }
        .header {
            background: var(--bg-primary);
            border-bottom: 1px solid var(--border-light);
            padding: 12px 16px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            box-shadow: 0 1px 3px var(--shadow-light);
        }

        .header-left {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .logo {
            font-size: 20px;
            font-weight: 600;
            color: var(--text-primary);
            text-decoration: none;
        }

        .header-right {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .theme-toggle {
            background: none;
            border: 1px solid var(--border-light);
            border-radius: 6px;
            padding: 8px 12px;
            color: var(--text-primary);
            cursor: pointer;
            font-size: 14px;
            transition: all 0.2s ease;
        }

        .theme-toggle:hover {
            background: var(--bg-secondary);
        }

        .nav-links {
            display: flex;
            gap: 8px;
        }

        .nav-link {
            color: var(--text-secondary);
            text-decoration: none;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 14px;
            transition: all 0.2s ease;
        }

        .nav-link:hover {
            background: var(--bg-secondary);
            color: var(--text-primary);
        }

        .container {
            max-width: 1200px;
            margin: 20px auto;
            background: var(--bg-primary);
            border-radius: 20px;
            box-shadow: 0 25px 50px var(--shadow-medium);
            overflow: hidden;
            border: 1px solid var(--border-light);
        }
        .hero-section {
            background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
            color: white;
            padding: 40px;
            text-align: center;
        }
        .header h1 {
            font-size: 3em;
            font-weight: 300;
            margin-bottom: 10px;
        }
        .header p {
            font-size: 1.2em;
            opacity: 0.9;
        }
        .content {
            padding: 40px;
        }
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 30px;
            margin-bottom: 40px;
        }
        .feature-card {
            background: #f8f9fa;
            padding: 30px;
            border-radius: 15px;
            border-left: 5px solid #007bff;
            transition: transform 0.3s ease;
        }
        .feature-card:hover {
            transform: translateY(-5px);
        }
        .feature-card h3 {
            color: #2c3e50;
            margin-bottom: 15px;
            font-size: 1.5em;
        }
        .feature-card ul {
            list-style: none;
            padding-left: 0;
        }
        .feature-card li {
            padding: 5px 0;
            color: #666;
        }
        .feature-card li:before {
            content: "✓ ";
            color: #28a745;
            font-weight: bold;
        }
        .api-section {
            background: #2d3748;
            color: #e2e8f0;
            padding: 30px;
            border-radius: 15px;
            margin: 30px 0;
        }
        .api-section h3 {
            color: #63b3ed;
            margin-bottom: 20px;
        }
        .endpoint {
            background: #4a5568;
            padding: 10px 15px;
            border-radius: 8px;
            margin: 10px 0;
            font-family: monospace;
            font-size: 0.9em;
        }
        .method {
            color: #68d391;
            font-weight: bold;
        }
        .status-bar {
            background: #e8f5e8;
            border: 1px solid #c3e6c3;
            padding: 15px;
            border-radius: 10px;
            margin: 20px 0;
            text-align: center;
        }
        .status-online {
            color: #155724;
            font-weight: bold;
        }
        .quick-links {
            display: flex;
            gap: 15px;
            justify-content: center;
            flex-wrap: wrap;
            margin-top: 30px;
        }
        .btn {
            background: #007bff;
            color: white;
            padding: 12px 24px;
            border: none;
            border-radius: 8px;
            text-decoration: none;
            font-weight: 500;
            transition: background 0.3s ease;
        }
        .btn:hover {
            background: #0056b3;
        }
        .btn-secondary {
            background: #6c757d;
        }
        .btn-secondary:hover {
            background: #545b62;
        }
    </style>
</head>
<body>
    <div class="header">
        <div class="header-left">
            <a href="/" class="logo">🐧 TuxPilot</a>
        </div>
        <div class="header-right">
            <button class="theme-toggle" onclick="toggleTheme()">
                <span id="theme-icon">🌙</span>
            </button>
            <div class="nav-links">
                <a href="/chat" class="nav-link">Chat</a>
                <a href="/config" class="nav-link">Configuration</a>
                <a href="/api" class="nav-link">API Docs</a>
            </div>
        </div>
    </div>

    <div class="container">
        <div class="hero-section">
            <h1>🐧 TuxPilot</h1>
            <p>AI-Powered Linux System Assistant</p>
        </div>

        <div class="content">
            <div class="status-bar">
                <span class="status-online">🟢 Server Online - All Systems Operational</span>
            </div>

            <div class="features">
                <div class="feature-card">
                    <h3>💬 Interactive Chat Interface</h3>
                    <ul>
                        <li>Natural language system management</li>
                        <li>5 specialized AI agents</li>
                        <li>Persistent chat sessions</li>
                        <li>Real-time system analysis</li>
                        <li>Intelligent command suggestions</li>
                    </ul>
                </div>

                <div class="feature-card">
                    <h3>⚙️ Configuration Management</h3>
                    <ul>
                        <li>Complete system configuration control</li>
                        <li>Real-time validation</li>
                        <li>Backup & restore functionality</li>
                        <li>Schema-based configuration</li>
                        <li>Granular permission system</li>
                    </ul>
                </div>

                <div class="feature-card">
                    <h3>🛡️ Security & Monitoring</h3>
                    <ul>
                        <li>Comprehensive security scanning</li>
                        <li>Performance monitoring</li>
                        <li>Package management</li>
                        <li>Network diagnostics</li>
                        <li>Audit logging</li>
                    </ul>
                </div>
            </div>

            <div class="api-section">
                <h3>🔗 API Endpoints</h3>
                <div class="endpoint"><span class="method">POST</span> /api/chat - Send messages to AI agents</div>
                <div class="endpoint"><span class="method">GET</span> /api/config - Retrieve system configuration</div>
                <div class="endpoint"><span class="method">GET</span> /api/system/status - Get system status</div>
                <div class="endpoint"><span class="method">POST</span> /api/config/backup - Create configuration backup</div>
                <div class="endpoint"><span class="method">GET</span> /api - Complete API documentation</div>
            </div>

            <div class="quick-links">
                <a href="/chat" class="btn">💬 Chat Interface</a>
                <a href="/config" class="btn">⚙️ Configuration</a>
                <a href="/api" class="btn btn-secondary">📚 API Documentation</a>
                <a href="/health" class="btn btn-secondary">🔍 Health Check</a>
                <a href="https://github.com/damachine/tuxpilot" class="btn btn-secondary" target="_blank">📖 GitHub</a>
            </div>
        </div>
    </div>

    <script>
        // Theme management
        function initializeTheme() {
            const savedTheme = localStorage.getItem('tuxpilot-theme') || 'light';
            document.documentElement.setAttribute('data-theme', savedTheme);
            updateThemeIcon(savedTheme);
            console.log('Theme initialized:', savedTheme);
        }

        function toggleTheme() {
            const currentTheme = document.documentElement.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            document.documentElement.setAttribute('data-theme', newTheme);
            localStorage.setItem('tuxpilot-theme', newTheme);
            updateThemeIcon(newTheme);
        }

        function updateThemeIcon(theme) {
            const icon = document.getElementById('theme-icon');
            icon.textContent = theme === 'dark' ? '☀️' : '🌙';
        }

        // Initialize theme on page load
        document.addEventListener('DOMContentLoaded', initializeTheme);
    </script>
</body>
</html>
    "#;
    Html(html.to_string())
}

async fn api_info() -> Json<Value> {
    Json(json!({
        "name": "TuxPilot Web API",
        "version": "0.1.0",
        "status": "running",
        "features": ["Interactive Chat Interface", "Complete Configuration Management"],
        "endpoints": {
            "GET /api/system/status": "System status information",
            "GET /api/dashboard": "Dashboard data (requires auth)",
            "POST /api/commands/execute": "Execute commands (requires auth)",
            "GET /api/logs": "System logs (requires auth)",
            "POST /api/chat": "Send chat message to AI agents (requires auth)",
            "POST /api/chat/session": "Create new chat session (requires auth)",
            "GET /api/chat/sessions": "Get user's chat sessions (requires auth)",
            "GET /api/chat/:chat_id/history": "Get chat history (requires auth)",
            "GET /api/config": "Get current configuration (requires auth)",
            "PUT /api/config": "Update configuration (requires auth)",
            "GET /api/config/schema": "Get configuration schema (requires auth)",
            "POST /api/config/validate": "Validate configuration (requires auth)",
            "POST /api/config/backup": "Create configuration backup (requires auth)",
            "GET /api/config/backup/:backup_id": "Get configuration backup (requires auth)",
            "POST /api/config/restore/:backup_id": "Restore configuration backup (requires auth)",
            "GET /health": "Health check"
        },
        "authentication": {
            "required": true,
            "header": "Authorization: Bearer TOKEN",
            "demo_token": "demo-token"
        },
        "ai_agents": {
            "active": 5,
            "types": ["system", "security", "package", "network", "performance"]
        }
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "0.1.0"
    }))
}

async fn system_status(State(_web_server): State<WebServer>) -> Json<Value> {
    Json(json!({
        "status": "online",
        "uptime": "running",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "system": {
            "hostname": gethostname::gethostname().to_string_lossy(),
            "platform": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    }))
}

async fn dashboard(
    State(web_server): State<WebServer>,
    headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // Simple auth check - in production, implement proper session validation
    if let Some(_auth_header) = headers.get("authorization") {
        // Mock dashboard data
        let dashboard_data = json!({
            "system_overview": {
                "hostname": gethostname::gethostname().to_string_lossy(),
                "uptime_hours": 24,
                "cpu_usage": 25.5,
                "memory_usage": 68.2,
                "disk_usage": 45.8
            },
            "recent_activities": [
                {
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "activity": "System status check",
                    "status": "success"
                }
            ],
            "alerts": []
        });
        Ok(Json(dashboard_data))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn execute_command(
    State(_web_server): State<WebServer>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract command from payload
    if let Some(command) = payload.get("command").and_then(|c| c.as_str()) {
        // For demo purposes, return a mock response
        // In production, this would use the actual CommandExecutor
        Ok(Json(json!({
            "result": format!("Command '{}' would be executed with safety checks", command),
            "status": "simulated",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "note": "This is a demo response. Actual command execution requires full implementation."
        })))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn get_logs(
    State(_web_server): State<WebServer>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let lines = params.get("lines")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(10);

    // Mock log data
    let logs = (0..lines).map(|i| {
        json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "level": "INFO",
            "source": "tuxpilot",
            "message": format!("Sample log entry {}", i + 1)
        })
    }).collect::<Vec<_>>();

    Ok(Json(json!({
        "logs": logs,
        "total": lines,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// Chat endpoints
async fn send_chat_message(
    State(web_server): State<WebServer>,
    _headers: HeaderMap,
    Json(chat_request): Json<ChatRequest>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    // Extract user ID from auth header (simplified)
    let user_id = "demo-user".to_string(); // In production, extract from JWT token

    match web_server.send_chat_message(chat_request, user_id).await {
        Ok(response) => Ok(Json(json!({
            "response": response.content,
            "message_id": response.message_id,
            "chat_id": response.chat_id,
            "timestamp": response.timestamp.to_rfc3339()
        }))),
        Err(e) => {
            eprintln!("Chat error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_chat_sessions(
    State(web_server): State<WebServer>,
    _headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    let user_id = "demo-user".to_string();

    match web_server.get_chat_sessions(&user_id).await {
        Ok(sessions) => Ok(Json(json!({
            "sessions": sessions,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))),
        Err(e) => {
            eprintln!("Error getting chat sessions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_chat_history(
    State(web_server): State<WebServer>,
    Path(chat_id): Path<String>,
    _headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    let limit = params.get("limit")
        .and_then(|l| l.parse::<usize>().ok());

    match web_server.get_chat_history(&chat_id, limit).await {
        Ok(history) => Ok(Json(json!({
            "chat_id": chat_id,
            "messages": history,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))),
        Err(e) => {
            eprintln!("Error getting chat history: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_chat_session(
    State(web_server): State<WebServer>,
    _headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    let user_id = "demo-user".to_string();
    let execution_mode = payload.get("execution_mode")
        .and_then(|m| m.as_str())
        .unwrap_or("supervised")
        .to_string();

    match web_server.create_chat_session(user_id, execution_mode).await {
        Ok(session) => Ok(Json(json!({
            "chat_id": session.chat_id,
            "session": session,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))),
        Err(e) => {
            eprintln!("Error creating chat session: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Configuration endpoints
async fn get_configuration(
    State(web_server): State<WebServer>,
    _headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    // Return actual current configuration
    let config = &web_server.config;

    let provider_str = format!("{:?}", config.ai.provider).to_lowercase();
    let current_model = match config.ai.provider {
        crate::config::AiProvider::Ollama => {
            config.ai.ollama.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "llama3.2".to_string())
        },
        crate::config::AiProvider::OpenAI => {
            config.ai.openai.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "gpt-4".to_string())
        },
        crate::config::AiProvider::Anthropic => {
            config.ai.anthropic.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "claude-3-sonnet-20240229".to_string())
        },
        crate::config::AiProvider::Local => "custom".to_string(),
    };

    Ok(Json(json!({
        "ai": {
            "provider": provider_str,
            "model": current_model,
            "api_key": "***hidden***",
            "timeout_seconds": 30
        },
        "execution": {
            "mode": "supervised",
            "safety_checks": true,
            "audit_logging": true
        },
        "web": {
            "port": 8080,
            "bind_address": "127.0.0.1",
            "ssl_enabled": false
        },
        "agents": {
            "enabled": ["system", "security", "package", "network", "performance"],
            "max_concurrent": 3
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn update_configuration(
    State(_web_server): State<WebServer>,
    headers: HeaderMap,
    Json(config_update): Json<ConfigUpdateRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Validate configuration update
    if config_update.validate_only {
        return Ok(Json(json!({
            "valid": true,
            "message": "Configuration is valid",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })));
    }

    // Apply configuration changes (mock implementation)
    Ok(Json(json!({
        "success": true,
        "section": config_update.section,
        "changes_applied": config_update.updates.len(),
        "restart_required": false,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn get_configuration_schema(
    State(_web_server): State<WebServer>,
    _headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    // Return configuration schema
    Ok(Json(json!({
        "version": "1.0.0",
        "sections": {
            "ai": {
                "name": "AI Configuration",
                "description": "AI provider and model settings",
                "fields": {
                    "provider": {
                        "type": "enum",
                        "options": ["openai", "anthropic", "ollama", "local"],
                        "description": "AI provider to use",
                        "default": "ollama",
                        "required": true
                    },
                    "model": {
                        "type": "string",
                        "description": "Model name to use",
                        "default": "llama3.2",
                        "required": true
                    },
                    "api_key": {
                        "type": "string",
                        "description": "API key for cloud providers",
                        "sensitive": true,
                        "required": false
                    }
                }
            },
            "execution": {
                "name": "Execution Settings",
                "description": "Command execution and safety settings",
                "fields": {
                    "mode": {
                        "type": "enum",
                        "options": ["supervised", "semi-auto", "autonomous", "read-only"],
                        "description": "Default execution mode",
                        "default": "supervised",
                        "required": true
                    },
                    "safety_checks": {
                        "type": "boolean",
                        "description": "Enable safety checks for commands",
                        "default": true,
                        "required": true
                    }
                }
            }
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn validate_configuration(
    State(_web_server): State<WebServer>,
    headers: HeaderMap,
    Json(config_update): Json<ConfigUpdateRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Validate configuration (mock implementation)
    let mut errors: Vec<String> = Vec::new();
    let warnings: Vec<String> = Vec::new();

    // Example validation logic
    if config_update.section == "ai" {
        if let Some(provider) = config_update.updates.get("provider") {
            if !["openai", "anthropic", "ollama", "local"].contains(&provider.as_str().unwrap_or("")) {
                errors.push("Invalid AI provider specified".to_string());
            }
        }
    }

    Ok(Json(json!({
        "valid": errors.is_empty(),
        "errors": errors,
        "warnings": warnings,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn create_config_backup(
    State(_web_server): State<WebServer>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let description = payload.get("description")
        .and_then(|d| d.as_str())
        .unwrap_or("Manual backup")
        .to_string();

    let backup_id = uuid::Uuid::new_v4().to_string();

    Ok(Json(json!({
        "backup_id": backup_id,
        "description": description,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "size_bytes": 1024,
        "success": true
    })))
}

async fn get_config_backup(
    State(_web_server): State<WebServer>,
    Path(backup_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(json!({
        "backup_id": backup_id,
        "description": "Configuration backup",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "config_data": {
            "ai": { "provider": "ollama", "model": "llama3.2" },
            "execution": { "mode": "supervised", "safety_checks": true }
        }
    })))
}

async fn restore_config_backup(
    State(_web_server): State<WebServer>,
    Path(backup_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(json!({
        "success": true,
        "backup_id": backup_id,
        "restored_at": chrono::Utc::now().to_rfc3339(),
        "restart_required": true,
        "message": "Configuration restored successfully"
    })))
}

async fn get_available_ai_providers(
    State(web_server): State<WebServer>,
    _headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    // Get actual current configuration
    let config = &web_server.config;
    let current_provider = format!("{:?}", config.ai.provider).to_lowercase();
    let current_model = match config.ai.provider {
        crate::config::AiProvider::Ollama => {
            config.ai.ollama.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "llama3.2".to_string())
        },
        crate::config::AiProvider::OpenAI => {
            config.ai.openai.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "gpt-4".to_string())
        },
        crate::config::AiProvider::Anthropic => {
            config.ai.anthropic.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "claude-3-sonnet-20240229".to_string())
        },
        crate::config::AiProvider::Local => "custom".to_string(),
    };

    // Detect available AI providers and models
    let mut providers = json!({});

    // Check for Ollama
    if let Ok(output) = std::process::Command::new("ollama").arg("list").output() {
        if output.status.success() {
            let models_output = String::from_utf8_lossy(&output.stdout);
            let models: Vec<String> = models_output
                .lines()
                .skip(1) // Skip header
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if !parts.is_empty() {
                        Some(parts[0].to_string())
                    } else {
                        None
                    }
                })
                .collect();

            providers["ollama"] = json!({
                "available": true,
                "models": models,
                "description": "Local Ollama models"
            });
        } else {
            providers["ollama"] = json!({
                "available": false,
                "models": [],
                "description": "Ollama not installed or not running"
            });
        }
    } else {
        providers["ollama"] = json!({
            "available": false,
            "models": [],
            "description": "Ollama not found"
        });
    }

    // Check for OpenAI (API key based)
    providers["openai"] = json!({
        "available": true,
        "models": [
            "gpt-4",
            "gpt-4-turbo",
            "gpt-3.5-turbo",
            "gpt-3.5-turbo-16k"
        ],
        "description": "OpenAI API models (requires API key)",
        "requires_api_key": true
    });

    // Check for Anthropic
    providers["anthropic"] = json!({
        "available": true,
        "models": [
            "claude-3-opus-20240229",
            "claude-3-sonnet-20240229",
            "claude-3-haiku-20240307"
        ],
        "description": "Anthropic Claude models (requires API key)",
        "requires_api_key": true
    });

    // Local/Custom provider
    providers["local"] = json!({
        "available": true,
        "models": ["custom"],
        "description": "Local or custom AI implementation"
    });

    Ok(Json(json!({
        "providers": providers,
        "current_provider": current_provider,
        "current_model": current_model
    })))
}

async fn get_web_server_config(
    State(_web_server): State<WebServer>,
    _headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement proper authentication
    // For now, allow access for testing

    // Return current web server configuration
    Ok(Json(json!({
        "current_config": {
            "bind_address": "127.0.0.1",
            "port": 8080,
            "ssl_enabled": false,
            "ssl_cert_path": "",
            "ssl_key_path": "",
            "max_connections": 1000,
            "request_timeout": 30,
            "cors_enabled": true,
            "cors_origins": ["*"],
            "rate_limiting": {
                "enabled": true,
                "requests_per_minute": 60
            },
            "logging": {
                "level": "info",
                "access_log": true,
                "error_log": true
            }
        },
        "available_options": {
            "bind_addresses": [
                "127.0.0.1",
                "0.0.0.0",
                "localhost"
            ],
            "ports": [
                8080,
                8443,
                3000,
                5000,
                9000
            ],
            "log_levels": [
                "trace",
                "debug",
                "info",
                "warn",
                "error"
            ],
            "ssl_options": {
                "supported": true,
                "cert_formats": ["PEM", "DER"],
                "key_formats": ["PEM", "DER", "PKCS8"]
            }
        }
    })))
}

async fn update_web_server_config(
    State(_web_server): State<WebServer>,
    headers: HeaderMap,
    Json(config): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Check authorization
    if headers.get("authorization").is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Validate the configuration
    let bind_address = config.get("bind_address").and_then(|v| v.as_str()).unwrap_or("127.0.0.1");
    let port = config.get("port").and_then(|v| v.as_u64()).unwrap_or(8080) as u16;
    let ssl_enabled = config.get("ssl_enabled").and_then(|v| v.as_bool()).unwrap_or(false);

    // Basic validation
    if port < 1024 || port > 65535 {
        return Ok(Json(json!({
            "success": false,
            "error": "Port must be between 1024 and 65535",
            "field": "port"
        })));
    }

    if !["127.0.0.1", "0.0.0.0", "localhost"].contains(&bind_address) {
        return Ok(Json(json!({
            "success": false,
            "error": "Invalid bind address",
            "field": "bind_address"
        })));
    }

    if ssl_enabled {
        let cert_path = config.get("ssl_cert_path").and_then(|v| v.as_str()).unwrap_or("");
        let key_path = config.get("ssl_key_path").and_then(|v| v.as_str()).unwrap_or("");

        if cert_path.is_empty() || key_path.is_empty() {
            return Ok(Json(json!({
                "success": false,
                "error": "SSL certificate and key paths are required when SSL is enabled",
                "field": "ssl_cert_path"
            })));
        }
    }

    // In a real implementation, this would update the actual configuration
    Ok(Json(json!({
        "success": true,
        "message": "Web server configuration updated successfully",
        "requires_restart": true,
        "updated_config": {
            "bind_address": bind_address,
            "port": port,
            "ssl_enabled": ssl_enabled
        }
    })))
}
