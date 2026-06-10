#!/usr/bin/env bash
set -euo pipefail

# Batch runner for model benchmark suite
# Runs json-bench-3step.yml with each model, validates output, accumulates results
# Usage: ./scripts/run-model-bench-suite.sh [start_index] [end_index]

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
RUN_ID="run-$(date +%Y%m%d-%H%M%S)"
BASE_DIR="$PROJECT_DIR/docs/benchmarks/outputs/model-bench/$RUN_ID"
REPORT="$BASE_DIR/results-report.md"

START_IDX="${1:-1}"
END_IDX="${2:-50}"

mkdir -p "$BASE_DIR"

# Model list (50 unique models, niche-first)
MODELS=(
"Phi-4-mini-instruct-abliterated-Q3_K_M"
"Qwen3-4B-Hivemind-Inst-Hrtic-Ablit-Uncensored-Q4_K_M-imat"
"Qwen3-4B-Hivemind-Inst-Hrtic-Ablit-Uncensored-Q6_K-imat"
"Qwen3-4B-TripleX-Heretic-Uncensored-Q8_0"
"Qwen2.5-7B-Instruct-1M-Thinking-Claude-Gemini-GPT5.2-DISTILL-PaperWitch-heresy.i1-Q4_K_M"
"Qwen3-6B-Hivemind-Inst-Hrtic-Ablit-Uncensored-Q6_K-imat"
"Qwen2.5-7B-Instruct-1M-Thinking-Claude-Gemini-GPT5.2-DISTILL-PaperWitch-heresy.i1-Q6_K"
"Qwen3-8B-Hivemind-Inst-Hrtic-Ablit-Uncensored-Q6_K-imat"
"Qwen3-0.6B-abliterated-q4_k_m"
"Qwen3-0.6B-abliterated-q8_0"
"Qwen3-1.7B-abliterated-q4_k_m"
"Qwen3-1.7B-abliterated-q8_0"
"Qwen3-4B-abliterated-q4_1"
"Qwen3-4B-Hivemind-Instruct-NeoMAX-D_AU-Q4_K_M-imat"
"Qwen3-4B-abliterated-q6_k_m"
"Qwen3-4B-Hivemind-Instruct-NeoMAX-D_AU-Q6_K-imat"
"Qwen2.5-7B-Instruct-1M-abliterated.Q4_K_M"
"Qwen2.5-7B-Instruct-1M-abliterated.Q6_K"
"Phi-4-mini-reasoning-Q4_K_M"
"ProLong-512k-8B-CLIPPER.Q4_K_M"
"Llama-3.1-Nemotron-8B-UltraLong-2M-Instruct.Q4_K_M"
"Llama-3.1-Nemotron-8B-UltraLong-4M-Instruct.Q4_K_M"
"llama3.1_6.5b_mergkit_prunme.Q6_K"
"osmosis-structure-0.6b-q4_k_m"
"PromptBridge-0.6b-Alpha.Q4_K_M"
"PromptBridge-0.6b-Alpha.Q6_K"
"Luth-0.6B-Instruct-Q8_0"
"theta-crucis-0.6b-turbo1-q8_0"
"LFM2.5-1.2B-Instruct-Q8_0"
"LFM2-1.2B-Q8_0"
"LFM2.5-1.2B-Thinking-Q8_0"
"starcoder2-3b-Q4_K_M"
"ReaderLM-v2.Q8_0"
"SmolLM3-Q4_K_M"
"SmolLM3-3B-128K-UD-Q4_K_XL"
"Ministral-3-3B-Instruct-2512-Q4_K_M"
"starcoder2-3b-instruct.i1-Q5_K_M"
"starcoder2-3b.Q6_K"
"Phi-4-mini-instruct-Q4_K_M"
"starcoder2-3b-instruct.i1-Q6_K"
"granite-4.0-h-micro-Q6_K"
"LFM2-2.6B-SDG-q8"
"lfm2-2.6b-lmsguide-q8_0"
"granite-4.0-micro-Q6_K"
"granite-3b-code-instruct-128k.i1-Q6_K"
"Phi-4-mini-instruct-Q6_K"
"SmolLM3-Q8_0"
"Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M"
"granite-3b-code-base-128k.Q8_0"
"LWM-Text-Chat-1M-Q4_K_M"
)

echo "# Model Benchmark Results — $RUN_ID" > "$REPORT"
echo "" >> "$REPORT"
echo "Started: $(date -Iseconds)" >> "$REPORT"
echo "Workflow: json-bench-3step.yml (3 steps: read TOML → format JSON → validate)" >> "$REPORT"
echo "Models tested: $((END_IDX - START_IDX + 1))" >> "$REPORT"
echo "" >> "$REPORT"
echo "| # | Model | Size | Niche | Engine | JSON Valid | Has Deps | Count Match | Sorted | Score | Time (s) |" >> "$REPORT"
echo "|---|-------|------|-------|--------|-----------|----------|-------------|--------|-------|----------|" >> "$REPORT"

PASSED=0
FAILED=0
TOTAL=0

for IDX in $(seq $START_IDX $END_IDX); do
  ARRAY_IDX=$((IDX - 1))
  if [ $ARRAY_IDX -ge ${#MODELS[@]} ]; then
    echo "[bench] Index $IDX out of range (max ${#MODELS[@]})"
    break
  fi
  
  MODEL="${MODELS[$ARRAY_IDX]}"
  # Sanitize model name for directory
  SAFE_NAME=$(echo "$MODEL" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9-]/-/g' | sed 's/--*/-/g' | sed 's/^-//;s/-$//')
  MODEL_DIR="$BASE_DIR/$(printf '%02d' $IDX)-$SAFE_NAME"
  
  TOTAL=$((TOTAL + 1))
  echo ""
  echo "========================================"
  echo "[bench] $IDX/$END_IDX: $MODEL"
  echo "========================================"
  
  # Run the benchmark
  "$SCRIPT_DIR/run-model-bench.sh" "$MODEL" "$MODEL_DIR" 2>&1 || true
  
  # Read timing results
  if [ -f "$MODEL_DIR/timing.json" ]; then
    ENGINE_PASS=$(python3 -c "import json; d=json.load(open('$MODEL_DIR/timing.json')); print(d.get('engine_pass', False))")
    ELAPSED=$(python3 -c "import json; d=json.load(open('$MODEL_DIR/timing.json')); print(d.get('elapsed_ms', 0))")
    ELAPSED_S=$(python3 -c "print(round($ELAPSED / 1000, 1))")
  else
    ENGINE_PASS="False"
    ELAPSED_S="?"
  fi
  
  # Read validation results
  VALID_LOG="$MODEL_DIR/validate.log"
  if [ -f "$VALID_LOG" ]; then
    # Parse validation results from log
    JSON_VALID=$(grep -c "✅ valid_json" "$VALID_LOG" || echo 0)
    HAS_DEPS=$(grep -c "✅ has_dependencies" "$VALID_LOG" || echo 0)
    COUNT_MATCH=$(grep -c "✅ count_matches" "$VALID_LOG" || echo 0)
    SORTED=$(grep -c "✅ sorted_alphabetically" "$VALID_LOG" || echo 0)
    ALL_PASS=$(grep -c "PASS:" "$VALID_LOG" || echo 0)
    SCORE=$(python3 -c "
with open('$VALID_LOG') as f:
    content = f.read()
    checks = content.count('✅')
    total = checks + content.count('❌')
    print(f'{checks}/{total}' if total > 0 else '0/0')
")
    PASS_FAIL="PASS" if [ "$ALL_PASS" -gt 0 ] || PASS_FAIL="FAIL"
  else
    JSON_VALID=0; HAS_DEPS=0; COUNT_MATCH=0; SORTED=0; SCORE="0/0"; PASS_FAIL="FAIL"
  fi
  
  if [ "$PASS_FAIL" = "PASS" ]; then
    PASSED=$((PASSED + 1))
  else
    FAILED=$((FAILED + 1))
  fi
  
  # Append to report
  echo "| $IDX | ${MODEL:0:50} | - | - | $ENGINE_PASS | $JSON_VALID | $HAS_DEPS | $COUNT_MATCH | $SORTED | $SCORE | ${ELAPSED_S}s |" >> "$REPORT"
  
  echo "[bench] $PASS_FAIL | Score: $SCORE | Time: ${ELAPSED_S}s"
done

# Summary
echo "" >> "$REPORT"
echo "## Summary" >> "$REPORT"
echo "" >> "$REPORT"
echo "- **Total tested**: $TOTAL" >> "$REPORT"
echo "- **Passed**: $PASSED" >> "$REPORT"
echo "- **Failed**: $FAILED" >> "$REPORT"
echo "- **Completed**: $(date -Iseconds)" >> "$REPORT"

echo ""
echo "========================================"
echo "[bench] COMPLETE: $PASSED passed, $FAILED failed out of $TOTAL"
echo "[bench] Report: $REPORT"
echo "========================================"
