# TuxPilot API Documentation

This document provides comprehensive documentation for the TuxPilot REST API and WebSocket interfaces.

## 🌐 Base URL

```
http://127.0.0.1:8080/api
```

For HTTPS (when SSL is enabled):
```
https://127.0.0.1:8080/api
```

## 🔐 Authentication

Currently, TuxPilot uses session-based authentication for the web interface. API keys and token-based authentication are planned for future releases.

### Session Authentication

```bash
# Login (if authentication is enabled)
curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "user", "password": "password"}'

# Use session cookie for subsequent requests
curl -X GET http://127.0.0.1:8080/api/config \
  -H "Cookie: session_id=your-session-id"
```

## 💬 Chat API

### Send Chat Message

**Endpoint**: `POST /api/chat`

Send a message to the AI assistant and receive a response.

**Request Body**:
```json
{
  "message": "Show me system information",
  "chat_id": "unique-chat-session-id",
  "context": {
    "execution_mode": "supervised",
    "user_preferences": {}
  }
}
```

**Response**:
```json
{
  "response": "Here's your system information:\n\nHostname: myserver\nUptime: 2 days, 4 hours\nCPU: Intel Core i7-9700K\nMemory: 16GB (8GB used)\nDisk: 512GB SSD (60% used)",
  "chat_id": "unique-chat-session-id",
  "timestamp": "2024-01-15T10:30:00Z",
  "agent_used": "system_agent",
  "commands_suggested": [
    "uname -a",
    "uptime",
    "free -h",
    "df -h"
  ]
}
```

**Example**:
```bash
curl -X POST http://127.0.0.1:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Check disk usage and clean up if needed",
    "chat_id": "maintenance-session-001"
  }'
```

## ⚙️ Configuration API

### Get Configuration

**Endpoint**: `GET /api/config`

Retrieve the current TuxPilot configuration.

**Response**:
```json
{
  "ai": {
    "provider": "ollama",
    "model": "llama3.1:8b",
    "temperature": 0.8,
    "max_tokens": 4096
  },
  "execution": {
    "mode": "supervised",
    "require_confirmation": true,
    "timeout": 300
  },
  "web": {
    "port": 8080,
    "bind_address": "127.0.0.1",
    "ssl_enabled": false
  },
  "safety": {
    "risk_threshold": "medium",
    "enable_audit": true
  }
}
```

### Update Configuration

**Endpoint**: `POST /api/config`

Update TuxPilot configuration settings.

**Request Body**:
```json
{
  "ai": {
    "provider": "ollama",
    "model": "llama3.1:8b",
    "temperature": 0.7
  },
  "execution": {
    "mode": "semi-auto",
    "require_confirmation": false
  }
}
```

**Response**: `200 OK` on success, `400 Bad Request` on validation error.

**Example**:
```bash
curl -X POST http://127.0.0.1:8080/api/config \
  -H "Content-Type: application/json" \
  -d '{
    "ai": {
      "provider": "openai",
      "model": "gpt-4",
      "api_key": "your-api-key"
    }
  }'
```

## 📊 Dashboard API

### Get Dashboard Data

**Endpoint**: `GET /api/dashboard`

Retrieve comprehensive system dashboard data.

**Response**:
```json
{
  "system_overview": {
    "hostname": "myserver",
    "uptime_hours": 48.5,
    "cpu_usage": 25.3,
    "memory_usage": 67.8,
    "disk_usage": 45.2,
    "load_average": [0.8, 0.9, 1.1]
  },
  "performance_metrics": {
    "cpu_cores": 8,
    "total_memory": 16384,
    "available_memory": 5242,
    "disk_total": 512000,
    "disk_used": 231424,
    "network_rx": 1024000,
    "network_tx": 512000
  },
  "alerts": [
    {
      "id": "alert-001",
      "type": "warning",
      "message": "High memory usage detected",
      "timestamp": "2024-01-15T10:25:00Z",
      "severity": "medium"
    }
  ],
  "recent_activities": [
    {
      "id": "activity-001",
      "activity": "Installed package: nginx",
      "timestamp": "2024-01-15T10:20:00Z",
      "status": "success",
      "user": "admin"
    }
  ]
}
```

## 🖥️ System Status API

### Get System Status

**Endpoint**: `GET /api/system/status`

Get current system status and health information.

**Response**:
```json
{
  "status": "healthy",
  "uptime": 175320,
  "load_average": [0.8, 0.9, 1.1],
  "memory": {
    "total": 16777216,
    "used": 11374182,
    "available": 5403034,
    "percentage": 67.8
  },
  "cpu": {
    "cores": 8,
    "usage_percentage": 25.3,
    "temperature": 45.2
  },
  "disk": {
    "total": 512000000,
    "used": 231424000,
    "available": 280576000,
    "percentage": 45.2
  },
  "services": [
    {
      "name": "nginx",
      "status": "active",
      "enabled": true
    },
    {
      "name": "ssh",
      "status": "active",
      "enabled": true
    }
  ]
}
```

### Get System Logs

**Endpoint**: `GET /api/system/logs`

Retrieve system logs with optional filtering.

**Query Parameters**:
- `level`: Log level filter (debug, info, warn, error)
- `limit`: Number of log entries to return (default: 100)
- `since`: ISO timestamp to filter logs since
- `service`: Filter logs for specific service

**Response**:
```json
{
  "logs": [
    {
      "timestamp": "2024-01-15T10:30:00Z",
      "level": "info",
      "service": "nginx",
      "message": "Server started successfully",
      "pid": 1234
    },
    {
      "timestamp": "2024-01-15T10:29:45Z",
      "level": "warn",
      "service": "systemd",
      "message": "Service restart detected",
      "pid": 1
    }
  ],
  "total_count": 1250,
  "filtered_count": 2
}
```

**Example**:
```bash
curl "http://127.0.0.1:8080/api/system/logs?level=error&limit=50&since=2024-01-15T00:00:00Z"
```

## 🔧 Command Execution API

### Execute Command

**Endpoint**: `POST /api/execute`

Execute a command or request through the TuxPilot agent system.

**Request Body**:
```json
{
  "command": "install docker and configure it",
  "mode": "supervised",
  "confirm": true,
  "context": {
    "user_id": "admin",
    "session_id": "session-123"
  }
}
```

**Response**:
```json
{
  "success": true,
  "output": "Docker installed successfully and configured for development use.\n\nServices started:\n- docker.service\n- docker.socket\n\nUser added to docker group: admin",
  "error": null,
  "execution_time": 45.2,
  "commands_executed": [
    "sudo pacman -S docker docker-compose",
    "sudo systemctl enable docker",
    "sudo systemctl start docker",
    "sudo usermod -aG docker admin"
  ]
}
```

**Error Response**:
```json
{
  "success": false,
  "output": "",
  "error": "Package 'nonexistent-package' not found in repositories",
  "execution_time": 2.1,
  "commands_executed": []
}
```

## 🔌 WebSocket API

### Connection

**Endpoint**: `ws://127.0.0.1:8080/ws`

Establish a WebSocket connection for real-time communication.

### Message Types

#### Client to Server Messages

**Chat Message**:
```json
{
  "type": "ChatMessage",
  "message": "Show me current CPU usage",
  "chat_id": "realtime-session"
}
```

**Execute Command**:
```json
{
  "type": "ExecuteCommand",
  "command": "restart nginx service",
  "mode": "supervised"
}
```

**Subscribe to Updates**:
```json
{
  "type": "SubscribeToUpdates",
  "types": ["system_status", "alerts", "command_output"]
}
```

#### Server to Client Messages

**Chat Response**:
```json
{
  "type": "ChatResponse",
  "response": "Current CPU usage is 25.3%",
  "chat_id": "realtime-session",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

**System Status Update**:
```json
{
  "type": "SystemStatus",
  "cpu_usage": 28.7,
  "memory_usage": 69.2,
  "disk_usage": 45.2,
  "alerts": []
}
```

**Command Output**:
```json
{
  "type": "CommandOutput",
  "output": "nginx.service restarted successfully",
  "success": true,
  "execution_time": 1.2
}
```

**Error Message**:
```json
{
  "type": "Error",
  "message": "Command execution failed",
  "code": "EXECUTION_ERROR"
}
```

### WebSocket Example (JavaScript)

```javascript
const ws = new WebSocket('ws://127.0.0.1:8080/ws');

ws.onopen = function() {
    console.log('Connected to TuxPilot WebSocket');
    
    // Subscribe to system updates
    ws.send(JSON.stringify({
        type: 'SubscribeToUpdates',
        types: ['system_status', 'alerts']
    }));
};

ws.onmessage = function(event) {
    const message = JSON.parse(event.data);
    
    switch(message.type) {
        case 'SystemStatus':
            updateDashboard(message);
            break;
        case 'ChatResponse':
            displayChatMessage(message);
            break;
        case 'Error':
            console.error('WebSocket error:', message.message);
            break;
    }
};

// Send a chat message
function sendChatMessage(text) {
    ws.send(JSON.stringify({
        type: 'ChatMessage',
        message: text,
        chat_id: 'web-session-' + Date.now()
    }));
}
```

## 📝 Response Codes

### HTTP Status Codes

- `200 OK`: Request successful
- `400 Bad Request`: Invalid request format or parameters
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Endpoint not found
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error

### Custom Error Codes

- `INVALID_CONFIG`: Configuration validation failed
- `AI_UNAVAILABLE`: AI provider not accessible
- `EXECUTION_FAILED`: Command execution failed
- `PERMISSION_DENIED`: Insufficient permissions for operation
- `RATE_LIMITED`: Too many requests

## 🔒 Security Considerations

### Rate Limiting

The API implements rate limiting to prevent abuse:
- **Default limit**: 60 requests per minute per IP
- **Burst limit**: 10 requests per second
- **Headers**: `X-RateLimit-Remaining`, `X-RateLimit-Reset`

### Input Validation

All API inputs are validated:
- **Command injection protection**
- **SQL injection prevention**
- **XSS protection**
- **Parameter validation**

### CORS Policy

Cross-Origin Resource Sharing (CORS) is configured for web interface access:
```
Access-Control-Allow-Origin: http://localhost:3000
Access-Control-Allow-Methods: GET, POST, PUT, DELETE
Access-Control-Allow-Headers: Content-Type, Authorization
```

## 📚 SDK and Libraries

### Python SDK Example

```python
import requests
import json

class TuxPilotClient:
    def __init__(self, base_url="http://127.0.0.1:8080"):
        self.base_url = base_url
        self.session = requests.Session()
    
    def chat(self, message, chat_id=None):
        response = self.session.post(
            f"{self.base_url}/api/chat",
            json={
                "message": message,
                "chat_id": chat_id or f"python-client-{int(time.time())}"
            }
        )
        return response.json()
    
    def get_system_status(self):
        response = self.session.get(f"{self.base_url}/api/system/status")
        return response.json()
    
    def execute_command(self, command, mode="supervised"):
        response = self.session.post(
            f"{self.base_url}/api/execute",
            json={
                "command": command,
                "mode": mode
            }
        )
        return response.json()

# Usage
client = TuxPilotClient()
result = client.chat("Show me disk usage")
print(result["response"])
```

### JavaScript/Node.js SDK Example

```javascript
class TuxPilotClient {
    constructor(baseUrl = 'http://127.0.0.1:8080') {
        this.baseUrl = baseUrl;
    }
    
    async chat(message, chatId = null) {
        const response = await fetch(`${this.baseUrl}/api/chat`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                message,
                chat_id: chatId || `js-client-${Date.now()}`
            })
        });
        return response.json();
    }
    
    async getSystemStatus() {
        const response = await fetch(`${this.baseUrl}/api/system/status`);
        return response.json();
    }
    
    async executeCommand(command, mode = 'supervised') {
        const response = await fetch(`${this.baseUrl}/api/execute`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                command,
                mode
            })
        });
        return response.json();
    }
}

// Usage
const client = new TuxPilotClient();
client.chat('Check system performance').then(result => {
    console.log(result.response);
});
```
