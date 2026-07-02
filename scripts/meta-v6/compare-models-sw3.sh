#!/bin/bash
# scripts/meta-v6/compare-models-sw3.sh
# Compare SW3 (agentic categorization) output across different models.
#
# Uses existing P05 v4 SW1+SW2 outputs as fixed input, runs SW3 with each
# candidate model, captures timing + categorized.md for human comparison.
#
# GPU SAFETY MODE (per user directive 2026-07-02):
#   New models MUST be tested CPU-only first (--cpu-only) to get quality
#   baseline. Then GPU ramp: --gpu-layers 10, 20, 50 incrementally.
#   Each level monitored for vk::DeviceLostError or VRAM exhaustion.
#   Engine respects per-request gpu_layers via LLAMA_ARG_N_GPU_LAYERS env.
#
# RX 580 8GB safe thresholds:
#   - Q4 7B+9B models: gpu_layers 0 (CPU) or 99 (full offload) — no middle ground
#     (partial offload causes PCIe thrash, slower than CPU)
#   - Q4 4B models: gpu_layers 0 or 99 (fits comfortably)
#   - Always check `docker logs` for vk:: errors after each run
#
# Usage:
#   compare-models-sw3.sh [model-id1] [model-id2] ...
#   compare-models-sw3.sh --cpu-only Falcon-H1-7B-Instruct-Q4_K_M
#   compare-models-sw3.sh --gpu-layers 10 Falcon-H1-7B-Instruct-Q4_K_M
#   Default: runs Qwen3-5-9B-Q4_K_M (baseline) + 3 alternatives, GPU 99

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
INPUT_RUN_ID="meta-meta-v6-20260702-031311"
INPUT_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${INPUT_RUN_ID}-sw2-20260702-032411"
RESULTS_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/model-comparison/sw3-$(date +%Y%m%d-%H%M%S)"
mkdir -p "${RESULTS_DIR}"

GPU_LAYERS=99
MODELS=()

while [ $# -gt 0 ]; do
  case "$1" in
    --cpu-only) GPU_LAYERS=0; shift ;;
    --gpu-layers) GPU_LAYERS="$2"; shift 2 ;;
    --help|-h) grep '^#' "$0" | head -30; exit 0 ;;
    *) MODELS+=("$1"); shift ;;
  esac
done

if [ ${#MODELS[@]} -eq 0 ]; then
  MODELS=(
    "Qwen3-5-9B-Q4_K_M"
    "Qwen3-4B-Thinking-2507-Q4_K_M"
    "Falcon-H1-7B-Instruct-Q4_K_M"
    "Hermes-2-Pro-Mistral-7B.Q4_K_M"
  )
fi

echo "=== SW3 Model Comparison ==="
echo "Input: ${INPUT_RUN_ID} (P05 v4 SW2 outputs)"
echo "Results: ${RESULTS_DIR}"
echo "Models: ${MODELS[*]}"
echo "GPU layers: ${GPU_LAYERS} (0=CPU, 99=full offload)"
echo ""

if [ ! -f "${INPUT_DIR}/sw2/outputs.md" ]; then
  echo "FATAL: SW2 outputs.md not found at ${INPUT_DIR}/sw2/outputs.md"
  exit 1
fi

SUMMARY_TSV="${RESULTS_DIR}/summary.tsv"
echo -e "model\tgpu_layers\tduration_sec\tcategories_bytes\tcategorized_steps\ttokens_generated\texit_code" > "$SUMMARY_TSV"

run_one_model() {
  local MODEL_ID="$1"
  local SAFE_NAME=$(echo "$MODEL_ID" | tr '.' '_' | tr '/' '_')
  local OUT_DIR="${RESULTS_DIR}/${SAFE_NAME}"
  mkdir -p "${OUT_DIR}"/{sw3,logs,input}

  # Copy fixed input from P05 v4 SW2 outputs
  cp "${INPUT_DIR}/sw2/outputs.md" "${OUT_DIR}/input/outputs.md"
  cp "${INPUT_DIR}/sw2/outputs.md" "${OUT_DIR}/input/tasks.md" 2>/dev/null || \
    cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${INPUT_RUN_ID}-sw1-20260702-031327/sw1/00-bootstrap.txt" "${OUT_DIR}/input/tasks.md"

  # Build custom sw3-runtime.yml with ONLY this model declared
  # (avoids any chance of model-swap weirdness)
  # RUN_ID is the path under docs/benchmarks/outputs/meta-workflow/ that
  # matches the actual output dir, so save_to + shell hooks write to OUT_DIR
  local RUN_ID="model-comparison/$(basename "${RESULTS_DIR}")/${SAFE_NAME}"
  python3 -c "
import re
src = open('${REPO}/docs/benchmarks/workflows/sw3-agentic-categorization.yml').read()

# Replace models block: keep ONLY the test model
new_models = '''models:
  \"qwen35\":
    name: \"${MODEL_ID}.gguf\"
    host:
      type: llama_cpp_with_vulkan
    load_params:
      context_size: 32768
      batch_size: 2048
      ubatch_size: 512
      cache_type_k: \"q8_0\"
      cache_type_v: \"q8_0\"
      gpu_layers: ${GPU_LAYERS}
      threads: 5
      use_mmap: true
      flash_attn: true
      cont_batching: false
      no_cache_prompt: true
      parallel: 1
    sampling:
      temperature: 0.2
      max_tokens: 4096
'''
# Replace existing models: block up to first non-models top-level key
src = re.sub(r'^models:.*?(?=^workflow_execution_strategy:)', new_models, src, count=1, flags=re.MULTILINE | re.DOTALL)

# Substitute run ID
src = src.replace('__RUN_ID__', '${RUN_ID}')

open('${OUT_DIR}/sw3-runtime.yml', 'w').write(src)
"

  # Verify YAML parses
  if ! python3 -c "import yaml; yaml.safe_load(open('${OUT_DIR}/sw3-runtime.yml'))" 2>/dev/null; then
    echo "[${MODEL_ID}] SKIP: YAML does not parse"
    return 1
  fi

  echo "[${MODEL_ID}] running SW3..."
  local START_TS=$(date +%s)

  # Run with filter-name = exact model id (regex anchored, with .gguf suffix)
  cd "${REPO}"
  timeout 900 ./target/release/whitt benchmark \
    --workflow "${OUT_DIR}/sw3-runtime.yml" \
    --output-dir "${OUT_DIR}" \
    --models-dir "${REPO}/models" \
    --filter-name "^${MODEL_ID}\.gguf$" \
    --load-timeout 180 \
    > "${OUT_DIR}/benchmark.log" 2>&1
  local EXIT_CODE=$?
  cd - > /dev/null

  local END_TS=$(date +%s)
  local DURATION=$((END_TS - START_TS))

  # Collect output stats
  local CAT_FILE="${OUT_DIR}/sw3/03-categorized.md"
  local CAT_BYTES=0
  local CAT_STEPS=0
  local TOKENS=0

  if [ -f "$CAT_FILE" ]; then
    CAT_BYTES=$(wc -c < "$CAT_FILE")
    CAT_STEPS=$(grep -cE "^### T[0-9]" "$CAT_FILE" || echo 0)
  fi

  # Total tokens from log
  TOKENS=$(grep -oE 'token_count=[0-9]+|"token_count":[0-9]+' "${OUT_DIR}/benchmark.log" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END {print s+0}')

  echo "[${MODEL_ID}] done in ${DURATION}s, ${CAT_BYTES}B categorized, ${TOKENS} tokens, exit=${EXIT_CODE}"
  echo -e "${MODEL_ID}\t${GPU_LAYERS}\t${DURATION}\t${CAT_BYTES}\t${CAT_STEPS}\t${TOKENS}\t${EXIT_CODE}" >> "$SUMMARY_TSV"

  # GPU safety check: scan docker logs for vk:: errors after each run
  if [ "${GPU_LAYERS}" != "0" ]; then
    if docker logs whitt-llama-server 2>&1 | tail -200 | grep -qE "vk::|DeviceLost|Vulkan.*Error"; then
      echo "[${MODEL_ID}] WARNING: vk:: error detected in docker logs at GPU layers=${GPU_LAYERS}"
      echo "  Recommend: retry with lower --gpu-layers, or use --cpu-only for this model"
    fi
  fi

  return 0
}

# Run each candidate sequentially
for model in "${MODELS[@]}"; do
  run_one_model "$model" || true
done

echo ""
echo "=== Summary ==="
column -t -s $'\t' "$SUMMARY_TSV"
echo ""
echo "Results dir: ${RESULTS_DIR}"
echo "Per-model output: ${RESULTS_DIR}/<model>/sw3/03-categorized.md"
