#!/bin/bash
# scripts/meta-v6/run-remaining-cycle3.sh
# Runs P12, P13, P15 sequentially with corrected prompt paths.
# Logs progress to /tmp/cycle3-remaining-progress.log
# Self-contained — no external dependency.

set -uo pipefail
REPO="/home/jon/code/whitt-execution-engine"
LOG="/tmp/cycle3-remaining-progress.log"

echo "=== Cycle 3 remaining prompts start: $(date) ===" > "$LOG"

run_one() {
  local n="$1"
  echo "" >> "$LOG"
  echo "=== [P${n}] start: $(date) ===" >> "$LOG"

  cd "$REPO"
  bash scripts/meta-v6/run-prompt-end-to-end.sh "$n" >> "$LOG" 2>&1
  local rc=$?

  echo "[P${n}] exit=${rc} at $(date)" >> "$LOG"

  # Extract result
  local score_file="docs/plans/meta-workflow-parity/cycle-3-results/prompt-${n}/exec-score.txt"
  if [ -f "$score_file" ]; then
    local total=$(grep "^TOTAL:" "$score_file" | awk '{print $2}')
    local verdict=$(grep "^VERDICT:" "$score_file" | awk '{$1=""; print $0}' | sed 's/^ *//')
    echo "[P${n}] RESULT: total=${total} verdict=${verdict}" >> "$LOG"
  fi
}

for n in 12 13 15; do
  run_one "$n"
done

echo "" >> "$LOG"
echo "=== ALL DONE: $(date) ===" >> "$LOG"
