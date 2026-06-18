#!/bin/bash
# META-v6 pipeline summary: extract token efficiency, iteration counts, quality metrics.
# Usage: pipeline-summary.sh <META_RUN_ID>
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
META_RUN_ID="${1:?Usage: pipeline-summary.sh <META_RUN_ID>}"
META_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"

if [[ ! -d "${META_DIR}" ]]; then
  echo "FATAL: META dir not found: ${META_DIR}"
  exit 1
fi

echo "=== META-v6 PIPELINE SUMMARY ==="
echo "Run ID: ${META_RUN_ID}"
echo "Date: $(date)"
echo ""

echo "=== PER-SW METRICS ==="
printf "%-6s %-12s %-10s %-10s %-12s %-10s\n" "SW" "Duration" "Tokens" "Tok/sec" "Iterations" "Output"
printf "%-6s %-12s %-10s %-10s %-12s %-10s\n" "--" "--------" "------" "-------" "----------" "------"

for SW in sw1 sw2 sw3 sw4 sw5; do
  SW_DIR=$(ls -d "${REPO}"/docs/benchmarks/outputs/meta-workflow/meta-${META_RUN_ID}-${SW}-* 2>/dev/null | tail -1)
  if [[ ! -d "${SW_DIR}" ]]; then
    printf "%-6s %-12s %-10s %-10s %-12s %-10s\n" "$SW" "SKIPPED" "-" "-" "-" "-"
    continue
  fi

  LOG="${SW_DIR}/benchmark.log"

  # Total duration (ms)
  DURATIONS=$(grep -oE 'duration_ms=[0-9]+' "${LOG}" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END {print s}')
  DURATION_SEC=$((DURATIONS / 1000))

  # Total tokens generated (JSON format: "token_count":N)
  TOKENS=$(grep -oE '"token_count":[0-9]+' "${LOG}" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END {print s}')

  # Token efficiency
  if [[ -n "$TOKENS" && "$DURATION_SEC" -gt 0 ]]; then
    TPS=$((TOKENS / DURATION_SEC))
  else
    TPS=0
  fi

  # Iterations (count evaluator runs)
  EVALS=$(grep -c 'step_0[34]_evaluate\|step_04_evaluate\|step_03_evaluate' "${LOG}" 2>/dev/null || echo 0)
  FIXES=$(grep -c 'routing from step.*to step_0[45]_fix\|routing from step.*to step_05_fix' "${LOG}" 2>/dev/null || echo 0)

  # Output file size
  case $SW in
    sw1) OUT_FILE="${SW_DIR}/sw1/tasks.md" ;;
    sw2) OUT_FILE="${SW_DIR}/sw2/outputs.md" ;;
    sw3) OUT_FILE="${SW_DIR}/sw3/categories.md" ;;
    sw4) OUT_FILE="${SW_DIR}/sw4/structs.md" ;;
    sw5) OUT_FILE="${SW_DIR}/sw5/workflow.yml" ;;
  esac

  if [[ -f "${OUT_FILE}" ]]; then
    LINES=$(wc -l < "${OUT_FILE}")
    OUT_METRIC="${LINES}L"
  else
    OUT_METRIC="missing"
  fi

  printf "%-6s %-12s %-10s %-10s %-12s %-10s\n" \
    "$SW" "${DURATION_SEC}s" "${TOKENS}" "${TPS}tps" "${EVALS}e/${FIXES}f" "${OUT_METRIC}"
done

echo ""
echo "=== PIPELINE TOTALS ==="
TOTAL_DUR=0
TOTAL_TOK=0
for SW in sw1 sw2 sw3 sw4 sw5; do
  SW_DIR=$(ls -d "${REPO}"/docs/benchmarks/outputs/meta-workflow/meta-${META_RUN_ID}-${SW}-* 2>/dev/null | tail -1)
  [[ ! -d "${SW_DIR}" ]] && continue
  LOG="${SW_DIR}/benchmark.log"
  D=$(grep -oE 'duration_ms=[0-9]+' "${LOG}" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END {print s}')
  T=$(grep -oE '"token_count":[0-9]+' "${LOG}" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END {print s}')
  TOTAL_DUR=$((TOTAL_DUR + ${D:-0}))
  TOTAL_TOK=$((TOTAL_TOK + ${T:-0}))
done
TOTAL_DUR_SEC=$((TOTAL_DUR / 1000))
echo "Total duration: ${TOTAL_DUR_SEC}s ($(( TOTAL_DUR_SEC / 60 ))m $(( TOTAL_DUR_SEC % 60 ))s)"
echo "Total tokens: ${TOTAL_TOK}"
if [[ $TOTAL_DUR_SEC -gt 0 ]]; then
  echo "Avg token rate: $(( TOTAL_TOK / TOTAL_DUR_SEC )) tps"
fi

echo ""
echo "=== FINAL OUTPUT ==="
WF="${META_DIR}/meta/generated-workflow.yml"
if [[ -f "${WF}" ]]; then
  LINES=$(wc -l < "${WF}")
  BYTES=$(wc -c < "${WF}")
  STEPS=$(grep -c 'step_' "${WF}" 2>/dev/null || echo 0)
  HOOKS=$(grep -c -e 'save_to:' -e 'log:' -e 'gwt:' -e 'shell:' "${WF}" 2>/dev/null || echo 0)
  echo "workflow.yml: ${LINES}L, ${BYTES}B"
  echo "Steps: ${STEPS}"
  echo "Hooks: ${HOOKS}"
  if python3 -c "import yaml; yaml.safe_load(open('${WF}'))" 2>/dev/null; then
    echo "YAML validity: VALID"
  else
    echo "YAML validity: INVALID"
  fi
else
  echo "No final workflow.yml found"
fi

echo ""
echo "=== END SUMMARY ==="
