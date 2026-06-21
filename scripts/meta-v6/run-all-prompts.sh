#!/bin/bash
# Run all active test prompts through meta-v6 pipeline.
# For each: run meta-v6 → validate SW5 workflow.yml → try whitt workflow load.
# Usage: run-all-prompts.sh [--start N] [--end M] [--only NN]
# Output: docs/benchmarks/outputs/meta-workflow/all-prompts-results-<timestamp>.md
set -uo pipefail  # NOT -e: we want to continue on failures

REPO="/home/jon/code/whitt-execution-engine"
cd "${REPO}"

PROMPTS_DIR="docs/plans/meta-workflow-qwen35/test-prompts/real"
RESULTS_DIR="docs/benchmarks/outputs/meta-workflow"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
RESULTS_FILE="${RESULTS_DIR}/all-prompts-results-${TIMESTAMP}.md"

# Parse args
START=5
END=15
ONLY=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --start) START="$2"; shift 2 ;;
    --end) END="$2"; shift 2 ;;
    --only) ONLY="$2"; shift 2 ;;
    *) echo "Unknown arg: $1" >&2; exit 1 ;;
  esac
done

mkdir -p "${RESULTS_DIR}"

# Start results file
cat > "${RESULTS_FILE}" <<EOF
# All Prompts Test Results

**Started:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Range:** prompts ${START}-${END}
**Pipeline:** meta-workflow-v6.yml @ gpu_layers=99

| # | Prompt | Status | SW1-SW5 iter | Output lines | YAML valid | whitt loads | Notes |
|---|--------|--------|--------------|--------------|-----------|-------------|-------|
EOF

echo "=========================================="
echo "META-V6 ALL PROMPTS TEST RUN"
echo "Range: ${START}-${END} (only: ${ONLY:-all})"
echo "Results: ${RESULTS_FILE}"
echo "=========================================="

run_one_prompt() {
  local num="$1"
  local prompt_file="$2"
  local short_name=$(basename "$prompt_file" .md | head -c 60)

  echo ""
  echo "--- PROMPT ${num}: ${short_name} ---"

  local RUN_ID="meta-allprompts-${num}-$(date +%Y%m%d-%H%M%S)"
  local OUT_DIR="${REPO}/${RESULTS_DIR}/${RUN_ID}"
  mkdir -p "${OUT_DIR}/logs"

  # Edit meta-v6.yml prompt path inline by using env substitution? Simpler: use a wrapper.
  # Actually bootstrap.sh takes prompt as $1 — but meta-v6.yml hardcodes it.
  # Workaround: use sed to create runtime version
  local RUNTIME_YML="${OUT_DIR}/meta-v6-runtime.yml"
  sed "s|docs/plans/meta-workflow-qwen35/test-prompts/real/prompt-[0-9]*-[a-z0-9-]*\.md|${prompt_file}|g" \
    "${REPO}/docs/benchmarks/workflows/meta-workflow-v6.yml" > "${RUNTIME_YML}"

  local START_TIME=$(date +%s)

  # Run pipeline (foreground, will block ~45 min)
  ./target/release/whitt benchmark \
    --workflow "${RUNTIME_YML}" \
    --output-dir "${OUT_DIR}" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${OUT_DIR}/meta-benchmark.log" 2>&1
  local WHITT_EXIT=$?

  local END_TIME=$(date +%s)
  local DURATION=$((END_TIME - START_TIME))

  # Locate generated workflow
  local SW5_DIR=$(ls -d ${OUT_DIR}-sw5-* 2>/dev/null | head -1)
  local WORKFLOW_YML="${SW5_DIR}/sw5/workflow.yml"

  local STATUS="FAIL"
  local ITER_INFO="-"
  local LINES=0
  local YAML_VALID="no"
  local WHITT_LOADS="no"
  local NOTES=""

  if [[ $WHITT_EXIT -ne 0 ]]; then
    NOTES="meta-v6 exit ${WHITT_EXIT}"
  elif [[ ! -f "${WORKFLOW_YML}" ]]; then
    NOTES="workflow.yml missing"
  else
    LINES=$(wc -l < "${WORKFLOW_YML}")

    # YAML validity
    if python3 -c "import yaml; yaml.safe_load(open('${WORKFLOW_YML}'))" 2>/dev/null; then
      YAML_VALID="yes"
    else
      YAML_VALID="no"
      NOTES="yaml parse failed"
    fi

    # whitt workflow load test (only if YAML valid)
    if [[ "$YAML_VALID" == "yes" ]]; then
      if ./target/release/whitt workflow "${WORKFLOW_YML}" > "${OUT_DIR}/whitt-load.log" 2>&1; then
        WHITT_LOADS="yes"
      else
        WHITT_LOADS="no"
        NOTES="whitt workflow load failed: $(tail -1 ${OUT_DIR}/whitt-load.log | head -c 100)"
      fi
    fi

    # SW iter counts
    local ITER_INFO=""
    for sw in 1 2 3 4 5; do
      local SW_D=$(ls -d ${OUT_DIR}-sw${sw}-* 2>/dev/null | head -1)
      local ITER=$(cat "${SW_D}/sw${sw}/iteration-counter.txt" 2>/dev/null || echo "?")
      ITER_INFO="${ITER_INFO}${sw}=${ITER} "
    done

    if [[ "$YAML_VALID" == "yes" && "$WHITT_LOADS" == "yes" ]]; then
      STATUS="PASS"
      NOTES="duration=${DURATION}s"
    else
      STATUS="FAIL"
      NOTES="${NOTES} duration=${DURATION}s"
    fi
  fi

  echo "  STATUS=${STATUS} iter=[${ITER_INFO}] lines=${LINES} yaml=${YAML_VALID} whitt=${WHITT_LOADS}"

  # Append to results file
  echo "| ${num} | ${short_name} | ${STATUS} | ${ITER_INFO} | ${LINES} | ${YAML_VALID} | ${WHITT_LOADS} | ${NOTES} |" >> "${RESULTS_FILE}"
}

# Iterate prompts
for num in $(seq -w ${START} ${END}); do
  # Only single prompt mode
  if [[ -n "${ONLY}" && "${ONLY}" != "${num}" ]]; then
    continue
  fi

  # Find prompt file (may have varying suffixes)
  prompt_file=$(ls ${PROMPTS_DIR}/prompt-${num}-*.md 2>/dev/null | head -1)
  if [[ -z "${prompt_file}" ]]; then
    echo "Prompt ${num}: no file found, skipping"
    echo "| ${num} | (not found) | SKIP | - | - | - | - | no file |" >> "${RESULTS_FILE}"
    continue
  fi

  run_one_prompt "${num}" "${prompt_file}"
done

# Summary
echo ""
echo "=========================================="
echo "TEST RUN COMPLETE"
echo "=========================================="
PASS_COUNT=$(grep -c "| PASS |" "${RESULTS_FILE}" || echo 0)
FAIL_COUNT=$(grep -c "| FAIL |" "${RESULTS_FILE}" || echo 0)
SKIP_COUNT=$(grep -c "| SKIP |" "${RESULTS_FILE}" || echo 0)
echo "PASS: ${PASS_COUNT}  FAIL: ${FAIL_COUNT}  SKIP: ${SKIP_COUNT}"
echo "Full results: ${RESULTS_FILE}"

# Append summary to file
cat >> "${RESULTS_FILE}" <<EOF

## Summary

- **PASS:** ${PASS_COUNT}
- **FAIL:** ${FAIL_COUNT}
- **SKIP:** ${SKIP_COUNT}
- **Completed:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
EOF

echo "DONE"
