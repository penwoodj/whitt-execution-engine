#!/usr/bin/env bash
# Generate opencode baseline for a single prompt using local Qwen3-5-9B-Q4_K_M.
#
# This is the "single-shot with same model" baseline for apples-to-apples
# comparison vs SW1-SW5 pipeline. Both use the same model (Qwen3-5-9B-Q4_K_M
# via local llama.cpp Docker). The baseline does NOT use tools/skills/plugins
# (those are what SW1-SW5 tries to surpass).
#
# Usage:
#   ./scripts/meta-v6/opencode-baseline.sh <prompt.md> <output.md>
#
# Output: <output.md> contains the model's response.
set -euo pipefail

PROMPT_FILE="${1:?Usage: $0 <prompt.md> <output.md>}"
OUTPUT_FILE="${2:?Usage: $0 <prompt.md> <output.md>}"

if [ ! -f "$PROMPT_FILE" ]; then
  echo "FATAL: prompt file not found: $PROMPT_FILE" >&2
  exit 1
fi

SERVER="http://localhost:8080"
MODEL="Qwen3-5-9B-Q4_K_M"

# Sanity check server is up
if ! curl -sf "${SERVER}/health" >/dev/null 2>&1; then
  echo "FATAL: llama.cpp server not responding at ${SERVER}/health" >&2
  exit 1
fi

# Load prompt content
PROMPT_TEXT=$(cat "$PROMPT_FILE")

T0=$(date +%s)

# Single-shot completion: model sees only the prompt, no tools
RESPONSE=$(curl -sf -X POST "${SERVER}/v1/chat/completions" \
  -H "Content-Type: application/json" \
  -d "$(python3 -c "
import json, sys
prompt = sys.stdin.read()
payload = {
    'model': '$MODEL',
    'messages': [
        {'role': 'user', 'content': prompt}
    ],
    'temperature': 0.3,
    'max_tokens': 8192,
    'stream': False
}
print(json.dumps(payload))
" <<< "$PROMPT_TEXT")")

T1=$(date +%s)
DURATION=$((T1-T0))

# Extract response content + write metadata footer
DURATION_FINAL=$DURATION
echo "$RESPONSE" | DURATION="$DURATION_FINAL" MODEL_NAME="$MODEL" python3 -c "
import json, os, sys
data = json.load(sys.stdin)
content = data['choices'][0]['message']['content']
usage = data.get('usage', {})
sys.stdout.write(content)
sys.stdout.write('\n\n---\n')
sys.stderr.write(f'_Baseline metadata: duration={os.environ[\"DURATION\"]}s, prompt_tokens={usage.get(\"prompt_tokens\", \"?\")}, completion_tokens={usage.get(\"completion_tokens\", \"?\")}, model={os.environ[\"MODEL_NAME\"]}_\n')
" > "$OUTPUT_FILE" 2> "$OUTPUT_FILE.meta"

BYTES=$(wc -c < "$OUTPUT_FILE")
echo "BASELINE_DONE: $OUTPUT_FILE ($BYTES bytes, ${DURATION}s)"