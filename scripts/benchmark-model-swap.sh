#!/bin/bash
# DEPRECATED: Use `whitt benchmark` and `whitt model swap` instead.
# This script is retained for backward compatibility but all functionality
# has been migrated to the Rust CLI binary.
set -uo pipefail

SERVER_URL="${LLAMA_SERVER_URL:-http://localhost:8080}"
MODEL_ID="${1:-Qwen2.5-0.5B-Instruct-Q4_K_M}"
ROUNDS="${2:-3}"
DRIVE_LABEL="${3:-$(df -h "$(dirname "$(readlink -f ./models 2>/dev/null || echo /tmp)")" 2>/dev/null | awk 'NR==2{print $1}')}"

GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

log_detail() { echo -e "${CYAN}[DATA]${NC} $1"; }

if ! curl -sf "${SERVER_URL}/health" > /dev/null 2>&1; then
    echo "ERROR: Server not reachable at ${SERVER_URL}"
    exit 1
fi

echo "=========================================="
echo "  Model Load/Unload Benchmark"
echo "=========================================="
log_detail "Model:      $MODEL_ID"
log_detail "Rounds:     $ROUNDS"
log_detail "Drive:      $DRIVE_LABEL"
log_detail "Server:     $SERVER_URL"
echo ""

poll_status() {
    local model_id="$1"
    local expected="$2"
    local timeout="${3:-120}"
    local elapsed=0
    while [ $elapsed -lt $timeout ]; do
        local current
        current=$(curl -sf "${SERVER_URL}/v1/models" 2>/dev/null \
            | jq -r --arg id "$model_id" '.data[] | select(.id == $id) | .status.value // "unknown"' 2>/dev/null || echo "unknown")
        [ "$current" = "$expected" ] && return 0
        sleep 0.5
        elapsed=$((elapsed + 1))
    done
    return 1
}

api_load() {
    local code
    code=$(curl -sf -o /dev/null -w "%{http_code}" -X POST "${SERVER_URL}/models/load" \
        -H "Content-Type: application/json" \
        -d "{\"model\":\"$MODEL_ID\"}" 2>/dev/null || echo "000")
    [ "$code" = "200" ]
}

api_unload() {
    local code
    code=$(curl -sf -o /dev/null -w "%{http_code}" -X POST "${SERVER_URL}/models/unload" \
        -H "Content-Type: application/json" \
        -d "{\"model\":\"$MODEL_ID\"}" 2>/dev/null || echo "000")
    [ "$code" = "200" ] || [ "$code" = "202" ]
}

load_times=()
unload_times=()
swap_times=()

for i in $(seq 1 "$ROUNDS"); do
    echo "--- Round $i/$ROUNDS ---"

    load_start=$(date +%s%N)
    if api_load; then
        poll_status "$MODEL_ID" "loaded" 120
    else
        log_detail "Load:  skipped (already loaded)"
    fi
    load_end=$(date +%s%N)
    load_ms=$(( (load_end - load_start) / 1000000 ))
    load_times+=("$load_ms")
    log_detail "Load:   ${load_ms}ms"

    chat_start=$(date +%s%N)
    curl -sf "${SERVER_URL}/v1/chat/completions" \
        -H "Content-Type: application/json" \
        -d "{\"model\":\"$MODEL_ID\",\"messages\":[{\"role\":\"user\",\"content\":\"hi\"}],\"max_tokens\":5}" > /dev/null 2>&1 || true
    chat_end=$(date +%s%N)
    chat_ms=$(( (chat_end - chat_start) / 1000000 ))
    log_detail "Chat:   ${chat_ms}ms"

    unload_start=$(date +%s%N)
    if api_unload; then
        poll_status "$MODEL_ID" "unloaded" 60
    else
        log_detail "Unload: skipped (not loaded)"
    fi
    unload_end=$(date +%s%N)
    unload_ms=$(( (unload_end - unload_start) / 1000000 ))
    unload_times+=("$unload_ms")
    log_detail "Unload: ${unload_ms}ms"

    swap_ms=$((load_ms + unload_ms))
    swap_times+=("$swap_ms")
    log_detail "Swap:   ${swap_ms}ms"
    echo ""
done

avg() {
    local sum=0
    for v in "$@"; do sum=$((sum + v)); done
    echo $((sum / $#))
}

min_val() {
    local min="${1:-0}"
    for v in "$@"; do [ "$v" -lt "$min" ] && min="$v"; done
    echo "$min"
}

max_val() {
    local max="${1:-0}"
    for v in "$@"; do [ "$v" -gt "$max" ] && max="$v"; done
    echo "$max"
}

echo "=========================================="
echo "  Results Summary"
echo "=========================================="
log_detail "Drive:     $DRIVE_LABEL"
log_detail "Model:     $MODEL_ID"
log_detail "File size: $(du -h ./models/${MODEL_ID}.gguf 2>/dev/null | awk '{print $1}')"
echo ""
printf "  %-10s %8s %8s %8s %8s\n" "Metric" "Min" "Avg" "Max" "Rounds"
printf "  %-10s %8s %8s %8s %8s\n" "------" "---" "---" "---" "------"
printf "  %-10s %6dms %6dms %6dms %8d\n" "Load" "$(min_val "${load_times[@]}")" "$(avg "${load_times[@]}")" "$(max_val "${load_times[@]}")" "$ROUNDS"
printf "  %-10s %6dms %6dms %6dms %8d\n" "Unload" "$(min_val "${unload_times[@]}")" "$(avg "${unload_times[@]}")" "$(max_val "${unload_times[@]}")" "$ROUNDS"
printf "  %-10s %6dms %6dms %6dms %8d\n" "Swap" "$(min_val "${swap_times[@]}")" "$(avg "${swap_times[@]}")" "$(max_val "${swap_times[@]}")" "$ROUNDS"
echo ""
