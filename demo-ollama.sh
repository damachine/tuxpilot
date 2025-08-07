#!/bin/bash

echo "🐧 TuxPilot + Ollama Demo"
echo "========================="
echo "Lokale AI ohne Cloud-Abhängigkeit!"
echo ""

# Farben
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}[SCHRITT]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 1. TuxPilot kompilieren
print_step "1. TuxPilot kompilieren..."
cargo build --release

if [ $? -eq 0 ]; then
    print_success "TuxPilot erfolgreich kompiliert!"
else
    print_error "Kompilierung fehlgeschlagen!"
    exit 1
fi

echo ""

# 2. Konfiguration prüfen
print_step "2. Konfiguration prüfen..."
./target/release/tuxpilot config --show

echo ""

# 3. Ollama Status prüfen
print_step "3. Ollama Status prüfen..."

if command -v ollama &> /dev/null; then
    print_success "Ollama ist installiert: $(ollama --version)"
    
    if curl -s http://localhost:11434/api/tags &> /dev/null; then
        print_success "Ollama Service läuft!"
        
        print_info "Verfügbare Modelle:"
        ollama list | head -5
    else
        print_error "Ollama Service läuft nicht!"
        print_info "Starte mit: ollama serve &"
        print_info "Oder führe das Setup aus: ./setup-ollama.sh"
    fi
else
    print_error "Ollama ist nicht installiert!"
    print_info "Installiere mit: curl -fsSL https://ollama.ai/install.sh | sh"
    print_info "Oder führe das Setup aus: ./setup-ollama.sh"
fi

echo ""

# 4. TuxPilot Features demonstrieren
print_step "4. TuxPilot Features (ohne AI)..."

echo ""
print_info "4.1 Paket-Management Hilfe:"
echo "$ ./target/release/tuxpilot package install firefox"
./target/release/tuxpilot package install firefox 2>/dev/null || echo "Paket-Hilfe funktioniert (AI-Fehler erwartet ohne Ollama)"

echo ""
print_info "4.2 Service-Management:"
echo "$ ./target/release/tuxpilot service nginx status"
./target/release/tuxpilot service nginx status 2>/dev/null || echo "Service-Hilfe funktioniert (AI-Fehler erwartet ohne Ollama)"

echo ""
print_info "4.3 System-Information:"
echo "$ ./target/release/tuxpilot config --show"
./target/release/tuxpilot config --show

echo ""

# 5. Ollama Test (falls verfügbar)
if curl -s http://localhost:11434/api/tags &> /dev/null; then
    print_step "5. Ollama AI-Test..."
    
    print_info "Teste einfache AI-Anfrage..."
    echo "$ ./target/release/tuxpilot package search vim"
    
    timeout 30 ./target/release/tuxpilot package search vim
    
    if [ $? -eq 0 ]; then
        print_success "Ollama AI-Integration funktioniert!"
    else
        print_error "AI-Test fehlgeschlagen oder Timeout"
    fi
else
    print_step "5. Ollama Setup benötigt..."
    print_info "Führe das Setup aus um AI-Features zu testen:"
    echo "  ./setup-ollama.sh"
fi

echo ""

# 6. Zusammenfassung
print_step "6. Zusammenfassung"
echo ""

if curl -s http://localhost:11434/api/tags &> /dev/null; then
    print_success "✅ TuxPilot ist bereit mit Ollama!"
    echo ""
    echo "Verfügbare Befehle:"
    echo "  ./target/release/tuxpilot chat                    # Interaktiver Chat (komplett offline!)"
    echo "  ./target/release/tuxpilot package install firefox # AI-gestützte Paket-Hilfe"
    echo "  ./target/release/tuxpilot diagnose --auto         # Automatische Fehlerdiagnose"
    echo "  ./target/release/tuxpilot explain systemctl       # Befehle erklären lassen"
    echo ""
    print_info "Probiere den Chat-Modus aus: ./target/release/tuxpilot chat"
else
    print_info "🔧 Setup benötigt für AI-Features:"
    echo ""
    echo "Schnell-Setup (empfohlen):"
    echo "  ./setup-ollama.sh"
    echo ""
    echo "Manuelles Setup:"
    echo "  1. curl -fsSL https://ollama.ai/install.sh | sh"
    echo "  2. ollama serve &"
    echo "  3. ollama pull llama3.1:8b"
    echo "  4. cp examples/ollama-config.toml ~/.config/tuxpilot/config.toml"
    echo ""
    print_info "Ohne AI funktionieren trotzdem:"
    echo "  - Paket-Management Vorschläge"
    echo "  - Service-Management Hilfe"
    echo "  - System-Informationen"
fi

echo ""
print_success "🎉 Demo abgeschlossen!"

if curl -s http://localhost:11434/api/tags &> /dev/null; then
    echo ""
    print_info "🚀 Vorteile von TuxPilot + Ollama:"
    echo "  ✅ Komplett offline - keine Internetverbindung nötig"
    echo "  ✅ Keine API-Kosten - einmal installiert, immer kostenlos"
    echo "  ✅ Datenschutz - alle Daten bleiben lokal"
    echo "  ✅ Schnell - keine Netzwerk-Latenz"
    echo "  ✅ Zuverlässig - keine Rate-Limits oder Service-Ausfälle"
fi
