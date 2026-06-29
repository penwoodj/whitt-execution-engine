#!/usr/bin/env bash
# Re-run P21 with auto-cap-large-cat fix (commit 085a5c6).
# P21 failed because SW4 emitted 'cat src/benchmark/runner.rs' (249KB) without slicing.
# Auto-cap fix in build-workflow.py will wrap with 'head -c 50000' during SW5 assembly.
set -euo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for batch runner + first re-run script to finish
for PID in 452133 478920; do
  if kill -0 $PID 2>/dev/null; then
    echo "[p21-rerun] Waiting for PID $PID..."
    while kill -0 $PID 2>/dev/null; do
      sleep 60
    done
  fi
done

echo "[p21-rerun] All prior runs done. Restarting Docker..."
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 10

PROMPT=$PROMPT_DIR/prompt-21-implement-sub-workflow-execution.md
META_RUN_ID="p21-rerun-$(date +%Y%m%d-%H%M%S)"
LOG=/tmp/p21-rerun-v2.log

echo "[p21-rerun] Starting P21 ($META_RUN_ID)"
T0=$(date +%s)
bash $REPO/scripts/meta-v6/debug/pipeline.sh "$PROMPT" "/tmp/p21-deliverable-v2.md" "$META_RUN_ID" > "$LOG" 2>&1
EXIT=$?
T1=$(date +%s)
DURATION=$((T1-T0))

META_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/$META_RUN_ID
if [ -f "$META_DIR/deliverables/deliverable.md" ]; then
  SIZE=$(wc -c < "$META_DIR/deliverables/deliverable.md")
  echo "[p21-rerun] DONE (${DURATION}s, ${SIZE}B deliverable)"
elif [ -f "$META_DIR/exec/outputs/deliverable.md" ]; then
  cp "$META_DIR/exec/outputs/deliverable.md" "$META_DIR/deliverables/deliverable.md"
  SIZE=$(wc -c < "$META_DIR/deliverables/deliverable.md")
  echo "[p21-rerun] DONE (${DURATION}s, ${SIZE}B deliverable from exec/outputs/)"
else
  echo "[p21-rerun] FAILED (${DURATION}s, no deliverable, exit=$EXIT)"
fi

echo "[p21-rerun] Complete."
