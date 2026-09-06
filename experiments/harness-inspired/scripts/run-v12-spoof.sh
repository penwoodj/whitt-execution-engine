#!/usr/bin/env bash
# v12 spoof battery runner. Sequential whitt benchmark runs over generated
# spoof workflows. ZERO model loads: every generative step is skip_step'd;
# canned fixtures feed the REAL gate scripts.
#
# Usage: bash run-v12-spoof.sh <suite:hd|hc|all> [scenario-suffix:""]
# Env:   WHITT=<binary path> (default /home/jon/code/whitt-execution-engine/target/release/whitt)
#        OUT=<output dir>    (default experiments/harness-inspired/results/v12-spoof-output)
set -u
WHITT="${WHITT:-/home/jon/code/whitt-execution-engine/target/release/whitt}"
ROOT="$(git rev-parse --show-toplevel)"
OUT="${OUT:-experiments/harness-inspired/results/v12-spoof-output}"
SUITE="${1:-all}"
SUFFIX="${2:-}"
export WHITT_ZOMBIE_MAX=5
cd "$ROOT"
OUT_ABS="$ROOT/$OUT"

mkdir -p "$OUT"
: > "$OUT/battery.log"

case "$SUITE" in
  hd)  PAT="v12-hd-" ;;
  hc)  PAT="v12-hc-" ;;
  all) PAT="v12-" ;;
  *) echo "bad suite: $SUITE"; exit 2 ;;
esac

# Zero-LLM preflight: count llama-server children now; assert unchanged after.
LS_BEFORE=$(pgrep -c -f "llama-server" 2>/dev/null || echo 0)
echo "llama-server procs before: $LS_BEFORE" | tee -a "$OUT/battery.log"

PASS=0; FAIL=0; N=0
for WF in $ROOT/experiments/harness-inspired/workflows/${PAT}*${SUFFIX}-spoof.yml; do
  [ -e "$WF" ] || continue
  N=$((N+1))
  T0=$(date +%s)
  "$WHITT" benchmark --workflow "$WF" --output-dir "$OUT_ABS" >> "$OUT/battery.log" 2>&1
  RC=$?
  T1=$(date +%s)
  echo "case $(basename "$WF" -spoof.yml) rc=$RC secs=$((T1-T0))" >> "$OUT/battery.log"
done

LS_AFTER=$(pgrep -c -f "llama-server" 2>/dev/null || echo 0)
echo "llama-server procs after: $LS_AFTER" | tee -a "$OUT/battery.log"
echo "ran $N workflows" | tee -a "$OUT/battery.log"
