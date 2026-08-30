#!/usr/bin/env bash
# Probe one model: preflight -> generate workflow -> run -> collect.
# Usage: run-probe-model.sh MODEL [--limit N]
# Resume: skips if results/model-probe/MODEL/summary.json exists (unless --limit).
set -uo pipefail

MODEL="${1:?model required}"; shift || true
LIMIT=""
[[ "${1:-}" == "--limit" ]] && LIMIT="$2"

EXP="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO="$(cd "$EXP/../.." && pwd)"
OUT="$EXP/results/model-probe/$MODEL"
WHITT="/home/jon/code/whitt-execution-engine/target/release/whitt"

RAM_GB=$(free -b | awk '/^Mem:/{printf "%.1f", $NF/1e9}')
RAM_OK=$(free -b | awk '/^Mem:/{v=$NF/1e9; print (v>=3.0)?1:0}')
[[ "$RAM_OK" == 1 ]] || { echo "ABORT: RAM avail ${RAM_GB}GB < 3GB"; exit 2; }

SWAP_MB=$(vmstat 1 2 | tail -1 | awk '{printf "%.1f", ($7+$8)*4096/1e6}')
SWAP_OK=$(vmstat 1 2 | tail -1 | awk '{v=($7+$8)*4096/1e6; print (v<=25)?1:0}')
[[ "$SWAP_OK" == 1 ]] || { echo "ABORT: swap activity ${SWAP_MB}MB > 25MB"; exit 2; }

if [[ -f "$OUT/summary.json" && -z "$LIMIT" ]]; then
  echo "[skip] $OUT/summary.json exists"; exit 0
fi

curl -s -m 3 http://localhost:8080/health >/dev/null || {
  echo "ABORT: llama server unhealthy"; exit 2; }

ZOMBIES=$(docker exec whitt-llama-server ps aux 2>/dev/null | grep -c "[l]lama-server")
if [[ "$ZOMBIES" -gt 2 ]]; then
  echo "[safety] $ZOMBIES zombie llama-server procs -> restart + settle"
  docker restart whitt-llama-server >/dev/null; sleep 35
fi

if docker logs --tail 50 whitt-llama-server 2>&1 | \
   grep -qE 'vk::|DeviceLost'; then
  echo "[safety] Vulkan error -> restart + settle"
  docker restart whitt-llama-server; sleep 30
fi

mkdir -p "$OUT"
LOADED=$(curl -s -m 5 http://localhost:8080/models 2>/dev/null | python3 -c 'import json,sys
try:
  print(" ".join(m if isinstance(m,str) else m.get("id",m.get("name","")) for m in json.load(sys.stdin).get("models",[])))
except Exception: print("")')
for M in $LOADED; do
  [[ "$M" == *"$MODEL"* ]] && continue
  curl -s -m 30 -X POST http://localhost:8080/models/unload \
    -H "Content-Type: application/json" -d "{\"model\":\"$M\"}" >/dev/null 2>&1 || true
  sleep 5
done

PRELOAD_OK=0
for ATTEMPT in 1 2 3; do
  ST=$(python3 "$EXP/scripts/wait-loaded.py" --model "$MODEL" --timeout 10 || true)
  [[ "$ST" == "loaded" ]] && { PRELOAD_OK=1; break; }
  curl -s -m 240 -X POST http://localhost:8080/models/load \
    -H "Content-Type: application/json" \
    -d "{\"model\":\"$MODEL\",\"n_gpu_layers\":99}" >/dev/null 2>&1 || true
  ST=$(python3 "$EXP/scripts/wait-loaded.py" --model "$MODEL" --timeout 120 || true)
  [[ "$ST" == "loaded" ]] && { PRELOAD_OK=1; break; }
  echo "[preload] attempt $ATTEMPT failed ($ST) — restart + retry"
  docker restart whitt-llama-server >/dev/null; sleep 35
done
[[ "$PRELOAD_OK" == 1 ]] || { echo "ABORT: preload failed after 3 attempts"; exit 2; }
echo "[preload] $MODEL loaded"

GEN_ARGS=(--model "$MODEL" --out-dir "$OUT")
[[ -n "$LIMIT" ]] && GEN_ARGS+=(--limit "$LIMIT" --out "$EXP/workflows/probe-$MODEL-smoke.yml")
python3 "$EXP/scripts/gen-probe-workflow.py" "${GEN_ARGS[@]}"

WF="$EXP/workflows/probe-$MODEL.yml"
[[ -n "$LIMIT" ]] && WF="$EXP/workflows/probe-$MODEL-smoke.yml"

TS=$(date +%s)
set +e
export WHITT_MAX_CONCURRENT_INFERENCES=1
timeout 900 "$WHITT" benchmark \
  --workflow "$WF" \
  --output-dir "$OUT" \
  --models-dir /models \
  --load-timeout 180 \
  --prompts 1 \
  >> "$OUT/run-$TS.log" 2>&1
RC=$?
set -e

if [[ -f "$OUT/summary.json" ]]; then
  python3 -c "import json; d=json.load(open('$OUT/summary.json')); print('[done] $MODEL rc=$RC', d['total_pass'], d['category_pass'])"
  exit 0
fi

python3 "$EXP/scripts/collect-probe.py" --run-dir "$OUT" || true
if [[ -f "$OUT/summary.json" ]]; then
  echo "[done-recovered] $MODEL"; exit 0
fi
echo "[fail] $MODEL rc=$RC — no summary.json; log: $OUT/run-$TS.log"
exit 1
