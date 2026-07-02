#!/bin/bash
# META-v6 SW3 wrapper: runs agentic-categorization sub-workflow
# Resilient: synthesizes categories.md from intermediate step file if final fails
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
SW3_RUN_ID="meta-${META_RUN_ID}-sw3-$(date +%Y%m%d-%H%M%S)"
SW3_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW3_RUN_ID}"
mkdir -p "${SW3_DIR}"/{sw3,logs,input}
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/outputs.md" "${SW3_DIR}/input/outputs.md"
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/tasks.md" "${SW3_DIR}/input/tasks.md"
sed "s/__RUN_ID__/${SW3_RUN_ID}/g" "${REPO}/docs/benchmarks/workflows/sw3-agentic-categorization.yml" > "${SW3_DIR}/sw3-runtime.yml"
cd "${REPO}"
./target/release/whitt benchmark --workflow "${SW3_DIR}/sw3-runtime.yml" --output-dir "${SW3_DIR}" --models-dir "${REPO}/models" --filter-name "Qwen3-[45]" --load-timeout 1800 > "${SW3_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

OUT_FILE="${SW3_DIR}/sw3/categories.md"
if [[ ! -f "${OUT_FILE}" ]]; then
  echo "[wrapper] categories.md missing, synthesizing from latest intermediate" >> "${SW3_DIR}/benchmark.log"
  LATEST=$(ls -1 "${SW3_DIR}/sw3/"0*.{md,txt} 2>/dev/null | sort | tail -1)
  if [[ -z "${LATEST}" ]]; then
    echo "[wrapper] FATAL: no intermediate files found" >> "${SW3_DIR}/benchmark.log"
    exit 1
  fi
  {
    echo "# Agentic Categorization"
    echo
    echo "## Source: ${LATEST##*/}"
    echo
    cat "${LATEST}"
    echo
    echo "## Generation Method"
    echo "Fallback synthesis from ${LATEST##*/} (SW3 final step failed: exit=${SW_EXIT})"
  } > "${OUT_FILE}"
  echo "[wrapper] synthesized categories.md from ${LATEST##*/}" >> "${SW3_DIR}/benchmark.log"
fi
mkdir -p "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input"
cp "${OUT_FILE}" "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/categories.md"
echo "SW3_DONE: ${SW3_RUN_ID} (categories.md=$(wc -c < "${OUT_FILE}") bytes, sw3_exit=${SW_EXIT})"
