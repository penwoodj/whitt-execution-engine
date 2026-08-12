#!/bin/bash
# Safe runner for atomic reasoning experiments.
# Hard-enforces: preflight, sequential (concurrent=1), RAM watchdog, 60s cooldown,
#                hard timeout per run, max consecutive tests before forced cooldown.
#
# Usage: run-atom.sh <variant-yml> <input-file> <output-dir> [model-filter] [timeout-secs]
set -uo pipefail

REPO_ROOT="/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor"
MAIN_REPO="/home/jon/code/whitt-execution-engine"
SCRIPTS="${REPO_ROOT}/experiments/atomic-reasoning/scripts"
WHITT="${MAIN_REPO}/target/release/whitt"
MODELS_DIR="${MAIN_REPO}/models"

YML="${1:?usage: run-atom.sh <yml> <input> <outdir> [model] [timeout]}"
INPUT="${2:?missing input file}"
OUTDIR="${3:?missing output dir}"
MODEL="${4:-Qwen3-4B-Instruct}"
TIMEOUT_SECS="${5:-180}"
COOLDOWN_FILE="${REPO_ROOT}/experiments/atomic-reasoning/results/.last-run-ts"
COOLDOWN_SECS=60
RAM_WATCHDOG_THRESHOLD_GB=2

INPUT_ABS="$(readlink -f "${INPUT}")"
OUTDIR_ABS="$(readlink -f "${OUTDIR}")"
YML_ABS="$(readlink -f "${YML}")"

mkdir -p "${OUTDIR_ABS}"
RUN_YML="${OUTDIR_ABS}/runtime.yml"
RUN_LOG="${OUTDIR_ABS}/run.log"

# 1. PREFLIGHT
if ! bash "${SCRIPTS}/preflight.sh" >> "${RUN_LOG}" 2>&1; then
  echo "[run-atom] PREFLIGHT FAILED — aborting" | tee -a "${RUN_LOG}"
  exit 2
fi

# 2. COOLDOWN
if [ -f "${COOLDOWN_FILE}" ]; then
  LAST_TS=$(cat "${COOLDOWN_FILE}")
  NOW_TS=$(date +%s)
  ELAPSED=$((NOW_TS - LAST_TS))
  if [ "${ELAPSED}" -lt "${COOLDOWN_SECS}" ]; then
    WAIT=$((COOLDOWN_SECS - ELAPSED))
    echo "[run-atom] cooldown: waiting ${WAIT}s" | tee -a "${RUN_LOG}"
    sleep "${WAIT}"
  fi
fi

# 3. PATH SUBSTITUTION
sed \
  -e "s|__REPO_ROOT__|${REPO_ROOT}|g" \
  -e "s|__OUTPUT_DIR__|${OUTDIR_ABS}|g" \
  -e "s|__INPUT_FILE__|${INPUT_ABS}|g" \
  "${YML_ABS}" > "${RUN_YML}"

echo "[run-atom] input=${INPUT_ABS}" | tee -a "${RUN_LOG}"
echo "[run-atom] output=${OUTDIR_ABS}" | tee -a "${RUN_LOG}"
echo "[run-atom] model=${MODEL}" | tee -a "${RUN_LOG}"
echo "[run-atom] timeout=${TIMEOUT_SECS}s" | tee -a "${RUN_LOG}"

# 4. UNLOAD NON-TARGET MODELS
CURRENT_LOADED=$(curl -s -m 5 http://localhost:8080/v1/models 2>/dev/null | \
  python3 -c "import json,sys;d=json.load(sys.stdin);print(' '.join(m['id'] for m in d.get('data',[]) if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null || echo "")
for M in ${CURRENT_LOADED}; do
  if [[ "${M}" != *"${MODEL}"* ]]; then
    echo "[run-atom] unloading '${M}' (not target)" | tee -a "${RUN_LOG}"
    curl -s -m 30 -X POST http://localhost:8080/v1/models/unload -H "Content-Type: application/json" -d "{\"model\":\"${M}\"}" >> "${RUN_LOG}" 2>&1 || true
    sleep 5
  fi
done

# 5. START
date +%s > "${COOLDOWN_FILE}"
START=$(date +%s)

cd "${REPO_ROOT}" || exit 1

# 6. RUN WITH TIMEOUT + WATCHDOG (concurrent=1 prevents 4x parallelism crash)
# Force sequential via env var (engine auto-detects 4× which crashes 8GB systems)
export WHITT_MAX_CONCURRENT_INFERENCES=1
timeout "${TIMEOUT_SECS}" "${WHITT}" benchmark \
  --workflow "${RUN_YML}" \
  --output-dir "${OUTDIR_ABS}" \
  --filter-name "${MODEL}" \
  --models-dir "${MODELS_DIR}" \
  --load-timeout 60 \
  --prompts 1 \
  >> "${RUN_LOG}" 2>&1 &
WHITT_PID=$!

bash "${SCRIPTS}/ram-watchdog.sh" "${WHITT_PID}" "${RAM_WATCHDOG_THRESHOLD_GB}" &
WATCHDOG_PID=$!

wait "${WHITT_PID}" 2>/dev/null
EXIT=$?

kill "${WATCHDOG_PID}" 2>/dev/null || true
wait "${WATCHDOG_PID}" 2>/dev/null || true

END=$(date +%s)
DUR=$((END - START))

if [ ${EXIT} -eq 124 ]; then
  echo "[run-atom] TIMEOUT after ${TIMEOUT_SECS}s" | tee -a "${RUN_LOG}"
elif [ ${EXIT} -ne 0 ]; then
  echo "[run-atom] FAILED exit=${EXIT} duration=${DUR}s" | tee -a "${RUN_LOG}"
else
  echo "[run-atom] SUCCESS exit=0 duration=${DUR}s" | tee -a "${RUN_LOG}"
fi

# 7. METRICS CAPTURE
python3 - "${OUTDIR_ABS}/output/benchmark_report.json" "${OUTDIR_ABS}/metrics.json" "${DUR}" "${EXIT}" <<'PYEOF' || true
import json, sys
report_path, metrics_path, dur, exit_code = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
try:
    with open(report_path) as f:
        r = json.load(f)
    inferences = []
    for result in r.get('results', []):
        for inf in result.get('inference_results', []):
            inferences.append({
                'tokens': inf.get('tokens', 0),
                'duration_ms': inf.get('duration_ms', 0),
                'tokens_per_second': inf.get('tokens_per_second', 0),
            })
    metrics = {
        'wall_clock_seconds': dur,
        'exit_code': exit_code,
        'timed_out': exit_code == 124,
        'total_inferences': len(inferences),
        'inferences': inferences,
        'aggregate_tokens': sum(i['tokens'] for i in inferences),
        'avg_tokens_per_second': (sum(i['tokens_per_second'] for i in inferences) / len(inferences)) if inferences else 0,
    }
    with open(metrics_path, 'w') as f:
        json.dump(metrics, f, indent=2)
    print(f"[run-atom] metrics: {len(inferences)} inferences, {dur}s, exit={exit_code}")
except Exception as e:
    print(f"[run-atom] metrics capture failed: {e}", file=sys.stderr)
PYEOF

# 8. POST-RUN COOLDOWN
echo "[run-atom] post-run cooldown 30s" | tee -a "${RUN_LOG}"
sleep 30
bash "${SCRIPTS}/preflight.sh" >> "${RUN_LOG}" 2>&1 || true

# Move literal-${OUTPUT_DIR} outputs (legacy fallback)
LITERAL_DIR="${REPO_ROOT}/\${OUTPUT_DIR}"
if [ -d "${LITERAL_DIR}" ]; then
  mv "${LITERAL_DIR}"/* "${OUTDIR_ABS}/" 2>/dev/null || true
  rmdir "${LITERAL_DIR}" 2>/dev/null || true
fi

exit ${EXIT}
