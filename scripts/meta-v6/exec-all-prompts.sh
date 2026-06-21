#!/bin/bash
# Execute all 11 generated workflows to confirm they run end-to-end.
# Validates: workflow can be loaded by whitt benchmark, executes without crash,
# produces output files for each step.
set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
TEST_BASE="/tmp/exec-all-prompts"
mkdir -p "${TEST_BASE}"

declare -A PROMPT_TS
PROMPT_TS[05]="113305"
PROMPT_TS[06]="122447"
PROMPT_TS[07]="131256"
PROMPT_TS[08]="140236"
PROMPT_TS[09]="144611"
PROMPT_TS[10]="153433"
PROMPT_TS[11]="162609"
PROMPT_TS[12]="171422"
PROMPT_TS[13]="180828"
PROMPT_TS[14]="185610"
PROMPT_TS[15]="192508"

RESULTS_FILE="${REPO}/docs/benchmarks/outputs/meta-workflow/exec-results-$(date +%Y%m%d-%H%M%S).md"

cat > "${RESULTS_FILE}" <<EOF
# Generated Workflow Execution Results

**Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Test:** Each SW5 output workflow.yml executed by whitt benchmark.

| # | Steps executed | Output files | Hook errors | Status | Notes |
|---|----------------|--------------|-------------|--------|-------|
EOF

echo "Executing 11 generated workflows..."

for num in 05 06 07 08 09 10 11 12 13 14 15; do
  ts=${PROMPT_TS[$num]}
  sw5dir=$(ls -d ${REPO}/docs/benchmarks/outputs/meta-workflow/meta-meta-v6-20260619-${ts}-sw5-* 2>/dev/null | head -1)
  src_wf="${sw5dir}/sw5/workflow.yml"

  if [ ! -f "$src_wf" ]; then
    echo "| ${num} | - | - | - | SKIP | workflow.yml missing |" >> "${RESULTS_FILE}"
    continue
  fi

  test_dir="${TEST_BASE}/prompt-${num}"
  rm -rf "$test_dir"
  mkdir -p "${test_dir}"/{outputs,logs,out}
  cp "$src_wf" "${test_dir}/workflow.yml"

  echo "  Running prompt-${num}..."

  cd "${test_dir}"
  /home/jon/code/whitt-execution-engine/target/release/whitt benchmark \
    --workflow workflow.yml \
    --output-dir ./out \
    --models-dir "${REPO}/models" \
    --filter-name Qwen3-5-9B \
    --load-timeout 900 \
    > run.log 2>&1
  whitt_exit=$?

  cd "${REPO}"

  steps_exec=$(grep -oE "executing step [a-z_0-9]+" "${test_dir}/run.log" 2>/dev/null | sort -u | wc -l)
  outputs_created=$(ls "${test_dir}/outputs/" 2>/dev/null | wc -l)
  hook_errors=$(grep -c "hook error\|hook_action.*FAIL\|Failed to deserialize hook" "${test_dir}/run.log" 2>/dev/null || echo 0)

  if [ $whitt_exit -eq 0 ] && [ "$outputs_created" -gt 0 ]; then
    status="PASS"
  else
    status="FAIL(exit=${whitt_exit})"
  fi

  echo "| ${num} | ${steps_exec} | ${outputs_created} | ${hook_errors} | ${status} | - |" >> "${RESULTS_FILE}"
done

PASS_COUNT=$(grep -c "| PASS |" "${RESULTS_FILE}" || echo 0)
FAIL_COUNT=$(grep -c "| FAIL" "${RESULTS_FILE}" || echo 0)

cat >> "${RESULTS_FILE}" <<EOF

## Summary

- **PASS:** ${PASS_COUNT}/11 (workflow executed end-to-end, produced outputs)
- **FAIL:** ${FAIL_COUNT}/11
EOF

echo ""
echo "=========================================="
echo "EXECUTION RESULTS: ${PASS_COUNT} PASS / ${FAIL_COUNT} FAIL of 11"
echo "Full report: ${RESULTS_FILE}"
