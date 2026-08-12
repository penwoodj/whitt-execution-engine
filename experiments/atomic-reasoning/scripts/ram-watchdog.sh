#!/bin/bash
# RAM watchdog. Runs in background while experiment executes.
# Kills the whitt benchmark process if RAM drops too low.
#
# Usage: ram-watchdog.sh <pid-to-kill> <threshold-gb>
# Background: ./ram-watchdog.sh $PID 2 &
set -uo pipefail

PID="${1:?usage: ram-watchdog.sh <pid> <threshold-gb>}"
THRESH="${2:-2}"
INTERVAL_SECS=5
LOG="/tmp/ram-watchdog-${PID}.log"

echo "[watchdog] monitoring PID ${PID}, threshold ${THRESH}GB available" > "${LOG}"

while kill -0 "${PID}" 2>/dev/null; do
  AVAIL_KB=$(awk '/MemAvailable/{print $2}' /proc/meminfo)
  AVAIL_GB=$((AVAIL_KB / 1024 / 1024))
  if [ "${AVAIL_GB}" -lt "${THRESH}" ]; then
    echo "[watchdog] $(date -Iseconds) RAM CRITICAL: ${AVAIL_GB}GB available < ${THRESH}GB threshold — killing ${PID}" >> "${LOG}"
    # Try graceful first, then SIGKILL after 3s
    kill -TERM "${PID}" 2>/dev/null || true
    sleep 3
    kill -KILL "${PID}" 2>/dev/null || true
    # Also kill any child whitt processes
    pkill -TERM -f "target/release/whitt" 2>/dev/null || true
    sleep 2
    pkill -KILL -f "target/release/whitt" 2>/dev/null || true
    exit 2
  fi
  echo "[watchdog] $(date -Iseconds) RAM available: ${AVAIL_GB}GB" >> "${LOG}"
  sleep "${INTERVAL_SECS}"
done

echo "[watchdog] PID ${PID} exited normally" >> "${LOG}"
exit 0
