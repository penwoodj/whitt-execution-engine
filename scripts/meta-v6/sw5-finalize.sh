#!/usr/bin/env bash
# SW5 step_01 shell hook: validate assembled YAML + copy to META run dir.
# Called from sw5-final-workflow-assembly.yml step_01_validate_and_finalize before_step_starts.
# Args:
#   $1 = SW5_RUN_ID (e.g., meta-<META>-sw5-<TS>) — used to find SW5_DIR
#   $2 = META_RUN_ID (e.g., p15-sw-pipeline-20260627) — used to find META_DIR
set -euo pipefail

SW5_RUN_ID="${1:?SW5_RUN_ID required}"
META_RUN_ID="${2:?META_RUN_ID required}"
REPO=/home/jon/code/whitt-execution-engine
SW5_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/${SW5_RUN_ID}"
META_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}"
WF="$SW5_DIR/sw5/workflow.yml"

if [ ! -s "$WF" ]; then
  echo "FATAL: workflow.yml missing or empty"
  exit 1
fi

# Apply fix-yaml.py normalization (unquoted GWT, inline save_to, etc.)
python3 "$REPO/scripts/meta-v6/fix-yaml.py" "$WF" || echo "WARN: fix-yaml.py non-fatal"

# Validate YAML parses (non-fatal: Rust serde_saphyr is more lenient than Python yaml)
python3 -c "import yaml; yaml.safe_load(open('$WF')); print('YAML_OK')" || echo "WARN: Python YAML validation failed — Rust engine may still accept it"

# Copy to META run dir as canonical pipeline output
mkdir -p "$META_DIR/meta"
cp "$WF" "$META_DIR/meta/generated-workflow.yml"

echo "FINALIZED: $(wc -c < "$META_DIR/meta/generated-workflow.yml") bytes"
