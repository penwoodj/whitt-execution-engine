#!/usr/bin/env bash
# Watch for fresh deliverables and auto-validate them.
# Runs parity-check + quality comparison on each completed prompt.
# Updates ALL20-VALIDATION-REPORT.md with results.
#
# Usage: ./scripts/meta-v6/watch-and-validate.sh
set -uo pipefail

REPO=/home/jon/code/whitt-execution-engine
cd "$REPO"

BASE_DIR=docs/benchmarks/outputs/meta-workflow/baselines-opencode-same-model
REPORT=docs/plans/meta-workflow-parity/comparisons/ALL20-VALIDATION-REPORT.md

SW_WINS=0
BASE_WINS=0
TIES=0
TOTAL_DONE=0

echo "| Prompt | SW Size | Base Size | SW Score | Base Score | Verdict | Steps | |" > /tmp/validation-results.txt
echo "|--------|---------|-----------|----------|------------|---------|-------|-|" >> /tmp/validation-results.txt

for p_num in 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24; do
  echo "[validate] Checking P$p_num..."

  P_DIR=$(ls -td docs/benchmarks/outputs/meta-workflow/p${p_num}-fresh-20260629-1* 2>/dev/null | head -1)
  if [ -z "$P_DIR" ]; then
    echo "  ⏳ No fresh run yet"
    echo "| P$p_num | — | — | — | — | ⏳ QUEUED | — | |" >> /tmp/validation-results.txt
    continue
  fi

  DELIV="$P_DIR/deliverables/deliverable.md"
  if [ ! -f "$DELIV" ]; then
    STATUS=$(tail -1 "$P_DIR/pipeline.log" 2>/dev/null | head -c 40)
    echo "  🔄 Running: $STATUS"
    echo "| P$p_num | — | — | — | — | 🔄 RUNNING | — | |" >> /tmp/validation-results.txt
    continue
  fi

  TOTAL_DONE=$((TOTAL_DONE+1))
  SW_SIZE=$(wc -c < "$DELIV")

  BASELINE="$BASE_DIR/p${p_num}-baseline.md"
  if [ ! -f "$BASELINE" ]; then
    echo "  ⚠️ No baseline"
    echo "| P$p_num | ${SW_SIZE}B | — | — | — | ⚠️ NO BASE | — | |" >> /tmp/validation-results.txt
    continue
  fi
  BASE_SIZE=$(wc -c < "$BASELINE")

  # Quality comparison
  COMP_OUTPUT=$(bash scripts/meta-v6/compare-quality.sh "p${p_num}" "$DELIV" "$BASELINE" 2>&1)
  SW_SCORE=$(echo "$COMP_OUTPUT" | grep "SW1-SW5:" | grep -oE '[0-9]+' | head -1)
  BASE_SCORE=$(echo "$COMP_OUTPUT" | grep "opencode baseline:" | grep -oE '[0-9]+' | head -1)
  VERDICT=$(echo "$COMP_OUTPUT" | grep "^VERDICT:" | head -1 | sed 's/VERDICT: //')

  # Check multi-step (count step output files)
  STEP_COUNT=$(ls "$P_DIR"/exec*/outputs/*.txt 2>/dev/null | wc -l)

  case "$VERDICT" in
    *SW_WINS*) SW_WINS=$((SW_WINS+1)); ICON="✅";;
    *BASELINE_WINS*) BASE_WINS=$((BASE_WINS+1)); ICON="❌";;
    *TIE*) TIES=$((TIES+1)); ICON="🟰";;
    *) ICON="❓";;
  esac

  echo "  $ICON SW=${SW_SIZE}B/${SW_SCORE}pt vs BASE=${BASE_SIZE}B/${BASE_SCORE}pt — $VERDICT (${STEP_COUNT} step outputs)"
  echo "| P$p_num | ${SW_SIZE}B | ${BASE_SIZE}B | ${SW_SCORE} | ${BASE_SCORE} | ${ICON} ${VERDICT} | ${STEP_COUNT} | |" >> /tmp/validation-results.txt
done

echo ""
echo "========================================"
echo "VALIDATION SUMMARY"
echo "========================================"
echo "Completed: $TOTAL_DONE/20"
echo "SW Wins:   $SW_WINS"
echo "Base Wins: $BASE_WINS"
echo "Ties:      $TIES"
echo ""
if [ "$TOTAL_DONE" -gt 0 ]; then
  WIN_RATE=$((SW_WINS * 100 / TOTAL_DONE))
  echo "SW Win Rate: ${WIN_RATE}%"
  if [ "$WIN_RATE" -ge 50 ]; then
    echo "✅ SW1-SW5 SURPASSES baseline (≥50% win rate)"
  else
    echo "❌ SW1-SW5 does NOT surpass baseline (<50% win rate)"
  fi
fi
echo ""
echo "Results table in /tmp/validation-results.txt"
