#!/bin/bash
# scripts/meta-v6/parity-check.sh
# Scores a generated workflow against 5 criteria per docs/plans/meta-workflow-parity/02-VALIDATION-CRITERIA.md

set -uo pipefail

WORKFLOW="${1:-}"
EXEC_DIR="${2:-}"

if [ -z "$WORKFLOW" ] || [ ! -f "$WORKFLOW" ]; then
  echo "Usage: $0 <workflow.yml> [exec-output-dir]"
  exit 1
fi

count() {
  local pattern="$1"
  local n
  n=$(grep -cE "$pattern" "$WORKFLOW" 2>/dev/null)
  echo "${n:-0}"
}

echo "=== Parity Check: $WORKFLOW ==="

echo ""
echo "## Criterion 1: YAML parses"
if python3 -c "import yaml; yaml.safe_load(open('$WORKFLOW'))" 2>/dev/null; then
  echo "PASS: valid YAML"; C1=5
else
  echo "FAIL: invalid YAML"; C1=0
fi

echo ""
echo "## Criterion 2: Workflow has no refusal patterns"
REFUSE_COUNT=$(count "I cannot|I'm unable|I don't have access|As an AI")
if [ "$REFUSE_COUNT" -eq 0 ]; then
  echo "PASS: no refusal patterns in workflow"; C2=5
else
  echo "FAIL: $REFUSE_COUNT refusal patterns"; C2=0
fi

echo ""
echo "## Criterion 3: No direct file access requests"
READ_COUNT=$(count 'Read [a-z_{]')
if [ "$READ_COUNT" -eq 0 ]; then
  echo "PASS: no direct file read requests"; C3=5
else
  echo "WARN: $READ_COUNT 'Read X/' patterns"; C3=$((5 - READ_COUNT)); [ $C3 -lt 0 ] && C3=0
fi

echo ""
echo "## Criterion 4: Uses shell hooks for file I/O"
SHELL_COUNT=$(count "^[[:space:]]+- shell:")
if [ "$SHELL_COUNT" -ge 1 ]; then
  echo "PASS: $SHELL_COUNT shell hooks present"; C4=5
else
  echo "INFO: no shell hooks"; C4=3
fi

echo ""
echo "## Criterion 5: All steps well-formed"
STEP_COUNT=$(count "^[[:space:]]+step_[a-z_0-9]+:")
GEN_ENTITY_COUNT=$(count "generative_entity:")
PROMPT_COUNT=$(count "prompt: \|")
if [ "$STEP_COUNT" -gt 0 ] && [ "$GEN_ENTITY_COUNT" -ge "$STEP_COUNT" ] && [ "$PROMPT_COUNT" -ge "$STEP_COUNT" ]; then
  echo "PASS: $STEP_COUNT steps, $GEN_ENTITY_COUNT entities, $PROMPT_COUNT prompts"; C5=5
else
  echo "FAIL: $STEP_COUNT steps, $GEN_ENTITY_COUNT entities, $PROMPT_COUNT prompts"; C5=2
fi

if [ -n "$EXEC_DIR" ] && [ -d "$EXEC_DIR" ]; then
  echo ""
  echo "## Execution Output Audit"
  OUTPUT_FILES=$(find "$EXEC_DIR" -type f -size +100c 2>/dev/null | wc -l)
  REFUSE_OUTPUTS=$(find "$EXEC_DIR" -type f -exec grep -lE "I cannot access|As an AI" {} \; 2>/dev/null | wc -l)
  echo "Output files >100 bytes: $OUTPUT_FILES"
  echo "Files with refusal patterns: $REFUSE_OUTPUTS"
fi

echo ""
echo "=== SUMMARY ==="
echo "C1 (parses):      $C1/5"
echo "C2 (no refuses):  $C2/5"
echo "C3 (no read X):   $C3/5"
echo "C4 (shell hooks): $C4/5"
echo "C5 (well-formed): $C5/5"
TOTAL=$((C1+C2+C3+C4+C5))
echo "TOTAL: $TOTAL/25"
if [ "$TOTAL" -ge 20 ]; then
  echo "VERDICT: PASS"
elif [ "$TOTAL" -ge 15 ]; then
  echo "VERDICT: PARTIAL"
else
  echo "VERDICT: FAIL"
fi
