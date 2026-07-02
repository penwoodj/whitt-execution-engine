#!/bin/bash
# scripts/meta-v6/compare-models-stress.sh
# Run model-stress-test.yml across multiple models, capturing metrics per model.
#
# Default candidate list: 19 models (15+ per user requirement).
# Pass model IDs as args to test subset.
# --cpu-only: force gpu_layers=0 (default; per safety rule)
# --gpu-layers N: override (use with caution; 7B+9B only stable at 0 or 99)
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

# Args
GPU_LAYERS=0
MODELS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --cpu-only) GPU_LAYERS=0; shift ;;
    --gpu-layers) GPU_LAYERS="$2"; shift 2 ;;
    *) MODELS+=("$1"); shift ;;
  esac
done

# Default candidate list (19 models)
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

echo "=== Model Stress Test ==="
echo "Timestamp: ${TS}"
echo "Results: ${RESULTS_ROOT}"
echo "GPU layers: ${GPU_LAYERS}"
echo "Models (${#MODELS[@]}):"
printf '  - %s\n' "${MODELS[@]}"
echo ""

# TSV header
echo -e "model\tconfig\tduration_sec\tstep_01_bytes\tstep_02_bytes\tstep_03_bytes\tstep_04_bytes\ttotal_bytes\tstep_01_tokens\tstep_02_tokens\tstep_03_tokens\tstep_04_tokens\tstep_01_quality\tstep_02_quality\tstep_03_quality\tstep_04_quality\trefusals_detected\tverdict" > "$SUMMARY_TSV"

run_model() {
  local MODEL_ID="$1"
  local SAFE_NAME=$(echo "$MODEL_ID" | sed 's/[^a-zA-Z0-9_-]/_/g')
  local OUT_DIR="${RESULTS_ROOT}/${SAFE_NAME}"
  local RUN_ID="model-comparison/stress-${TS}/${SAFE_NAME}"
  mkdir -p "$OUT_DIR/stress" "$OUT_DIR/logs"

  local WORKFLOW_TMP="${OUT_DIR}/runtime.yml"

  echo "[${MODEL_ID}] starting..."

  # Rewrite workflow YAML: replace model name + gpu_layers + __RUN_ID__
  python3 <<EOF
src = open('$WORKFLOW_SRC').read()
src = src.replace('Qwen3-5-9B-Q4_K_M.gguf', '${MODEL_ID}.gguf')
src = src.replace('gpu_layers: 0', 'gpu_layers: ${GPU_LAYERS}')
src = src.replace('__RUN_ID__', '${RUN_ID}')
open('$WORKFLOW_TMP', 'w').write(src)
EOF

  # Verify file exists
  if [ ! -f "${REPO}/models/${MODEL_ID}.gguf" ]; then
    echo "[${MODEL_ID}] SKIP: model file not found"
    echo -e "${MODEL_ID}\tmissing\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\tMISSING" >> "$SUMMARY_TSV"
    return 1
  fi

  local START=$(date +%s)
  cd "$REPO"
  timeout 1200 ./target/release/whitt benchmark \
    --workflow "$WORKFLOW_TMP" \
    --output-dir "$OUT_DIR" \
    --models-dir "${REPO}/models" \
    --filter-name "^${MODEL_ID}\.gguf$" \
    --load-timeout 180 \
    > "$OUT_DIR/benchmark.log" 2>&1
  local RC=$?
  local END=$(date +%s)
  local DUR=$((END - START))

  if [ $RC -ne 0 ]; then
    echo "[${MODEL_ID}] FAIL rc=${RC} dur=${DUR}s"
    echo -e "${MODEL_ID}\trc${RC}\t${DUR}\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\tRUNTIME_FAIL" >> "$SUMMARY_TSV"
    return 2
  fi

  # Extract metrics from logs/stress.log
  local LOG="${OUT_DIR}/logs/stress.log"
  if [ ! -f "$LOG" ]; then
    echo "[${MODEL_ID}] WARN: no stress.log, probably failed early"
    echo -e "${MODEL_ID}\tgpu${GPU_LAYERS}\t${DUR}\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\t0\tNO_LOG" >> "$SUMMARY_TSV"
    return 3
  fi

  # Extract per-step bytes/tokens/quality via single Python pass
  # (inline heredoc + f-strings caused shell-escape issues; use temp file)
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

  # GPU safety check (only if GPU used)
  if [ "$GPU_LAYERS" -gt 0 ]; then
    if docker logs whitt-llama-server 2>&1 | tail -100 | grep -qE "vk::|DeviceLost|Vulkan.*Error"; then
      echo "[${MODEL_ID}] !! GPU ERROR DETECTED — stopping GPU tests"
      return 99
    fi
  fi

  return 0
}

FAIL_COUNT=0
for m in "${MODELS[@]}"; do
  run_model "$m" || FAIL_COUNT=$((FAIL_COUNT+1))
done

echo ""
echo "=== Aggregation ==="
TOTAL=${#MODELS[@]}
echo "Tested: ${TOTAL}, Failed: ${FAIL_COUNT}"
echo "Summary: ${SUMMARY_TSV}"
column -t -s $'\t' "$SUMMARY_TSV" || cat "$SUMMARY_TSV"
