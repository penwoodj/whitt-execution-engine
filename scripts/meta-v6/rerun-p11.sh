#!/usr/bin/env bash
# Re-run P11 which failed during ALL20 due to unknown step fields (intent:, fit:).
# Fix deployed in commit 4befe25: strip_unknown_step_fields() in build-workflow.py.
# Must wait for P06/P07 re-run to complete first (no Docker contention).
set -uo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for P06/P07 re-run to complete
P0607_PID=635496
if kill -0 $P0607_PID 2>/dev/null; then
  echo "[p11-rerun] Waiting for P06/P07 re-run (PID $P0607_PID)..."
  while kill -0 $P0607_PID 2>/dev/null; do sleep 60; done
  echo "[p11-rerun] P06/P07 done."
fi

echo "[p11-rerun] Starting at $(date)"
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 15

PROMPT_FILE=$(ls "$PROMPT_DIR"/prompt-11-*.md 2>/dev/null | head -1)
if [ -z "$PROMPT_FILE" ]; then
  echo "[p11-rerun] FATAL: no prompt file found"
  exit 1
fi

RUN_ID="p11-rerun-$(date +%Y%m%d-%H%M%S)"
OUTPUT_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/$RUN_ID"
mkdir -p "$OUTPUT_DIR/deliverables" "$OUTPUT_DIR/input" "$OUTPUT_DIR/logs" "$OUTPUT_DIR/meta"

cp "$PROMPT_FILE" "$OUTPUT_DIR/input/prompt.txt"

cd "$REPO"
timeout 5400 bash scripts/meta-v6/debug/pipeline.sh \
  "$PROMPT_FILE" \
  "$OUTPUT_DIR/deliverables/deliverable.md" \
  "$RUN_ID" 2>&1 | tee "$OUTPUT_DIR/pipeline.log"

DELIV="$OUTPUT_DIR/deliverables/deliverable.md"
if [ -f "$DELIV" ] && [ $(wc -c < "$DELIV") -gt 100 ]; then
  SIZE=$(wc -c < "$DELIV")
  echo "[p11-rerun] SUCCESS: deliverable=$DELIV (${SIZE}B)"
else
  echo "[p11-rerun] FAILED: no deliverable produced"
fi
echo "[p11-rerun] Done at $(date)"
