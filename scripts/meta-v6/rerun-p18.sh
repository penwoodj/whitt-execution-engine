#!/usr/bin/env bash
# Re-run P18 which failed due to Fit: (capital F) field not caught by case-sensitive regex.
# Fix deployed in commit 81a42e0: strip_unknown_step_fields now case-insensitive.
set -uo pipefail
REPO=/home/jon/code/whitt-execution-engine
PROMPT_DIR=$REPO/docs/plans/meta-workflow-qwen35/test-prompts/real

# Wait for P11 re-run to complete
P11_PID=665385
if kill -0 $P11_PID 2>/dev/null; then
  echo "[p18-rerun] Waiting for P11 re-run (PID $P11_PID)..."
  while kill -0 $P11_PID 2>/dev/null; do sleep 60; done
fi

echo "[p18-rerun] Starting at $(date)"
docker restart whitt-llama-server >/dev/null 2>&1 || true
sleep 15

PROMPT_FILE=$(ls "$PROMPT_DIR"/prompt-18-*.md 2>/dev/null | head -1)
RUN_ID="p18-rerun-$(date +%Y%m%d-%H%M%S)"
OUTPUT_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/$RUN_ID"
mkdir -p "$OUTPUT_DIR"/{deliverables,input,logs,meta}
cp "$PROMPT_FILE" "$OUTPUT_DIR/input/prompt.txt"

cd "$REPO"
timeout 5400 bash scripts/meta-v6/debug/pipeline.sh \
  "$PROMPT_FILE" "$OUTPUT_DIR/deliverables/deliverable.md" \
  "$RUN_ID" 2>&1 | tee "$OUTPUT_DIR/pipeline.log"

DELIV="$OUTPUT_DIR/deliverables/deliverable.md"
if [ -f "$DELIV" ] && [ $(wc -c < "$DELIV") -gt 100 ]; then
  echo "[p18-rerun] SUCCESS: $(wc -c < "$DELIV")B"
else
  echo "[p18-rerun] FAILED"
fi
