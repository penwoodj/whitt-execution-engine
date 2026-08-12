#!/bin/bash
# Pre-flight safety checks. Run BEFORE any whitt benchmark call.
# Returns 0 if safe to proceed, 1 if any check fails.
#
# Hard rules enforced (per AGENTS.md):
#   - RAM available ≥ 3 GB (else abort)
#   - Swap used < 4 GB (else abort — sign of pressure)
#   - Docker container healthy
#   - llama.cpp /health responds
#   - No vk::DeviceLostError in recent docker logs
#   - Load avg (5min) < 8.0
#   - Max 1 model loaded at a time
#   - Disk space /tmp ≥ 1 GB free
set -uo pipefail

THRESHOLD_RAM_AVAIL_GB=3
THRESHOLD_SWAP_USED_GB=4
THRESHOLD_LOAD_5MIN=8.0
THRESHOLD_TMP_MB=1024
SERVER_URL="${SERVER_URL:-http://localhost:8080}"

fail() {
  echo "[preflight] FAIL: $*" >&2
  exit 1
}

ok() {
  echo "[preflight] OK: $*"
}

echo "[preflight] running safety checks..."

# 1. RAM check (available, not free — available includes reclaimable cache)
RAM_AVAIL_KB=$(awk '/MemAvailable/{print $2}' /proc/meminfo)
RAM_AVAIL_GB=$((RAM_AVAIL_KB / 1024 / 1024))
if [ "${RAM_AVAIL_GB}" -lt "${THRESHOLD_RAM_AVAIL_GB}" ]; then
  fail "RAM available ${RAM_AVAIL_GB}GB < ${THRESHOLD_RAM_AVAIL_GB}GB threshold. Aborting to prevent OOM crash."
fi
ok "RAM available ${RAM_AVAIL_GB}GB ≥ ${THRESHOLD_RAM_AVAIL_GB}GB"

# 2. Swap pressure check
SWAP_USED_KB=$(awk '/SwapTotal/{t=$2}/SwapFree/{f=$2}END{print t-f}' /proc/meminfo)
SWAP_USED_GB=$((SWAP_USED_KB / 1024 / 1024))
if [ "${SWAP_USED_GB}" -ge "${THRESHOLD_SWAP_USED_GB}" ]; then
  fail "Swap used ${SWAP_USED_GB}GB ≥ ${THRESHOLD_SWAP_USED_GB}GB threshold. System under memory pressure."
fi
ok "Swap used ${SWAP_USED_GB}GB < ${THRESHOLD_SWAP_USED_GB}GB"

# 3. Load average check (5-min)
LOAD_5=$(awk '{print $2}' /proc/loadavg)
LOAD_OK=$(awk -v l="${LOAD_5}" -v t="${THRESHOLD_LOAD_5MIN}" 'BEGIN{print (l<t)?"1":"0"}')
if [ "${LOAD_OK}" != "1" ]; then
  fail "Load 5-min avg ${LOAD_5} ≥ ${THRESHOLD_LOAD_5MIN} threshold. System overloaded."
fi
ok "Load 5-min ${LOAD_5} < ${THRESHOLD_LOAD_5MIN}"

# 4. Disk space in /tmp
TMP_FREE_MB=$(df -m /tmp | awk 'NR==2{print $4}')
if [ "${TMP_FREE_MB}" -lt "${THRESHOLD_TMP_MB}" ]; then
  fail "/tmp free ${TMP_FREE_MB}MB < ${THRESHOLD_TMP_MB}MB threshold."
fi
ok "/tmp free ${TMP_FREE_MB}MB ≥ ${THRESHOLD_TMP_MB}MB"

# 5. Docker container health
if ! docker ps --format '{{.Names}}' | grep -q '^whitt-llama-server$'; then
  fail "whitt-llama-server container not running. Start with: docker compose -f /home/jon/code/whitt-execution-engine/docker/docker-compose.yml up -d"
fi
ok "whitt-llama-server container running"

# 6. llama.cpp /health endpoint
HEALTH=$(curl -s -m 5 "${SERVER_URL}/health" 2>/dev/null || echo "")
if [[ "${HEALTH}" != *"\"status\":\"ok\""* ]]; then
  fail "llama.cpp /health not OK: '${HEALTH}'. Try: docker restart whitt-llama-server && sleep 30"
fi
ok "llama.cpp /health OK"

# 7. Vulkan device-loss check (last 200 log lines)
RECENT_LOGS=$(docker logs whitt-llama-server --tail 200 2>&1 || true)
if echo "${RECENT_LOGS}" | grep -qE "vk::DeviceLostError|Vulkan.*Error|DeviceLost"; then
  fail "Vulkan device-loss detected in recent docker logs. Restart required: docker restart whitt-llama-server && sleep 60"
fi
ok "No Vulkan device-loss in recent logs"

# 8. Concurrent loaded model count (max 1)
LOADED_COUNT=$(curl -s -m 5 "${SERVER_URL}/v1/models" 2>/dev/null | \
  python3 -c "import json,sys;d=json.load(sys.stdin);print(sum(1 for m in d.get('data',[]) if m.get('status',{}).get('value')=='loaded'))" 2>/dev/null || echo "0")
if [ "${LOADED_COUNT}" -gt 1 ]; then
  fail "${LOADED_COUNT} models loaded simultaneously. Never load 2+ models (VRAM exceeded). Unload all but one."
fi
ok "Loaded models: ${LOADED_COUNT} (≤1)"

echo "[preflight] all checks passed"
exit 0
