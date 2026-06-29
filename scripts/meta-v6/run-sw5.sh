#!/bin/bash
# META-v6 SW5 wrapper: runs final-workflow-assembly sub-workflow (deterministic design).
# SW5 YAML invokes build-workflow.py via shell hooks — no LLM cascade.
# Output: <META_RUN_ID>/meta/generated-workflow.yml
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
SW5_RUN_ID="meta-${META_RUN_ID}-sw5-$(date +%Y%m%d-%H%M%S)"
SW5_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}"
mkdir -p "${SW5_DIR}"/{sw5,logs,input}
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/structs.md" "${SW5_DIR}/input/structs.md"
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/prompt.txt" "${SW5_DIR}/input/prompt.txt" 2>/dev/null || echo "[wrapper] WARN: prompt.txt missing" >> "${SW5_DIR}/benchmark.log"
sed -e "s/__RUN_ID__/${SW5_RUN_ID}/g" -e "s/__META_RUN_ID__/${META_RUN_ID}/g" "${REPO}/docs/benchmarks/workflows/sw5-final-workflow-assembly.yml" > "${SW5_DIR}/sw5-runtime.yml"
cd "${REPO}"
./target/release/whitt benchmark --workflow "${SW5_DIR}/sw5-runtime.yml" --output-dir "${SW5_DIR}" --models-dir "${REPO}/models" --filter-name "Qwen3-5-9B" --load-timeout 1800 > "${SW5_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

# SW5 YAML step_01 shell hook already wrote META_DIR/meta/generated-workflow.yml.
# Verify it exists; abort if not (no more fallback synthesis — deterministic SW5 must produce output).
GENERATED="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/meta/generated-workflow.yml"
if [[ ! -s "${GENERATED}" ]]; then
  echo "[wrapper] FATAL: generated-workflow.yml missing after SW5 (exit=${SW_EXIT})" >> "${SW5_DIR}/benchmark.log"
  tail -20 "${SW5_DIR}/benchmark.log" >> "${SW5_DIR}/benchmark.log"
  exit 1
fi

echo "SW5_DONE: ${SW5_RUN_ID} (generated-workflow.yml=$(wc -c < "${GENERATED}") bytes, sw5_exit=${SW_EXIT})"
