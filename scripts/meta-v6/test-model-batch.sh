#!/bin/bash
# scripts/meta-v6/test-model-batch.sh
# Batch test a single model across 3 prompts through full meta-workflow pipeline.
#
# For each prompt:
#   1. Substitute model in SW1-5 + meta-v6 yamls
#   2. Run meta-v6 generation (SW1-5)
#   3. If SW4 fails (no workflow.yml), retry with mixed config (model SW1-3 + Qwen3-5-9B SW4-5)
#   4. Swap model in output workflow for exec phase
#   5. Run exec phase
#   6. Run parity-check.sh
#   7. Collect results
#
# Usage: test-model-batch.sh <model-gguf-name> <prompt1> <prompt2> <prompt3>
# Example: test-model-batch.sh Falcon-H1-7B-Instruct-Q4_K_M.gguf 19 06 15

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
MODEL="${1:?usage: $0 <model-gguf-name> <p1> <p2> <p3>}"
shift
PROMPTS=("$@")

# Derive model ID without .gguf extension
MODEL_ID="${MODEL%.gguf}"
# Derive filter regex (first 20 chars of model name, enough to be unique)
FILTER_REGEX="${MODEL_ID:0:20}"

# Backup originals
BACKUP_DIR="/tmp/model-test-backup-$$"
mkdir -p "$BACKUP_DIR"
cp "$REPO/docs/benchmarks/workflows"/{sw1-task-deconstruction,sw2-desired-output-state,sw3-agentic-categorization,sw4-yaml-substructure-translation,sw5-final-workflow-assembly,meta-workflow-v6}.yml "$BACKUP_DIR/"
cp "$REPO/scripts/meta-v6"/{run-sw,run-prompt-end-to-end}.sh "$BACKUP_DIR/"

cleanup() {
  cp "$BACKUP_DIR"/*.yml "$REPO/docs/benchmarks/workflows/"
  cp "$BACKUP_DIR"/*.sh "$REPO/scripts/meta-v6/"
  rm -rf "$BACKUP_DIR"
  echo "[cleanup] originals restored"
}
trap cleanup EXIT

substitute_model() {
  local mode="${1:-full}"
  local files=(
    "$REPO/docs/benchmarks/workflows/sw1-task-deconstruction.yml"
    "$REPO/docs/benchmarks/workflows/sw2-desired-output-state.yml"
    "$REPO/docs/benchmarks/workflows/sw3-agentic-categorization.yml"
    "$REPO/docs/benchmarks/workflows/meta-workflow-v6.yml"
    "$REPO/scripts/meta-v6/run-sw.sh"
    "$REPO/scripts/meta-v6/run-prompt-end-to-end.sh"
  )
  if [ "$mode" = "full" ]; then
    files+=(
      "$REPO/docs/benchmarks/workflows/sw4-yaml-substructure-translation.yml"
      "$REPO/docs/benchmarks/workflows/sw5-final-workflow-assembly.yml"
    )
  fi
  for f in "${files[@]}"; do
    sed -i "s|Qwen3-5-9B-Q4_K_M|${MODEL_ID}|g" "$f"
  done
  for f in "$REPO/scripts/meta-v6"/run-sw.sh "$REPO/scripts/meta-v6"/run-prompt-end-to-end.sh; do
    sed -i "s|--filter-name \"Qwen3-5-9B\"|--filter-name \"${FILTER_REGEX}\"|g" "$f"
    sed -i "s|--filter-name \"Yi-6B-200K\"|--filter-name \"${FILTER_REGEX}\"|g" "$f"
  done
  echo "[substitute] mode=$mode model=$MODEL_ID"
}

run_prompt() {
  local N="$1"
  local OUT="$REPO/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}"
  local LOG_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/integration-tests/live-runs/model-tests"
  mkdir -p "$LOG_DIR"
  rm -rf "$OUT"
  local START=$(date +%s)

  echo "=== [P${N}] ${MODEL_ID} ==="

  # Phase 1: meta-v6 generation (full model)
  substitute_model "full"
  timeout 3600 bash "$REPO/scripts/meta-v6/run-prompt-end-to-end.sh" "$N" 0 > "$LOG_DIR/${MODEL_ID}-P${N}-gen.log" 2>&1
  local GEN_RC=$?

  # Check if SW5 produced output
  local SW5_DIR=$(find "$REPO/docs/benchmarks/outputs/meta-workflow" -maxdepth 1 -type d -name "meta-meta-v6-*-sw5-*" 2>/dev/null | sort | tail -1)
  local SW5_FILE=""
  if [ -n "$SW5_DIR" ]; then
    SW5_FILE=$(find "$SW5_DIR" -maxdepth 2 -name "workflow.yml" 2>/dev/null | head -1)
  fi

  # If SW4 failed (no workflow.yml), retry with mixed config
  local CONFIG="full"
  if [ -z "$SW5_FILE" ]; then
    echo "[P${N}] SW4 failed with ${MODEL_ID}, retrying mixed config..."
    substitute_model "mixed"
    rm -rf "$OUT"
    timeout 3600 bash "$REPO/scripts/meta-v6/run-prompt-end-to-end.sh" "$N" 0 > "$LOG_DIR/${MODEL_ID}-P${N}-gen-mixed.log" 2>&1
    GEN_RC=$?
    CONFIG="mixed"
    SW5_DIR=$(find "$REPO/docs/benchmarks/outputs/meta-workflow" -maxdepth 1 -type d -name "meta-meta-v6-*-sw5-*" 2>/dev/null | sort | tail -1)
    if [ -n "$SW5_DIR" ]; then
      SW5_FILE=$(find "$SW5_DIR" -maxdepth 2 -name "workflow.yml" 2>/dev/null | head -1)
    fi
  fi

  if [ -z "$SW5_FILE" ]; then
    echo "[P${N}] FAIL: no SW5 output even with mixed config"
    local END=$(date +%s)
    echo -e "${N}\t${MODEL_ID}\t${CONFIG}\t0\t0\t0\t0\t0/50\tFAIL\t0\t" >> "$RESULTS_FILE"
    return 1
  fi

  # Manual assembly (script detection is flaky)
  cp "$SW5_FILE" "$OUT/workflow-raw.yml"
  python3 -c "
import re
content = open('$OUT/workflow-raw.yml').read()
content = re.sub(r'^\`\`\`(?:yaml|yml)?\s*\n', '', content, flags=re.MULTILINE)
content = re.sub(r'\n\`\`\`\s*\$', '', content, flags=re.MULTILINE)
open('$OUT/workflow-stripped.yml', 'w').write(content)
" 2>/dev/null
  cp "$OUT/workflow-stripped.yml" "$OUT/workflow-canonical.yml"
  cp "$OUT/workflow-canonical.yml" "$OUT/workflow-fixed.yml"
  python3 "$REPO/scripts/meta-v6/inject-shell-hooks.py" "$OUT/workflow-fixed.yml" > "$OUT/inject.log" 2>&1 || true

  # Validate YAML
  if ! python3 -c "import yaml; yaml.safe_load(open('$OUT/workflow-fixed.yml'))" 2>/dev/null; then
    echo "[P${N}] FAIL: output YAML invalid"
    local END=$(date +%s)
    echo -e "${N}\t${MODEL_ID}\t${CONFIG}\t0\t0\t0\t0\t0/50\tFAIL\t0\t" >> "$RESULTS_FILE"
    return 1
  fi

  # Swap model in output workflow for exec
  sed -i "s|Qwen3-5-9B-Q4_K_M|${MODEL_ID}|g" "$OUT/workflow-fixed.yml"

  # Phase 5: exec
  rm -rf "$OUT/exec" && mkdir -p "$OUT/exec"
  timeout 1800 "$REPO/target/release/whitt" benchmark \
    --workflow "$OUT/workflow-fixed.yml" \
    --output-dir "$OUT/exec" \
    --models-dir "$REPO/models" \
    --filter-name "$FILTER_REGEX" \
    --load-timeout 300 \
    > "$OUT/exec.log" 2>&1
  local EXEC_RC=$?

  local END=$(date +%s)
  local DURATION=$((END - START))

  # Collect metrics
  local DELIVERABLE="$OUT/exec/outputs/deliverable.md"
  local DELIV_SIZE=0
  local DELIV_LINES=0
  local REFUSALS=0
  local STEPS=0
  local PARITY_SCORE="?"
  local PARITY_VERDICT="?"

  if [ -f "$DELIVERABLE" ]; then
    DELIV_SIZE=$(stat -c %s "$DELIVERABLE")
    DELIV_LINES=$(wc -l < "$DELIVERABLE")
    REFUSALS=$(grep -ciE "I cannot|cannot access|do not have access|unable to" "$DELIVERABLE" 2>/dev/null)
    [ -z "$REFUSALS" ] && REFUSALS=0
  fi
  STEPS=$(grep -c "step_name=" "$OUT/exec.log" 2>/dev/null)
  [ -z "$STEPS" ] && STEPS=0

  # Parity check
  local PARITY_OUT=$(bash "$REPO/scripts/meta-v6/parity-check.sh" \
    "$OUT/workflow-fixed.yml" "$DELIVERABLE" "$OUT/exec" 2>&1)
  PARITY_SCORE=$(echo "$PARITY_OUT" | grep "^=== Score:" | grep -oE "[0-9]+/[0-9]+")
  PARITY_VERDICT=$(echo "$PARITY_OUT" | grep "^VERDICT:" | grep -oE "PASS|FAIL")

  # SW1 quality scores
  local SW1_QUALITY=$(find "$REPO/docs/benchmarks/outputs/meta-workflow" -maxdepth 3 -name "sw1.log" -newer "$LOG_DIR/${MODEL_ID}-P${N}-gen.log" -exec grep -oE '"quality_score":[0-9.]+' {} \; 2>/dev/null | head -6 | tr '\n' ',' | sed 's/,$//')

  echo "[P${N}] DONE: config=$CONFIG steps=$STEPS deliv=${DELIV_SIZE}B/${DELIV_LINES}L refusals=$REFUSALS parity=$PARITY_SCORE/$PARITY_VERDICT duration=${DURATION}s"
  echo -e "${N}\t${MODEL_ID}\t${CONFIG}\t${STEPS}\t${DELIV_SIZE}\t${DELIV_LINES}\t${REFUSALS}\t${PARITY_SCORE}\t${PARITY_VERDICT}\t${DURATION}\t${SW1_QUALITY}" >> "$RESULTS_FILE"
}

# Init results TSV
LOG_DIR="$REPO/docs/benchmarks/outputs/meta-workflow/integration-tests/live-runs/model-tests"
mkdir -p "$LOG_DIR"
echo -e "prompt\tmodel\tconfig\tsteps\tdeliv_bytes\tdeliv_lines\trefusals\tparity_score\tparity_verdict\tduration_sec\tsw1_quality" > "$LOG_DIR/results-${MODEL_ID}.tsv"
RESULTS_FILE="$LOG_DIR/results-${MODEL_ID}.tsv"

# Override results file path used in run_prompt
export RESULTS_FILE

for n in "${PROMPTS[@]}"; do
  run_prompt "$n" 2>&1 | tee -a "$LOG_DIR/${MODEL_ID}-batch.log"
  # Backup prompt dir to model-specific location (prevents overwrite by next model)
  BACKUP_PROMPT_DIR="$LOG_DIR/prompt-${n}-${MODEL_ID}"
  rm -rf "$BACKUP_PROMPT_DIR"
  cp -r "$REPO/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${n}" "$BACKUP_PROMPT_DIR" 2>/dev/null || true
done

echo ""
echo "=== Batch Complete: ${MODEL_ID} ==="
echo "Results: $RESULTS_FILE"
column -t -s $'\t' "$RESULTS_FILE"
