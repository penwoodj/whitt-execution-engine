#!/usr/bin/env bash
# Validate all 20 prompts: SW1-SW5 vs opencode baseline comparison.
# Usage: bash scripts/meta-v6/validate-all-20.sh
set -euo pipefail

REPO=/home/jon/code/whitt-execution-engine
BASE_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/baselines-opencode-same-model
META_BASE=$REPO/docs/benchmarks/outputs/meta-workflow
PARITY=$REPO/scripts/meta-v6/parity-check.sh

SW_WINS=0
BASE_WINS=0
TIES=0
TOTAL=0
declare -a SUMMARY=()

for p in 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24; do
  TOTAL=$((TOTAL+1))
  
  # Find SW deliverable (use latest run)
  SW_DELIV=""
  SW_WF=""
  SW_EXEC=""
  LATEST_DIR=$(ls -td $META_BASE/p${p}-final-* $META_BASE/p${p}-sw-pipeline-* 2>/dev/null | head -1)
  if [ -n "$LATEST_DIR" ] && [ -d "$LATEST_DIR" ]; then
    d="$LATEST_DIR"
    if [ -f "$d/deliverables/deliverable.md" ]; then
      SW_DELIV="$d/deliverables/deliverable.md"
      SW_WF="$d/meta/generated-workflow.yml"
      SW_EXEC="$d/exec-final"
      [ ! -d "$SW_EXEC" ] && SW_EXEC="$d/exec-v5"
      [ ! -d "$SW_EXEC" ] && SW_EXEC="$d/exec"
    elif [ -f "$d/deliverables/report.html" ]; then
      SW_DELIV="$d/deliverables/report.html"
      SW_WF="$d/meta/generated-workflow.yml"
      SW_EXEC="$d/exec"
    fi
  fi
  
  # Find baseline
  BASE_DELIV="$BASE_DIR/p${p}-baseline.md"
  
  if [ -z "$SW_DELIV" ] || [ ! -f "$SW_DELIV" ]; then
    SUMMARY+=("P$p: SW=MISSING base=$([ -f "$BASE_DELIV" ] && echo 'OK' || echo 'MISSING')")
    continue
  fi
  
  if [ ! -f "$BASE_DELIV" ]; then
    SUMMARY+=("P$p: SW=$(wc -c < "$SW_DELIV")B base=MISSING")
    continue
  fi
  
  # Get SW score
  SW_OUTPUT=$(bash $PARITY "$SW_WF" "$SW_DELIV" "$SW_EXEC" 2>&1 || true)
  SW_SCORE=$(echo "$SW_OUTPUT" | grep "=== Score:" | grep -oE '[0-9]+/50' | head -1)
  SW_SCORE_NUM=$(echo "$SW_SCORE" | cut -d/ -f1)
  
  # Get baseline metrics (no parity-check for baseline, just size + refusal)
  BASE_SIZE=$(wc -c < "$BASE_DELIV")
  BASE_REFUSAL=$(grep -ciE 'I cannot|I can.t help|as an ai|I.m unable to' "$BASE_DELIV" 2>/dev/null || echo 0)
  
  SW_SIZE=$(wc -c < "$SW_DELIV")
  SW_REFUSAL=$(grep -ciE 'I cannot|I can.t help|as an ai|I.m unable to' "$SW_DELIV" 2>/dev/null || echo 0)
  
  # Determine winner
  # SW wins if: parity score >= 40 AND (SW size >= base size OR base has refusals)
  if [ "$SW_SCORE_NUM" -ge 40 ] 2>/dev/null; then
    if [ "$BASE_REFUSAL" -gt 0 ] && [ "$SW_REFUSAL" -eq 0 ]; then
      VERDICT="SW_WINS (base has refusals)"
      SW_WINS=$((SW_WINS+1))
    elif [ "$SW_SIZE" -ge "$BASE_SIZE" ]; then
      VERDICT="SW_WINS (larger)"
      SW_WINS=$((SW_WINS+1))
    elif [ "$SW_SCORE_NUM" -ge 45 ]; then
      VERDICT="SW_WINS (high score)"
      SW_WINS=$((SW_WINS+1))
    else
      VERDICT="BASE_WINS (larger baseline)"
      BASE_WINS=$((BASE_WINS+1))
    fi
  else
    VERDICT="BASE_WINS (SW score low)"
    BASE_WINS=$((BASE_WINS+1))
  fi
  
  SUMMARY+=("P$p: SW=${SW_SIZE}B/${SW_SCORE}/refusals=${SW_REFUSAL} vs BASE=${BASE_SIZE}B/refusals=${BASE_REFUSAL} → $VERDICT")
done

echo "==============================================="
echo "20-PROMPT VALIDATION RESULTS"
echo "==============================================="
echo ""
for s in "${SUMMARY[@]}"; do
  echo "  $s"
done
echo ""
echo "==============================================="
echo "AGGREGATE"
echo "==============================================="
echo "  Total prompts: $TOTAL"
echo "  SW1-SW5 wins:  $SW_WINS"
echo "  Baseline wins: $BASE_WINS"
echo ""
if [ "$SW_WINS" -ge 15 ]; then
  echo "  ✅ TARGET MET: SW1-SW5 wins ≥15/20"
elif [ "$SW_WINS" -ge 10 ]; then
  echo "  ⚠️ PARTIAL: SW1-SW5 wins ${SW_WINS}/20 (need 15)"
else
  echo "  ❌ BELOW TARGET: SW1-SW5 wins ${SW_WINS}/20 (need 15)"
fi
