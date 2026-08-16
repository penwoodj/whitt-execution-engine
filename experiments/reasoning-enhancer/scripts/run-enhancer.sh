#!/bin/bash
# Live runner for reasoning-enhancer atom. NOT executed during scaffolding —
# machine busy with another experiment (user directive). Runbook: ../RUNBOOK.md.
# Mirrors correction-atom run-correction.sh safeguard pattern; reuses
# atomic-reasoning preflight/ram-watchdog. Hard-enforces: preflight,
# sequential, RAM watchdog, RAM-gated cooldown, 300s timeout cap (SAFETY.md R4).
#
# Usage: run-enhancer.sh <case-yml> [timeout-secs]
#   Results: experiments/reasoning-enhancer/results/<ts>-<case_id>/
set -uo pipefail

REPO_ROOT="/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor"
MAIN_REPO="/home/jon/code/whitt-execution-engine"
SCRIPTS="${REPO_ROOT}/experiments/atomic-reasoning/scripts"
WHITT="${MAIN_REPO}/target/release/whitt"
MODELS_DIR="${MAIN_REPO}/models"
REA="${REPO_ROOT}/experiments/reasoning-enhancer"
YML="${REA_WORKFLOW:-${REA}/workflows/rea-v1.yml}"

CASE="${1:?usage: run-enhancer.sh <case-yml> [timeout-secs]}"
TIMEOUT_SECS="${2:-300}"
COOLDOWN_FILE="${REA}/results/.last-run-ts"
COOLDOWN_SECS=60

CASE_ABS="$(readlink -f "${CASE}")"
CASE_ID="$(python3 -c "import yaml,sys;print(yaml.safe_load(open(sys.argv[1]))['case_id'])" "${CASE_ABS}")"
STAMP="$(date +%Y%m%d-%H%M%S)"
OUTDIR_ABS="${REA}/results/${STAMP}-${CASE_ID}"

mkdir -p "${OUTDIR_ABS}"
RUN_YML="${OUTDIR_ABS}/runtime.yml"
RUN_LOG="${OUTDIR_ABS}/run.log"

PREFLIGHT_LOG="${OUTDIR_ABS}/preflight.log"
if ! bash "${SCRIPTS}/preflight.sh" >> "${RUN_LOG}" 2>&1; then
  # Shared gate aborts on swap-USED (pressure proxy). Long-uptime machines
  # accumulate unrelated cold pages (rust-analyzer, LSP servers) with ZERO
  # swap activity. Override: when swap-used is the ONLY failure, gate on
  # actual swap ACTIVITY (si/so over 5s) instead. Real thrash still aborts.
  tail -20 "${RUN_LOG}" > "${PREFLIGHT_LOG}"
  if grep -q 'FAIL: Swap used' "${PREFLIGHT_LOG}" && \
     [ "$(grep -c 'FAIL:' "${PREFLIGHT_LOG}")" -eq 1 ]; then
    ACTIVITY=$(vmstat 1 8 | awk 'NR>2 {si+=$7; so+=$8} END {print si+so}')
    if [ "${ACTIVITY}" -lt 25000 ]; then
      echo "[rea] swap-used gate overridden: si+so=${ACTIVITY}KB/8s = no thrash (cold pages from unrelated procs)" | tee -a "${RUN_LOG}"
    else
      echo "[rea] PREFLIGHT FAILED — ACTIVE swap thrash (si+so=${ACTIVITY}KB/8s). Aborting." | tee -a "${RUN_LOG}"
      exit 2
    fi
  else
    echo "[rea] PREFLIGHT FAILED — aborting" | tee -a "${RUN_LOG}"
    exit 2
  fi
fi

wait_for_ram() {
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

if [ -f "${COOLDOWN_FILE}" ]; then
  LAST_TS=$(cat "${COOLDOWN_FILE}")
  ELAPSED=$(( $(date +%s) - LAST_TS ))
  if [ "${ELAPSED}" -lt "${COOLDOWN_SECS}" ]; then
    if ! wait_for_ram 3072 120; then
      echo "[rea] RAM low after wait — restarting docker" | tee -a "${RUN_LOG}"
      docker restart whitt-llama-server >> "${RUN_LOG}" 2>&1 || true
      sleep 45
    fi
  fi
fi

sed \
  -e "s|__REPO_ROOT__|${REPO_ROOT}|g" \
  -e "s|__OUTPUT_DIR__|${OUTDIR_ABS}|g" \
  -e "s|__CASE_FILE__|${CASE_ABS}|g" \
  "${YML}" > "${RUN_YML}"

echo "[rea] case=${CASE_ABS} id=${CASE_ID}" | tee -a "${RUN_LOG}"
echo "[rea] output=${OUTDIR_ABS}" | tee -a "${RUN_LOG}"
echo "[rea] timeout=${TIMEOUT_SECS}s" | tee -a "${RUN_LOG}"

# Sequential-only: unload everything first — REA swaps 1.7B/4B mid-run
# via unload_unused, so the server must start empty. Exception: when
# REA_KEEP_LOADED names the workflow's resident model (baseline phase
# keeps 9B loaded to avoid per-run 5.6GB load storms), skip unloading it.
CURRENT_LOADED=$(curl -s -m 5 http://localhost:8080/v1/models 2>/dev/null | \
  python3 -c "import json,sys;d=json.load(sys.stdin);print(' '.join(m['id'] for m in d.get('data',[]) if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null || echo "")
for M in ${CURRENT_LOADED}; do
  if [ -n "${REA_KEEP_LOADED:-}" ] && [ "${M}" = "${REA_KEEP_LOADED}" ]; then
    echo "[rea] keeping '${M}' loaded (REA_KEEP_LOADED)" | tee -a "${RUN_LOG}"
    continue
  fi
  echo "[rea] unloading '${M}'" | tee -a "${RUN_LOG}"
  curl -s -m 30 -X POST http://localhost:8080/models/unload -H "Content-Type: application/json" -d "{\"model\":\"${M}\"}" >> "${RUN_LOG}" 2>&1 || true
  sleep 5
done

date +%s > "${COOLDOWN_FILE}"
START=$(date +%s)

cd "${REPO_ROOT}" || exit 1

export WHITT_MAX_CONCURRENT_INFERENCES=1
timeout "${TIMEOUT_SECS}" "${WHITT}" benchmark \
  --workflow "${RUN_YML}" \
  --output-dir "${OUTDIR_ABS}" \
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

DUR=$(( $(date +%s) - START ))

if [ ${EXIT} -eq 124 ]; then
  echo "[rea] TIMEOUT after ${TIMEOUT_SECS}s" | tee -a "${RUN_LOG}"
elif [ ${EXIT} -ne 0 ]; then
  echo "[rea] FAILED exit=${EXIT} duration=${DUR}s" | tee -a "${RUN_LOG}"
else
  echo "[rea] SUCCESS exit=0 duration=${DUR}s" | tee -a "${RUN_LOG}"
fi

python3 - "${OUTDIR_ABS}" "${DUR}" "${EXIT}" <<'PYEOF' || true
import json, os, sys

outdir, dur, exit_code = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
rounds = sum(os.path.exists(os.path.join(outdir, f"enhanced-answer-r{i}.txt")) for i in (1, 2))
mode = selected = passed = None
sb_path = os.path.join(outdir, "select-best.json")
if os.path.exists(sb_path):
    try:
        sb = json.load(open(sb_path))
        mode, selected, passed = sb.get("mode"), sb.get("selected"), sb.get("passed")
    except Exception:
        pass
metrics = {
    "wall_clock_seconds": dur,
    "exit_code": exit_code,
    "timed_out": exit_code == 124,
    "rounds_completed": rounds,
    "mode": mode,
    "selected": selected,
    "case_passed": passed,
}
with open(os.path.join(outdir, "metrics.json"), "w") as f:
    json.dump(metrics, f, indent=2)
print(f"[rea] metrics: {rounds} rounds, {dur}s, exit={exit_code}, mode={mode}, passed={passed}")
PYEOF

wait_for_ram 3072 60 || true

exit ${EXIT}
