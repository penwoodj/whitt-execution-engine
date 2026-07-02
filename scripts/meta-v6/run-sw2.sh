#!/bin/bash
# META-v6 SW2 wrapper: runs desired-output-state sub-workflow
# Resilient: synthesizes outputs.md from intermediate step file if final fails
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
SW2_RUN_ID="meta-${META_RUN_ID}-sw2-$(date +%Y%m%d-%H%M%S)"
SW2_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW2_RUN_ID}"
mkdir -p "${SW2_DIR}"/{sw2,logs,input}
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/tasks.md" "${SW2_DIR}/input/tasks.md"
sed "s/__RUN_ID__/${SW2_RUN_ID}/g" "${REPO}/docs/benchmarks/workflows/sw2-desired-output-state.yml" > "${SW2_DIR}/sw2-runtime.yml"
cd "${REPO}"
./target/release/whitt benchmark --workflow "${SW2_DIR}/sw2-runtime.yml" --output-dir "${SW2_DIR}" --models-dir "${REPO}/models" --filter-name "Qwen3-[45]" --load-timeout 1800 > "${SW2_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

OUT_FILE="${SW2_DIR}/sw2/outputs.md"
if [[ ! -f "${OUT_FILE}" ]]; then
  echo "[wrapper] outputs.md missing, synthesizing from latest intermediate" >> "${SW2_DIR}/benchmark.log"
  LATEST=$(ls -1 "${SW2_DIR}/sw2/"0*.{md,txt} 2>/dev/null | sort | tail -1)
  if [[ -z "${LATEST}" ]]; then
    echo "[wrapper] FATAL: no intermediate files found" >> "${SW2_DIR}/benchmark.log"
    exit 1
  fi
  {
    echo "# Desired Output States"
    echo
    echo "## Source: ${LATEST##*/}"
    echo
    cat "${LATEST}"
    echo
    echo "## Generation Method"
    echo "Fallback synthesis from ${LATEST##*/} (SW2 final step failed: exit=${SW_EXIT})"
  } > "${OUT_FILE}"
  echo "[wrapper] synthesized outputs.md from ${LATEST##*/}" >> "${SW2_DIR}/benchmark.log"
fi
mkdir -p "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input"
cp "${OUT_FILE}" "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/outputs.md"
echo "SW2_DONE: ${SW2_RUN_ID} (outputs.md=$(wc -c < "${OUT_FILE}") bytes, sw2_exit=${SW_EXIT})"
