#!/bin/bash
# batch1.sh — batch 1 live runs (9 flagship cases), sequential w/ RAM safety.
set -u
cd /home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor
W=target/release/whitt
LOGD=experiments/self-healing/results/whitt-logs
CASES='oi-pld-a-clean oi-pld-b-f3 oi-trb-a-f4 oi-trf-a-f1p oi-trf-b-f2 px-ana-a-f2 px-ana-a-f4 px-ana-b-f3 px-gen-a-clean px-gen-b-f1'
for cid in $CASES; do
  avail=$(free -m | awk '/^Mem:/{print $7}')
  if [ "$avail" -lt 3000 ]; then
    echo "=== $cid low RAM ${avail}MB — docker restart + wait"
    docker restart whitt-llama-server
    sleep 60
  fi
  echo "=== $cid START $(date -Is) RAM=${avail}MB"
  WHITT_ZOMBIE_MAX=8 $W benchmark \
    --workflow experiments/self-healing/workflows/generated-live-v2/sh-live-v2-$cid.yml \
    --output-dir $LOGD > $LOGD/$cid-b1.log 2>&1
  echo "=== $cid EXIT=$? $(date -Is)"
  sleep 10
done
echo "BATCH1-DONE"
