#!/bin/bash
# Direct-answer run of one specialist model on the stage2 train suite.
# Mirrors run-probe-model.sh mechanics; emits check-*.json + summary.json
# into results/stage2-direct/<model>/.
# Usage: run-stage2-model.sh <model-name>
set -uo pipefail

REPO_ROOT="/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor"
MAIN_REPO="/home/jon/code/whitt-execution-engine"
WHITT="${MAIN_REPO}/target/release/whitt"
MODELS_DIR="${MAIN_REPO}/models"
EXP="${REPO_ROOT}/experiments/reasoning-enhancer-plus"
MODEL="${1:?usage: run-stage2-model.sh <model>}"
OUT="${EXP}/results/stage2-direct/${MODEL}"
WF="${EXP}/workflows/stage2-${MODEL}.yml"
RAM_OK=3072

avail=$(free -m | awk '/Mem:/{print $7}')
if [ "${avail}" -lt "${RAM_OK}" ]; then
  echo "[safety] only ${avail}MB RAM free — restarting docker"
  docker restart whitt-llama-server >/dev/null 2>&1
  sleep 40
fi

ZOMBIES=$(docker exec whitt-llama-server ps aux 2>/dev/null | grep -c "[l]lama-server")
if [[ "${ZOMBIES}" -gt 2 ]]; then
  echo "[safety] ${ZOMBIES} zombie llama-server procs -> restart + settle"
  docker restart whitt-llama-server >/dev/null
  sleep 40
fi

curl -s -m 3 http://localhost:8080/health >/dev/null || {
  echo "ABORT: llama server unhealthy"; exit 2; }

for LOADED in $(curl -s -m 5 http://localhost:8080/models | python3 -c "import json,sys;print(' '.join(m['id'] for m in json.load(sys.stdin)['data'] if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null); do
  [[ "${LOADED}" == "${MODEL}" ]] && continue
  curl -s -m 30 -X POST http://localhost:8080/models/unload \
    -H "Content-Type: application/json" -d "{\"model\":\"${LOADED}\"}" >/dev/null 2>&1 || true
  sleep 5
done

for ATTEMPT in 1 2 3; do
  curl -s -m 240 -X POST http://localhost:8080/models/load \
    -H "Content-Type: application/json" \
    -d "{\"model\":\"${MODEL}\",\"n_gpu_layers\":99}" >/dev/null 2>&1 || true
  ST=$(python3 "${EXP}/scripts/wait-loaded.py" --model "${MODEL}" --timeout 150 2>/dev/null)
  [[ "${ST}" == "loaded" ]] && break
  echo "[preload] attempt ${ATTEMPT} state=${ST} — restarting server"
  docker restart whitt-llama-server >/dev/null 2>&1
  sleep 40
done
[[ "${ST}" == "loaded" ]] || { echo "ABORT: preload failed"; exit 3; }
echo "[preload] ${MODEL} loaded"

python3 "${EXP}/scripts/gen-probe-workflow.py" --model "${MODEL}" \
  --stage2 --out-dir "${OUT}" --out "${WF}" >/dev/null || exit 4

rm -f "${OUT}"/check-*.json "${OUT}"/ans-*.txt
mkdir -p "${OUT}"
export WHITT_MAX_CONCURRENT_INFERENCES=1
"${WHITT}" benchmark --workflow "${WF}" --output-dir "${OUT}" \
  --models-dir "${MODELS_DIR}" --load-timeout 180 --prompts 1 \
  > "${OUT}/run-$(date +%s).log" 2>&1
RC=$?

python3 "${EXP}/scripts/collect-probe.py" --run-dir "${OUT}" || RC=5
exit "${RC}"
