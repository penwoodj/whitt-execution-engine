#!/bin/bash
# META-v6 generic SW wrapper: runs SW<N> sub-workflow for the active meta run.
# Replaces run-sw1.sh .. run-sw5.sh (5 scripts → 1).
#
# Per-SW config is table-driven (see case block below).
# SW1-4: LLM-based, with fallback synthesis if final output missing.
# SW5: deterministic (YAML invokes build-workflow.py via shell hooks).
#
# Usage:
#   run-sw.sh <sw-num> [meta-run-id] [prompt-file]
#
# Output:
#   docs/benchmarks/outputs/meta-workflow/<SW_RUN_ID>/sw<N>/<output-file>
#   Copies result to <META_RUN_ID>/input/<output-file> for next SW.
set -uo pipefail  # NOT -e: we handle errors manually

SW_NUM="${1:?Usage: run-sw.sh <sw-num> [meta-run-id] [prompt-file]}"
REPO="/home/jon/code/whitt-execution-engine"
META_RUN_ID="${2:-$(cat "${REPO}/.current-meta-run" 2>/dev/null || echo "meta-default")}"
PROMPT_FILE="${3:-${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/input/prompt.txt}"

# --- Per-SW configuration table ---------------------------------------------
# Fields per SW:
#   WORKFLOW_YML  : source workflow YAML name in docs/benchmarks/workflows/
#   OUTPUT_FILE   : expected final artifact name (relative to sw<N>/)
#   INPUT_FILES   : space-separated list of files to copy FROM <META>/input/ TO <SW>/input/
#   FALLBACK      : "synth-priority" (SW1), "synth-latest" (SW2-4), "none" (SW5)
#   SYNTH_HEADER  : header text for synthesized output (when fallback triggers)
#   SYNTH_SECTION : section header for synthesized output
case "$SW_NUM" in
  1)
    WORKFLOW_YML="sw1-task-deconstruction.yml"
    OUTPUT_FILE="tasks.md"
    INPUT_FILES="prompt.txt"
    FALLBACK="synth-priority"
    SYNTH_HEADER="# Task Breakdown"
    SYNTH_SECTION="## Tasks"
    ;;
  2)
    WORKFLOW_YML="sw2-desired-output-state.yml"
    OUTPUT_FILE="outputs.md"
    INPUT_FILES="tasks.md"
    FALLBACK="synth-latest"
    SYNTH_HEADER="# Desired Output State"
    SYNTH_SECTION="## Outputs"
    ;;
  3)
    WORKFLOW_YML="sw3-agentic-categorization.yml"
    OUTPUT_FILE="categories.md"
    INPUT_FILES="outputs.md tasks.md"
    FALLBACK="synth-latest"
    SYNTH_HEADER="# Agentic Categorization"
    SYNTH_SECTION="## Categories"
    ;;
  4)
    WORKFLOW_YML="sw4-yaml-substructure-translation.yml"
    OUTPUT_FILE="structs.md"
    INPUT_FILES="categories.md"
    FALLBACK="synth-latest"
    SYNTH_HEADER="# YAML Substructures"
    SYNTH_SECTION="## Structs"
    ;;
  5)
    WORKFLOW_YML="sw5-final-workflow-assembly.yml"
    OUTPUT_FILE="generated-workflow.yml"
    INPUT_FILES="structs.md prompt.txt"
    FALLBACK="none"
    SYNTH_HEADER=""
    SYNTH_SECTION=""
    ;;
  *)
    echo "FATAL: invalid SW number '$SW_NUM' (expected 1-5)" >&2
    exit 2
    ;;
esac

META_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"
SW_RUN_ID="meta-${META_RUN_ID}-sw${SW_NUM}-$(date +%Y%m%d-%H%M%S)"
SW_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${SW_RUN_ID}"
SW_SUBDIR="${SW_DIR}/sw${SW_NUM}"

mkdir -p "${SW_SUBDIR}" "${SW_DIR}/logs" "${SW_DIR}/input"

for f in $INPUT_FILES; do
  SRC="${META_DIR}/input/${f}"
  if [[ -f "${SRC}" ]]; then
    cp "${SRC}" "${SW_DIR}/input/${f}"
  elif [[ "${f}" = "prompt.txt" && -f "${PROMPT_FILE}" ]]; then
    cp "${PROMPT_FILE}" "${SW_DIR}/input/prompt.txt"
  else
    echo "[sw${SW_NUM}] WARN: input file '${f}' missing" >&2
  fi
done

sed -e "s/__RUN_ID__/${SW_RUN_ID}/g" -e "s/__META_RUN_ID__/${META_RUN_ID}/g" \
  "${REPO}/docs/benchmarks/workflows/${WORKFLOW_YML}" \
  > "${SW_DIR}/sw${SW_NUM}-runtime.yml"

cd "${REPO}"
./target/release/whitt benchmark \
  --workflow "${SW_DIR}/sw${SW_NUM}-runtime.yml" \
  --output-dir "${SW_DIR}" \
  --models-dir "${REPO}/models" \
  --filter-name "Qwen3-5-9B" \
  --load-timeout 1800 \
  > "${SW_DIR}/benchmark.log" 2>&1
SW_EXIT=$?

# Locate final output or run fallback synthesis
FINAL="${SW_SUBDIR}/${OUTPUT_FILE}"
if [[ ! -f "${FINAL}" ]]; then
  echo "[sw${SW_NUM}] ${OUTPUT_FILE} missing, attempting fallback (${FALLBACK})" >> "${SW_DIR}/benchmark.log"

  case "${FALLBACK}" in
    none)
      echo "[sw${SW_NUM}] FATAL: no fallback for SW${SW_NUM} (exit=${SW_EXIT})" >> "${SW_DIR}/benchmark.log"
      echo "SW${SW_NUM}_FAILED: ${SW_RUN_ID} (no ${OUTPUT_FILE}, no fallback)"
      exit 1
      ;;
    synth-priority)
      SRC=""
      for cand in "04-corrected.md" "02-expanded.md" "01-initial.txt"; do
        if [[ -f "${SW_SUBDIR}/${cand}" ]]; then
          SRC="${SW_SUBDIR}/${cand}"
          break
        fi
      done
      ;;
    synth-latest)
      SRC=$(ls -1 "${SW_SUBDIR}"/0*.{md,txt} 2>/dev/null | sort | tail -1)
      ;;
  esac

  if [[ -z "${SRC}" || ! -f "${SRC}" ]]; then
    echo "[sw${SW_NUM}] FATAL: no source files found for synthesis" >> "${SW_DIR}/benchmark.log"
    echo "SW${SW_NUM}_FAILED: ${SW_RUN_ID} (no ${OUTPUT_FILE}, no source for synth)"
    exit 1
  fi

  {
    echo "${SYNTH_HEADER}"
    echo
    if [[ -f "${PROMPT_FILE}" ]]; then
      echo -n "**Source prompt**: "
      head -c 200 "${PROMPT_FILE}"
      echo "..."
      echo
    fi
    echo "${SYNTH_SECTION}"
    echo
    cat "${SRC}"
    echo
    echo "## Generation Method"
    echo "Fallback synthesis from ${SRC##*/} (SW${SW_NUM} exit=${SW_EXIT})"
  } > "${FINAL}"
  echo "[sw${SW_NUM}] synthesized ${OUTPUT_FILE} from ${SRC##*/} ($(wc -c < "${FINAL}") bytes)" >> "${SW_DIR}/benchmark.log"
fi

# Propagate result to META input dir for next SW
mkdir -p "${META_DIR}/input"
cp "${FINAL}" "${META_DIR}/input/${OUTPUT_FILE}"

BYTES=$(wc -c < "${FINAL}")
echo "SW${SW_NUM}_DONE: ${SW_RUN_ID} (${OUTPUT_FILE}=${BYTES} bytes, sw_exit=${SW_EXIT})"
exit 0
