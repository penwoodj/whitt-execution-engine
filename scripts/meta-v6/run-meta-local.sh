#!/bin/bash
# scripts/meta-v6/run-meta-local.sh
# Full META-v6 pipeline on a local machine (macOS-friendly, no Docker):
#   prompt → SW1-SW5 generation → post-process → execute generated workflow
#
# Backend defaults to LM Studio (localhost:1234). Set WHITT_BACKEND=llamacpp
# to use the original Docker llama.cpp setup.
#
# Usage:
#   run-meta-local.sh <prompt-file> [output-label]
#   echo "build me X" | run-meta-local.sh - my-label
set -uo pipefail

WHITT_BACKEND="${WHITT_BACKEND:-lmstudio}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/env.sh"

PROMPT_ARG="${1:?usage: $0 <prompt-file|-> [label]}"
LABEL="${2:-local}"

RESULTS_DIR="${REPO}/docs/benchmarks/outputs/meta-workflow/local-runs/${LABEL}-$(date +%Y%m%d-%H%M%S)"
mkdir -p "${RESULTS_DIR}"

# Resolve prompt (file or stdin)
PROMPT_FILE="${RESULTS_DIR}/prompt.md"
if [[ "${PROMPT_ARG}" == "-" ]]; then
  cat > "${PROMPT_FILE}"
else
  cp "${PROMPT_ARG}" "${PROMPT_FILE}"
fi
[[ -s "${PROMPT_FILE}" ]] || { echo "FATAL: empty prompt"; exit 1; }

echo "=== META-v6 local run ==="
echo "backend:  ${WHITT_BACKEND}"
echo "repo:     ${REPO}"
echo "results:  ${RESULTS_DIR}"
echo "prompt:   $(head -c 120 "${PROMPT_FILE}")..."

# ── Phase 1: generation (SW1-SW5 orchestrated by meta-workflow-v6.yml) ──
ORCH="${RESULTS_DIR}/meta-workflow-v6-runtime.yml"
localize_workflow "${REPO}/docs/benchmarks/workflows/meta-workflow-v6.yml" "${ORCH}"
# Point the bootstrap hook at OUR prompt file instead of the hardcoded prompt-14
sed -e "s|docs/plans/meta-workflow-qwen35/test-prompts/real/prompt-14-task-add-true-parallel-inference-for-same-model-multi-target.md|${PROMPT_FILE}|g" \
  "${ORCH}" > "${ORCH}.tmp" && mv "${ORCH}.tmp" "${ORCH}"

cd "${REPO}"
echo "[phase 1] SW1-SW5 generation (this is the long part)"
./target/release/whitt benchmark \
  --workflow "${ORCH}" \
  --output-dir "${RESULTS_DIR}/meta-out" \
  $(model_flags) \
  --load-timeout 900 \
  > "${RESULTS_DIR}/meta.log" 2>&1
GEN_EXIT=$?

META_RUN_ID="$(cat "${REPO}/.current-meta-run" 2>/dev/null || true)"
GENERATED="${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}/meta/generated-workflow.yml"
if [[ ! -s "${GENERATED}" ]]; then
  echo "FAIL: no generated workflow (gen_exit=${GEN_EXIT}); see ${RESULTS_DIR}/meta.log"
  tail -30 "${RESULTS_DIR}/meta.log"
  exit 2
fi
cp "${GENERATED}" "${RESULTS_DIR}/workflow-raw.yml"
echo "[phase 1] OK: generated-workflow.yml ($(wc -c < "${GENERATED}") bytes, gen_exit=${GEN_EXIT})"

# ── Phase 2: post-process (strip fences → inject hooks → validate) ──
echo "[phase 2] post-process"
python3 - "$RESULTS_DIR" <<'PYEOF'
import re, sys
d = sys.argv[1]
content = open(f'{d}/workflow-raw.yml').read()
content = re.sub(r'^```(?:yaml|yml)?\s*\n', '', content, flags=re.MULTILINE)
content = re.sub(r'\n```\s*$', '', content, flags=re.MULTILINE)
open(f'{d}/workflow-fixed.yml', 'w').write(content)
print('stripped')
PYEOF
python3 "${REPO}/scripts/meta-v6/inject-shell-hooks.py" "${RESULTS_DIR}/workflow-fixed.yml" > "${RESULTS_DIR}/inject.log" 2>&1 || true
python3 -c "import yaml; yaml.safe_load(open('${RESULTS_DIR}/workflow-fixed.yml'))" || {
  echo "WARN: python yaml parse failed (Rust engine may still accept it)"
}

# ── Phase 3: execute the generated workflow ──
echo "[phase 3] execute generated workflow"
rm -rf "${RESULTS_DIR}/exec" && mkdir -p "${RESULTS_DIR}/exec"
WHITT_OUTPUT_DIR="${RESULTS_DIR}/exec" ./target/release/whitt benchmark \
  --workflow "${RESULTS_DIR}/workflow-fixed.yml" \
  --output-dir "${RESULTS_DIR}/exec" \
  $(model_flags) \
  --load-timeout 900 \
  > "${RESULTS_DIR}/exec.log" 2>&1
EXEC_EXIT=$?

echo "=== RESULT ==="
echo "exec_exit: ${EXEC_EXIT}"
echo "deliverables:"
find "${REPO}/docs/benchmarks/outputs/meta-workflow/${META_RUN_ID}" "${RESULTS_DIR}/exec" -path '*deliverables*' -type f 2>/dev/null | head
echo "run dir: ${RESULTS_DIR}"
exit ${EXEC_EXIT}
