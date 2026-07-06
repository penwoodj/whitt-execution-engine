#!/bin/bash
# META-v6 bootstrap: generate META_RUN_ID, create dirs, copy prompt
# Usage: bootstrap.sh <PROMPT_FILE>
# Writes META_RUN_ID to ./.current-meta-run for downstream wrappers
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/env.sh"
PROMPT_FILE="${1:-${REPO}/docs/plans/meta-workflow-qwen35/test-prompts/real/prompt-14-task-add-true-parallel-inference-for-same-model-multi-target.md}"
META_RUN_ID="meta-v6-$(date +%Y%m%d-%H%M%S)"
META_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"

mkdir -p "${META_DIR}"/{meta,logs,input,sw1,sw2,sw3,sw4,sw5}
cp "${PROMPT_FILE}" "${META_DIR}/input/prompt.txt"
echo -n "${META_RUN_ID}" > "${REPO}/.current-meta-run"
echo "META_RUN_ID=${META_RUN_ID}"
echo "META_DIR=${META_DIR}"
echo "BOOTSTRAP_DONE"
