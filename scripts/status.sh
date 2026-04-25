#!/bin/bash
set -euo pipefail

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
if ! docker compose ps &> /dev/null; then
    log_warn "Docker compose not running"
    exit 1
fi
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
NETWORK_MODE=$(docker inspect whitt-llama-server --format='{{.HostConfig.NetworkMode}}' 2>/dev/null || echo "unknown")
if [ "$NETWORK_MODE" = "host" ]; then
    echo "  Network: host (server directly on http://localhost:8080)"
else
    PORT=$(docker port whitt-llama-server 2>/dev/null | head -n1)
    echo "  Port: ${PORT:-not mapped}"
fi

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
MODEL_INFO=$(curl -sf http://localhost:8080/v1/models 2>/dev/null | jq -r '.data[] | select(.status.value == "loaded") | "\(.id) (\(.status.args[.status.args | index("--model") + 1] | split("/") | last))"' 2>/dev/null)
if [ -n "$MODEL_INFO" ]; then
    echo "  $MODEL_INFO"
else
    echo "  No model loaded"
fi
