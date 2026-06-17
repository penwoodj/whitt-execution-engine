#!/bin/bash
# META-v6 SW5 wrapper: runs final-workflow-assembly sub-workflow
# Resilient: synthesizes workflow.yml from intermediate step file if final fails
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
SW5_RUN_ID="meta-${META_RUN_ID}-sw5-$(date +%Y%m%d-%H%M%S)"
SW5_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}"
mkdir -p "${SW5_DIR}"/{sw5,logs,input}
cp "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/structs.md" "${SW5_DIR}/input/structs.md"
sed "s/__RUN_ID__/${SW5_RUN_ID}/g" "${REPO}/docs/benchmarks/workflows/sw5-final-workflow-assembly.yml" > "${SW5_DIR}/sw5-runtime.yml"
cd "${REPO}"
./target/release/whitt benchmark --workflow "${SW5_DIR}/sw5-runtime.yml" --output-dir "${SW5_DIR}" --models-dir "${REPO}/models" --filter-name "Qwen3-5-9B" --load-timeout 1800 > "${SW5_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

OUT_FILE="${SW5_DIR}/sw5/workflow.yml"
if [[ ! -f "${OUT_FILE}" ]]; then
  echo "[wrapper] workflow.yml missing, synthesizing from latest intermediate" >> "${SW5_DIR}/benchmark.log"
  LATEST=$(ls -1 "${SW5_DIR}/sw5/"0*.{yml,yaml,md,txt} 2>/dev/null | sort | tail -1)
  if [[ -z "${LATEST}" ]]; then
    echo "[wrapper] FATAL: no intermediate files found" >> "${SW5_DIR}/benchmark.log"
    exit 1
  fi
  case "${LATEST##*.}" in
    yml|yaml)
      cp "${LATEST}" "${OUT_FILE}"
      ;;
    *)
      {
        echo "# Generated workflow (fallback synthesis)"
        echo "# Source: ${LATEST##*/}"
        echo "# SW5 final step failed: exit=${SW_EXIT}"
        echo ""
        cat "${LATEST}"
      } > "${OUT_FILE}"
      ;;
  esac
  echo "[wrapper] synthesized workflow.yml from ${LATEST##*/}" >> "${SW5_DIR}/benchmark.log"
fi
mkdir -p "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/meta"

# Auto-fix common YAML issues (unquoted GWT expressions, inline save_to arrays)
if [[ -f "${OUT_FILE}" ]] && command -v python3 >/dev/null 2>&1; then
  python3 "${REPO}/scripts/meta-v6/fix-yaml.py" "${OUT_FILE}" >> "${SW5_DIR}/benchmark.log" 2>&1 || echo "[wrapper] WARNING: fix-yaml.py failed" >> "${SW5_DIR}/benchmark.log"
fi

cp "${OUT_FILE}" "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/meta/generated-workflow.yml"
echo "SW5_DONE: ${SW5_RUN_ID} (workflow.yml=$(wc -c < "${OUT_FILE}") bytes, sw5_exit=${SW_EXIT})"
