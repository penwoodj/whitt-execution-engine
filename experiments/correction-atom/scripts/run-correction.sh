#!/bin/bash
# Runner for correction atom experiment.
# Reuses safeguard pattern from atomic-reasoning/scripts/run-atom.sh
# Hard-enforces: preflight, sequential, RAM watchdog, 60s cooldown, 180s timeout.
#
# Usage: run-correction.sh <workflow-yml> <case-yml> <output-dir>
set -uo pipefail

REPO_ROOT="/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor"
MAIN_REPO="/home/jon/code/whitt-execution-engine"
SCRIPTS="${REPO_ROOT}/experiments/atomic-reasoning/scripts"
WHITT="${MAIN_REPO}/target/release/whitt"
MODELS_DIR="${MAIN_REPO}/models"

YML="${1:?usage: run-correction.sh <yml> <case> <outdir> [model] [timeout]}"
CASE="${2:?missing case file}"
OUTDIR="${3:?missing output dir}"
MODEL="${4:-Ministral-3}"
TIMEOUT_SECS="${5:-600}"
COOLDOWN_FILE="${REPO_ROOT}/experiments/correction-atom/results/.last-run-ts"
COOLDOWN_SECS=60

CASE_ABS="$(readlink -f "${CASE}")"
OUTDIR_ABS="$(readlink -f "${OUTDIR}")"
YML_ABS="$(readlink -f "${YML}")"

mkdir -p "${OUTDIR_ABS}"
RUN_YML="${OUTDIR_ABS}/runtime.yml"
RUN_LOG="${OUTDIR_ABS}/run.log"

# Preflight
if ! bash "${SCRIPTS}/preflight.sh" >> "${RUN_LOG}" 2>&1; then
  echo "[run-correction] PREFLIGHT FAILED — aborting" | tee -a "${RUN_LOG}"
  exit 2
fi

wait_for_ram() {  # $1=min_mb  $2=max_wait_secs
  local waited=0
  while [ "${waited}" -lt "${2}" ]; do
    local avail
    avail=$(free -m | awk '/Mem:/{print $7}')
    if [ "${avail}" -ge "${1}" ]; then
      return 0
    fi
    sleep 5
    waited=$((waited + 5))
  done
  return 1
}

# Cooldown: RAM-gated (AGENTS.md batch policy: >=3GB between model runs).
# Replaces fixed 60s wait + 30s post-run sleep (~90s dead time per case).
if [ -f "${COOLDOWN_FILE}" ]; then
  LAST_TS=$(cat "${COOLDOWN_FILE}")
  NOW_TS=$(date +%s)
  ELAPSED=$((NOW_TS - LAST_TS))
  if [ "${ELAPSED}" -lt "${COOLDOWN_SECS}" ]; then
    if ! wait_for_ram 3072 120; then
      echo "[run-correction] RAM low after wait — restarting docker" | tee -a "${RUN_LOG}"
      docker restart whitt-llama-server >> "${RUN_LOG}" 2>&1 || true
      sleep 45
    fi
  fi
fi

# Path substitution
sed \
  -e "s|__REPO_ROOT__|${REPO_ROOT}|g" \
  -e "s|__OUTPUT_DIR__|${OUTDIR_ABS}|g" \
  -e "s|__CASE_FILE__|${CASE_ABS}|g" \
  "${YML_ABS}" > "${RUN_YML}"

echo "[run-correction] case=${CASE_ABS}" | tee -a "${RUN_LOG}"
echo "[run-correction] output=${OUTDIR_ABS}" | tee -a "${RUN_LOG}"
echo "[run-correction] model=${MODEL}" | tee -a "${RUN_LOG}"
echo "[run-correction] timeout=${TIMEOUT_SECS}s" | tee -a "${RUN_LOG}"

# Unload non-target models
CURRENT_LOADED=$(curl -s -m 5 http://localhost:8080/v1/models 2>/dev/null | \
  python3 -c "import json,sys;d=json.load(sys.stdin);print(' '.join(m['id'] for m in d.get('data',[]) if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null || echo "")
for M in ${CURRENT_LOADED}; do
  if [[ "${M}" != *"${MODEL}"* ]]; then
    echo "[run-correction] unloading '${M}'" | tee -a "${RUN_LOG}"
    curl -s -m 30 -X POST http://localhost:8080/models/unload -H "Content-Type: application/json" -d "{\"model\":\"${M}\"}" >> "${RUN_LOG}" 2>&1 || true
    sleep 5
  fi
done

date +%s > "${COOLDOWN_FILE}"
START=$(date +%s)

cd "${REPO_ROOT}" || exit 1

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

bash "${SCRIPTS}/ram-watchdog.sh" "${WHITT_PID}" 2 &
WATCHDOG_PID=$!

wait "${WHITT_PID}" 2>/dev/null
EXIT=$?

kill "${WATCHDOG_PID}" 2>/dev/null || true
wait "${WATCHDOG_PID}" 2>/dev/null || true

END=$(date +%s)
DUR=$((END - START))

if [ ${EXIT} -eq 124 ]; then
  echo "[run-correction] TIMEOUT after ${TIMEOUT_SECS}s" | tee -a "${RUN_LOG}"
elif [ ${EXIT} -ne 0 ]; then
  echo "[run-correction] FAILED exit=${EXIT} duration=${DUR}s" | tee -a "${RUN_LOG}"
else
  echo "[run-correction] SUCCESS exit=0 duration=${DUR}s" | tee -a "${RUN_LOG}"
fi

# Write metrics (angle outcomes from select-best.json — early-exit aware)
python3 - "${OUTDIR_ABS}" "${DUR}" "${EXIT}" <<'PYEOF' || true
import glob
import json
import os
import sys

outdir, dur, exit_code = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
angle_files = sorted(glob.glob(os.path.join(outdir, "after-angle-*.txt")))
angles_present = len(angle_files)
selected_angle, case_passed = None, None
sb_path = os.path.join(outdir, "select-best.json")
if os.path.exists(sb_path):
    try:
        sb = json.load(open(sb_path))
        selected_angle = sb.get("selected_angle")
        case_passed = sb.get("passed")
    except Exception:
        pass
metrics = {
    "wall_clock_seconds": dur,
    "exit_code": exit_code,
    "timed_out": exit_code == 124,
    "angles_completed": angles_present,
    "total_angles": 5,
    "selected_angle": selected_angle,
    "case_passed": case_passed,
}
with open(os.path.join(outdir, "metrics.json"), "w") as f:
    json.dump(metrics, f, indent=2)
print(f"[run-correction] metrics: {angles_present}/5 angle files, {dur}s, exit={exit_code}, selected={selected_angle}, passed={case_passed}")
PYEOF

# Post-run: settle + RAM gate only (fixed 30s sleep removed)
wait_for_ram 3072 60 || true

exit ${EXIT}
