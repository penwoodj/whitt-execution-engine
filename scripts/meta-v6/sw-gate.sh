#!/bin/bash
# SW evaluator gate: PASS if verdict=PASS OR iteration >= 4 (cap).
# Usage: sw-gate.sh <eval_file> <counter_file>
set -uo pipefail

EVAL_FILE="$1"
COUNTER_FILE="$2"

ITER=$(cat "$COUNTER_FILE" 2>/dev/null || echo 0)
ITER=$((ITER + 1))
echo "$ITER" > "$COUNTER_FILE"

if grep -qi 'PASS' "$EVAL_FILE" 2>/dev/null || [ "$ITER" -ge 4 ]; then
  RESULT=PASS
else
  RESULT=FAIL
fi

# Debug: capture args + result
echo "[$(date -u +%T)] args=$* ITER=$ITER RESULT=$RESULT cwd=$(pwd)" >> ./sw-gate-debug.log

printf '%s' "$RESULT"


