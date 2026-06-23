#!/bin/bash
# scripts/meta-v6/exec-existing-workflow.sh
# Re-execute existing cycle-2 workflow using inject-shell-hooks + line-range loading.
# Skips 45min generation phase by reusing cycle-2/workflow-fixed.yml.

set -uo pipefail
REPO="/home/jon/code/whitt-execution-engine"
RESULTS_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-3-results"

N="${1:?usage: $0 <prompt_number>}"

SRC="${REPO}/docs/plans/meta-workflow-parity/cycle-2-results/prompt-${N}/workflow-fixed.yml"
[ -f "$SRC" ] || { echo "[P${N}] no cycle-2 source workflow"; exit 1; }

OUT="${RESULTS_DIR}/prompt-${N}"
mkdir -p "${OUT}"

# Copy cycle-2 workflow as raw
cp "$SRC" "${OUT}/workflow-raw.yml"

# Strip markdown fences
python3 -c "
import re
content = open('${OUT}/workflow-raw.yml').read()
content = re.sub(r'^\`\`\`(?:yaml|yml)?\s*\n', '', content, flags=re.MULTILINE)
content = re.sub(r'\n\`\`\`\s*$', '', content, flags=re.MULTILINE)
open('${OUT}/workflow-stripped.yml', 'w').write(content)
"
cp "${OUT}/workflow-stripped.yml" "${OUT}/workflow-fixed.yml"

# Inject shell hooks with line-range optimization
python3 "${REPO}/scripts/meta-v6/inject-shell-hooks.py" "${OUT}/workflow-fixed.yml" > "${OUT}/inject.log" 2>&1 || true

# Validate
python3 -c "import yaml; yaml.safe_load(open('${OUT}/workflow-fixed.yml'))" 2>&1 || {
  echo "[P${N}] FAIL: workflow-fixed.yml does not parse"
  exit 4
}
echo "[P${N}] workflow-valid: YES"

# Restart docker
docker restart whitt-llama-server > /dev/null 2>&1
sleep 8

# Backup src/
BACKUP="${OUT}/src-backup"
mkdir -p "$BACKUP"
rsync -a --exclude=target --exclude=outputs --exclude=models --exclude=.git "${REPO}/src/" "$BACKUP/" 2>/dev/null

# Execute
rm -rf "${OUT}/exec"
mkdir -p "${OUT}/exec"
cd "$REPO"
timeout 1800 ./target/release/whitt benchmark \
  --workflow "${OUT}/workflow-fixed.yml" \
  --output-dir "${OUT}/exec" \
  --models-dir "${REPO}/models" \
  --filter-name "Qwen3-5-9B" \
  --load-timeout 60 \
  > "${OUT}/exec.log" 2>&1
EXEC_RC=$?

# Restore src/
rsync -a --delete "$BACKUP/" "${REPO}/src/" 2>/dev/null

# Score
bash "${REPO}/scripts/meta-v6/parity-check.sh" "${OUT}/workflow-fixed.yml" "${OUT}/exec" > "${OUT}/exec-score.txt" 2>&1
TOTAL=$(grep "^TOTAL:" "${OUT}/exec-score.txt" | awk '{print $2}')
VERDICT=$(grep "^VERDICT:" "${OUT}/exec-score.txt" | awk '{$1=""; print $0}' | sed 's/^ *//')
STEPS_EXEC=$(grep -c "Hook log:" "${OUT}/exec.log" 2>/dev/null || echo 0)
REFUSALS=$(grep -cE "cannot access|cannot read|do not have access" "${OUT}/exec.log" 2>/dev/null || echo 0)

echo "[P${N}] exec_rc=${EXEC_RC} total=${TOTAL} verdict=${VERDICT} steps=${STEPS_EXEC} refuses=${REFUSALS}"
echo "${N}|?|${TOTAL}|${STEPS_EXEC}|${REFUSALS}|${VERDICT}" >> "${RESULTS_DIR}/summary-cycle-3.tsv"
