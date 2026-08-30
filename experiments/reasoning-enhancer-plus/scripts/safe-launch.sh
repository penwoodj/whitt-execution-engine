#!/bin/bash
# Safe launcher for live agentic workflow runs.
# Exists because of the 2026-08-23 incident: manual model load left
# resident + launch without unload preamble + WHITT_ZOMBIE_MAX=8
# masking preflight = 3 concurrent model children on an 8 GB card =
# VRAM spill -> swap thrash -> unresponsive machine.
# Usage: safe-launch.sh <workflow.yml> <run-dir> <logfile>
set -uo pipefail

WF="${1:?workflow yml}"
RD="${2:?run dir}"
LOG="${3:?log path}"

LOADED=$(curl -s -m 5 http://localhost:8080/models | python3 -c "import json,sys;print(' '.join(m['id'] for m in json.load(sys.stdin)['data'] if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null)
if [ -n "${LOADED}" ]; then
  echo "[safe-launch] unloading before start: ${LOADED}"
  for M in ${LOADED}; do
    curl -s -m 60 -X POST http://localhost:8080/models/unload \
      -H "Content-Type: application/json" -d "{\"model\":\"${M}\"}" >/dev/null
    sleep 5
  done
fi

CHILDREN=$(docker exec whitt-llama-server ps aux 2>/dev/null | grep -c "[l]lama-server" )
# children count includes router's grep line? no: [l] excludes grep; router has --models-dir so count-1
LOAD_CHILDREN=$((CHILDREN - 1))
if [ "${LOAD_CHILDREN}" -gt 0 ]; then
  echo "[safe-launch] ABORT: ${LOAD_CHILDREN} model child(ren) still alive after unload — restart router manually"
  exit 2
fi

curl -s -m 5 http://localhost:8080/health >/dev/null || {
  echo "[safe-launch] ABORT: router unhealthy"; exit 3; }

FREE_MB=$(free -m | awk '/Mem:/{print $7}')
if [ "${FREE_MB}" -lt 6000 ]; then
  echo "[safe-launch] ABORT: only ${FREE_MB}MB RAM free (need 6000) — let swap drain"
  exit 4
fi

# NOTE: deliberately NO WHITT_ZOMBIE_MAX here. Default threshold 2 is
# the guard that stops multi-child oversubscription. The override
# exists ONLY for zero-load spoof runs.
mkdir -p "${RD}"
setsid nohup env WHITT_MAX_CONCURRENT_INFERENCES=1 \
  /home/jon/code/whitt-execution-engine/target/release/whitt benchmark \
  --workflow "${WF}" --output-dir "${RD}" \
  --models-dir /home/jon/code/whitt-execution-engine/models \
  --load-timeout 180 --prompts 1 --cooldown 0 --min-tmp-space 50 \
  > "${LOG}" 2>&1 < /dev/null &
echo "[safe-launch] launched pid $! (clean state, guards passed)"
