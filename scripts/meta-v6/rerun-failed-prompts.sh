#!/usr/bin/env bash
# Re-run failed prompts P16-P19 with fixed binary + build-workflow.py.
# These failed because they used the old binary (before save_to output_dir fix).
#
# This script should be run AFTER the batch runner (PID 452133) completes P21-P24.
# Usage: bash scripts/meta-v6/rerun-failed-prompts.sh
set -euo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for batch runner to finish if still running
BATCH_PID=452133
if kill -0 $BATCH_PID 2>/dev/null; then
  echo "[rerun] Batch runner (PID $BATCH_PID) still running. Waiting..."
  while kill -0 $BATCH_PID 2>/dev/null; do
    sleep 60
    echo "[rerun] Still waiting... $(date +%H:%M:%S)"
  done
  echo "[rerun] Batch runner done."
fi

# Restart Docker for clean state
echo "[rerun] Restarting Docker..."
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 10

# Re-run each failed prompt (P10: old binary pre-topo-sort, P16-P19: old binary pre-save_to fix)
for p_num in 10 16 17 18 19; do
  PROMPT=$(ls $PROMPT_DIR/prompt-${p_num}-*.md 2>/dev/null | head -1)
  if [ -z "$PROMPT" ]; then
    echo "[rerun] P${p_num}: SKIP (no prompt file)"
    continue
  fi

  META_RUN_ID="p${p_num}-rerun-$(date +%Y%m%d-%H%M%S)"
  LOG=/tmp/p${p_num}-rerun.log

  echo "[rerun] P${p_num}: starting ($META_RUN_ID)"

  T0=$(date +%s)
  bash $REPO/scripts/meta-v6/debug/pipeline.sh "$PROMPT" "/tmp/p${p_num}-deliverable.md" "$META_RUN_ID" > "$LOG" 2>&1
  EXIT=$?
  T1=$(date +%s)
  DURATION=$((T1-T0))

  META_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/$META_RUN_ID

  # Check for deliverable (check both locations)
  DELIV=""
  if [ -f "$META_DIR/deliverables/deliverable.md" ]; then
    DELIV="$META_DIR/deliverables/deliverable.md"
  elif [ -f "$META_DIR/exec/outputs/deliverable.md" ]; then
    cp "$META_DIR/exec/outputs/deliverable.md" "$META_DIR/deliverables/deliverable.md"
    DELIV="$META_DIR/deliverables/deliverable.md"
  fi

  if [ -n "$DELIV" ]; then
    SIZE=$(wc -c < "$DELIV")
    echo "[rerun] P${p_num}: DONE (${DURATION}s, ${SIZE}B deliverable)"
  else
    echo "[rerun] P${p_num}: FAILED (${DURATION}s, no deliverable, exit=$EXIT)"
    echo "[rerun]   Log: $LOG"
  fi

  # Restart Docker between prompts for clean slot state
  echo "[rerun] Restarting Docker..."
  docker restart whitt-llama-server >/dev/null 2>&1 || true
  sleep 10
done

echo "[rerun] All re-runs complete."
