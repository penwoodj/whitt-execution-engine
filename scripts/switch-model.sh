#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Parse arguments
MODEL_CONFIG="$1"

if [ -z "$MODEL_CONFIG" ]; then
    log_error "Usage: $0 <model-config-path>"
    log_error "Example: $0 configs/models/llama-3.2-1b.yml"
    exit 1
fi

if [ ! -f "$MODEL_CONFIG" ]; then
    log_error "Model config not found: $MODEL_CONFIG"
    exit 1
fi

log_info "Switching to model: $MODEL_CONFIG"

# Backup current config
if [ -f "config.yml" ]; then
    log_info "Backing up current config to config.yml.backup"
    cp config.yml config.yml.backup
fi

# Copy new config
log_info "Copying new config to config.yml"
cp "$MODEL_CONFIG" config.yml

# Verify checksum if specified in new config
if command -v yq &> /dev/null; then
    model_path=$(yq eval '.model.path' config.yml 2>/dev/null)
    expected_sha256=$(yq eval '.model.huggingface.sha256 // ""' "$MODEL_CONFIG" 2>/dev/null || echo "")

    if [ -n "$expected_sha256" ] && [ -f "$model_path" ]; then
        log_info "Verifying model checksum..."
        actual_sha256=$(sha256sum "$model_path" | awk '{print $1}')
        if [ "$actual_sha256" != "$expected_sha256" ]; then
            log_error "Checksum verification failed!"
            log_error "Expected: $expected_sha256"
            log_error "Actual: $actual_sha256"
            log_error "Model file may be corrupted. Re-download required."
            exit 1
        else
            log_info "Checksum verified successfully"
        fi
    fi
fi

# Restart container
log_info "Restarting container..."
docker compose restart

# Wait for container to be healthy
log_info "Waiting for container to be healthy..."
MAX_WAIT=60
WAIT_TIME=0

while [ $WAIT_TIME -lt $MAX_WAIT ]; do
    STATUS=$(docker compose ps --format "{{.Health}}" | head -n1)
    if [ "$STATUS" = "healthy" ]; then
        log_info "Container is healthy"
        break
    fi
    sleep 2
    WAIT_TIME=$((WAIT_TIME + 2))
done

if [ $WAIT_TIME -ge $MAX_WAIT ]; then
    log_warn "Container did not become healthy within $MAX_WAIT seconds"
    log_warn "Check logs: docker compose logs"
fi

# Show current model
log_info "Current model:"
if command -v yq &> /dev/null; then
    yq eval '.model.path' config.yml
else
    grep -A 2 "^model:" config.yml | grep "path:" | awk '{print $2}'
fi

log_info "Model switch complete"
