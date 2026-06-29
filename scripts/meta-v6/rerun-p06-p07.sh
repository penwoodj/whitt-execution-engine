#!/usr/bin/env bash
# Re-run P06 and P07 which failed during ALL20 re-run due to:
# - P06: Docker restart killed step_t18 mid-inference
# - P07: 27-step workflow, old 1800s timeout killed execution
# Both need 5400s timeout (commit 8e56cd7) + Docker recovery (commit 7457ac4).
set -uo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for ALL20 re-run to complete
ALL20_PID=573680
if kill -0 $ALL20_PID 2>/dev/null; then
  echo "[p06p07-rerun] Waiting for ALL20 (PID $ALL20_PID)..."
  while kill -0 $ALL20_PID 2>/dev/null; do sleep 60; done
  echo "[p06p07-rerun] ALL20 done."
fi

echo "[p06p07-rerun] Starting at $(date)"
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 10

for p_num in 06 07; do
  PROMPT=$(ls $PROMPT_DIR/prompt-${p_num}-*.md 2>/dev/null | head -1)
  [ -z "$PROMPT" ] && echo "[p06p07-rerun] P${p_num}: no prompt" && continue

  META_RUN_ID="p${p_num}-retry2-$(date +%Y%m%d-%H%M%S)"
  LOG=/tmp/p${p_num}-retry2.log
  echo "[p06p07-rerun] P${p_num}: starting ($META_RUN_ID)"

  T0=$(date +%s)
  bash $REPO/scripts/meta-v6/debug/pipeline.sh "$PROMPT" "/tmp/p${p_num}-retry2-deliv.md" "$META_RUN_ID" > "$LOG" 2>&1
  EXIT=$?
  T1=$(date +%s)
  DURATION=$((T1-T0))

  META_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/$META_RUN_ID
  for loc in "deliverables/deliverable.md" "exec/outputs/deliverable.md"; do
    if [ -f "$META_DIR/$loc" ]; then
      mkdir -p "$META_DIR/deliverables"
      [ "$loc" = "exec/outputs/deliverable.md" ] && cp "$META_DIR/$loc" "$META_DIR/deliverables/deliverable.md"
      SIZE=$(wc -c < "$META_DIR/deliverables/deliverable.md")
      echo "[p06p07-rerun] P${p_num}: DONE (${DURATION}s, ${SIZE}B)"
      break
    fi
  done || echo "[p06p07-rerun] P${p_num}: FAILED (${DURATION}s, exit=$EXIT)"

  docker restart whitt-llama-server >/dev/null 2>&1 || true
  sleep 10
done

echo "[p06p07-rerun] COMPLETE at $(date)"
