#!/bin/bash
# META-v6 SW4 wrapper: runs yaml-substructure-translation sub-workflow
# Resilient: synthesizes structs.md from intermediate step file if final fails
set -uo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/env.sh"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
SW4_RUN_ID="meta-${META_RUN_ID}-sw4-$(date +%Y%m%d-%H%M%S)"
SW4_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW4_RUN_ID}"
mkdir -p "${SW4_DIR}"/{sw4,logs,input}
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/categories.md" "${SW4_DIR}/input/categories.md"
sed "s/__RUN_ID__/${SW4_RUN_ID}/g" "${REPO}/docs/benchmarks/workflows/sw4-yaml-substructure-translation.yml" > "${SW4_DIR}/sw4-runtime.yml"
localize_workflow "${SW4_DIR}/sw4-runtime.yml" "${SW4_DIR}/sw4-runtime.yml"
cd "${REPO}"
./target/release/whitt benchmark --workflow "${SW4_DIR}/sw4-runtime.yml" --output-dir "${SW4_DIR}" $(model_flags) --load-timeout 1800 > "${SW4_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

OUT_FILE="${SW4_DIR}/sw4/structs.md"
if [[ ! -f "${OUT_FILE}" ]]; then
  echo "[wrapper] structs.md missing, synthesizing from latest intermediate" >> "${SW4_DIR}/benchmark.log"
  LATEST=$(ls -1 "${SW4_DIR}/sw4/"0*.{md,txt} 2>/dev/null | sort | tail -1)
  if [[ -z "${LATEST}" ]]; then
    echo "[wrapper] FATAL: no intermediate files found" >> "${SW4_DIR}/benchmark.log"
    exit 1
  fi
  {
    echo "# YAML Substructure Translation"
    echo
    echo "## Source: ${LATEST##*/}"
    echo
    cat "${LATEST}"
    echo
    echo "## Generation Method"
    echo "Fallback synthesis from ${LATEST##*/} (SW4 final step failed: exit=${SW_EXIT})"
  } > "${OUT_FILE}"
  echo "[wrapper] synthesized structs.md from ${LATEST##*/}" >> "${SW4_DIR}/benchmark.log"
fi
mkdir -p "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input"
cp "${OUT_FILE}" "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/structs.md"
echo "SW4_DONE: ${SW4_RUN_ID} (structs.md=$(wc -c < "${OUT_FILE}") bytes, sw4_exit=${SW_EXIT})"
