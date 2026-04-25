#!/bin/bash
set -euo pipefail

SERVER_URL="${SERVER_URL:-http://localhost:8080}"
PROMPT="The quick brown fox jumps over the lazy dog."
MAX_TOKENS=100
CONCURRENT=1

if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq."
    exit 1
fi

MODEL_ID=$(curl -sf "$SERVER_URL/v1/models" | jq -r '.data[] | select(.status.value == "loaded") | .id' | head -n 1)
if [ -z "$MODEL_ID" ]; then
    echo "Error: No model is currently loaded on the server."
    echo "Please load a model first: ./scripts/switch-model-v2.sh load <model-name>"
    exit 1
fi

echo "=== LLM Server Benchmark ==="
echo "Model: $MODEL_ID"
echo "Server URL: $SERVER_URL"
echo "Prompt: $PROMPT"
echo "Max tokens: $MAX_TOKENS"
echo "Concurrent requests: $CONCURRENT"
echo ""

# Single request benchmark
echo "--- Single Request ---"
START_TIME=$(date +%s%N)
RESPONSE=$(curl -s -X POST "$SERVER_URL/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -d "{
    \"model\": \"$MODEL_ID\",
    \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT\"}],
    \"max_tokens\": $MAX_TOKENS,
    \"temperature\": 0.7,
    \"stream\": false
  }")
END_TIME=$(date +%s%N)

if echo "$RESPONSE" | jq -e '.error' > /dev/null 2>&1; then
    echo "Error from server: $(echo "$RESPONSE" | jq -r '.error.message // .error')"
    exit 1
fi

ELAPSED_MS=$(( (END_TIME - START_TIME) / 1000000 ))
TOTAL_TOKENS=$(echo "$RESPONSE" | jq -r '.usage.total_tokens')
TPS=$(awk "BEGIN {printf \"%.2f\", $TOTAL_TOKENS / ($ELAPSED_MS / 1000)}")

echo "Elapsed time: ${ELAPSED_MS}ms"
echo "Total tokens: $TOTAL_TOKENS"
echo "Tokens per second: $TPS"
echo ""

# Extract timing info (using native endpoint for detailed timings)
TIMING=$(curl -s -X POST "$SERVER_URL/completion" \
  -H "Content-Type: application/json" \
  -d "{
    \"prompt\": \"$PROMPT\",
    \"n_predict\": $MAX_TOKENS,
    \"temperature\": 0.7,
    \"stream\": false,
    \"model\": \"$MODEL_ID\"
  }" | jq '.timings')

if [ "$TIMING" = "null" ] || [ -z "$TIMING" ]; then
    echo "Timings: not available (router mode)"
else
    echo "Timings:"
    echo "$TIMING" | jq
fi

# Concurrent requests (if specified)
if [ "$CONCURRENT" -gt 1 ]; then
    echo "--- Concurrent Requests ($CONCURRENT) ---"
    START_TIME=$(date +%s%N)

    for i in $(seq 1 $CONCURRENT); do
        curl -s -X POST "$SERVER_URL/v1/chat/completions" \
          -H "Content-Type: application/json" \
          -d "{
            \"model\": \"$MODEL_ID\",
            \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT $i\"}],
            \"max_tokens\": $MAX_TOKENS,
            \"temperature\": 0.7,
            \"stream\": false
          }" > /dev/null &
    done

    wait

    END_TIME=$(date +%s%N)
    ELAPSED_MS=$(( (END_TIME - START_TIME) / 1000000 ))

    echo "Total elapsed time: ${ELAPSED_MS}ms"
    echo "Average per request: $(( ELAPSED_MS / CONCURRENT ))ms"
fi

echo ""
echo "=== Benchmark Complete ==="
