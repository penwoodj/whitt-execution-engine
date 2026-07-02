#!/bin/bash
# scripts/meta-v6/run-prompt-end-to-end.sh
# Full pipeline: meta-v6 generation → strip fences → inject hooks → exec → score
#
# Usage: run-prompt-end-to-end.sh <prompt_number> [skip_gen]
#   skip_gen=1 will reuse existing SW5 output if present

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPTS_DIR="${REPO}/docs/plans/meta-workflow-qwen35/test-prompts/real"
RESULTS_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-3-results"

N="${1:?usage: $0 <prompt_number> [skip_gen]}"
SKIP_GEN="${2:-0}"

PROMPT_FILE=$(ls "${PROMPTS_DIR}/prompt-${N}-"*.md 2>/dev/null | head -1)
[ -z "$PROMPT_FILE" ] && { echo "[P${N}] no prompt file"; exit 1; }

OUT="${RESULTS_DIR}/prompt-${N}"
mkdir -p "${OUT}"

echo "=== [P${N}] $(basename "$PROMPT_FILE") ==="

# Phase 1: Generate via meta-v6 (or reuse)
SW5_FILE=""
if [ "$SKIP_GEN" = "1" ]; then
  SW5_FILE=$(ls "${OUT}/workflow-raw.yml" 2>/dev/null)
fi

if [ -z "$SW5_FILE" ]; then
  echo "[P${N}] Phase 1: meta-v6 generation (~45min)"
  docker restart whitt-llama-server > /dev/null 2>&1
  sleep 8
  cd "$REPO"

  # Patch meta-v6.yml to use THIS prompt (workflow hardcodes prompt-14 path)
  ORCHESTRATOR_TMP=$(mktemp --suffix=.yml)
  sed "s|prompt-14-task-add-true-parallel-inference-for-same-model-multi-target\.md|$(basename "$PROMPT_FILE")|g" \
    ./docs/benchmarks/workflows/meta-workflow-v6.yml > "$ORCHESTRATOR_TMP"

  timeout 3600 ./target/release/whitt benchmark \
    --workflow "$ORCHESTRATOR_TMP" \
    --output-dir "${OUT}/meta-out" \
    --models-dir "${REPO}/models" \
    --filter-name "Qwen3-5-9B" \
    --load-timeout 900 \
    > "${OUT}/meta.log" 2>&1 || {
    echo "[P${N}] FAIL: meta-v6 generation"
    rm -f "$ORCHESTRATOR_TMP"
    exit 2
  }
  rm -f "$ORCHESTRATOR_TMP"

  # Locate latest SW5 output by timestamp sort (avoid find -newer bug + sort bug)
  # Pattern requires digits after meta-v6- to exclude old "iter5", "sw2-fix", "q8" named dirs
  # that sort AFTER digit-only timestamps due to ASCII order (letters > digits)
  SW5_FILE=$(find "${REPO}/docs/benchmarks/outputs/meta-workflow" -maxdepth 1 -type d -name "meta-meta-v6-20260[0-9][0-9][0-9]-*-sw5-*" 2>/dev/null | sort | tail -1 | xargs -I{} find "{}" -maxdepth 2 -name "03-assembled.yml" 2>/dev/null | head -1)
  [ -z "$SW5_FILE" ] && SW5_FILE=$(find "${REPO}/docs/benchmarks/outputs/meta-workflow" -maxdepth 1 -type d -name "meta-meta-v6-20260[0-9][0-9][0-9]-*-sw5-*" 2>/dev/null | sort | tail -1 | xargs -I{} find "{}" -maxdepth 2 -name "workflow.yml" 2>/dev/null | head -1)
  [ -z "$SW5_FILE" ] && { echo "[P${N}] FAIL: no SW5 output"; exit 3; }

  cp "$SW5_FILE" "${OUT}/workflow-raw.yml"
fi

echo "[P${N}] SW5 source: $SW5_FILE"

# Phase 2: Strip markdown fences
echo "[P${N}] Phase 2: strip markdown fences"
python3 -c "
import re, sys
content = open('${OUT}/workflow-raw.yml').read()
content = re.sub(r'^\`\`\`(?:yaml|yml)?\s*\n', '', content, flags=re.MULTILINE)
content = re.sub(r'\n\`\`\`\s*$', '', content, flags=re.MULTILINE)
open('${OUT}/workflow-stripped.yml', 'w').write(content)
print('stripped')
"

# Phase 3: Canonicalize indent (optional, may fail gracefully)
echo "[P${N}] Phase 3: canonicalize (skip if crashes)"
python3 "${REPO}/scripts/meta-v6/canonicalize-workflow.py" "${OUT}/workflow-stripped.yml" > "${OUT}/canonicalize.log" 2>&1
if [ ! -f "${OUT}/workflow-canonical.yml" ]; then
  echo "[P${N}] canonicalize produced no output, using stripped file as-is"
  cp "${OUT}/workflow-stripped.yml" "${OUT}/workflow-canonical.yml"
fi

# Phase 4: Inject shell hooks
echo "[P${N}] Phase 4: inject shell hooks"
cp "${OUT}/workflow-canonical.yml" "${OUT}/workflow-fixed.yml"
python3 "${REPO}/scripts/meta-v6/inject-shell-hooks.py" "${OUT}/workflow-fixed.yml" > "${OUT}/inject.log" 2>&1 || true

# Validate
python3 -c "import yaml; yaml.safe_load(open('${OUT}/workflow-fixed.yml'))" 2>&1 || {
  echo "[P${N}] FAIL: workflow-fixed.yml does not parse, retrying with stripped file"
  cp "${OUT}/workflow-stripped.yml" "${OUT}/workflow-fixed.yml"
  python3 "${REPO}/scripts/meta-v6/inject-shell-hooks.py" "${OUT}/workflow-fixed.yml" > "${OUT}/inject.log" 2>&1 || true
  python3 -c "import yaml; yaml.safe_load(open('${OUT}/workflow-fixed.yml'))" 2>&1 || {
    echo "[P${N}] FAIL: workflow-fixed.yml STILL does not parse"
    exit 4
  }
}
echo "[P${N}] workflow-valid: YES"

# Phase 5: Execute workflow
echo "[P${N}] Phase 5: execute workflow (~15min)"
# Optimization (C1): skip docker restart when running batch (saves ~25s per prompt).
# Set SKIP_DOCKER_RESTART=1 to enable. Default behavior unchanged for single-prompt runs.
if [ "${SKIP_DOCKER_RESTART:-0}" != "1" ]; then
  docker restart whitt-llama-server > /dev/null 2>&1
  sleep 8
fi
cd "$REPO"

# Backup src/
BACKUP="${OUT}/src-backup"
mkdir -p "$BACKUP"
rsync -a --exclude=target --exclude=outputs --exclude=models --exclude=.git "${REPO}/src/" "$BACKUP/" 2>/dev/null

rm -rf "${OUT}/exec"
mkdir -p "${OUT}/exec"
timeout 1800 ./target/release/whitt benchmark \
  --workflow "${OUT}/workflow-fixed.yml" \
  --output-dir "${OUT}/exec" \
  --models-dir "${REPO}/models" \
  --filter-name "Qwen3-[45]" \
  --load-timeout 60 \
  > "${OUT}/exec.log" 2>&1
EXEC_RC=$?

# Restore src/
rsync -a --delete "$BACKUP/" "${REPO}/src/" 2>/dev/null

if [ "$EXEC_RC" -ne 0 ]; then
  echo "[P${N}] exec exit=${EXEC_RC}"
fi

# Phase 6: Score
echo "[P${N}] Phase 6: score"
bash "${REPO}/scripts/meta-v6/parity-check.sh" "${OUT}/workflow-fixed.yml" "${OUT}/exec" > "${OUT}/exec-score.txt" 2>&1
STRUCT=$(grep "^STRUCTURAL:" "${OUT}/exec-score.txt" 2>/dev/null | awk '{print $2}')
TOTAL=$(grep "^TOTAL:" "${OUT}/exec-score.txt" 2>/dev/null | awk '{print $2}')
VERDICT=$(grep "^VERDICT:" "${OUT}/exec-score.txt" 2>/dev/null | awk '{$1=""; print $0}' | sed 's/^ *//')

# Count real step executions
STEPS_EXEC=$(grep -c "^\[.*\] step_name=" "${OUT}/exec.log" 2>/dev/null || echo 0)
REFUSALS=$(grep -cE "cannot access|cannot read|do not have access" "${OUT}/exec.log" 2>/dev/null || echo 0)

echo ""
echo "=== [P${N}] RESULT ==="
echo "  struct:     ${STRUCT}"
echo "  exec total: ${TOTAL}"
echo "  verdict:    ${VERDICT}"
echo "  steps ran:  ${STEPS_EXEC}"
echo "  refusals:   ${REFUSALS}"
echo ""
echo "${N}|${STRUCT}|${TOTAL}|${STEPS_EXEC}|${REFUSALS}|${VERDICT}" >> "${RESULTS_DIR}/summary-cycle-3.tsv"
