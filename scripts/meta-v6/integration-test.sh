#!/bin/bash
# scripts/meta-v6/integration-test.sh
# Main integration test runner.
#
# Re-executes all 20 prompts through the full pipeline with current fixes applied:
#   meta-v6 generation → strip fences → inject hooks → execute → check quality
#   → extract/compile code → compare vs cycle-3 baseline → write JSON result
#
# Semi-automated: stops at checkpoint between each prompt (user can approve/inspect).
# Use `--no-stop` for fully automated run (e.g. CI).
#
# Pass criteria (per user spec, ALL three required):
#   1. SW_WINS (workflow achieves task — parity-check.sh verdict=PASS)
#   2. shell_fail_rate < 10% (in exec log)
#   3. zero refusals in step outputs
#
# Usage:
#   integration-test.sh [prompt-numbers...] [--no-stop] [--skip-gen]
#   integration-test.sh                  # run all 20, stop at each
#   integration-test.sh 05 06 07         # run only P05, P06, P07
#   integration-test.sh --no-stop        # run all 20 fully automated
#   integration-test.sh --skip-gen       # reuse existing SW5 outputs (faster)

set -uo pipefail

REPO="/home/jon/code/whitt-execution-engine"
PROMPTS_DIR="${REPO}/docs/plans/meta-workflow-qwen35/test-prompts/real"
RESULTS_ROOT="${REPO}/docs/benchmarks/outputs/meta-workflow/integration-tests"
RUN_ID="run-$(date +%Y%m%d-%H%M%S)"
RESULTS_DIR="${RESULTS_ROOT}/${RUN_ID}"
mkdir -p "${RESULTS_DIR}"

NO_STOP=0
SKIP_GEN=0
PROMPTS=()

# Arg parsing
while [ $# -gt 0 ]; do
  case "$1" in
    --no-stop) NO_STOP=1; shift ;;
    --skip-gen) SKIP_GEN=1; shift ;;
    --help|-h)
      grep '^#' "$0" | head -30
      exit 0 ;;
    *) PROMPTS+=("$1"); shift ;;
  esac
done

# Default: all 20 active prompts
if [ ${#PROMPTS[@]} -eq 0 ]; then
  for n in 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24; do
    PROMPTS+=("$n")
  done
fi

echo "=== Integration Test Runner ==="
echo "Run ID: ${RUN_ID}"
echo "Results: ${RESULTS_DIR}"
echo "Prompts: ${PROMPTS[*]}"
echo "No-stop: ${NO_STOP}  Skip-gen: ${SKIP_GEN}"
echo ""

SUMMARY_TSV="${RESULTS_DIR}/summary.tsv"
echo -e "prompt\tsw_wins\tshell_fail_rate\trefusals\tstep_quality\trust_compiles\tts_compiles\tduration_sec\tverdict" \
  > "$SUMMARY_TSV"

run_prompt() {
  local N="$1"
  local OUT="${RESULTS_DIR}/prompt-${N}"
  mkdir -p "$OUT"

  local PROMPT_FILE=$(ls "${PROMPTS_DIR}/prompt-${N}-"*.md 2>/dev/null | head -1)
  if [ -z "$PROMPT_FILE" ]; then
    echo "[P${N}] SKIP: no prompt file"
    return 1
  fi

  local START_TS=$(date +%s)
  echo "[P${N}] $(basename "$PROMPT_FILE")"

  # Phase 1-5: reuse run-prompt-end-to-end.sh
  if [ "$SKIP_GEN" = "1" ] && [ -d "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}" ]; then
    # Skip generation, reuse existing cycle-3 workflow + exec
    echo "[P${N}] SKIP-GEN: copying cycle-3 workflow + exec"
    cp "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/workflow-fixed.yml" \
       "${OUT}/workflow.yml" 2>/dev/null || true
    cp -r "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/exec" \
       "${OUT}/" 2>/dev/null || true
    cp "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/exec.log" \
       "${OUT}/exec.log" 2>/dev/null || true
    cp "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/exec-score.txt" \
       "${OUT}/exec-score.txt" 2>/dev/null || true
  fi

  if [ ! -f "${OUT}/workflow.yml" ]; then
    # Run full pipeline with NEW generator (includes 3 fixes)
    bash "${REPO}/scripts/meta-v6/run-prompt-end-to-end.sh" "$N" 0 \
      > "${OUT}/pipeline.log" 2>&1
    PIPE_RC=$?
    if [ $PIPE_RC -ne 0 ]; then
      echo "[P${N}] pipeline FAIL rc=${PIPE_RC}"
      echo -e "${N}\tfalse\t1.0\t999\tfalse\tfalse\tfalse\t0\tPIPELINE_FAIL" >> "$SUMMARY_TSV"
      return 2
    fi
    # Copy resulting workflow + exec from cycle-3 dir (run-prompt writes there)
    cp "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/workflow-fixed.yml" \
       "${OUT}/workflow.yml" 2>/dev/null || true
    cp -r "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/exec" \
       "${OUT}/" 2>/dev/null || true
    cp "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/exec.log" \
       "${OUT}/exec.log" 2>/dev/null || true
    cp "${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}/exec-score.txt" \
       "${OUT}/exec-score.txt" 2>/dev/null || true
  fi

  # Locate exec dir
  EXEC_DIR="${OUT}/exec"
  if [ ! -d "$EXEC_DIR" ]; then
    echo "[P${N}] no exec dir, SKIP quality checks"
    echo -e "${N}\tfalse\t1.0\t999\tfalse\tfalse\tfalse\t0\tNO_EXEC" >> "$SUMMARY_TSV"
    return 3
  fi

  # Check 1: step outputs (empty/refusal/hallucination/placeholder)
  echo "[P${N}] check-step-outputs..."
  bash "${REPO}/scripts/meta-v6/check-step-outputs.sh" "$EXEC_DIR" \
    > "${OUT}/step-quality.json" 2>&1 || true

  # Check 2: extract + compile Rust/TS
  echo "[P${N}] extract-and-compile..."
  bash "${REPO}/scripts/meta-v6/extract-and-compile.sh" "$EXEC_DIR" "${OUT}/compile-scratch" \
    > "${OUT}/compile.json" 2>&1 || true

  # Check 3: compare before/after
  echo "[P${N}] compare-before-after..."
  bash "${REPO}/scripts/meta-v6/compare-before-after.sh" "$N" "$EXEC_DIR" \
    > "${OUT}/comparison.json" 2>&1 || true

  # Aggregate verdict
  python3 -c "
import json, sys

# Load step quality
try:
    sq = json.load(open('${OUT}/step-quality.json'))
except Exception:
    sq = {'pass_step_quality': False, 'refusal': 999, 'total_steps': 0}

# Load compile result
try:
    cc = json.load(open('${OUT}/compile.json'))
except Exception:
    cc = {'rust': None, 'typescript': None}

rust_compiles = cc['rust']['compiles'] if cc.get('rust') else None
ts_compiles   = cc['typescript']['compiles'] if cc.get('typescript') else None

# Read exec log for shell stats
import re
log = open('${OUT}/exec.log').read() if __import__('os').path.exists('${OUT}/exec.log') else ''
shell_total = len(re.findall(r'\\[shell\\]', log))
shell_fail  = len(re.findall(r'shell.*(?:FAIL|Error|failed)', log, re.I))
shell_rate  = (shell_fail / shell_total) if shell_total > 0 else 0.0

# Read parity score
score_text = open('${OUT}/exec-score.txt').read() if __import__('os').path.exists('${OUT}/exec-score.txt') else ''
sw_wins = 'VERDICT: PASS' in score_text

refusals = sq.get('refusal', 0)
step_q   = sq.get('pass_step_quality', False)

# Pass criteria: ALL three
verdict = 'PASS' if (sw_wins and shell_rate < 0.10 and refusals == 0) else 'FAIL'

result = {
    'prompt': 'P${N}',
    'sw_wins': sw_wins,
    'shell_fail_rate': round(shell_rate, 3),
    'shell_total': shell_total,
    'shell_fail': shell_fail,
    'refusals': refusals,
    'step_quality_pass': step_q,
    'rust_compiles': rust_compiles,
    'ts_compiles': ts_compiles,
    'verdict': verdict,
}
print(json.dumps(result, indent=2))
open('${OUT}/result.json', 'w').write(json.dumps(result, indent=2))

# Append to summary TSV
import sys
with open('${SUMMARY_TSV}', 'a') as f:
    f.write('${N}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n'.format(
        str(sw_wins).lower(),
        shell_rate,
        refusals,
        str(step_q).lower(),
        str(rust_compiles).lower() if rust_compiles is not None else 'na',
        str(ts_compiles).lower() if ts_compiles is not None else 'na',
        0,
        verdict,
    ))
"

  local END_TS=$(date +%s)
  local DUR=$((END_TS - START_TS))
  echo "[P${N}] done in ${DUR}s"

  # Checkpoint between prompts (unless --no-stop)
  if [ "$NO_STOP" = "0" ]; then
    echo ""
    echo "[CHECKPOINT] P${N} complete. Result: $(cat ${OUT}/result.json 2>/dev/null | python3 -c 'import json,sys; print(json.load(sys.stdin).get("verdict","?"))' 2>/dev/null || echo '?')"
    echo "  Outputs at: ${OUT}"
    echo "  Summary: ${SUMMARY_TSV}"
    read -p "  Continue to next prompt? [Y/n/q] " ans
    case "$ans" in
      [nN]*) echo "Stopping per user."; exit 130 ;;
      [qQ]*) echo "Quit."; exit 130 ;;
      *) ;;
    esac
  fi

  return 0
}

FAIL_COUNT=0
PASS_COUNT=0
for n in "${PROMPTS[@]}"; do
  run_prompt "$n" || true
done

# Final summary
echo ""
echo "=== Run Summary ==="
TOTAL=$((PASS_COUNT + FAIL_COUNT))
echo "Total: ${#PROMPTS[@]}  (see ${SUMMARY_TSV})"
column -t -s $'\t' "$SUMMARY_TSV" || cat "$SUMMARY_TSV"

# Generate dashboard
echo ""
echo "[Dashboard] generating HTML..."
python3 "${REPO}/scripts/meta-v6/generate-dashboard.py" \
  --run-id "$RUN_ID" \
  --results-dir "$RESULTS_DIR" \
  --summary-tsv "$SUMMARY_TSV" \
  > "${RESULTS_DIR}/dashboard.html" 2>&1 || echo "[Dashboard] generation failed (non-fatal)"

echo ""
echo "=== DONE: ${RUN_ID} ==="
echo "Results: ${RESULTS_DIR}"
echo "Dashboard: ${RESULTS_DIR}/dashboard.html"
