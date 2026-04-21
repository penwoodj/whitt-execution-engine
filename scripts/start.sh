#!/bin/bash
set -e

GREEN='\033[0;32m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

# Detect GPU
GPU_TYPE=$(./scripts/detect-gpu.sh 2>/dev/null || echo "cpu")

log_info "Starting LLM server with $GPU_TYPE GPU..."

# Choose compose files based on GPU
case "$GPU_TYPE" in
    nvidia)
        docker compose -f docker-compose.yml -f docker-compose.nvidia.yml up -d
        ;;
    amd)
        docker compose -f docker-compose.yml -f docker-compose.amd.yml up -d
        ;;
    *)
        docker compose up -d
        ;;
esac

log_info "Server starting. Check health with: docker compose ps"
log_info "View logs: docker compose logs -f"
