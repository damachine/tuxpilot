# 🐧 TuxPilot + Ollama Setup Guide

**Local AI without cloud dependency and API costs!**

## 🚀 **Quick Setup (Automatic)**

```bash
# Install and configure everything automatically
./setup-ollama.sh
```

The script does everything for you:
- ✅ Install Ollama
- ✅ Start service
- ✅ Download model
- ✅ Configure TuxPilot
- ✅ Test integration

## 🔧 **Manual Setup**

### **1. Install Ollama**

```bash
# Official installation script
curl -fsSL https://ollama.ai/install.sh | sh

# Or on Arch Linux
sudo pacman -S ollama

# Or on Ubuntu/Debian
curl -fsSL https://ollama.ai/install.sh | sh
```

### **2. Start Ollama Service**

```bash
# Start Ollama server
ollama serve &

# Or as systemd service
sudo systemctl enable --now ollama
```

### **3. Download AI Model**

```bash
# Download recommended model (4GB)
ollama pull llama3.1:8b

# Or smaller model for limited resources (2GB)
ollama pull llama3.1:7b

# Or larger model for better performance (14GB)
ollama pull llama3.1:70b
```

### **4. Test Ollama**

```bash
# Test if Ollama is working
curl http://localhost:11434/api/tags

# Interactive test
ollama run llama3.1:8b
```

### **5. Configure TuxPilot**

TuxPilot automatically detects Ollama! No configuration needed.

```bash
# TuxPilot will automatically use Ollama
./target/release/tuxpilot

# Force Ollama usage
./target/release/tuxpilot --local
```

## 🎯 **Model Recommendations**

### **For Most Users: llama3.1:8b**
- **Size**: ~4GB
- **Performance**: Excellent for Linux tasks
- **Speed**: Fast responses
- **Memory**: 8GB RAM recommended

### **For Limited Resources: llama3.1:7b**
- **Size**: ~2GB
- **Performance**: Good for basic tasks
- **Speed**: Very fast
- **Memory**: 4GB RAM minimum

### **For Power Users: llama3.1:70b**
- **Size**: ~14GB
- **Performance**: Best quality responses
- **Speed**: Slower but more accurate
- **Memory**: 16GB+ RAM required

## 🔧 **Advanced Configuration**

### **Custom Ollama Configuration**

```bash
# ~/.config/tuxpilot/config.toml
[ai]
provider = "Ollama"

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"
timeout = 30
max_tokens = 2048
temperature = 0.7
```

### **Multiple Models**

```bash
# Download multiple models for different tasks
ollama pull llama3.1:8b      # General purpose
ollama pull codellama:13b     # Code generation
ollama pull mistral:7b        # Fast responses

# Switch models in TuxPilot config
model = "codellama:13b"  # For development tasks
```

### **Performance Tuning**

```bash
# Increase Ollama memory limit
export OLLAMA_MAX_LOADED_MODELS=2
export OLLAMA_NUM_PARALLEL=4

# GPU acceleration (if available)
export OLLAMA_GPU_LAYERS=35
```

## 🚀 **Usage Examples**

### **Basic Usage**

```bash
# Start TuxPilot with Ollama
./target/release/tuxpilot

# Ask questions
tuxpilot> How do I install Docker?
🤖 TuxPilot: To install Docker on your system...

# Execute commands with AI assistance
tuxpilot execute "install nginx and start it"
```

### **Advanced Features**

```bash
# Autonomous mode with local AI
tuxpilot chat --execute-mode autonomous

# System diagnosis with local AI
tuxpilot diagnose --auto --fix

# Package management with AI help
tuxpilot package search "web server"
```

## 🛠️ **Troubleshooting**

### **Ollama Not Starting**

```bash
# Check if Ollama is running
ps aux | grep ollama

# Check logs
journalctl -u ollama -f

# Restart service
sudo systemctl restart ollama
```

### **Model Download Issues**

```bash
# Check available space
df -h

# Clear Ollama cache
ollama rm llama3.1:8b
ollama pull llama3.1:8b

# Check network connection
curl -I https://ollama.ai
```

### **TuxPilot Not Detecting Ollama**

```bash
# Check Ollama API
curl http://localhost:11434/api/tags

# Force Ollama in TuxPilot
export TUXPILOT_AI_PROVIDER=ollama

# Check TuxPilot config
tuxpilot config --show
```

### **Performance Issues**

```bash
# Check system resources
htop

# Reduce model size
ollama pull llama3.1:7b

# Adjust TuxPilot timeout
# In config.toml:
timeout = 60  # Increase timeout
```

## 🔒 **Security & Privacy**

### **Benefits of Local AI**

✅ **Complete Privacy** - All data stays on your machine
✅ **No Internet Required** - Works completely offline
✅ **No API Costs** - Free forever
✅ **No Rate Limits** - Use as much as you want
✅ **Custom Models** - Train your own models
✅ **Enterprise Ready** - Full control over AI

### **Security Considerations**

- Ollama runs locally on port 11434
- No data sent to external servers
- Models stored in `~/.ollama/models/`
- All processing happens locally

## 📊 **Performance Comparison**

| Model | Size | Speed | Quality | RAM Required |
|-------|------|-------|---------|--------------|
| llama3.1:7b | 2GB | ⚡⚡⚡ | ⭐⭐⭐ | 4GB |
| llama3.1:8b | 4GB | ⚡⚡ | ⭐⭐⭐⭐ | 8GB |
| llama3.1:70b | 14GB | ⚡ | ⭐⭐⭐⭐⭐ | 16GB+ |

## 🎉 **Success!**

If everything is working, you should see:

```bash
$ tuxpilot config --show
[2024-01-15T10:30:00Z INFO tuxpilot::config] Ollama detected, switching to local AI provider

AI Provider: Ollama
Model: llama3.1:8b
Status: ✅ Connected
```

**You now have a completely local, private AI assistant for Linux! 🚀**

## 🆘 **Need Help?**

- 📖 [TuxPilot Documentation](README.md)
- 🐛 [Report Issues](https://github.com/yourusername/tuxpilot/issues)
- 💬 [Community Discussions](https://github.com/yourusername/tuxpilot/discussions)
- 🌐 [Ollama Documentation](https://ollama.ai/docs)

---

**TuxPilot + Ollama: The perfect combination for private, powerful Linux AI assistance! 🐧🤖**
