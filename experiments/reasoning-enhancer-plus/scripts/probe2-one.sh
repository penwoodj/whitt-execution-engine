#!/bin/bash
# One-model probe driver for the 2026-08-18 think on/off sweep.
# Usage: probe2-one.sh <model> <off|on>
# - off: unload others, load, capability+smoke check, battery with
#   /no_think (if capable) at 250 tok; always-on models run natural
#   mode at 3500 (cannot disable - noted); plain run for non-hybrid.
# - on:  battery with /think (if capable) at 3500 tok. Non-hybrid
#   models: skipped (aggregator reuses off result - identical config).
# Resume-safe: skips when summary.json exists.
set -uo pipefail

REPO_ROOT="/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor"
MAIN_REPO="/home/jon/code/whitt-execution-engine"
WHITT="${MAIN_REPO}/target/release/whitt"
EXP="${REPO_ROOT}/experiments/reasoning-enhancer-plus"
ROOT="${EXP}/results/probe-think"
MODEL="${1:?model}"
MODE="${2:?off|on}"
CAP_JSON="${ROOT}/capability.json"
STAMP=$(date +%s)

mkdir -p "${ROOT}"
OUT="${ROOT}/${MODEL}/${MODE}"
[[ -f "${OUT}/summary.json" ]] && { echo "[skip] ${MODEL} ${MODE} done"; exit 0; }

cap_get() { python3 - "${CAP_JSON}" "${MODEL}" "$1" <<'PY'
import json, sys
try:
    d = json.load(open(sys.argv[1]))
    m = d.get(sys.argv[2], {})
    print(m.get(sys.argv[3], ""))
except Exception:
    print("")
PY
}

cap_put() { python3 - "${CAP_JSON}" "${MODEL}" "$1" "$2" <<'PY'
import json, sys, os, tempfile
p, model, k, v = sys.argv[1:5]
d = {}
if os.path.exists(p):
    try: d = json.load(open(p))
    except Exception: d = {}
d.setdefault(model, {})[k] = v
t = tempfile.NamedTemporaryFile("w", dir=os.path.dirname(p), delete=False)
json.dump(d, t, indent=2); t.close(); os.replace(t.name, p)
PY
}

unload_all() {
  for L in $(curl -s -m 5 http://localhost:8080/models | python3 -c "import json,sys;print(' '.join(m['id'] for m in json.load(sys.stdin)['data'] if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null); do
    [[ "$L" == "$MODEL" ]] && continue
    curl -s -m 30 -X POST http://localhost:8080/models/unload -H "Content-Type: application/json" -d "{\"model\":\"${L}\"}" >/dev/null 2>&1 || true
    sleep 3
  done
}

ensure_loaded() {
  ST=$(python3 "${EXP}/scripts/wait-loaded.py" --model "${MODEL}" --timeout 20 2>/dev/null)
  [[ "$ST" == "loaded" ]] && return 0
  unload_all
  for A in 1 2 3; do
    curl -s -m 240 -X POST http://localhost:8080/models/load -H "Content-Type: application/json" -d "{\"model\":\"${MODEL}\",\"n_gpu_layers\":99}" >/dev/null 2>&1 || true
    ST=$(python3 "${EXP}/scripts/wait-loaded.py" --model "${MODEL}" --timeout 150 2>/dev/null)
    [[ "$ST" == "loaded" ]] && return 0
    docker restart whitt-llama-server >/dev/null 2>&1; sleep 45
  done
  return 1
}

avail=$(free -m | awk '/Mem:/{print $7}')
if [ "${avail}" -lt 3072 ]; then
  docker restart whitt-llama-server >/dev/null 2>&1; sleep 45
fi
Z=$(docker exec whitt-llama-server ps aux 2>/dev/null | grep -c "[l]lama-server")
if [ "${Z}" -gt 2 ]; then
  docker restart whitt-llama-server >/dev/null 2>&1; sleep 45
fi
curl -s -m 3 http://localhost:8080/health >/dev/null || { echo "ABORT unhealthy"; exit 2; }

if ! ensure_loaded; then
  cap_put load "fail"; echo "[load-fail] ${MODEL}"; exit 3
fi
echo "[loaded] ${MODEL}"

if [[ "${MODE}" == "off" ]]; then
  TMPL=$(curl -s -m 10 http://localhost:8080/props | python3 -c "import json,sys;print(json.load(sys.stdin).get('chat_template',''))" 2>/dev/null || echo "")
  if echo "${TMPL}" | grep -qE "enable_thinking|/no_think"; then CAP=yes; else CAP=no; fi
  if [[ "${MODEL}" =~ Thinking|reasoning ]]; then CAP="always"; fi
  cap_put think_capable "${CAP}"
  SMOKE=$(curl -s -m 120 http://localhost:8080/v1/chat/completions -H "Content-Type: application/json" -d "{\"model\":\"${MODEL}\",\"max_tokens\":60,\"messages\":[{\"role\":\"user\",\"content\":\"Reply with exactly OK\"}]}" | python3 -c "import json,sys
j=json.load(sys.stdin)
c=(j.get('choices') or [{}])[0].get('message',{})
print('OK' if (c.get('content') or '').strip() else 'EMPTY')" 2>/dev/null || echo ERR)
  cap_put smoke "${SMOKE}"
  echo "[cap] ${MODEL} think=${CAP} smoke=${SMOKE}"
  if [[ "${SMOKE}" != "OK" ]]; then
    echo "[smoke-fail] ${MODEL} — still running battery for record"; fi
fi
CAP=$(cap_get think_capable)
[[ -z "${CAP}" ]] && CAP=unknown

DIRECTIVE=""; MAXTOK=250
if [[ "${MODE}" == "off" ]]; then
  if [[ "${CAP}" == "yes" ]]; then DIRECTIVE="/no_think"; fi
  if [[ "${CAP}" == "always" ]]; then MAXTOK=3500; fi
else
  if [[ "${CAP}" == "no" ]]; then
    echo "[skip] ${MODEL} non-hybrid — on≡off, aggregator reuses"; exit 0
  fi
  if [[ "${CAP}" == "yes" ]]; then DIRECTIVE="/think"; fi
  MAXTOK=3500
fi
if [[ "${CAP}" == "unknown" ]]; then
  case "${MODEL}" in
    *Thinking*|*reasoning*) [[ "${MODE}" == "on" ]] && MAXTOK=3500 || MAXTOK=3500 ;;
  esac
fi

WF="${EXP}/workflows/p2-${MODEL}-${MODE}.yml"
python3 "${EXP}/scripts/gen-probe-workflow.py" --model "${MODEL}" \
  ${DIRECTIVE:+--directive "${DIRECTIVE}"} --max-tok "${MAXTOK}" \
  --out-dir "${OUT}" --out "${WF}" >/dev/null || exit 4

mkdir -p "${OUT}"; rm -f "${OUT}"/check-*.json "${OUT}"/ans-*.txt
export WHITT_MAX_CONCURRENT_INFERENCES=1
timeout 780 "${WHITT}" benchmark --workflow "${WF}" --output-dir "${OUT}" \
  --models-dir "${MAIN_REPO}/models" --load-timeout 180 --prompts 1 \
  > "${OUT}/run-${STAMP}.log" 2>&1
RC=$?
python3 "${EXP}/scripts/collect-probe.py" --run-dir "${OUT}" || RC=5
echo "[done] ${MODEL} ${MODE} rc=${RC} cap=${CAP} tok=${MAXTOK} dir=${DIRECTIVE:-none}"
exit "${RC}"
