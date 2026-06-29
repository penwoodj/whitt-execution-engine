#!/usr/bin/env bash
# SW5 step_00 shell hook: invoke build-workflow.py to assemble final workflow.
# Called from sw5-final-workflow-assembly.yml step_00_assemble before_step_starts.
# Args:
#   $1 = SW5_RUN_ID (e.g., meta-<META>-sw5-<TS>) — used to find SW5_DIR
#   $2 = META_RUN_ID (e.g., p15-sw-pipeline-20260627) — used to find META_DIR
set -euo pipefail

SW5_RUN_ID="${1:?SW5_RUN_ID required}"
META_RUN_ID="${2:?META_RUN_ID required}"
REPO=/home/jon/code/whitt-execution-engine
SW5_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}"
META_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"

mkdir -p "$SW5_DIR/sw5"

python3 "$REPO/scripts/meta-v6/build-workflow.py" \
  "$SW5_DIR/input/structs.md" \
  "$SW5_DIR/sw5/workflow.yml" \
  "$META_DIR/input/prompt.txt" \
  "$META_DIR" \
  "$META_RUN_ID"

echo "ASSEMBLED: $(wc -c < "$SW5_DIR/sw5/workflow.yml") bytes"
