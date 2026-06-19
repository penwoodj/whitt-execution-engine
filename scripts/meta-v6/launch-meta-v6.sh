#!/bin/bash
# META-v6 launcher: fully detaches from caller via setsid
# Survives parent shell termination
# Usage: launch-meta-v6.sh [PROMPT_FILE]
set -euo pipefail
REPO="/home/jon/code/whitt-execution-engine"
PROMPT_FILE="${1:-docs/plans/meta-workflow-qwen35/test-prompts/real/prompt-14-task-add-true-parallel-inference-for-same-model-multi-target.md}"
RUN_ID="meta-v6-e2e-$(date +%Y%m%d-%H%M%S)"
OUT_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${RUN_ID}"
mkdir -p "${OUT_DIR}/logs"

cd "${REPO}"
# Run synchronously inside detached session
./target/release/whitt benchmark \
  --workflow ./docs/benchmarks/workflows/meta-workflow-v6.yml \
  --output-dir "${OUT_DIR}" \
  --models-dir "${REPO}/models" \
  --filter-name "Qwen3-5-9B" \
  --load-timeout 900 \
  > "${OUT_DIR}/meta-benchmark.log" 2>&1

echo "META_V6_DONE: ${RUN_ID}" >> "${OUT_DIR}/meta-benchmark.log"
echo "${RUN_ID}" > "${REPO}/.last-successful-meta-run"
