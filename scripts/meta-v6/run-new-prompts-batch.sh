#!/usr/bin/env bash
# Run SW1-SW5 pipeline for prompts 16-24 sequentially.
# Each prompt takes ~25-60 minutes. Total: ~4-9 hours.
#
# Usage: nohup bash scripts/meta-v6/run-new-prompts-batch.sh > /tmp/batch-run.log 2>&1 &
set -euo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real
PIPELINE=$REPO/scripts/meta-v6/debug/pipeline.sh

declare -a RESULTS=()

for p in 16 17 18 19 20 21 22 23 24; do
  PROMPT=$(ls $PROMPT_DIR/prompt-${p}-*.md 2>/dev/null | head -1)
  if [ -z "$PROMPT" ]; then
    echo "[batch] P$p: SKIP (no prompt file)"
    continue
  fi

  META_RUN_ID=p${p}-final-$(date +%Y%m%d-%H%M%S)
  DELIV=/tmp/p${p}-deliverable.md
  echo ""
  echo "[batch] ================================================"
  echo "[batch] P$p: START (META_RUN_ID=$META_RUN_ID)"
  echo "[batch] ================================================"
  T0=$(date +%s)

  # Run pipeline with 90min timeout per prompt
  timeout 5400 bash $PIPELINE "$PROMPT" "$DELIV" "$META_RUN_ID" 2>&1 || {
    echo "[batch] P$p: PIPELINE FAILED (exit=$?)"
    RESULTS+=("P$p: FAIL")
    continue
  }

  T1=$(date +%s)
  DURATION=$((T1-T0))

  # Check deliverable
  META_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/$META_RUN_ID
  DELIV_FILE=""
  if [ -f "$META_DIR/deliverables/deliverable.md" ]; then
    DELIV_FILE="$META_DIR/deliverables/deliverable.md"
  elif [ -f "$REPO/outputs/deliverable.md" ]; then
    # Engine wrote to repo root (working_dir issue)
    mkdir -p "$META_DIR/deliverables"
    cp "$REPO/outputs/deliverable.md" "$META_DIR/deliverables/deliverable.md"
    DELIV_FILE="$META_DIR/deliverables/deliverable.md"
    rm -rf "$REPO/outputs"
  fi

  if [ -n "$DELIV_FILE" ]; then
    SIZE=$(wc -c < "$DELIV_FILE")
    WF=$META_DIR/meta/generated-workflow.yml
    if [ -f "$WF" ] && [ -f "$DELIV_FILE" ]; then
      SCORE=$(bash $REPO/scripts/meta-v6/parity-check.sh "$WF" "$DELIV_FILE" "$META_DIR/exec-final" 2>&1 | grep "=== Score:" | head -1)
      VERDICT=$(bash $REPO/scripts/meta-v6/parity-check.sh "$WF" "$DELIV_FILE" "$META_DIR/exec-final" 2>&1 | grep "^VERDICT:" | head -1)
      echo "[batch] P$p: DONE (${DURATION}s) deliv=${SIZE}B $SCORE $VERDICT"
      RESULTS+=("P$p: $SCORE $VERDICT (${DURATION}s, ${SIZE}B)")
    else
      echo "[batch] P$p: DONE (${DURATION}s) deliv=${SIZE}B (no WF for parity check)"
      RESULTS+=("P$p: ${SIZE}B (${DURATION}s)")
    fi
  else
    echo "[batch] P$p: NO DELIVERABLE"
    RESULTS+=("P$p: NO DELIVERABLE")
  fi
done

echo ""
echo "[batch] ================================================"
echo "[batch] FINAL RESULTS"
echo "[batch] ================================================"
for r in "${RESULTS[@]}"; do
  echo "[batch] $r"
done
echo "[batch] BATCH COMPLETE"
