#!/usr/bin/env bash
# Quality-focused comparison: SW1-SW5 vs opencode baseline.
# Metrics: code blocks, section structure, refusals, task completion, actionable depth.
# Size is normalized (not absolute) — smaller high-quality output beats larger verbose output.
set -euo pipefail

PROMPT_ID="${1:?Usage: $0 <prompt_id> <sw_deliverable> <baseline>}"
SW_PATH="${2:?missing sw_deliverable}"
BASE_PATH="${3:?missing baseline}"

[ ! -f "$SW_PATH" ] && { echo "FATAL: SW not found: $SW_PATH" >&2; exit 1; }
[ ! -f "$BASE_PATH" ] && { echo "FATAL: baseline not found: $BASE_PATH" >&2; exit 1; }

SW_BYTES=$(wc -c < "$SW_PATH")
BASE_BYTES=$(wc -c < "$BASE_PATH")

# Code blocks (``` pairs / 2)
SW_CODE_BLOCKS=$(grep -c '```' "$SW_PATH" 2>/dev/null || true)
SW_CODE_BLOCKS=${SW_CODE_BLOCKS:-0}
SW_CODE_BLOCKS=$((SW_CODE_BLOCKS / 2))
BASE_CODE_BLOCKS=$(grep -c '```' "$BASE_PATH" 2>/dev/null || true)
BASE_CODE_BLOCKS=${BASE_CODE_BLOCKS:-0}
BASE_CODE_BLOCKS=$((BASE_CODE_BLOCKS / 2))

# Section headers (# or ##)
SW_HEADERS=$(grep -cE '^#{1,3} ' "$SW_PATH" 2>/dev/null || true)
SW_HEADERS=${SW_HEADERS:-0}
BASE_HEADERS=$(grep -cE '^#{1,3} ' "$BASE_PATH" 2>/dev/null || true)
BASE_HEADERS=${BASE_HEADERS:-0}

# Refusal patterns (HEAVY penalty: -5 each)
SW_REFUSAL=$(grep -ciE 'I cannot|I can.t help|as an ai|I.m unable to|I.m not able to' "$SW_PATH" 2>/dev/null | head -1 || true)
SW_REFUSAL=${SW_REFUSAL:-0}
BASE_REFUSAL=$(grep -ciE 'I cannot|I can.t help|as an ai|I.m unable to|I.m not able to' "$BASE_PATH" 2>/dev/null | head -1 || true)
BASE_REFUSAL=${BASE_REFUSAL:-0}

# Substantive check (>500B)
SW_SUBST=0; [ "$SW_BYTES" -gt 500 ] && SW_SUBST=1
BASE_SUBST=0; [ "$BASE_BYTES" -gt 500 ] && BASE_SUBST=1

# Size ratio (how close SW is to baseline)
if [ "$BASE_BYTES" -gt 0 ]; then
  SIZE_RATIO=$((SW_BYTES * 100 / BASE_BYTES))
else
  SIZE_RATIO=0
fi

echo "=========================================="
echo "PROMPT: $PROMPT_ID"
echo "=========================================="
echo "SW1-SW5:     ${SW_BYTES}B, ${SW_CODE_BLOCKS} code blocks, ${SW_HEADERS} headers, ${SW_REFUSAL} refusals"
echo "Baseline:    ${BASE_BYTES}B, ${BASE_CODE_BLOCKS} code blocks, ${BASE_HEADERS} headers, ${BASE_REFUSAL} refusals"
echo "Size ratio:  ${SIZE_RATIO}%"
echo ""

# Quality score (higher = better)
SW_SCORE=0
BASE_SCORE=0

# Substantive (+3)
[ "$SW_SUBST" = 1 ] && SW_SCORE=$((SW_SCORE+3))
[ "$BASE_SUBST" = 1 ] && BASE_SCORE=$((BASE_SCORE+3))

# Code blocks (+2 each, max 10)
SW_CODE_PTS=$((SW_CODE_BLOCKS * 2)); [ "$SW_CODE_PTS" -gt 10 ] && SW_CODE_PTS=10
BASE_CODE_PTS=$((BASE_CODE_BLOCKS * 2)); [ "$BASE_CODE_PTS" -gt 10 ] && BASE_CODE_PTS=10
SW_SCORE=$((SW_SCORE + SW_CODE_PTS))
BASE_SCORE=$((BASE_SCORE + BASE_CODE_PTS))

# Section structure (+1 each, max 5)
SW_HDR_PTS=$SW_HEADERS; [ "$SW_HDR_PTS" -gt 5 ] && SW_HDR_PTS=5
BASE_HDR_PTS=$BASE_HEADERS; [ "$BASE_HDR_PTS" -gt 5 ] && BASE_HDR_PTS=5
SW_SCORE=$((SW_SCORE + SW_HDR_PTS))
BASE_SCORE=$((BASE_SCORE + BASE_HDR_PTS))

# Refusal penalty (-5 each)
SW_SCORE=$((SW_SCORE - SW_REFUSAL * 5))
BASE_SCORE=$((BASE_SCORE - BASE_REFUSAL * 5))

# Size adequacy (+2 if >2KB, +1 if >1KB)
if [ "$SW_BYTES" -gt 2000 ]; then SW_SCORE=$((SW_SCORE+2))
elif [ "$SW_BYTES" -gt 1000 ]; then SW_SCORE=$((SW_SCORE+1)); fi
if [ "$BASE_BYTES" -gt 2000 ]; then BASE_SCORE=$((BASE_SCORE+2))
elif [ "$BASE_BYTES" -gt 1000 ]; then BASE_SCORE=$((BASE_SCORE+1)); fi

# Relative size bonus (if SW ≥ 50% of baseline, +1; if ≥ 80%, +2)
if [ "$SIZE_RATIO" -ge 80 ]; then SW_SCORE=$((SW_SCORE+2))
elif [ "$SIZE_RATIO" -ge 50 ]; then SW_SCORE=$((SW_SCORE+1)); fi
if [ "$SIZE_RATIO" -le 125 ]; then BASE_SCORE=$((BASE_SCORE+1)); fi
if [ "$BASE_BYTES" -gt 0 ] && [ "$SW_BYTES" -gt 0 ] && [ "$BASE_BYTES" -le $((SW_BYTES * 200 / 100)) ]; then BASE_SCORE=$((BASE_SCORE+1)); fi

echo "=========================================="
echo "QUALITY SCORECARD"
echo "=========================================="
echo "SW1-SW5:       $SW_SCORE"
echo "Baseline:      $BASE_SCORE"
echo ""

if [ "$SW_SCORE" -gt "$BASE_SCORE" ]; then
  echo "VERDICT: SW_WINS (SW=$SW_SCORE vs BASE=$BASE_SCORE)"
elif [ "$BASE_SCORE" -gt "$SW_SCORE" ]; then
  echo "VERDICT: BASELINE_WINS (SW=$SW_SCORE vs BASE=$BASE_SCORE)"
else
  echo "VERDICT: TIE (both=$SW_SCORE)"
fi
