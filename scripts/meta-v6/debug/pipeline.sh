#!/bin/bash
# scripts/meta-v6/debug/pipeline.sh
# End-to-end SW1-SW5 + execute + validate pipeline with structured logging.
#
# Each SW emits [swN:start], [swN:output], [swN:end] log lines.
# Final execute step runs the generated workflow.
# Validator runs the honest parity-check.sh at end.
#
# Usage: pipeline.sh <prompt-file> <deliverable-path> [meta-run-id]

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPT_FILE="${1:-}"
DELIVERABLE_PATH="${2:-}"
META_RUN_ID="${3:-meta-debug-$(date +%Y%m%d-%H%M%S)}"

if [ -z "$PROMPT_FILE" ] || [ ! -f "$PROMPT_FILE" ]; then
  echo "Usage: $0 <prompt-file> <deliverable-path> [meta-run-id]"
  exit 1
fi

if [ -z "$DELIVERABLE_PATH" ]; then
  echo "FAIL: deliverable-path required"
  exit 1
fi

# Recreate logs/ (Docker creates it as root, causing permission denied)
rm -rf "$REPO/logs" 2>/dev/null || true
mkdir -p "$REPO/logs" 2>/dev/null || true

META_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"
mkdir -p "${META_DIR}"/{input,meta,logs,deliverables}
cp "${PROMPT_FILE}" "${META_DIR}/input/prompt.txt"
echo "${META_RUN_ID}" > "${REPO}/.current-meta-run"

PIPELINE_LOG="${META_DIR}/pipeline.log"
exec > >(tee -a "$PIPELINE_LOG") 2>&1

log() { echo "[$(date -u +%H:%M:%S)] [pipeline] $*"; }

log "START meta_run_id=${META_RUN_ID}"
log "prompt=${PROMPT_FILE} ($(wc -c < "$PROMPT_FILE") bytes)"
log "deliverable=${DELIVERABLE_PATH}"

run_sw() {
  local sw_num="$1"
  local script="$REPO/scripts/meta-v6/run-sw.sh"
  log "SW${sw_num}:start"
  local t0=$(date +%s)
  bash "$script" "$sw_num" "${META_RUN_ID}" 2>&1 | tail -5
  local exit_code=$?
  local t1=$(date +%s)
  log "SW${sw_num}:end duration=$((t1-t0))s exit=${exit_code}"
  return $exit_code
}

run_sw 1 || { log "FATAL: SW1 failed"; exit 1; }
run_sw 2 || { log "FATAL: SW2 failed"; exit 1; }
run_sw 3 || { log "FATAL: SW3 failed"; exit 1; }
run_sw 4 || { log "FATAL: SW4 failed"; exit 1; }

log "SW5:start (deterministic)"
run_sw 5 2>&1 | tee -a "$PIPELINE_LOG" | tail -3
SW5_EXIT=${PIPESTATUS[0]}

GENERATED="${META_DIR}/meta/generated-workflow.yml"
if [ ! -s "$GENERATED" ]; then
  log "FATAL: SW5 produced no workflow.yml"
  exit 1
fi
log "SW5:end workflow.yml=$(wc -c < "$GENERATED") bytes"

# Deliverable written by build-workflow.py's synthesis step
ACTUAL_DELIVERABLE="${META_DIR}/deliverables/deliverable.md"
if [ -f "$ACTUAL_DELIVERABLE" ]; then
  DELIVERABLE_PATH="$ACTUAL_DELIVERABLE"
  log "deliverable-path: $DELIVERABLE_PATH"
fi

# Validate generated workflow YAML
log "validate:start"
if ! python3 "$REPO/scripts/validate-yaml.py" "$GENERATED" 2>&1 | tail -3; then
  log "FATAL: generated workflow YAML invalid"
  exit 1
fi
log "validate:pass"

# Execute generated workflow
log "execute:start"
EXEC_DIR="${META_DIR}/exec"
mkdir -p "$EXEC_DIR"
SWEEPT0=$(date +%s)
timeout 5400 "$REPO/target/release/whitt" benchmark \
  --workflow "$GENERATED" \
  --output-dir "$EXEC_DIR" \
  --models-dir "$REPO/models" \
  --filter-name "Qwen3-5-9B" \
  --load-timeout 1800 > "$EXEC_DIR/benchmark.log" 2>&1
EXEC_EXIT=$?
SWEEPT1=$(date +%s)
log "execute:end duration=$((SWEEPT1-SWEEPT0))s exit=${EXEC_EXIT}"

if [ $EXEC_EXIT -ne 0 ]; then
  log "FATAL: generated workflow execution failed"
  tail -20 "$EXEC_DIR/benchmark.log" | tee -a "$PIPELINE_LOG"
  exit 1
fi

# Run honest validator
log "validate-deliverable:start"
bash "$REPO/scripts/meta-v6/parity-check.sh" "$GENERATED" "$DELIVERABLE_PATH" "$EXEC_DIR" 2>&1 | tee -a "$PIPELINE_LOG"
log "validate-deliverable:end"

log "PIPELINE_DONE"
