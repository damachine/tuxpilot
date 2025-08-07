#!/bin/bash

echo "🐧 TuxPilot Ollama Setup"
echo "======================="
echo ""

# Farben für bessere Ausgabe
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 1. Ollama Installation prüfen
print_status "Prüfe Ollama Installation..."

if command -v ollama &> /dev/null; then
    print_success "Ollama ist bereits installiert: $(ollama --version)"
else
    print_warning "Ollama ist nicht installiert. Installiere Ollama..."
    
    # Ollama installieren
    curl -fsSL https://ollama.ai/install.sh | sh
    
    if command -v ollama &> /dev/null; then
        print_success "Ollama erfolgreich installiert!"
    else
        print_error "Ollama Installation fehlgeschlagen!"
        exit 1
    fi
fi

echo ""

# 2. Ollama Service starten
print_status "Starte Ollama Service..."

# Prüfen ob Ollama bereits läuft
if curl -s http://localhost:11434/api/tags &> /dev/null; then
    print_success "Ollama Service läuft bereits!"
else
    print_status "Starte Ollama Service im Hintergrund..."
    
    # Ollama im Hintergrund starten
    nohup ollama serve > /tmp/ollama.log 2>&1 &
    
    # Warten bis Service verfügbar ist
    echo -n "Warte auf Ollama Service"
    for i in {1..10}; do
        if curl -s http://localhost:11434/api/tags &> /dev/null; then
            echo ""
            print_success "Ollama Service ist bereit!"
            break
        fi
        echo -n "."
        sleep 1
    done
    
    if ! curl -s http://localhost:11434/api/tags &> /dev/null; then
        print_error "Ollama Service konnte nicht gestartet werden!"
        print_error "Prüfe die Logs: tail /tmp/ollama.log"
        exit 1
    fi
fi

echo ""

# 3. Modell herunterladen
print_status "Prüfe verfügbare Modelle..."

# Prüfen welche Modelle bereits installiert sind
INSTALLED_MODELS=$(ollama list 2>/dev/null | grep -v "NAME" | awk '{print $1}' | head -5)

if [ -n "$INSTALLED_MODELS" ]; then
    print_success "Bereits installierte Modelle:"
    echo "$INSTALLED_MODELS" | while read model; do
        echo "  - $model"
    done
else
    print_warning "Keine Modelle installiert. Lade empfohlenes Modell herunter..."
    
    print_status "Lade llama3.1:8b herunter (das kann einige Minuten dauern)..."
    ollama pull llama3.1:8b
    
    if [ $? -eq 0 ]; then
        print_success "Modell llama3.1:8b erfolgreich heruntergeladen!"
    else
        print_error "Fehler beim Herunterladen des Modells!"
        exit 1
    fi
fi

echo ""

# 4. TuxPilot Konfiguration
print_status "Konfiguriere TuxPilot für Ollama..."

CONFIG_DIR="$HOME/.config/tuxpilot"
CONFIG_FILE="$CONFIG_DIR/config.toml"

# Konfigurationsverzeichnis erstellen
mkdir -p "$CONFIG_DIR"

# Ollama-Konfiguration kopieren
if [ -f "examples/ollama-config.toml" ]; then
    cp examples/ollama-config.toml "$CONFIG_FILE"
    print_success "Ollama-Konfiguration nach $CONFIG_FILE kopiert!"
else
    print_warning "Erstelle Standard-Konfiguration..."
    
    cat > "$CONFIG_FILE" << 'EOF'
[ai]
provider = "Ollama"

[ai.ollama]
base_url = "http://localhost:11434"
model = "llama3.1:8b"
temperature = 0.7
context_size = 4096
timeout_seconds = 30

[system]
package_manager = "Pacman"
service_manager = "Systemd"
log_paths = [
    "/var/log/syslog",
    "/var/log/messages",
    "/var/log/kern.log",
    "/var/log/auth.log",
    "/var/log/pacman.log"
]

[ui]
theme = "default"
show_tips = true
auto_suggest = true
EOF
    
    print_success "Standard-Konfiguration erstellt!"
fi

echo ""

# 5. TuxPilot kompilieren
print_status "Kompiliere TuxPilot..."

cargo build --release

if [ $? -eq 0 ]; then
    print_success "TuxPilot erfolgreich kompiliert!"
else
    print_error "TuxPilot Kompilierung fehlgeschlagen!"
    exit 1
fi

echo ""

# 6. Test durchführen
print_status "Teste TuxPilot mit Ollama..."

echo "Teste einfache Anfrage..."
timeout 30 ./target/release/tuxpilot package install firefox

if [ $? -eq 0 ]; then
    print_success "Test erfolgreich!"
else
    print_warning "Test mit Timeout oder Fehler beendet (normal bei erstem Start)"
fi

echo ""

# 7. Zusammenfassung
print_success "🎉 Ollama Setup abgeschlossen!"
echo ""
echo "Verfügbare Befehle:"
echo "  ./target/release/tuxpilot chat                    # Interaktiver Chat-Modus"
echo "  ./target/release/tuxpilot package install firefox # Paket-Hilfe"
echo "  ./target/release/tuxpilot diagnose --auto         # Automatische Diagnose"
echo "  ./target/release/tuxpilot monitor                 # System-Monitoring"
echo ""
echo "Konfiguration: $CONFIG_FILE"
echo "Ollama Modelle: ollama list"
echo "Ollama Status: curl http://localhost:11434/api/tags"
echo ""
print_status "Viel Spaß mit TuxPilot + Ollama! 🚀"
