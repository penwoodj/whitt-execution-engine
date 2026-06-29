#!/usr/bin/env bash
# Re-run P22 with duplicate depends_on fix.
# Waits for batch runner + first re-run script to complete first.
set -euo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for all prior background processes
for PID in 452133 478920 496364; do
  if kill -0 $PID 2>/dev/null; then
    echo "[p22-rerun] Waiting for PID $PID..."
    while kill -0 $PID 2>/dev/null; do sleep 60; done
    echo "[p22-rerun] PID $PID done."
  fi
done

echo "[p22-rerun] Restarting Docker..."
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 10

PROMPT=$(ls $PROMPT_DIR/prompt-22-*.md 2>/dev/null | head -1)
META_RUN_ID="p22-rerun-$(date +%Y%m%d-%H%M%S)"
LOG=/tmp/p22-rerun.log

echo "[p22-rerun] Starting P22 ($META_RUN_ID)"
T0=$(date +%s)
bash $REPO/scripts/meta-v6/debug/pipeline.sh "$PROMPT" "/tmp/p22-deliverable.md" "$META_RUN_ID" > "$LOG" 2>&1
EXIT=$?
T1=$(date +%s)
DURATION=$((T1-T0))

META_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/$META_RUN_ID
DELIV=""
for loc in "deliverables/deliverable.md" "exec/outputs/deliverable.md"; do
  if [ -f "$META_DIR/$loc" ]; then
    [ "$loc" = "exec/outputs/deliverable.md" ] && cp "$META_DIR/$loc" "$META_DIR/deliverables/deliverable.md"
    DELIV="$META_DIR/deliverables/deliverable.md"
    break
  fi
done

if [ -n "$DELIV" ]; then
  SIZE=$(wc -c < "$DELIV")
  echo "[p22-rerun] DONE: ${DURATION}s, ${SIZE}B deliverable"
else
  echo "[p22-rerun] FAILED: ${DURATION}s, no deliverable, exit=$EXIT"
fi
