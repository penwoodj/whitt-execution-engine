#!/bin/bash
set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo "=== LLM Server Status ==="
echo ""

# Container status
log_info "Container Status:"
docker compose ps

echo ""

# Health check
HEALTH=$(docker compose ps --format "{{.Health}}" | head -n1)
if [ "$HEALTH" = "healthy" ]; then
    log_info "Health: $HEALTH"
else
    log_warn "Health: $HEALTH"
fi

echo ""

# Port binding
log_info "Port Binding:"
docker compose ps --format "  Port: {{.Ports}}"

echo ""

# GPU usage (if available)
if command -v nvidia-smi &> /dev/null; then
    log_info "NVIDIA GPU Usage:"
    nvidia-smi --query-gpu=index,utilization.gpu,memory.used,memory.total --format=csv,noheader
    echo ""
elif command -v rocminfo &> /dev/null; then
    log_info "AMD GPU Info:"
    rocminfo | grep "Compute Unit:" | head -n1
    echo ""
fi

# Model
log_info "Current Model:"
if [ -f "config.yml" ]; then
    if command -v yq &> /dev/null; then
        yq eval '.model.path' config.yml
    else
        grep -A 2 "^model:" config.yml | grep "path:" | awk '{print $2}'
    fi
else
    echo "  No config.yml found"
fi
