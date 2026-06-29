#!/usr/bin/env bash
# Compare SW1-SW5 pipeline deliverable vs opencode baseline (same model).
#
# Usage:
#   ./scripts/meta-v6/compare-prompt.sh <prompt_id> <sw_deliverable_path> <baseline_path>
#
# Outputs:
#   - File sizes (SW vs baseline)
#   - Line counts
#   - Refusal patterns check
#   - Verdict: SW_WINS / BASELINE_WINS / TIE
set -euo pipefail

PROMPT_ID="${1:?Usage: $0 <prompt_id> <sw_deliverable> <baseline>}"
SW_PATH="${2:?missing sw_deliverable path}"
BASELINE_PATH="${3:?missing baseline path}"

if [ ! -f "$SW_PATH" ]; then
  echo "FATAL: SW deliverable not found: $SW_PATH" >&2
  exit 1
fi
if [ ! -f "$BASELINE_PATH" ]; then
  echo "FATAL: baseline not found: $BASELINE_PATH" >&2
  exit 1
fi

SW_BYTES=$(wc -c < "$SW_PATH")
SW_LINES=$(wc -l < "$SW_PATH")
BASE_BYTES=$(wc -c < "$BASELINE_PATH")
BASE_LINES=$(wc -l < "$BASELINE_PATH")

# Refusal patterns (any case-insensitive match = -3 points each)
SW_REFUSAL=$(grep -ciE 'I cannot|I can.t help|as an ai|I.m unable to' "$SW_PATH" 2>/dev/null | head -1 || true)
SW_REFUSAL=${SW_REFUSAL:-0}
BASE_REFUSAL=$(grep -ciE 'I cannot|I can.t help|as an ai|I.m unable to' "$BASELINE_PATH" 2>/dev/null | head -1 || true)
BASE_REFUSAL=${BASE_REFUSAL:-0}

# Substantive content (>500 bytes = substantive)
SW_SUBST="NO"
[ "$SW_BYTES" -gt 500 ] && SW_SUBST="YES"
BASE_SUBST="NO"
[ "$BASE_BYTES" -gt 500 ] && BASE_SUBST="YES"

echo "=========================================="
echo "PROMPT: $PROMPT_ID"
echo "=========================================="
echo "SW1-SW5 pipeline:"
echo "  Size:  $SW_BYTES bytes ($SW_LINES lines)"
echo "  Substantive (>500B): $SW_SUBST"
echo "  Refusal patterns:    $SW_REFUSAL"
echo ""
echo "opencode baseline (single-shot Qwen3-5-9B-Q4_K_M):"
echo "  Size:  $BASE_BYTES bytes ($BASE_LINES lines)"
echo "  Substantive (>500B): $BASE_SUBST"
echo "  Refusal patterns:    $BASE_REFUSAL"
echo ""

# Score (higher = better)
SW_SCORE=0
BASE_SCORE=0

# Size comparison (winner gets +1)
if [ "$SW_BYTES" -gt "$BASE_BYTES" ]; then
  SW_SCORE=$((SW_SCORE+1))
  SIZE_WINNER="SW"
elif [ "$BASE_BYTES" -gt "$SW_BYTES" ]; then
  BASE_SCORE=$((BASE_SCORE+1))
  SIZE_WINNER="BASELINE"
else
  SIZE_WINNER="TIE"
fi

# Substantive
[ "$SW_SUBST" = "YES" ] && SW_SCORE=$((SW_SCORE+2))
[ "$BASE_SUBST" = "YES" ] && BASE_SCORE=$((BASE_SCORE+2))

# No refusals (penalty for each refusal)
SW_SCORE=$((SW_SCORE - SW_REFUSAL * 3))
BASE_SCORE=$((BASE_SCORE - BASE_REFUSAL * 3))

# Size >= 0.8x of opponent (full-credit deliverable)
if [ "$SW_BYTES" -ge $((BASE_BYTES * 80 / 100)) ]; then
  SW_SCORE=$((SW_SCORE+1))
fi
if [ "$BASE_BYTES" -ge $((SW_BYTES * 80 / 100)) ]; then
  BASE_SCORE=$((BASE_SCORE+1))
fi

echo "=========================================="
echo "SCORECARD"
echo "=========================================="
echo "SW1-SW5:       $SW_SCORE"
echo "opencode base: $BASE_SCORE"
echo ""

if [ "$SW_SCORE" -gt "$BASE_SCORE" ]; then
  echo "VERDICT: SW_WINS (SW=$SW_SCORE vs BASE=$BASE_SCORE)"
elif [ "$BASE_SCORE" -gt "$SW_SCORE" ]; then
  echo "VERDICT: BASELINE_WINS (SW=$SW_SCORE vs BASE=$BASE_SCORE)"
else
  echo "VERDICT: TIE (both=$SW_SCORE)"
fi