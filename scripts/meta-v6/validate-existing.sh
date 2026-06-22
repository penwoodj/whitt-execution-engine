#!/bin/bash
# Validate existing SW5 outputs from prior meta-v6 runs.
# Usage: validate-existing.sh

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
OUTPUTS_BASE="${REPO}/docs/benchmarks/outputs/meta-workflow"
RESULTS_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-2-results"

# Map prompt number to meta-v6 run timestamp (from prior batch)
declare -A RUN_TS=(
  [05]="20260622-013146"
  [06]="20260622-021726"
  [07]="20260622-031921"
  [08]="20260622-042259"
  [09]="20260622-052122"
  [10]="20260622-062845"
  [11]="20260622-072429"
)

mkdir -p "$RESULTS_DIR"

validate_prompt() {
  local n="$1"
  local ts="${RUN_TS[$n]:-}"
  [ -z "$ts" ] && { echo "[P${n}] SKIP: no run timestamp"; return; }

  local sw5_dir
  sw5_dir=$(find "${OUTPUTS_BASE}" -maxdepth 1 -type d -name "meta-meta-v6-${ts}-sw5-*" 2>/dev/null | head -1)
  [ -z "$sw5_dir" ] && { echo "[P${n}] FAIL: no SW5 dir for ts ${ts}"; return; }

  local sw5_yml="${sw5_dir}/sw5/workflow.yml"
  [ ! -f "$sw5_yml" ] && sw5_yml="${sw5_dir}/sw5/03-assembled.yml"
  [ ! -f "$sw5_yml" ] && { echo "[P${n}] FAIL: no workflow.yml in ${sw5_dir}/sw5/"; return; }

  local prompt_dir="${RESULTS_DIR}/prompt-${n}"
  mkdir -p "$prompt_dir"

  # Copy + run fix-yaml
  cp "$sw5_yml" "${prompt_dir}/workflow-raw.yml"
  python3 "${REPO}/scripts/meta-v6/fix-yaml.py" "$sw5_yml" > "${prompt_dir}/fix-yaml.log" 2>&1
  cp "$sw5_yml" "${prompt_dir}/workflow-fixed.yml"

  # Score structure
  bash "${REPO}/scripts/meta-v6/parity-check.sh" "${prompt_dir}/workflow-fixed.yml" > "${prompt_dir}/structure-score.txt" 2>&1
  local struct_score
  struct_score=$(grep "^TOTAL:" "${prompt_dir}/structure-score.txt" | awk '/structural/{print $2}' | head -1)
  struct_score=$(grep "^STRUCTURAL:" "${prompt_dir}/structure-score.txt" | awk '{print $2}' | tr -d '/25')

  echo "[P${n}] sw5=${sw5_yml}"
  echo "[P${n}] struct score: ${struct_score}/25"
  echo "[P${n}] verdict: $(grep "VERDICT:" "${prompt_dir}/structure-score.txt" | tail -1)"
}

echo "=== Validating existing SW5 outputs ==="
echo ""
for n in 05 06 07 08 09 10 11; do
  validate_prompt "$n"
  echo ""
done

echo "=== Summary ==="
for n in 05 06 07 08 09 10 11; do
  f="${RESULTS_DIR}/prompt-${n}/structure-score.txt"
  [ -f "$f" ] && echo "P${n}: $(grep -E "^VERDICT:|^STRUCTURAL:" "$f" | tr '\n' ' ')"
done
