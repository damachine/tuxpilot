# TuxPilot Web Interface Guide

The TuxPilot web interface provides a modern, ChatGPT-inspired experience for managing your Linux system through an intuitive browser-based interface.

## 🚀 Getting Started

### Starting the Web Server

```bash
# Start the web server (default port 8080)
tuxpilot web

# Start on custom port
tuxpilot web --port 8082

# Bind to all interfaces (for remote access)
tuxpilot web --bind 0.0.0.0
```

### Accessing the Interface

Open your web browser and navigate to:
- **Local access**: http://127.0.0.1:8080
- **Remote access**: http://your-server-ip:8080

## 🎨 Interface Overview

### Design Philosophy

The TuxPilot web interface follows a **ChatGPT-inspired design** with:
- **Clean, modern aesthetic** with professional gray color scheme
- **Dark theme by default** with light theme toggle
- **Responsive design** that works on desktop, tablet, and mobile
- **Accessibility-first approach** with WCAG AA compliance

### Main Components

1. **Header Bar**
   - TuxPilot logo and branding
   - Navigation menu
   - Theme toggle (dark/light)
   - User session information

2. **Sidebar Navigation**
   - Chat interface
   - Dashboard
   - Settings
   - Quick actions

3. **Main Content Area**
   - Dynamic content based on selected page
   - Real-time updates
   - Interactive components

## 💬 Chat Interface

### Features

- **Real-time AI communication** with WebSocket support
- **Message history** with persistent conversations
- **Example prompts** to get started quickly
- **Typing indicators** and loading states
- **Code syntax highlighting** in responses
- **Copy/paste functionality** for commands

### Using the Chat

1. **Starting a Conversation**
   ```
   Type your question or request in the input field
   Press Enter or click Send
   ```

2. **Example Interactions**
   ```
   "Check my system performance"
   "Install Docker and configure it"
   "Show me disk usage"
   "Optimize my system for gaming"
   ```

3. **Chat Features**
   - **Auto-complete**: Suggestions as you type
   - **Command history**: Use ↑/↓ arrows to navigate
   - **Multi-line input**: Shift+Enter for new lines
   - **Clear conversation**: Reset chat history

### Message Types

- **User messages**: Your input and requests
- **AI responses**: TuxPilot's analysis and suggestions
- **System messages**: Status updates and notifications
- **Command outputs**: Results from executed commands

## 📊 Dashboard

### System Overview

The dashboard provides real-time monitoring of your system:

1. **System Information**
   - Hostname and uptime
   - Operating system details
   - Kernel version
   - Hardware specifications

2. **Performance Metrics**
   - **CPU Usage**: Real-time CPU utilization with visual graphs
   - **Memory Usage**: RAM usage with available/used breakdown
   - **Disk Usage**: Storage utilization across all mounted filesystems
   - **Network Activity**: Upload/download statistics

3. **System Alerts**
   - **Warning alerts**: Performance issues, high resource usage
   - **Error alerts**: System problems requiring attention
   - **Info alerts**: General system notifications

4. **Recent Activities**
   - Command execution history
   - System changes and modifications
   - User actions and timestamps
   - Success/failure status indicators

### Dashboard Features

- **Auto-refresh**: Real-time updates every 5 seconds
- **Interactive charts**: Click to drill down into metrics
- **Alert management**: Dismiss or acknowledge alerts
- **Export functionality**: Download reports and logs

## ⚙️ Settings & Configuration

### Configuration Management

The settings page provides comprehensive system configuration:

1. **AI Configuration**
   - **Provider selection**: Ollama, OpenAI, Anthropic
   - **Model selection**: Dynamic list based on provider
   - **Temperature**: Control AI response creativity (0.0-2.0)
   - **Max tokens**: Limit response length

2. **Execution Settings**
   - **Mode**: Supervised, Semi-Automatic, Autonomous, Read-Only
   - **Confirmation**: Require user approval for commands
   - **Timeout**: Command execution timeout (1-300 seconds)

3. **Web Server Configuration**
   - **Port**: Web server listening port (1-65535)
   - **Bind address**: Interface binding (localhost/all interfaces)
   - **SSL**: Enable HTTPS/SSL encryption

### Configuration Features

- **Real-time validation**: Immediate feedback on invalid settings
- **Auto-save**: Automatic saving of configuration changes
- **Backup/restore**: Configuration backup and restoration
- **Reset to defaults**: Restore factory settings

## 🔒 Security & Authentication

### Security Features

1. **Session Management**
   - Secure session tokens
   - Automatic session expiration
   - Session activity tracking

2. **Access Control**
   - IP-based access restrictions
   - User authentication (when enabled)
   - Permission-based access control

3. **Data Protection**
   - HTTPS encryption support
   - Secure cookie handling
   - XSS and CSRF protection

### Best Practices

- **Use HTTPS** in production environments
- **Restrict access** to trusted networks
- **Regular updates** of TuxPilot
- **Monitor access logs** for suspicious activity

## 🎨 Themes & Customization

### Theme System

1. **Dark Theme (Default)**
   - Professional gray color scheme
   - Easy on the eyes for extended use
   - ChatGPT-inspired design elements

2. **Light Theme**
   - Clean, bright interface
   - High contrast for accessibility
   - Traditional web application feel

### Theme Toggle

- **Automatic detection**: Respects system preference
- **Manual override**: Toggle button in header
- **Persistent choice**: Saved in browser localStorage
- **Smooth transitions**: Animated theme switching

## 📱 Mobile & Responsive Design

### Mobile Features

- **Touch-optimized interface**: Large touch targets
- **Responsive layout**: Adapts to screen size
- **Mobile navigation**: Collapsible sidebar
- **Gesture support**: Swipe navigation

### Tablet Support

- **Optimized for tablets**: Perfect for iPad and Android tablets
- **Landscape/portrait**: Adapts to orientation changes
- **Touch interactions**: Native touch support

## 🔧 Advanced Features

### WebSocket Communication

- **Real-time updates**: Live system monitoring
- **Instant messaging**: No page refresh needed
- **Connection management**: Automatic reconnection
- **Error handling**: Graceful degradation

### API Integration

- **RESTful API**: Full API access through web interface
- **Real-time data**: Live system metrics
- **Command execution**: Execute commands through web UI
- **Configuration management**: Modify settings via API

## 🐛 Troubleshooting

### Common Issues

1. **Cannot access web interface**
   ```bash
   # Check if server is running
   ps aux | grep tuxpilot
   
   # Check port availability
   netstat -tlnp | grep 8080
   
   # Check firewall settings
   sudo ufw status
   ```

2. **Web UI not loading**
   - Clear browser cache
   - Check browser console for errors
   - Verify JavaScript is enabled
   - Try different browser

3. **Theme not switching**
   - Clear localStorage
   - Refresh the page
   - Check browser compatibility

### Performance Optimization

- **Browser cache**: Enable caching for better performance
- **Network optimization**: Use local network for best speed
- **Resource monitoring**: Monitor server resources

## 🔄 Updates & Maintenance

### Updating the Web Interface

```bash
# Update TuxPilot (includes web interface)
git pull origin main
./build.sh

# Rebuild web interface only
./build-web-ui.sh
```

### Maintenance Tasks

- **Regular updates**: Keep TuxPilot updated
- **Log rotation**: Manage web server logs
- **Performance monitoring**: Monitor resource usage
- **Security updates**: Apply security patches promptly
