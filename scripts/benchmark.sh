#!/bin/bash
set -e

SERVER_URL="${SERVER_URL:-http://localhost:8080}"
PROMPT="The quick brown fox jumps over the lazy dog."
MAX_TOKENS=100
CONCURRENT=1

echo "=== LLM Server Benchmark ==="
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
    \"model\": \"model\",
    \"messages\": [{\"role\": \"user\", \"content\": \"$PROMPT\"}],
    \"max_tokens\": $MAX_TOKENS,
    \"temperature\": 0.7,
    \"stream\": false
  }")
END_TIME=$(date +%s%N)

ELAPSED_MS=$(( (END_TIME - START_TIME) / 1000000 ))
TOTAL_TOKENS=$(echo "$RESPONSE" | jq -r '.usage.total_tokens')
TPS=$(echo "scale=2; $TOTAL_TOKENS / ($ELAPSED_MS / 1000)" | bc)

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
    \"stream\": false
  }" | jq '.timings')

echo "Timings:"
echo "$TIMING" | jq

# Concurrent requests (if specified)
if [ "$CONCURRENT" -gt 1 ]; then
    echo "--- Concurrent Requests ($CONCURRENT) ---"
    START_TIME=$(date +%s%N)

    for i in $(seq 1 $CONCURRENT); do
        curl -s -X POST "$SERVER_URL/v1/chat/completions" \
          -H "Content-Type: application/json" \
          -d "{
            \"model\": \"model\",
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
