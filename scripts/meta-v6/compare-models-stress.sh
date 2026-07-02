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

# Args
GPU_OVERRIDE=""
MODELS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --cpu-only) GPU_OVERRIDE="0"; shift ;;
    --gpu-layers) GPU_OVERRIDE="$2"; shift 2 ;;
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

echo -e "model\tconfig\tduration_sec\tstep_01_bytes\tstep_02_bytes\tstep_03_bytes\tstep_04_bytes\ttotal_bytes\tstep_01_tokens\tstep_02_tokens\tstep_03_tokens\tstep_04_tokens\tstep_01_quality\tstep_02_quality\tstep_03_quality\tstep_04_quality\trefusals_detected\tverdict" > "$SUMMARY_TSV"

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
  mkdir -p "$OUT_DIR/stress" "$OUT_DIR/logs"
  WORKFLOW_TMP="${OUT_DIR}/runtime.yml"

  echo "[${MODEL_ID}] starting (gpu=${GPU_LAYERS})..."

  if [ ! -f "${REPO}/models/${MODEL_ID}.gguf" ]; then
    echo "[${MODEL_ID}] SKIP: model file not found"
    echo -e "${MODEL_ID}\tmissing\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\tMISSING" >> "$SUMMARY_TSV"
    continue
  fi

  # Rewrite workflow YAML: substitute model name + gpu_layers + RUN_ID
  python3 <<EOF
src = open('$WORKFLOW_SRC').read()
src = src.replace('Qwen3-5-9B-Q4_K_M.gguf', '${MODEL_ID}.gguf')
src = src.replace('gpu_layers: 0', 'gpu_layers: ${GPU_LAYERS}')
src = src.replace('__RUN_ID__', '${RUN_ID}')
open('$WORKFLOW_TMP', 'w').write(src)
EOF

  START=$(date +%s)
  cd "$REPO"
  timeout 1200 ./target/release/whitt benchmark \
    --workflow "$WORKFLOW_TMP" \
    --output-dir "$OUT_DIR" \
    --models-dir "${REPO}/models" \
    --filter-name "^${MODEL_ID}\.gguf$" \
    --load-timeout 180 \
    > "$OUT_DIR/benchmark.log" 2>&1
  RC=$?
  END=$(date +%s)
  DUR=$((END - START))

  if [ $RC -ne 0 ]; then
    echo "[${MODEL_ID}] FAIL rc=${RC} dur=${DUR}s"
    echo -e "${MODEL_ID}\tgpu${GPU_LAYERS}\t${DUR}\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\tRUNTIME_FAIL" >> "$SUMMARY_TSV"
    CONSECUTIVE_FAILURES=$((CONSECUTIVE_FAILURES+1))
    if [ "$CONSECUTIVE_FAILURES" -ge "$MAX_CONSECUTIVE_FAILURES" ]; then
      echo "[SAFETY] $CONSECUTIVE_FAILURES consecutive failures, ABORTING"
      exit 3
    fi
    continue
  fi

  CONSECUTIVE_FAILURES=0
  LOG="${OUT_DIR}/logs/stress.log"

  if [ ! -f "$LOG" ]; then
    echo "[${MODEL_ID}] WARN: no stress.log (early failure?)"
    echo -e "${MODEL_ID}\tgpu${GPU_LAYERS}\t${DUR}\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\tNO_LOG" >> "$SUMMARY_TSV"
    continue
  fi

  # Parse metrics via temp Python file (avoids shell-escape issues with inline heredoc)
  cat > "$OUT_DIR/parse_metrics.py" <<'PYEOF'
import json, re, sys, os
log_path = os.environ['LOG']
summary_path = os.environ['SUMMARY_TSV']
model_id = os.environ['MODEL_ID']
gpu_layers = os.environ['GPU_LAYERS']
dur = os.environ['DUR']

log = open(log_path).read()
steps = ['step_01_refusal_resistance', 'step_02_code_generation', 'step_03_multi_perspective_eval', 'step_04_yaml_output']
short = ['s1', 's2', 's3', 's4']
out = {}
for sid, short_id in zip(steps, short):
    pattern = rf'step_name={sid} duration_ms=(\d+).*?(\{{.*?"output":".*?"quality_score.*?\}})'
    m = re.search(pattern, log, re.DOTALL)
    if m:
        try:
            d = json.loads(m.group(2))
            out[f'{short_id}_bytes'] = len(d.get('output', ''))
            out[f'{short_id}_tokens'] = d.get('token_count', 0)
            out[f'{short_id}_quality'] = d.get('quality_score', 0)
        except json.JSONDecodeError:
            out[f'{short_id}_bytes'] = 0
            out[f'{short_id}_tokens'] = 0
            out[f'{short_id}_quality'] = 0
    else:
        out[f'{short_id}_bytes'] = 0
        out[f'{short_id}_tokens'] = 0
        out[f'{short_id}_quality'] = 0

refusal_patterns = ['I cannot', "I can't", 'I am unable', "I'm unable", 'As an AI', 'do not have access', 'cannot access', 'cannot read']
refusals = 0
for sid in steps:
    pattern = rf'step_name={sid} duration_ms=\d+.*?(\{{.*?"output":"(.*?)".*?"quality_score.*?\}})'
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

total_bytes = sum(out[f'{s}_bytes'] for s in short)
verdict = 'PASS' if (refusals == 0 and total_bytes > 2000) else 'FAIL'

row = f'{model_id}\tgpu{gpu_layers}\t{dur}\t'
row += '\t'.join(str(out[f'{s}_bytes']) for s in short) + '\t'
row += str(total_bytes) + '\t'
row += '\t'.join(str(out[f'{s}_tokens']) for s in short) + '\t'
row += '\t'.join(str(out[f'{s}_quality']) for s in short) + '\t'
row += str(refusals) + '\t' + verdict
with open(summary_path, 'a') as f:
    f.write(row + '\n')
print(f'parsed: bytes={total_bytes} refusals={refusals} verdict={verdict}')
PYEOF
  LOG="$LOG" SUMMARY_TSV="$SUMMARY_TSV" MODEL_ID="$MODEL_ID" GPU_LAYERS="$GPU_LAYERS" DUR="$DUR" \
    python3 "$OUT_DIR/parse_metrics.py"
  echo "[${MODEL_ID}] done dur=${DUR}s"
done

echo ""
echo "=== Aggregation ==="
TOTAL=${#MODELS[@]}
echo "Tested: ${TOTAL}"
echo "Summary: ${SUMMARY_TSV}"
column -t -s $'\t' "$SUMMARY_TSV" || cat "$SUMMARY_TSV"
