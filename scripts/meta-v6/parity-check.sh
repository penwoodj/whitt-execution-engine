#!/bin/bash
# scripts/meta-v6/parity-check.sh
# Scores generated workflow + execution outputs against 10 criteria
# Per docs/plans/meta-workflow-parity/02-VALIDATION-CRITERIA.md

set -uo pipefail

WORKFLOW="${1:-}"
EXEC_DIR="${2:-}"
PROMPT_FILE="${3:-}"

if [ -z "$WORKFLOW" ] || [ ! -f "$WORKFLOW" ]; then
  echo "Usage: $0 <workflow.yml> [exec-output-dir] [prompt-file]"
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
echo "## C1: YAML parses"
if python3 -c "import yaml; yaml.safe_load(open('$WORKFLOW'))" 2>/dev/null; then
  echo "PASS"; C1=5
else
  echo "FAIL"; C1=0
fi

echo ""
echo "## C2: No refusal patterns in workflow"
REFUSE_COUNT=$(count "I cannot|I'm unable|I don't have access|As an AI")
if [ "$REFUSE_COUNT" -eq 0 ]; then
  echo "PASS"; C2=5
else
  echo "FAIL: $REFUSE_COUNT refusal patterns"; C2=0
fi

echo ""
echo "## C3: No direct file access requests"
READ_COUNT=$(count 'Read [a-z_{]')
if [ "$READ_COUNT" -eq 0 ]; then
  echo "PASS"; C3=5
else
  echo "WARN: $READ_COUNT patterns"; C3=$((5 - READ_COUNT)); [ $C3 -lt 0 ] && C3=0
fi

echo ""
echo "## C4: Uses shell hooks for file I/O"
SHELL_COUNT=$(count "^[[:space:]]+- shell:")
if [ "$SHELL_COUNT" -ge 1 ]; then
  echo "PASS: $SHELL_COUNT shell hooks"; C4=5
else
  echo "INFO: no shell hooks"; C4=3
fi

echo ""
echo "## C5: All steps well-formed"
STEP_COUNT=$(count "^[[:space:]]+step_[a-z_0-9]+:")
GEN_ENTITY_COUNT=$(count "generative_entity:")
PROMPT_COUNT=$(count "prompt: \|")
if [ "$STEP_COUNT" -gt 0 ] && [ "$GEN_ENTITY_COUNT" -ge "$STEP_COUNT" ] && [ "$PROMPT_COUNT" -ge "$STEP_COUNT" ]; then
  echo "PASS: $STEP_COUNT steps"; C5=5
else
  echo "FAIL: $STEP_COUNT steps, $GEN_ENTITY_COUNT entities"; C5=2
fi

if [ -n "$EXEC_DIR" ] && [ -d "$EXEC_DIR" ]; then
  echo ""
  echo "## C6: Output files substantive (>100 bytes)"
  OUTPUT_FILES=$(find "$EXEC_DIR" -type f -size +100c 2>/dev/null | wc -l)
  if [ "$OUTPUT_FILES" -ge 3 ]; then
    echo "PASS: $OUTPUT_FILES output files"; C6=5
  elif [ "$OUTPUT_FILES" -ge 1 ]; then
    echo "PARTIAL: $OUTPUT_FILES output files"; C6=3
  else
    echo "FAIL: no output files"; C6=0
  fi

  echo ""
  echo "## C7: Output files free of refusal text"
  REFUSE_OUTPUTS=$(find "$EXEC_DIR" -type f -exec grep -lE "I cannot access|As an AI|I'm unable to" {} \; 2>/dev/null | wc -l)
  if [ "$REFUSE_OUTPUTS" -eq 0 ] && [ "$OUTPUT_FILES" -gt 0 ]; then
    echo "PASS"; C7=5
  elif [ "$OUTPUT_FILES" -eq 0 ]; then
    echo "NA: no outputs to check"; C7=0
  else
    RATIO=$(( (OUTPUT_FILES - REFUSE_OUTPUTS) * 5 / OUTPUT_FILES ))
    echo "PARTIAL: $REFUSE_OUTPUTS/$OUTPUT_FILES have refuses"; C7=$RATIO
  fi

  echo ""
  echo "## C8: Output content substantive (avg >500 bytes)"
  if [ "$OUTPUT_FILES" -gt 0 ]; then
    TOTAL_SIZE=$(find "$EXEC_DIR" -type f -size +100c -exec stat -c %s {} \; 2>/dev/null | awk '{s+=$1} END{print s}')
    AVG_SIZE=$((TOTAL_SIZE / OUTPUT_FILES))
    if [ "$AVG_SIZE" -ge 500 ]; then
      echo "PASS: avg $AVG_SIZE bytes"; C8=5
    elif [ "$AVG_SIZE" -ge 200 ]; then
      echo "PARTIAL: avg $AVG_SIZE bytes"; C8=3
    else
      echo "FAIL: avg $AVG_SIZE bytes"; C8=1
    fi
  else
    echo "NA"; C8=0
  fi

  echo ""
  echo "## C9: Code changes detected (src/ modified or new files)"
  SRC_CHANGED=0
  if [ -d "$EXEC_DIR/../../src" ]; then
    LAST_MIN=$(find "$EXEC_DIR/../../src" -type f -newer "$WORKFLOW" 2>/dev/null | wc -l)
    [ "$LAST_MIN" -gt 0 ] && SRC_CHANGED=1
  fi
  if grep -qE "(shell:.*cp |shell:.*cargo (check|build|test))" "$WORKFLOW" 2>/dev/null; then
    echo "PASS: workflow has cp/cargo shell hooks"; C9=5
  elif [ "$SRC_CHANGED" -eq 1 ]; then
    echo "PARTIAL: src/ modified but no cargo hook"; C9=3
  else
    echo "INFO: no code modification hooks"; C9=2
  fi

  echo ""
  echo "## C10: Verification artifacts (logs, benchmark)"
  LOGS=$(find "$EXEC_DIR" -name "*.log" -o -name "benchmark*" 2>/dev/null | wc -l)
  if [ "$LOGS" -ge 1 ]; then
    echo "PASS: $LOGS log files"; C10=5
  else
    echo "FAIL: no logs"; C10=0
  fi
else
  C6=0; C7=0; C8=0; C9=0; C10=0
fi

echo ""
echo "=== SUMMARY ==="
echo "C1 (yaml parses):     $C1/5"
echo "C2 (no refuses yaml): $C2/5"
echo "C3 (no read X):       $C3/5"
echo "C4 (shell hooks):     $C4/5"
echo "C5 (steps wellformed):$C5/5"
if [ -n "$EXEC_DIR" ] && [ -d "$EXEC_DIR" ]; then
  echo "C6 (outputs exist):   $C6/5"
  echo "C7 (no refuse out):   $C7/5"
  echo "C8 (substantive):     $C8/5"
  echo "C9 (code changes):    $C9/5"
  echo "C10 (verify trials):  $C10/5"
fi
TOTAL_STRUCT=$((C1+C2+C3+C4+C5))
TOTAL_EXEC=$((C6+C7+C8+C9+C10))
TOTAL=$((TOTAL_STRUCT + TOTAL_EXEC))
echo ""
echo "STRUCTURAL: $TOTAL_STRUCT/25"
if [ -n "$EXEC_DIR" ] && [ -d "$EXEC_DIR" ]; then
  echo "EXECUTION:  $TOTAL_EXEC/25"
  echo "TOTAL:      $TOTAL/50"
  if [ "$TOTAL" -ge 40 ]; then
    echo "VERDICT: PASS (parity achieved)"
  elif [ "$TOTAL" -ge 30 ]; then
    echo "VERDICT: PARTIAL"
  else
    echo "VERDICT: FAIL"
  fi
else
  echo "TOTAL:      $TOTAL_STRUCT/25 (structural only)"
  if [ "$TOTAL_STRUCT" -ge 20 ]; then
    echo "VERDICT: STRUCT_PASS"
  elif [ "$TOTAL_STRUCT" -ge 15 ]; then
    echo "VERDICT: STRUCT_PARTIAL"
  else
    echo "VERDICT: STRUCT_FAIL"
  fi
fi
