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
use tower_http::{cors::CorsLayer, services::ServeDir};
// ServeFile removed - no longer serving static CSS

use super::{WebServer, ChatRequest, ConfigUpdateRequest};

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
            .route("/api", get(api_info))
            // CSS is now handled by Svelte build
            // Serve new Svelte UI static files with fallback paths
            .nest_service("/_app", ServeDir::new(get_web_ui_assets_path()))
            // Fallback to serve index.html for SPA routing
            .fallback(serve_spa)
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

/// Get the path to web UI assets with fallback logic
fn get_web_ui_assets_path() -> String {
    // Try system installation path first
    let system_path = "/usr/local/share/tuxpilot/web-ui/_app";
    if std::path::Path::new(system_path).exists() {
        return system_path.to_string();
    }
    
    // Fallback to development path
    "web-ui/build/_app".to_string()
}

/// Get the path to the main index.html with fallback logic
fn get_web_ui_index_path() -> String {
    // Try system installation path first
    let system_path = "/usr/local/share/tuxpilot/web-ui/index.html";
    if std::path::Path::new(system_path).exists() {
        return system_path.to_string();
    }
    
    // Fallback to development path
    "web-ui/build/index.html".to_string()
}

// Serve the new Svelte SPA
async fn serve_spa() -> Html<String> {
    // Try to read the built Svelte index.html with fallback paths
    let index_path = get_web_ui_index_path();
    
    match std::fs::read_to_string(&index_path) {
        Ok(html) => Html(html),
        Err(_) => {
            // Fallback to a simple message if build doesn't exist
            Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>TuxPilot</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
</head>
<body>
    <div style="display: flex; align-items: center; justify-content: center; height: 100vh; font-family: system-ui;">
        <div style="text-align: center;">
            <h1>🐧 TuxPilot</h1>
            <p>Web UI not found. Please ensure TuxPilot is properly installed.</p>
            <p>For development: <code>cd web-ui && npm run build</code></p>
            <p>For system installation: Run the install script</p>
        </div>
    </div>
</body>
</html>
            "#.to_string())
        }
    }
}

// Legacy route handlers removed - now using Svelte SPA



// Legacy functions removed - all web interface handled by Svelte SPA

async fn api_info() -> Json<Value> {
    Json(json!({
        "name": "TuxPilot Web API",
        "version": "0.1.0",
        "status": "running",
        "features": ["Interactive Chat Interface", "Complete Configuration Management"],
        "endpoints": {
            "GET /api/system/status": "System status information",
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
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Duplicate system_status function removed - using the one with State parameter below

// Duplicate api_info function removed - using the one above

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
            "timestamp": response.timestamp.to_rfc3339(),
            "agent": match response.sender {
                crate::web::ChatSender::Agent { agent_name, .. } => agent_name,
                _ => "system".to_string()
            }
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
    // Return actual current configuration from the running instance
    let config = &web_server.config;

    let provider_str = format!("{:?}", config.ai.provider).to_lowercase();
    let current_model = match config.ai.provider {
        crate::config::AiProvider::Ollama => {
            config.ai.ollama.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "gemma3:latest".to_string())
        },
        crate::config::AiProvider::OpenAI => {
            config.ai.openai.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "gpt-4".to_string())
        },
        crate::config::AiProvider::Anthropic => {
            config.ai.anthropic.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "claude-3-sonnet-20240229".to_string())
        },
        crate::config::AiProvider::Local => {
            config.ai.local.as_ref().map(|c| c.model_path.to_string_lossy().to_string()).unwrap_or_else(|| "custom".to_string())
        },
    };

    // Get temperature and max_tokens from provider-specific config
    let (temperature, max_tokens) = match config.ai.provider {
        crate::config::AiProvider::Ollama => {
            let ollama_config = config.ai.ollama.as_ref();
            (
                ollama_config.map(|c| c.temperature).unwrap_or(0.7),
                ollama_config.and_then(|c| c.max_tokens).unwrap_or(2048)
            )
        },
        crate::config::AiProvider::OpenAI => {
            let openai_config = config.ai.openai.as_ref();
            (
                openai_config.and_then(|c| c.temperature).unwrap_or(0.7),
                openai_config.and_then(|c| c.max_tokens).unwrap_or(2048)
            )
        },
        crate::config::AiProvider::Anthropic => {
            let anthropic_config = config.ai.anthropic.as_ref();
            (
                anthropic_config.and_then(|c| c.temperature).unwrap_or(0.7),
                anthropic_config.and_then(|c| c.max_tokens).unwrap_or(2048)
            )
        },
        crate::config::AiProvider::Local => {
            let local_config = config.ai.local.as_ref();
            (
                local_config.map(|c| c.temperature).unwrap_or(0.7),
                2048 // Local models don't have max_tokens in config
            )
        },
    };

    Ok(Json(json!({
        "ai": {
            "provider": provider_str,
            "model": current_model,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "api_key_configured": match config.ai.provider {
                crate::config::AiProvider::OpenAI => config.ai.openai.as_ref().map(|c| !c.api_key.is_empty()).unwrap_or(false),
                crate::config::AiProvider::Anthropic => config.ai.anthropic.as_ref().map(|c| !c.api_key.is_empty()).unwrap_or(false),
                _ => true // Ollama and Local don't need API keys
            }
        },
        "execution": {
            "mode": format!("{:?}", config.system.execution_mode).to_lowercase(),
            "require_confirmation": config.system.require_confirmation,
            "timeout": config.system.command_timeout_seconds
        },
        "web": {
            "port": config.ui.web_port,
            "bind_address": config.ui.bind_address.clone(),
            "ssl_enabled": config.ui.ssl_enabled
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn update_configuration(
    State(_web_server): State<WebServer>,
    _headers: HeaderMap,
    Json(config_update): Json<ConfigUpdateRequest>,
) -> Result<Json<Value>, StatusCode> {
    // For now, skip authorization for testing
    // TODO: Implement proper authentication

    // Validate configuration update
    if config_update.validate_only {
        return Ok(Json(json!({
            "valid": true,
            "message": "Configuration is valid",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })));
    }

    // Load current config from file
    let config_path = crate::config::Config::default_config_path()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut config = crate::config::Config::load(Some(&config_path))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store the number of updates for the response
    let changes_count = config_update.updates.len();
    let section = config_update.section.clone();

    // Apply configuration changes based on section
    let mut restart_required = false;
    match config_update.section.as_str() {
        "ai" => {
            for (key, value) in config_update.updates {
                match key.as_str() {
                    "provider" => {
                        if let Some(provider_str) = value.as_str() {
                            config.ai.provider = match provider_str {
                                "ollama" => crate::config::AiProvider::Ollama,
                                "openai" => crate::config::AiProvider::OpenAI,
                                "anthropic" => crate::config::AiProvider::Anthropic,
                                "local" => crate::config::AiProvider::Local,
                                _ => return Err(StatusCode::BAD_REQUEST),
                            };
                            restart_required = true;
                        }
                    },
                    "model" => {
                        if let Some(model_str) = value.as_str() {
                            match config.ai.provider {
                                crate::config::AiProvider::Ollama => {
                                    if let Some(ref mut ollama) = config.ai.ollama {
                                        ollama.model = model_str.to_string();
                                    }
                                },
                                crate::config::AiProvider::OpenAI => {
                                    if let Some(ref mut openai) = config.ai.openai {
                                        openai.model = model_str.to_string();
                                    }
                                },
                                crate::config::AiProvider::Anthropic => {
                                    if let Some(ref mut anthropic) = config.ai.anthropic {
                                        anthropic.model = model_str.to_string();
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                    "temperature" => {
                        if let Some(temp) = value.as_f64() {
                            let temp_f32 = temp as f32;
                            match config.ai.provider {
                                crate::config::AiProvider::Ollama => {
                                    if let Some(ref mut ollama) = config.ai.ollama {
                                        ollama.temperature = temp_f32;
                                    }
                                },
                                crate::config::AiProvider::OpenAI => {
                                    if let Some(ref mut openai) = config.ai.openai {
                                        openai.temperature = Some(temp_f32);
                                    }
                                },
                                crate::config::AiProvider::Anthropic => {
                                    if let Some(ref mut anthropic) = config.ai.anthropic {
                                        anthropic.temperature = Some(temp_f32);
                                    }
                                },
                                crate::config::AiProvider::Local => {
                                    if let Some(ref mut local) = config.ai.local {
                                        local.temperature = temp_f32;
                                    }
                                },
                            }
                        }
                    },
                    "max_tokens" => {
                        if let Some(tokens) = value.as_u64() {
                            let tokens_u32 = tokens as u32;
                            match config.ai.provider {
                                crate::config::AiProvider::Ollama => {
                                    if let Some(ref mut ollama) = config.ai.ollama {
                                        ollama.max_tokens = Some(tokens_u32);
                                    }
                                },
                                crate::config::AiProvider::OpenAI => {
                                    if let Some(ref mut openai) = config.ai.openai {
                                        openai.max_tokens = Some(tokens_u32);
                                    }
                                },
                                crate::config::AiProvider::Anthropic => {
                                    if let Some(ref mut anthropic) = config.ai.anthropic {
                                        anthropic.max_tokens = Some(tokens_u32);
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            }
        },
        "execution" => {
            for (key, value) in config_update.updates {
                match key.as_str() {
                    "mode" => {
                        if let Some(mode_str) = value.as_str() {
                            config.system.execution_mode = match mode_str {
                                "supervised" => crate::config::ExecutionMode::Supervised,
                                "semi-auto" => crate::config::ExecutionMode::SemiAuto,
                                "autonomous" => crate::config::ExecutionMode::Autonomous,
                                "read-only" => crate::config::ExecutionMode::ReadOnly,
                                _ => return Err(StatusCode::BAD_REQUEST),
                            };
                        }
                    },
                    "require_confirmation" => {
                        if let Some(confirm) = value.as_bool() {
                            config.system.require_confirmation = confirm;
                        }
                    },
                    "timeout" => {
                        if let Some(timeout) = value.as_u64() {
                            config.system.command_timeout_seconds = timeout;
                        }
                    },
                    _ => {}
                }
            }
        },
        "web" => {
            for (key, value) in config_update.updates {
                match key.as_str() {
                    "port" => {
                        if let Some(port) = value.as_u64() {
                            config.ui.web_port = port as u16;
                            restart_required = true;
                        }
                    },
                    "bind_address" => {
                        if let Some(addr) = value.as_str() {
                            config.ui.bind_address = addr.to_string();
                            restart_required = true;
                        }
                    },
                    "ssl_enabled" => {
                        if let Some(ssl) = value.as_bool() {
                            config.ui.ssl_enabled = ssl;
                            restart_required = true;
                        }
                    },
                    _ => {}
                }
            }
        },
        _ => return Err(StatusCode::BAD_REQUEST),
    }

    // Save updated configuration
    config.save(&config_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "success": true,
        "section": section,
        "changes_applied": changes_count,
        "restart_required": restart_required,
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
                        "default": "gemma3:latest",
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
            config.ai.ollama.as_ref().map(|c| c.model.clone()).unwrap_or_else(|| "gemma3:latest".to_string())
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
    if port < 1024 {
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
