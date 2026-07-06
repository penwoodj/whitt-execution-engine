#!/bin/bash
# META-v6 SW1 wrapper: runs task-deconstruction sub-workflow
# Resilient: if SW1 step_05 fails, synthesizes tasks.md from step_04 output
# Usage: run-sw1.sh [META_RUN_ID] [PROMPT_FILE]
set -uo pipefail  # NOT -e: we handle errors manually

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/env.sh"
META_RUN_ID="${1:-$(cat "${REPO}/.current-meta-run")}"
PROMPT_FILE="${2:-${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/prompt.txt}"
SW1_RUN_ID="meta-${META_RUN_ID}-sw1-$(date +%Y%m%d-%H%M%S)"
SW1_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW1_RUN_ID}"

mkdir -p "${SW1_DIR}"/{sw1,logs,input}
cp "${PROMPT_FILE}" "${SW1_DIR}/input/prompt.txt"
sed "s/__RUN_ID__/${SW1_RUN_ID}/g" "${REPO}/docs/benchmarks/workflows/sw1-task-deconstruction.yml" > "${SW1_DIR}/sw1-runtime.yml"
localize_workflow "${SW1_DIR}/sw1-runtime.yml" "${SW1_DIR}/sw1-runtime.yml"

cd "${REPO}"
./target/release/whitt benchmark \
  --workflow "${SW1_DIR}/sw1-runtime.yml" \
  --output-dir "${SW1_DIR}" \
  $(model_flags) \
  --load-timeout 1800 \
  > "${SW1_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

# Locate tasks.md OR synthesize from intermediate output
TASKS_FILE="${SW1_DIR}/sw1/tasks.md"
if [[ ! -f "${TASKS_FILE}" ]]; then
  echo "[wrapper] tasks.md missing, attempting fallback synthesis" >> "${SW1_DIR}/benchmark.log"
  CORRECTED="${SW1_DIR}/sw1/04-corrected.md"
  INITIAL="${SW1_DIR}/sw1/01-initial.txt"
  EXPANDED="${SW1_DIR}/sw1/02-expanded.md"
  SRC=""
  if [[ -f "${CORRECTED}" ]]; then
    SRC="${CORRECTED}"
  elif [[ -f "${EXPANDED}" ]]; then
    SRC="${EXPANDED}"
  elif [[ -f "${INITIAL}" ]]; then
    SRC="${INITIAL}"
  fi
  if [[ -n "${SRC}" && -f "${SRC}" ]]; then
    {
      echo "# Task Breakdown"
      echo
      echo -n "**Source prompt**: "
      head -c 200 "${PROMPT_FILE}"
      echo "..."
      echo
      echo "## Tasks"
      echo
      cat "${SRC}"
      echo
      echo "## Generation Method"
      echo "Fallback synthesis from ${SRC##*/} (SW1 step_05 failed: exit=${SW_EXIT})"
    } > "${TASKS_FILE}"
    echo "[wrapper] synthesized tasks.md from ${SRC##*/} ($(wc -c < "${TASKS_FILE}") bytes)" >> "${SW1_DIR}/benchmark.log"
  else
    echo "[wrapper] FATAL: no source files found for synthesis" >> "${SW1_DIR}/benchmark.log"
    exit 1
  fi
fi

# Always copy to META input if tasks.md exists
if [[ -f "${TASKS_FILE}" ]]; then
  mkdir -p "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input"
  cp "${TASKS_FILE}" "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/tasks.md"
  BYTES=$(wc -c < "${TASKS_FILE}")
  echo "SW1_DONE: ${SW1_RUN_ID} (tasks.md=${BYTES} bytes, sw1_exit=${SW_EXIT})"
  exit 0
fi

echo "SW1_FAILED: ${SW1_RUN_ID} (no tasks.md produced)"
exit 1
