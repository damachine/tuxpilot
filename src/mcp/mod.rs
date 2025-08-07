use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use uuid::Uuid;

pub mod tools;
pub mod resources;

use crate::config::Config;
use crate::linux_integration::LinuxIntegration;

/// MCP Server for TuxPilot - provides Linux system tools to AI models
pub struct MCPServer {
    config: Config,
    linux_integration: LinuxIntegration,
    tools: HashMap<String, Box<dyn MCPTool>>,
    resources: HashMap<String, Box<dyn MCPResource>>,
    sessions: HashMap<String, MCPSession>,
}

/// MCP Tool trait - represents a tool that can be called by AI models
#[async_trait::async_trait]
pub trait MCPTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value, context: &MCPContext) -> Result<MCPToolResult>;
}

/// MCP Resource trait - represents a resource that can be accessed by AI models
#[async_trait::async_trait]
pub trait MCPResource: Send + Sync {
    fn uri(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn mime_type(&self) -> &str;
    async fn read(&self, context: &MCPContext) -> Result<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPSession {
    pub id: String,
    pub client_info: MCPClientInfo,
    pub capabilities: MCPCapabilities,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPCapabilities {
    pub tools: bool,
    pub resources: bool,
    pub prompts: bool,
    pub logging: bool,
}

#[derive(Debug, Clone)]
pub struct MCPContext {
    pub session_id: String,
    pub linux_integration: LinuxIntegration,
    pub config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPToolResult {
    pub success: bool,
    pub content: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPMessage {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: Option<String>,
    pub params: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl MCPServer {
    pub async fn new(config: Config, linux_integration: LinuxIntegration) -> Result<Self> {
        let mut server = Self {
            config,
            linux_integration,
            tools: HashMap::new(),
            resources: HashMap::new(),
            sessions: HashMap::new(),
        };

        // Register built-in tools
        server.register_builtin_tools().await?;
        server.register_builtin_resources().await?;

        Ok(server)
    }

    async fn register_builtin_tools(&mut self) -> Result<()> {
        // System information tools
        self.register_tool(Box::new(tools::SystemInfoTool::new())).await?;
        self.register_tool(Box::new(tools::PackageManagerTool::new())).await?;
        self.register_tool(Box::new(tools::ServiceManagerTool::new())).await?;
        self.register_tool(Box::new(tools::FileSystemTool::new())).await?;
        self.register_tool(Box::new(tools::ProcessManagerTool::new())).await?;
        self.register_tool(Box::new(tools::NetworkTool::new())).await?;
        self.register_tool(Box::new(tools::LogAnalyzerTool::new())).await?;

        Ok(())
    }

    async fn register_builtin_resources(&mut self) -> Result<()> {
        // System resources
        self.register_resource(Box::new(resources::SystemLogsResource::new())).await?;
        self.register_resource(Box::new(resources::ConfigFilesResource::new())).await?;
        self.register_resource(Box::new(resources::ProcessListResource::new())).await?;
        self.register_resource(Box::new(resources::SystemStatusResource::new())).await?;

        Ok(())
    }

    async fn register_tool(&mut self, tool: Box<dyn MCPTool>) -> Result<()> {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
        Ok(())
    }

    async fn register_resource(&mut self, resource: Box<dyn MCPResource>) -> Result<()> {
        let uri = resource.uri().to_string();
        self.resources.insert(uri, resource);
        Ok(())
    }

    pub async fn start(&mut self, port: u16) -> Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await
            .context("Failed to bind MCP server")?;

        println!("🔌 MCP Server listening on port {}", port);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("📡 New MCP client connected: {}", addr);
                    let server_clone = self.clone_for_session();
                    tokio::spawn(async move {
                        if let Err(e) = server_clone.handle_client(stream).await {
                            eprintln!("❌ Error handling MCP client: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to accept MCP connection: {}", e);
                }
            }
        }
    }

    fn clone_for_session(&self) -> MCPServerSession {
        MCPServerSession {
            config: self.config.clone(),
            linux_integration: self.linux_integration.clone(),
        }
    }

    async fn handle_client(&self, stream: TcpStream) -> Result<()> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            if let Ok(message) = serde_json::from_str::<MCPMessage>(&line.trim()) {
                let response = self.process_message(message).await?;
                let response_json = serde_json::to_string(&response)?;
                
                // Write response back to client
                // Note: In a real implementation, we'd need to handle the stream properly
                println!("📤 MCP Response: {}", response_json);
            }
            line.clear();
        }

        Ok(())
    }

    async fn process_message(&self, message: MCPMessage) -> Result<MCPMessage> {
        match message.method.as_deref() {
            Some("initialize") => self.handle_initialize(message).await,
            Some("tools/list") => self.handle_tools_list(message).await,
            Some("tools/call") => self.handle_tools_call(message).await,
            Some("resources/list") => self.handle_resources_list(message).await,
            Some("resources/read") => self.handle_resources_read(message).await,
            _ => Ok(MCPMessage {
                jsonrpc: "2.0".to_string(),
                id: message.id,
                method: None,
                params: None,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }),
        }
    }

    async fn handle_initialize(&self, message: MCPMessage) -> Result<MCPMessage> {
        Ok(MCPMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id,
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {
                        "listChanged": true
                    },
                    "resources": {
                        "subscribe": true,
                        "listChanged": true
                    },
                    "logging": {}
                },
                "serverInfo": {
                    "name": "TuxPilot MCP Server",
                    "version": "0.1.0"
                }
            })),
            error: None,
        })
    }

    async fn handle_tools_list(&self, message: MCPMessage) -> Result<MCPMessage> {
        let tools: Vec<serde_json::Value> = self.tools.values()
            .map(|tool| serde_json::json!({
                "name": tool.name(),
                "description": tool.description(),
                "inputSchema": tool.parameters()
            }))
            .collect();

        Ok(MCPMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id,
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "tools": tools
            })),
            error: None,
        })
    }

    async fn handle_tools_call(&self, message: MCPMessage) -> Result<MCPMessage> {
        if let Some(params) = message.params {
            if let (Some(name), Some(arguments)) = (
                params.get("name").and_then(|v| v.as_str()),
                params.get("arguments")
            ) {
                if let Some(tool) = self.tools.get(name) {
                    let context = MCPContext {
                        session_id: Uuid::new_v4().to_string(),
                        linux_integration: self.linux_integration.clone(),
                        config: self.config.clone(),
                    };

                    match tool.execute(arguments.clone(), &context).await {
                        Ok(result) => {
                            return Ok(MCPMessage {
                                jsonrpc: "2.0".to_string(),
                                id: message.id,
                                method: None,
                                params: None,
                                result: Some(serde_json::json!({
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": serde_json::to_string_pretty(&result.content)?
                                        }
                                    ]
                                })),
                                error: None,
                            });
                        }
                        Err(e) => {
                            return Ok(MCPMessage {
                                jsonrpc: "2.0".to_string(),
                                id: message.id,
                                method: None,
                                params: None,
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Tool execution failed: {}", e),
                                    data: None,
                                }),
                            });
                        }
                    }
                }
            }
        }

        Ok(MCPMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id,
            method: None,
            params: None,
            result: None,
            error: Some(MCPError {
                code: -32602,
                message: "Invalid params".to_string(),
                data: None,
            }),
        })
    }

    async fn handle_resources_list(&self, message: MCPMessage) -> Result<MCPMessage> {
        let resources: Vec<serde_json::Value> = self.resources.values()
            .map(|resource| serde_json::json!({
                "uri": resource.uri(),
                "name": resource.name(),
                "description": resource.description(),
                "mimeType": resource.mime_type()
            }))
            .collect();

        Ok(MCPMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id,
            method: None,
            params: None,
            result: Some(serde_json::json!({
                "resources": resources
            })),
            error: None,
        })
    }

    async fn handle_resources_read(&self, message: MCPMessage) -> Result<MCPMessage> {
        if let Some(params) = message.params {
            if let Some(uri) = params.get("uri").and_then(|v| v.as_str()) {
                if let Some(resource) = self.resources.get(uri) {
                    let context = MCPContext {
                        session_id: Uuid::new_v4().to_string(),
                        linux_integration: self.linux_integration.clone(),
                        config: self.config.clone(),
                    };

                    match resource.read(&context).await {
                        Ok(content) => {
                            return Ok(MCPMessage {
                                jsonrpc: "2.0".to_string(),
                                id: message.id,
                                method: None,
                                params: None,
                                result: Some(serde_json::json!({
                                    "contents": [
                                        {
                                            "uri": uri,
                                            "mimeType": resource.mime_type(),
                                            "text": content
                                        }
                                    ]
                                })),
                                error: None,
                            });
                        }
                        Err(e) => {
                            return Ok(MCPMessage {
                                jsonrpc: "2.0".to_string(),
                                id: message.id,
                                method: None,
                                params: None,
                                result: None,
                                error: Some(MCPError {
                                    code: -32603,
                                    message: format!("Resource read failed: {}", e),
                                    data: None,
                                }),
                            });
                        }
                    }
                }
            }
        }

        Ok(MCPMessage {
            jsonrpc: "2.0".to_string(),
            id: message.id,
            method: None,
            params: None,
            result: None,
            error: Some(MCPError {
                code: -32602,
                message: "Invalid params".to_string(),
                data: None,
            }),
        })
    }
}

#[derive(Debug, Clone)]
struct MCPServerSession {
    config: Config,
    linux_integration: LinuxIntegration,
}

impl MCPServerSession {
    async fn handle_client(&self, _stream: TcpStream) -> Result<()> {
        // Implementation would be similar to MCPServer::handle_client
        // but with session-specific state
        Ok(())
    }
}
