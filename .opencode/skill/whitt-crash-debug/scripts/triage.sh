#!/usr/bin/env bash
# triage.sh — post-crash evidence collection for whitt execution engine.
# Usage: triage.sh <whitt-run-log> [out-dir]
# Non-fatal per step: a crashed box may lack journalctl/dmesg rights.
set -u

RUN_LOG="${1:-}"
OUT="${2:-experiments/crash-reports/$(date +%Y%m%d-%H%M%S)}"
SERVER=whitt-llama-server

mkdir -p "$OUT"
log() { printf '%s\n' "$*" | tee -a "$OUT/evidence.md"; }

log "# Crash report $(date -Is)"
log "run_log: ${RUN_LOG:-<none>}"

# 1) Kernel OOM / kill events
if command -v journalctl >/dev/null 2>&1; then
  journalctl -k --since "-6h" 2>/dev/null | grep -iE "oom|out of memory|killed process|device lost|amdgpu.*error|gpu reset" > "$OUT/kernel-events.log" || true
fi
dmesg 2>/dev/null | grep -iE "oom|out of memory|killed process|amdgpu.*error|gpu reset" >> "$OUT/kernel-events.log" 2>/dev/null || true
log "kernel events: $(wc -l < "$OUT/kernel-events.log" 2>/dev/null || echo 0) lines (kernel-events.log)"

# 2) Docker server evidence
docker inspect "$SERVER" --format 'restarts={{.RestartCount}} oom={{.State.OOMKilled}} exit={{.State.ExitCode}} status={{.State.Status}}' > "$OUT/container-state.txt" 2>/dev/null || true
docker logs "$SERVER" 2>&1 | tail -n 400 > "$OUT/server-tail.log" 2>/dev/null || true
docker logs "$SERVER" 2>&1 | grep -iE "vk::|devicelost|vulkan.*error|outofmemory|failed to allocate|error" | tail -n 80 > "$OUT/server-errors.log" 2>/dev/null || true
log "server errors: $(wc -l < "$OUT/server-errors.log" 2>/dev/null || echo 0) lines (server-errors.log)"
log "container: $(cat "$OUT/container-state.txt" 2>/dev/null)"

# 3) Server config drift (ctx!)
docker exec "$SERVER" sh -c 'cat /proc/$(pgrep -f llama-server | head -1)/cmdline | tr "\0" " "' > "$OUT/server-cmdline.txt" 2>/dev/null || true
log "server cmdline: $(cat "$OUT/server-cmdline.txt" 2>/dev/null | grep -o '\-c [0-9]*' || echo '?')"

# 4) Whitt run log
if [ -n "$RUN_LOG" ] && [ -f "$RUN_LOG" ]; then
  cp "$RUN_LOG" "$OUT/whitt-run.log"
  grep -nE "RESOURCE_|max concurrent|already loaded|routed to|executing step|error|failed|500" "$RUN_LOG" | tail -n 120 > "$OUT/whitt-signals.log" || true
  log "whitt signals: $(wc -l < "$OUT/whitt-signals.log") lines (whitt-signals.log); full log copied"
else
  log "whitt run log missing — pass path as \$1 next time"
fi

# 5) Host memory snapshot (now, post-crash)
{ free -h; echo; cat /sys/class/drm/card*/device/mem_info_vram_total /sys/class/drm/card*/device/mem_info_vram_free 2>/dev/null; } > "$OUT/host-memory.txt" 2>/dev/null || true

# 6) Hypotheses starter
cat > "$OUT/hypotheses.md" <<'EOF'
| ID | Hypothesis | Evidence for | Evidence against | Test to write | Verdict |
|----|-----------|--------------|------------------|---------------|---------|
| H1 | External app consumed VRAM/RAM mid-inference |  |  | watchdog red-state test | |
| H2 | ctx/KV budget exceeded VRAM (config drift) |  |  | admit_load rejection test | |
| H3 | Concurrency >1 on 8GB card |  |  | detect_max_concurrent test | |
| H4 | Server slot error as HTTP 500, engine continued |  |  | step-fails propagation test | |
| H5 | Sensor unit bug (bytes vs KB) |  |  | sensor parser test | |
EOF

log "artifacts in $OUT — fill hypotheses.md, then TDD loop per SKILL.md"
