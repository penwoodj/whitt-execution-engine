#!/bin/bash
set -euo pipefail

SERVER_URL="${LLAMA_SERVER_URL:-http://localhost:8080}"

ARG1="${1:-}"
ARG2="${2:-}"
KNOWN_ACTIONS="list load unload swap status"

usage() {
    echo "Usage: $0 <model-id> [action]"
    echo ""
    echo "Actions:"
    echo "  swap   Unload current model, load new model (default)"
    echo "  load   Load model without unloading others"
    echo "  unload Unload a specific model"
    echo "  list   List all available models and their status"
    echo "  status Show status of a specific model"
    echo ""
    echo "Examples:"
    echo "  $0 Qwen2.5-0.5B-Instruct-Q4_K_M"
    echo "  $0 Qwen2.5-0.5B-Instruct-Q4_K_M load"
    echo "  $0 Qwen2.5-0.5B-Instruct-Q4_K_M unload"
    echo "  $0 list"
    echo "  $0 Qwen2.5-0.5B-Instruct-Q4_K_M status"
    echo ""
    echo "Env vars:"
    echo "  LLAMA_SERVER_URL  Server URL (default: http://localhost:8080)"
    exit 1
}

if [ -z "$ARG1" ] || [ "$ARG1" = "--help" ] || [ "$ARG1" = "-h" ]; then
    usage
fi

if echo "$KNOWN_ACTIONS" | grep -qw -- "$ARG1"; then
    if [ -n "$ARG2" ]; then
        MODEL_ID="$ARG2"
        ACTION="$ARG1"
    else
        MODEL_ID=""
        ACTION="$ARG1"
    fi
else
    MODEL_ID="$ARG1"
    ACTION="${ARG2:-swap}"
fi

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

check_server() {
    if ! curl -sf "${SERVER_URL}/health" > /dev/null 2>&1; then
        log_error "Server not reachable at ${SERVER_URL}"
        exit 1
    fi
}

wait_for_status() {
    local model_id="$1"
    local expected="$2"
    local timeout="${3:-120}"
    local elapsed=0
    local interval=1

    while [ $elapsed -lt $timeout ]; do
        local current
        current=$(curl -sf "${SERVER_URL}/v1/models" 2>/dev/null \
            | jq -r --arg id "$model_id" '.data[] | select(.id == $id) | .status.value // "unknown"' 2>/dev/null || echo "unknown")

        if [ "$current" = "$expected" ]; then
            return 0
        fi

        sleep $interval
        elapsed=$((elapsed + interval))
    done

    log_error "Model $model_id did not reach status '$expected' within ${timeout}s (current: $current)"
    return 1
}

do_list() {
    check_server
    local models
    models=$(curl -sf "${SERVER_URL}/v1/models" 2>/dev/null)

    if [ -z "$models" ]; then
        log_error "Failed to list models"
        exit 1
    fi

    echo "$models" | jq -r '.data[] | "\(.id)  \(.status.value // "unknown")  \(.name // .filename // "")"' 2>/dev/null \
        || echo "$models" | jq '.' 2>/dev/null
}

do_status() {
    check_server
    local model_id="$1"
    local info
    info=$(curl -sf "${SERVER_URL}/v1/models" 2>/dev/null \
        | jq -r --arg id "$model_id" '.data[] | select(.id == $id)' 2>/dev/null)

    if [ -z "$info" ] || [ "$info" = "null" ]; then
        log_error "Model '$model_id' not found"
        exit 1
    fi

    echo "$info" | jq '.' 2>/dev/null || echo "$info"
}

do_load() {
    local model_id="$1"
    log_info "Loading model: $model_id"

    local response
    response=$(curl -sf -X POST "${SERVER_URL}/models/load" \
        -H "Content-Type: application/json" \
        -d "{\"model\":\"$model_id\"}" 2>/dev/null)

    if [ -z "$response" ]; then
        log_error "Load request failed (no response)"
        exit 1
    fi

    local success
    success=$(echo "$response" | jq -r '.success // true' 2>/dev/null)

    if [ "$success" = "false" ]; then
        local err
        err=$(echo "$response" | jq -r '.error // "unknown"' 2>/dev/null)
        log_error "Load rejected: $err"
        exit 1
    fi

    log_info "Load accepted, waiting for model to become ready..."

    if wait_for_status "$model_id" "loaded" 120; then
        log_info "Model '$model_id' loaded successfully"
    else
        exit 1
    fi
}

do_unload() {
    local model_id="$1"
    log_info "Unloading model: $model_id"

    local response
    response=$(curl -sf -X POST "${SERVER_URL}/models/unload" \
        -H "Content-Type: application/json" \
        -d "{\"model\":\"$model_id\"}" 2>/dev/null)

    if [ -z "$response" ]; then
        log_error "Unload request failed (no response)"
        exit 1
    fi

    log_info "Unload accepted, waiting for model to be released..."

    if wait_for_status "$model_id" "unloaded" 60; then
        log_info "Model '$model_id' unloaded successfully"
    else
        exit 1
    fi
}

do_swap() {
    local new_model_id="$1"

    check_server

    local models_json
    models_json=$(curl -sf "${SERVER_URL}/v1/models" 2>/dev/null)
    if [ -z "$models_json" ]; then
        log_error "Failed to query models"
        exit 1
    fi

    local all_models
    all_models=$(echo "$models_json" | jq -r '.data[].id' 2>/dev/null || true)

    if ! echo "$all_models" | grep -qx "$new_model_id"; then
        log_error "Model '$new_model_id' not found on server"
        echo "Available models:" 1>&2
        echo "$all_models" | while read -r m; do echo "  - $m"; done 1>&2
        exit 1
    fi

    local loaded_models
    loaded_models=$(echo "$models_json" | jq -r '.data[] | select(.status.value == "loaded") | .id' 2>/dev/null || true)

    for model in $loaded_models; do
        if [ "$model" = "$new_model_id" ]; then
            log_info "Model '$new_model_id' already loaded, skipping swap"
            return 0
        fi
        log_info "Unloading current model: $model"
        do_unload "$model"
    done

    do_load "$new_model_id"
}

case "$ACTION" in
    --help|-h)
        usage
        ;;
    list)
        do_list
        ;;
    status)
        if [ -z "$MODEL_ID" ]; then usage; fi
        if echo "$KNOWN_ACTIONS" | grep -qw "$MODEL_ID"; then
            log_error "'$MODEL_ID' is an action, not a model name. Did you mean \`$0 $MODEL_ID\`?"
            exit 1
        fi
        do_status "$MODEL_ID"
        ;;
    load)
        if [ -z "$MODEL_ID" ]; then usage; fi
        if echo "$KNOWN_ACTIONS" | grep -qw "$MODEL_ID"; then
            log_error "'$MODEL_ID' is an action, not a model name. Did you mean \`$0 $MODEL_ID\`?"
            exit 1
        fi
        check_server
        do_load "$MODEL_ID"
        ;;
    unload)
        if [ -z "$MODEL_ID" ]; then usage; fi
        if echo "$KNOWN_ACTIONS" | grep -qw "$MODEL_ID"; then
            log_error "'$MODEL_ID' is an action, not a model name. Did you mean \`$0 $MODEL_ID\`?"
            exit 1
        fi
        check_server
        do_unload "$MODEL_ID"
        ;;
    swap)
        if [ -z "$MODEL_ID" ]; then usage; fi
        if echo "$KNOWN_ACTIONS" | grep -qw "$MODEL_ID"; then
            log_error "'$MODEL_ID' is an action, not a model name. Did you mean \`$0 $MODEL_ID\`?"
            exit 1
        fi
        do_swap "$MODEL_ID"
        ;;
    *)
        log_error "Unknown action: $ACTION"
        usage
        ;;
esac
