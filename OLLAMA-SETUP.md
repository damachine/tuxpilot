# 🐧 TuxPilot + Ollama Setup Guide

**Lokale AI ohne Cloud-Abhängigkeit und API-Kosten!**

## 🚀 **Schnell-Setup (Automatisch)**

```bash
# Alles automatisch installieren und konfigurieren
./setup-ollama.sh
```

Das Script macht alles für dich:
- ✅ Ollama installieren
- ✅ Service starten  
- ✅ Modell herunterladen
- ✅ TuxPilot konfigurieren
- ✅ Kompilieren und testen

## 🔧 **Manuelles Setup**

### **1. Ollama installieren**

```bash
# Ollama herunterladen und installieren
curl -fsSL https://ollama.ai/install.sh | sh

# Prüfen ob installiert
ollama --version
```

### **2. Ollama Service starten**

```bash
# Service im Hintergrund starten
ollama serve &

# Oder als systemd Service (empfohlen)
sudo systemctl enable ollama
sudo systemctl start ollama
```

### **3. AI-Modell herunterladen**

```bash
# Empfohlenes Modell (ca. 4.7 GB)
ollama pull llama3.1:8b

# Weitere Optionen:
ollama pull mistral:7b        # Schneller, kleiner
ollama pull codellama:7b      # Speziell für Code
ollama pull llama3.1:70b      # Sehr gut, braucht viel RAM (40+ GB)

# Verfügbare Modelle anzeigen
ollama list
```

### **4. TuxPilot konfigurieren**

```bash
# Konfigurationsverzeichnis erstellen
mkdir -p ~/.config/tuxpilot

# Ollama-Konfiguration kopieren
cp examples/ollama-config.toml ~/.config/tuxpilot/config.toml

# Oder manuell bearbeiten
nano ~/.config/tuxpilot/config.toml
```

**Beispiel-Konfiguration:**
```toml
[ai]
provider = "Ollama"

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"
temperature = 0.7
context_size = 4096
timeout_seconds = 30
```

### **5. TuxPilot kompilieren**

```bash
cargo build --release
```

## 🎯 **Verwendung**

### **Grundlegende Befehle:**

```bash
# Interaktiver Chat-Modus (komplett offline!)
./target/release/tuxpilot chat

# Paket-Management Hilfe
./target/release/tuxpilot package install firefox

# Automatische Fehlerdiagnose
./target/release/tuxpilot diagnose --auto

# System-Monitoring mit AI-Analyse
./target/release/tuxpilot monitor

# Befehl erklären lassen
./target/release/tuxpilot explain systemctl
```

### **Chat-Beispiele:**

```
tuxpilot> Mein System ist langsam, was kann ich tun?
🤖 TuxPilot: Ich helfe dir bei der Performance-Analyse...

tuxpilot> Wie installiere ich Docker auf Arch Linux?
🤖 TuxPilot: Für Docker auf Arch Linux verwendest du...

tuxpilot> nginx startet nicht, was ist das Problem?
🤖 TuxPilot: Lass mich den nginx Service analysieren...
```

## ⚙️ **Konfiguration**

### **Modell wechseln:**

```bash
# Anderes Modell herunterladen
ollama pull mistral:7b

# In config.toml ändern
[ai.ollama]
model = "mistral:7b"
```

### **Performance optimieren:**

```toml
[ai.ollama]
# Für schnellere Antworten
temperature = 0.3
context_size = 2048
timeout_seconds = 15

# Für bessere Qualität
temperature = 0.8
context_size = 8192
timeout_seconds = 60
```

### **Remote Ollama Server:**

```toml
[ai.ollama]
base_url = "http://192.168.1.100:11434"  # Anderer Server
model = "llama3.1:8b"
```

## 🔍 **Troubleshooting**

### **"Ollama API error" Fehler:**

```bash
# Prüfen ob Ollama läuft
curl http://localhost:11434/api/tags

# Service neu starten
pkill ollama
ollama serve &

# Logs prüfen
journalctl -u ollama -f
```

### **"Model not found" Fehler:**

```bash
# Verfügbare Modelle anzeigen
ollama list

# Modell herunterladen
ollama pull llama3.1:8b

# In config.toml korrigieren
model = "llama3.1:8b"  # Exakter Name aus 'ollama list'
```

### **Langsame Antworten:**

```bash
# Kleineres Modell verwenden
ollama pull mistral:7b

# Oder Timeout erhöhen
timeout_seconds = 120
```

### **Speicher-Probleme:**

```bash
# Kleineres Modell verwenden
ollama pull llama3.1:8b    # statt 70b

# Oder Context reduzieren
context_size = 2048        # statt 4096
```

## 📊 **Modell-Empfehlungen**

| Modell | Größe | RAM | Geschwindigkeit | Qualität | Verwendung |
|--------|-------|-----|----------------|----------|------------|
| `mistral:7b` | 4.1 GB | 8 GB | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | Schnelle Antworten |
| `llama3.1:8b` | 4.7 GB | 8 GB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | **Empfohlen** |
| `codellama:7b` | 3.8 GB | 8 GB | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Code-Probleme |
| `llama3.1:70b` | 40 GB | 64 GB | ⭐⭐ | ⭐⭐⭐⭐⭐ | Beste Qualität |

## 🎉 **Vorteile von Ollama + TuxPilot**

✅ **Komplett offline** - keine Internetverbindung nötig
✅ **Keine API-Kosten** - einmal installiert, immer kostenlos  
✅ **Datenschutz** - alle Daten bleiben lokal
✅ **Schnell** - keine Netzwerk-Latenz
✅ **Anpassbar** - verschiedene Modelle für verschiedene Zwecke
✅ **Zuverlässig** - keine Rate-Limits oder Service-Ausfälle

## 🔄 **Systemd Service (Optional)**

Für automatischen Start bei Boot:

```bash
# Service-Datei erstellen
sudo tee /etc/systemd/system/ollama.service << 'EOF'
[Unit]
Description=Ollama Service
After=network-online.target

[Service]
ExecStart=/usr/local/bin/ollama serve
User=ollama
Group=ollama
Restart=always
RestartSec=3

[Install]
WantedBy=default.target
EOF

# User erstellen
sudo useradd -r -s /bin/false -m -d /usr/share/ollama ollama

# Service aktivieren
sudo systemctl enable ollama
sudo systemctl start ollama
```

**Jetzt hast du eine komplett lokale AI-Lösung! 🚀**
