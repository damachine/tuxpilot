# TuxPilot Web Server Architecture

The TuxPilot web server provides a modern, ChatGPT-inspired web interface for system administration through a comprehensive REST API and real-time WebSocket communication.

## 🏗️ Architecture Overview

```
Web Server Architecture:
├── 🌐 HTTP Server (Axum-based)
├── 🔌 WebSocket Handler (Real-time communication)
├── 🛡️ Authentication & Authorization
├── 📡 REST API Endpoints
├── 🎨 Static File Serving (Svelte UI)
└── 🔒 Security Middleware
```

## 🌐 HTTP Server

**File**: `server.rs`

The main web server implementation using the Axum framework for high-performance async HTTP handling.

### Server Configuration

```rust
pub struct WebServer {
    config: Config,
    linux_integration: LinuxIntegration,
    command_executor: Arc<RwLock<CommandExecutor>>,
    agent_system: Arc<RwLock<AgentSystem>>,
    active_sessions: Arc<RwLock<HashMap<String, WebSession>>>,
    auth_manager: auth::AuthManager,
    ai_client: AiClient,
    config_backups: Arc<RwLock<HashMap<String, ConfigBackup>>>,
}
```

### Server Initialization

```rust
impl WebServer {
    pub async fn new(
        config: Config,
        linux_integration: LinuxIntegration,
        command_executor: CommandExecutor,
        agent_system: AgentSystem,
    ) -> Result<Self> {
        let ai_client = AiClient::new(&config.ai).await?;
        let auth_manager = auth::AuthManager::new(&config).await?;
        
        Ok(WebServer {
            config,
            linux_integration,
            command_executor: Arc::new(RwLock::new(command_executor)),
            agent_system: Arc::new(RwLock::new(agent_system)),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            auth_manager,
            ai_client,
            config_backups: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn start(&self, bind_addr: &str, port: u16) -> Result<()> {
        let app = self.create_router().await?;
        let addr = format!("{}:{}", bind_addr, port);
        
        println!("🌐 TuxPilot web server starting on http://{}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}
```

### Router Configuration

```rust
impl WebServer {
    async fn create_router(&self) -> Result<Router> {
        let app = Router::new()
            // Static file serving
            .nest_service("/", ServeDir::new("static"))
            
            // API routes
            .route("/api/chat", post(Self::handle_chat))
            .route("/api/config", get(Self::get_config))
            .route("/api/config", post(Self::update_config))
            .route("/api/dashboard", get(Self::get_dashboard_data))
            .route("/api/system/status", get(Self::get_system_status))
            .route("/api/system/logs", get(Self::get_system_logs))
            .route("/api/execute", post(Self::execute_command))
            
            // WebSocket endpoint
            .route("/ws", get(Self::websocket_handler))
            
            // Authentication routes
            .route("/api/auth/login", post(Self::login))
            .route("/api/auth/logout", post(Self::logout))
            .route("/api/auth/session", get(Self::get_session))
            
            // Middleware
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(Arc::new(self.clone()));
            
        Ok(app)
    }
}
```

## 📡 REST API Endpoints

**File**: `api.rs`

Comprehensive REST API for all TuxPilot functionality.

### Chat API

```rust
#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub chat_id: String,
    pub context: Option<ChatContext>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub chat_id: String,
    pub timestamp: DateTime<Utc>,
    pub agent_used: Option<String>,
    pub commands_suggested: Vec<String>,
}

impl WebServer {
    pub async fn handle_chat(
        State(server): State<Arc<WebServer>>,
        Json(request): Json<ChatRequest>,
    ) -> Result<Json<ChatResponse>, StatusCode> {
        let response = server.process_chat_message(&request, "web-user").await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        Ok(Json(ChatResponse {
            response: response.content,
            chat_id: request.chat_id,
            timestamp: Utc::now(),
            agent_used: response.agent_used,
            commands_suggested: response.suggested_commands,
        }))
    }
}
```

### Configuration API

```rust
#[derive(Serialize)]
pub struct ConfigResponse {
    pub ai: AiConfig,
    pub execution: ExecutionConfig,
    pub web: WebConfig,
    pub safety: SafetyConfig,
}

impl WebServer {
    pub async fn get_config(
        State(server): State<Arc<WebServer>>,
    ) -> Result<Json<ConfigResponse>, StatusCode> {
        let config = &server.config;
        
        Ok(Json(ConfigResponse {
            ai: config.ai.clone(),
            execution: config.execution.clone(),
            web: config.web.clone(),
            safety: config.safety.clone(),
        }))
    }
    
    pub async fn update_config(
        State(server): State<Arc<WebServer>>,
        Json(new_config): Json<ConfigResponse>,
    ) -> Result<StatusCode, StatusCode> {
        // Validate configuration
        server.validate_config(&new_config).await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
            
        // Create backup
        server.backup_config().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        // Apply new configuration
        server.apply_config(new_config).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        Ok(StatusCode::OK)
    }
}
```

### Dashboard API

```rust
#[derive(Serialize)]
pub struct DashboardData {
    pub system_overview: SystemOverview,
    pub performance_metrics: PerformanceMetrics,
    pub alerts: Vec<SystemAlert>,
    pub recent_activities: Vec<ActivityLog>,
}

#[derive(Serialize)]
pub struct SystemOverview {
    pub hostname: String,
    pub uptime_hours: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub load_average: [f64; 3],
}

impl WebServer {
    pub async fn get_dashboard_data(
        State(server): State<Arc<WebServer>>,
    ) -> Result<Json<DashboardData>, StatusCode> {
        let system_info = server.linux_integration.get_system_info().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        let performance = server.linux_integration.get_performance_metrics().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        let alerts = server.get_system_alerts().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        let activities = server.get_recent_activities().await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
        Ok(Json(DashboardData {
            system_overview: SystemOverview {
                hostname: system_info.hostname,
                uptime_hours: system_info.uptime.as_secs_f64() / 3600.0,
                cpu_usage: performance.cpu_usage,
                memory_usage: performance.memory_usage,
                disk_usage: performance.disk_usage,
                load_average: performance.load_average,
            },
            performance_metrics: performance,
            alerts,
            recent_activities: activities,
        }))
    }
}
```

### Command Execution API

```rust
#[derive(Deserialize)]
pub struct ExecuteRequest {
    pub command: String,
    pub mode: Option<ExecutionMode>,
    pub confirm: Option<bool>,
}

#[derive(Serialize)]
pub struct ExecuteResponse {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: f64,
    pub commands_executed: Vec<String>,
}

impl WebServer {
    pub async fn execute_command(
        State(server): State<Arc<WebServer>>,
        Json(request): Json<ExecuteRequest>,
    ) -> Result<Json<ExecuteResponse>, StatusCode> {
        let start_time = Instant::now();
        
        // Create execution context
        let context = ExecutionContext {
            user_id: "web-user".to_string(),
            session_id: "web-session".to_string(),
            mode: request.mode.unwrap_or(server.config.execution.mode),
            require_confirmation: request.confirm.unwrap_or(server.config.execution.require_confirmation),
        };
        
        // Execute command through agent system
        let result = server.agent_system.read().await
            .execute_user_request(&request.command, &context).await;
            
        let execution_time = start_time.elapsed().as_secs_f64();
        
        match result {
            Ok(results) => {
                let output = results.iter()
                    .map(|r| r.output.clone())
                    .collect::<Vec<_>>()
                    .join("\n");
                    
                let commands = results.iter()
                    .flat_map(|r| r.commands_executed.clone())
                    .collect();
                    
                Ok(Json(ExecuteResponse {
                    success: true,
                    output,
                    error: None,
                    execution_time,
                    commands_executed: commands,
                }))
            }
            Err(e) => {
                Ok(Json(ExecuteResponse {
                    success: false,
                    output: String::new(),
                    error: Some(e.to_string()),
                    execution_time,
                    commands_executed: vec![],
                }))
            }
        }
    }
}
```

## 🔌 WebSocket Communication

**File**: `websocket.rs`

Real-time communication for live updates and interactive features.

### WebSocket Handler

```rust
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(server): State<Arc<WebServer>>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, server))
}

async fn handle_websocket(socket: WebSocket, server: Arc<WebServer>) {
    let (mut sender, mut receiver) = socket.split();
    
    // Create session
    let session_id = Uuid::new_v4().to_string();
    
    // Handle incoming messages
    let server_clone = server.clone();
    let session_id_clone = session_id.clone();
    
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                if let Err(e) = handle_websocket_message(
                    msg, 
                    &server_clone, 
                    &session_id_clone
                ).await {
                    eprintln!("WebSocket message error: {}", e);
                    break;
                }
            }
        }
    });
    
    // Send periodic updates
    let server_clone = server.clone();
    let send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            // Send system status update
            if let Ok(status) = server_clone.get_system_status().await {
                let message = WebSocketMessage::SystemStatus(status);
                if let Ok(json) = serde_json::to_string(&message) {
                    if sender.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = receive_task => {},
        _ = send_task => {},
    }
}
```

### WebSocket Message Types

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    // Client to server
    ChatMessage {
        message: String,
        chat_id: String,
    },
    ExecuteCommand {
        command: String,
        mode: Option<ExecutionMode>,
    },
    SubscribeToUpdates {
        types: Vec<UpdateType>,
    },
    
    // Server to client
    ChatResponse {
        response: String,
        chat_id: String,
        timestamp: DateTime<Utc>,
    },
    SystemStatus {
        cpu_usage: f64,
        memory_usage: f64,
        disk_usage: f64,
        alerts: Vec<SystemAlert>,
    },
    CommandOutput {
        output: String,
        success: bool,
        execution_time: f64,
    },
    Error {
        message: String,
        code: Option<String>,
    },
}
```

## 🛡️ Authentication & Authorization

**File**: `auth.rs`

Comprehensive authentication and authorization system for web access.

### Authentication Manager

```rust
pub struct AuthManager {
    users: HashMap<String, User>,
    api_keys: HashMap<String, ApiKey>,
    config: Config,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub permissions: Vec<Permission>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub enum UserRole {
    Admin,
    User,
    ReadOnly,
}
```

### Session Management

```rust
#[derive(Clone, Debug)]
pub struct WebSession {
    pub id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub permissions: Vec<Permission>,
}

impl AuthManager {
    pub async fn create_session(
        &mut self,
        user_id: String,
        ip_address: String,
        user_agent: String,
    ) -> Result<WebSession> {
        let session = WebSession {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            ip_address,
            user_agent,
            permissions: self.get_user_permissions(&user_id).await?,
        };
        
        Ok(session)
    }
    
    pub async fn validate_session(&self, session_id: &str) -> Result<bool> {
        // Check session validity, expiration, etc.
        Ok(true) // Simplified
    }
}
```

### Permission System

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Permission {
    // System operations
    ReadSystem,
    WriteSystem,
    ExecuteCommands,
    
    // Configuration
    ReadConfig,
    WriteConfig,
    
    // User management
    ManageUsers,
    ViewUsers,
    
    // API access
    ApiAccess,
    AdminApi,
}

pub fn check_permission(
    required: Permission,
    user_permissions: &[Permission],
) -> bool {
    user_permissions.contains(&required) || 
    user_permissions.contains(&Permission::AdminApi)
}
```

## 🎨 Static File Serving

The web server serves the built Svelte application from the `static/` directory:

```rust
// Serve static files (Svelte app)
.nest_service("/", ServeDir::new("static"))

// Fallback for SPA routing
.fallback_service(ServeFile::new("static/index.html"))
```

### Build Integration

The static files are automatically generated by the build process:

```bash
# Build web UI and copy to static directory
cd web-ui
npm run build
cp -r build/* ../static/
```

## 🔒 Security Features

### CORS Configuration

```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
```

### Rate Limiting

```rust
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn check_rate_limit(&self, client_ip: &str) -> bool {
        let mut requests = self.requests.write().unwrap();
        let now = Instant::now();
        
        let client_requests = requests.entry(client_ip.to_string())
            .or_insert_with(Vec::new);
            
        // Remove old requests
        client_requests.retain(|&time| now.duration_since(time) < self.window);
        
        // Check if under limit
        if client_requests.len() < self.max_requests {
            client_requests.push(now);
            true
        } else {
            false
        }
    }
}
```

### Input Validation

```rust
pub fn validate_chat_request(request: &ChatRequest) -> Result<()> {
    if request.message.is_empty() {
        return Err(ValidationError::EmptyMessage);
    }
    
    if request.message.len() > 10000 {
        return Err(ValidationError::MessageTooLong);
    }
    
    if request.chat_id.is_empty() {
        return Err(ValidationError::InvalidChatId);
    }
    
    Ok(())
}
```

## 📊 Monitoring & Metrics

### Request Metrics

```rust
pub struct WebMetrics {
    pub total_requests: AtomicU64,
    pub successful_requests: AtomicU64,
    pub failed_requests: AtomicU64,
    pub average_response_time: AtomicU64,
    pub active_connections: AtomicU32,
    pub websocket_connections: AtomicU32,
}

impl WebMetrics {
    pub fn record_request(&self, duration: Duration, success: bool) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        
        if success {
            self.successful_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_requests.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update average response time
        let duration_ms = duration.as_millis() as u64;
        self.average_response_time.store(duration_ms, Ordering::Relaxed);
    }
}
```

## 🔧 Configuration

### Web Server Configuration

```toml
[web]
port = 8080
bind_address = "127.0.0.1"
ssl_enabled = false
ssl_cert_path = ""
ssl_key_path = ""
max_connections = 1000
request_timeout = "30s"
static_file_cache = "1h"

[web.cors]
allow_origins = ["http://localhost:3000"]
allow_credentials = true

[web.rate_limiting]
enabled = true
requests_per_minute = 60
burst_size = 10

[web.session]
timeout = "1h"
secure_cookies = true
same_site = "strict"
```

## 🚀 Usage Examples

### Starting the Web Server

```bash
# Start with default configuration
tuxpilot web

# Start with custom port
tuxpilot web --port 8082

# Start with SSL
tuxpilot web --ssl --cert /path/to/cert.pem --key /path/to/key.pem
```

### API Usage Examples

```bash
# Chat with AI
curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Show system status", "chat_id": "test"}'

# Get dashboard data
curl http://localhost:8080/api/dashboard

# Execute command
curl -X POST http://localhost:8080/api/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "ls -la /home", "mode": "supervised"}'

# Get configuration
curl http://localhost:8080/api/config

# Update configuration
curl -X POST http://localhost:8080/api/config \
  -H "Content-Type: application/json" \
  -d '{"ai": {"provider": "ollama", "model": "llama3.1:8b"}}'
```
