#!/usr/bin/env bash
# Re-run ALL 20 prompts with current binary + build-workflow.py.
# All existing deliverables were generated before critical fixes:
#   - max_tokens >=8192 enforcement (e1d2540)
#   - save_to hook injection (69649d5)
#   - fail_on_error: false (d0d4b89)
#   - Docker health check (7457ac4)
#   - Smart retry (07d8bed)
# Without these fixes, intermediate steps produce ~4 tokens each and
# synthesis falls back to single-shot. Re-runs needed for real multi-step quality.
set -uo pipefail

REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for all prior background processes
for PID in 452133 478920 496364 505387; do
  if kill -0 $PID 2>/dev/null; then
    echo "[rerun-all] Waiting for PID $PID..."
    while kill -0 $PID 2>/dev/null; do sleep 60; done
    echo "[rerun-all] PID $PID done."
  fi
done

echo "[rerun-all] Starting comprehensive re-runs at $(date)"
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 10

PASS=0
FAIL=0
for p_num in 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24; do
  PROMPT=$(ls $PROMPT_DIR/prompt-${p_num}-*.md 2>/dev/null | head -1)
  if [ -z "$PROMPT" ]; then
    echo "[rerun-all] P${p_num}: SKIP (no prompt file)"
    continue
  fi

  META_RUN_ID="p${p_num}-fresh-$(date +%Y%m%d-%H%M%S)"
  LOG=/tmp/p${p_num}-fresh.log
  echo "[rerun-all] P${p_num}: starting ($META_RUN_ID)"

  T0=$(date +%s)
  bash $REPO/scripts/meta-v6/debug/pipeline.sh "$PROMPT" "/tmp/p${p_num}-fresh-deliv.md" "$META_RUN_ID" > "$LOG" 2>&1
  EXIT=$?
  T1=$(date +%s)
  DURATION=$((T1-T0))

  META_DIR=$REPO/docs/benchmarks/outputs/meta-workflow/$META_RUN_ID
  DELIV=""
  for loc in "deliverables/deliverable.md" "exec/outputs/deliverable.md"; do
    if [ -f "$META_DIR/$loc" ]; then
      mkdir -p "$META_DIR/deliverables"
      [ "$loc" = "exec/outputs/deliverable.md" ] && cp "$META_DIR/$loc" "$META_DIR/deliverables/deliverable.md"
      DELIV="$META_DIR/deliverables/deliverable.md"
      break
    fi
  done

  if [ -n "$DELIV" ]; then
    SIZE=$(wc -c < "$DELIV")
    echo "[rerun-all] P${p_num}: DONE (${DURATION}s, ${SIZE}B)"
    PASS=$((PASS+1))
  else
    echo "[rerun-all] P${p_num}: FAILED (${DURATION}s, exit=$EXIT)"
    FAIL=$((FAIL+1))
  fi

  docker restart whitt-llama-server >/dev/null 2>&1 || true
  sleep 10
done

echo "[rerun-all] COMPLETE: PASS=$PASS FAIL=$FAIL at $(date)"
