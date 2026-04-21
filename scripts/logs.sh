#!/bin/bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_info "Showing logs (Ctrl+C to exit)..."
docker compose logs -f
