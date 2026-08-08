#!/bin/bash
# scripts/meta-v6/compare-models-stress.sh
# Run model-stress-test.yml across multiple models, capturing metrics per model.
#
# SAFETY (per AGENTS.md HARD rules, verified 2026-07-02 after crash):
# - Models ≤3B at Q4: safe at gpu_layers 0 OR 99
# - Models 7B+ at Q4: MUST use gpu_layers 99 (CPU crashes via thermal overload)
# - This script AUTO-DETECTS model size and sets gpu_layers accordingly.
#   Override with --gpu-layers N (1-98 forbidden — PCIe thrash)
#
# Output: docs/benchmarks/outputs/meta-workflow/model-comparison/stress-<TS>/<SAFE_NAME>/
# Aggregation: docs/benchmarks/reports/MODEL-STRESS-TEST-<TS>.md

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
WORKFLOW_SRC="${REPO}/docs/benchmarks/workflows/model-stress-test.yml"
TS=$(date +%Y%m%d-%H%M%S)
RESULTS_ROOT="${REPO}/docs/benchmarks/outputs/meta-workflow/model-comparison/stress-${TS}"
SUMMARY_TSV="${RESULTS_ROOT}/summary.tsv"
mkdir -p "$RESULTS_ROOT"

# Safety limits
MAX_CONSECUTIVE_FAILURES=3
MAX_MODELS_WITHOUT_COOLDOWN=5
RAM_MIN_KB=3000000  # 3GB (meminfo reports KB not bytes)
# Time-based comparison: each model gets equal wall-clock budget.
# Small models iterate more within budget; large models get fewer shots.
# Default 180s = enough for ~3 iterations of small models, ~1-2 of large.
TIME_BUDGET_SEC=180

# Args
GPU_OVERRIDE=""
MODELS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --cpu-only) GPU_OVERRIDE="0"; shift ;;
    --gpu-layers) GPU_OVERRIDE="$2"; shift 2 ;;
    --time-budget) TIME_BUDGET_SEC="$2"; shift 2 ;;
    *) MODELS+=("$1"); shift ;;
  esac
done

# Default candidate list (19 models, ≤3B uses CPU, 7B+ uses GPU 99)
if [ ${#MODELS[@]} -eq 0 ]; then
  MODELS=(
    "Qwen3-5-9B-Q4_K_M"
    "Falcon-H1-7B-Instruct-Q4_K_M"
    "Mistral-7B-Instruct-v0.3-Q4_K_M"
    "Hermes-2-Pro-Mistral-7B.Q4_K_M"
    "Phi-4-mini-reasoning-Q4_K_M"
    "Phi-4-mini-instruct-Q4_K_M"
    "Ministral-3-3B-Instruct-2512-Q4_K_M"
    "Llama-3.2-3B-Instruct-Q4_K_S"
    "Jan-v3-4b-base-instruct-Q4_K_M"
    "Qwen3-1.7B-abliterated-q4_k_m"
    "Qwen3-4B-Instruct-2507-Q4_K_M"
    "LFM2.5-1.2B-Instruct-Q8_0"
    "LFM2-2.6B-SDG-q8"
    "Yi-6G-200K-Airo-Claude-Puffin-Q4_K_M"
    "Instella-3B-Q8"
    "jan-nano-128k-Q6_K"
    "granite-4.0-h-tiny-Q4_K_M"
    "Falcon3-3B-Instruct-q8_0"
    "mistral-7b-instruct-v0.2.Q4_K_S"
  )
fi

# Safety check: refuse 1-98 gpu_layers (PCIe thrash)
if [ -n "$GPU_OVERRIDE" ] && [ "$GPU_OVERRIDE" -gt 0 ] && [ "$GPU_OVERRIDE" -lt 99 ]; then
  echo "FATAL: gpu_layers 1-98 forbidden (PCIe thrash, per AGENTS.md). Use 0 or 99 only."
  exit 2
fi

# Auto-detect safe gpu_layers based on model size
safe_gpu_layers() {
  local MODEL_ID="$1"
  if [ -n "$GPU_OVERRIDE" ]; then
    echo "$GPU_OVERRIDE"
    return
  fi
  local SIZE_BYTES=$(stat -c %s "${REPO}/models/${MODEL_ID}.gguf" 2>/dev/null || echo 0)
  # 4GB = 4000000000 bytes threshold (7B+ models are >4GB at Q4)
  if [ "$SIZE_BYTES" -gt 4000000000 ]; then
    echo 99
  else
    echo 0
  fi
}

# Safety check: verify RAM available before each model
check_ram() {
  local AVAILABLE_KB=$(awk '/MemAvailable/ {print $2}' /proc/meminfo 2>/dev/null || echo 0)
  if [ "$AVAILABLE_KB" -lt "$RAM_MIN_KB" ]; then
    echo "RAM LOW ($AVAILABLE_KB KB < $RAM_MIN_KB KB), restarting docker + waiting 30s"
    docker restart whitt-llama-server > /dev/null 2>&1
    sleep 30
  fi
}

# Safety check: docker health
check_docker() {
  if ! curl -sf --max-time 5 http://localhost:8080/health > /dev/null 2>&1; then
    echo "Docker health check failed, restarting"
    docker restart whitt-llama-server > /dev/null 2>&1
    sleep 30
  fi
  # Check for GPU errors in recent logs
  if docker logs whitt-llama-server 2>&1 | tail -100 | grep -qE "vk::|DeviceLost|Vulkan.*Error"; then
    echo "GPU error detected in docker logs, RESTARTING before next test"
    docker restart whitt-llama-server > /dev/null 2>&1
    sleep 30
  fi
}

echo "=== Model Stress Test ==="
echo "Timestamp: ${TS}"
echo "Results: ${RESULTS_ROOT}"
echo "Models (${#MODELS[@]}):"
printf '  - %s\n' "${MODELS[@]}"
echo ""

echo -e "model\tconfig\titerations\ttime_used_sec\tbest_step_02_quality\tbest_step_03_quality\ttotal_tokens\ttotal_bytes\trefusals_detected\tverdict" > "$SUMMARY_TSV"

CONSECUTIVE_FAILURES=0
MODELS_SINCE_COOLDOWN=0

for MODEL_ID in "${MODELS[@]}"; do
  # Safety checks before each model
  check_ram
  check_docker

  MODELS_SINCE_COOLDOWN=$((MODELS_SINCE_COOLDOWN+1))
  if [ "$MODELS_SINCE_COOLDOWN" -ge "$MAX_MODELS_WITHOUT_COOLDOWN" ]; then
    echo "[SAFETY] Mandatory 60s cooldown after $MODELS_SINCE_COOLDOWN models"
    sleep 60
    MODELS_SINCE_COOLDOWN=0
  fi

  GPU_LAYERS=$(safe_gpu_layers "$MODEL_ID")
  SAFE_NAME=$(echo "$MODEL_ID" | sed 's/[^a-zA-Z0-9_-]/_/g')
  OUT_DIR="${RESULTS_ROOT}/${SAFE_NAME}"
  RUN_ID="model-comparison/stress-${TS}/${SAFE_NAME}"
  mkdir -p "$OUT_DIR/stress" "$OUT_DIR/logs" "$OUT_DIR/iters"
  WORKFLOW_TMP="${OUT_DIR}/runtime.yml"

  echo "[${MODEL_ID}] starting (gpu=${GPU_LAYERS}, budget=${TIME_BUDGET_SEC}s)..."

  if [ ! -f "${REPO}/models/${MODEL_ID}.gguf" ]; then
    echo "[${MODEL_ID}] SKIP: model file not found"
    echo -e "${MODEL_ID}\tmissing\t0\t0\t0\t0\t0\t0\t0\tMISSING" >> "$SUMMARY_TSV"
    continue
  fi

  # Rewrite workflow YAML ONCE: substitute model name + gpu_layers + RUN_ID
  # Per-iteration outputs go to iters/iter-N/ via __RUN_ID__ already in YAML.
  # We vary RUN_ID per iteration by recreating YAML with new __ITER__ suffix.
  python3 <<EOF
src = open('$WORKFLOW_SRC').read()
src = src.replace('Qwen3-5-9B-Q4_K_M.gguf', '${MODEL_ID}.gguf')
src = src.replace('gpu_layers: 0', 'gpu_layers: ${GPU_LAYERS}')
# __RUN_ID__ left as placeholder, replaced per-iteration below
open('$WORKFLOW_TMP.tpl', 'w').write(src)
EOF

  # Time-budgeted iteration loop
  MODEL_START=$(date +%s)
  ITERATION=0
  BEST_S2_QUALITY=0
  BEST_S3_QUALITY=0
  TOTAL_TOKENS=0
  TOTAL_BYTES=0
  REFUSALS_TOTAL=0

  while :; do
    NOW=$(date +%s)
    ELAPSED=$((NOW - MODEL_START))
    REMAINING=$((TIME_BUDGET_SEC - ELAPSED))
    if [ "$REMAINING" -lt 30 ]; then
      # Not enough time for another full iteration (~30-60s each)
      break
    fi

    ITERATION=$((ITERATION+1))
    ITER_RUN_ID="${RUN_ID}/iter-${ITERATION}"
    ITER_OUT="${OUT_DIR}/iters/iter-${ITERATION}"
    mkdir -p "$ITER_OUT"

    # Per-iteration YAML with unique RUN_ID
    sed "s|__RUN_ID__|${ITER_RUN_ID}|g" "$WORKFLOW_TMP.tpl" > "$WORKFLOW_TMP"

    echo "[${MODEL_ID}] iter=${ITERATION} remaining=${REMAINING}s"

    cd "$REPO"
    # Cap each iteration at remaining time + 60s buffer (let model finish current gen)
    timeout $((REMAINING + 30)) ./target/release/whitt benchmark \
      --workflow "$WORKFLOW_TMP" \
      --output-dir "$ITER_OUT" \
      --models-dir "${REPO}/models" \
      --filter-name "^${MODEL_ID}\.gguf$" \
      --load-timeout 60 \
      > "$ITER_OUT/benchmark.log" 2>&1
    RC=$?

    if [ $RC -ne 0 ] && [ $RC -ne 124 ]; then
      # 124=timeout (acceptable). Other codes = real failure.
      echo "[${MODEL_ID}] iter=${ITERATION} FAIL rc=${RC}"
      CONSECUTIVE_FAILURES=$((CONSECUTIVE_FAILURES+1))
      if [ "$CONSECUTIVE_FAILURES" -ge "$MAX_CONSECUTIVE_FAILURES" ]; then
        echo "[SAFETY] $CONSECUTIVE_FAILURES consecutive failures, ABORTING model"
        break 2
      fi
      continue
    fi
    CONSECUTIVE_FAILURES=0

    LOG="${OUT_DIR}/iter-${ITERATION}/logs/stress.log"
    [ ! -f "$LOG" ] && LOG="${ITER_OUT}/logs/stress.log"
    [ ! -f "$LOG" ] && LOG="${ITER_OUT}/stress.log"

    if [ ! -f "$LOG" ]; then
      echo "[${MODEL_ID}] iter=${ITERATION} WARN: no stress.log"
      continue
    fi

    # Parse this iteration's metrics, update best
    ITER_METRICS=$(LOG="$LOG" python3 -c "
import json, re, os
log = open(os.environ['LOG']).read()
steps = ['step_01_refusal_resistance', 'step_02_code_generation', 'step_03_multi_perspective_eval', 'step_04_yaml_output']
qualities = {}
total_tokens = 0
total_bytes = 0
for sid in steps:
    pattern = rf'step_name={sid} duration_ms=(\d+).*?(\{{.*?\"output\":\".*?\".*?\"quality_score.*?\}})'
    m = re.search(pattern, log, re.DOTALL)
    if m:
        try:
            d = json.loads(m.group(2))
            q = d.get('quality_score', 0) or 0
            qualities[sid] = q
            total_tokens += d.get('token_count', 0) or 0
            total_bytes += len(d.get('output', ''))
        except: qualities[sid] = 0
    else: qualities[sid] = 0

refusal_patterns = ['I cannot', \"I can't\", 'I am unable', \"I'm unable\", 'As an AI', 'do not have access', 'cannot access', 'cannot read']
refusals = 0
for sid in steps:
    pattern = rf'step_name={sid} duration_ms=\d+.*?(\{{.*?\"output\":\"(.*?)\".*?\"quality_score.*?\}})'
    m = re.search(pattern, log, re.DOTALL)
    if m:
        try:
            d = json.loads(m.group(1))
            text = d.get('output', '').lower()
            for p in refusal_patterns:
                if p.lower() in text:
                    refusals += 1
                    break
        except: pass

print(f'{qualities.get(\"step_02_code_generation\", 0)}\t{qualities.get(\"step_03_multi_perspective_eval\", 0)}\t{total_tokens}\t{total_bytes}\t{refusals}')
")
    S2_Q=$(echo "$ITER_METRICS" | awk -F'\t' '{print $1}')
    S3_Q=$(echo "$ITER_METRICS" | awk -F'\t' '{print $2}')
    ITER_TOKENS=$(echo "$ITER_METRICS" | awk -F'\t' '{print $3}')
    ITER_BYTES=$(echo "$ITER_METRICS" | awk -F'\t' '{print $4}')
    ITER_REFUSALS=$(echo "$ITER_METRICS" | awk -F'\t' '{print $5}')

    # Track best across iterations
    BEST_S2_QUALITY=$(python3 -c "print(max($BEST_S2_QUALITY, $S2_Q))")
    BEST_S3_QUALITY=$(python3 -c "print(max($BEST_S3_QUALITY, $S3_Q))")
    TOTAL_TOKENS=$((TOTAL_TOKENS + ITER_TOKENS))
    TOTAL_BYTES=$((TOTAL_BYTES + ITER_BYTES))
    REFUSALS_TOTAL=$((REFUSALS_TOTAL + ITER_REFUSALS))

    echo "[${MODEL_ID}] iter=${ITERATION} s2_q=${S2_Q} s3_q=${S3_Q} tokens=${ITER_TOKENS} bytes=${ITER_BYTES} refusals=${ITER_REFUSALS}"
  done

  MODEL_END=$(date +%s)
  MODEL_DUR=$((MODEL_END - MODEL_START))

  # Verdict: PASS if best s2 quality > 0.5 AND refusals < iterations
  VERDICT="FAIL"
  if [ "$ITERATION" -gt 0 ] && python3 -c "exit(0 if $BEST_S2_QUALITY > 0.5 else 1)"; then
    if [ "$REFUSALS_TOTAL" -lt "$ITERATION" ]; then
      VERDICT="PASS"
    fi
  fi

  echo "[${MODEL_ID}] DONE iters=${ITERATION} time=${MODEL_DUR}s best_s2=${BEST_S2_QUALITY} best_s3=${BEST_S3_QUALITY} verdict=${VERDICT}"
  echo -e "${MODEL_ID}\tgpu${GPU_LAYERS}\t${ITERATION}\t${MODEL_DUR}\t${BEST_S2_QUALITY}\t${BEST_S3_QUALITY}\t${TOTAL_TOKENS}\t${TOTAL_BYTES}\t${REFUSALS_TOTAL}\t${VERDICT}" >> "$SUMMARY_TSV"
done

echo ""
echo "=== Aggregation ==="
TOTAL=${#MODELS[@]}
echo "Tested: ${TOTAL}"
echo "Summary: ${SUMMARY_TSV}"
column -t -s $'\t' "$SUMMARY_TSV" || cat "$SUMMARY_TSV"
