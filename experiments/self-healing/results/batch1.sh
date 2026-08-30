#!/bin/bash
# batch1.sh — batch 1 live runs (9 flagship cases), sequential w/ RAM safety.
set -u
cd /home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor
W=target/release/whitt
LOGD=experiments/self-healing/results/whitt-logs
CASES="flg-02-rs-ana-a-f1 flg-03-da-ana-a-f2 flg-04-da-ana-b-f3 flg-05-oi-trb-b-f4 flg-06-se-gen-b-f1p flg-07-sc-trf-a-f2 flg-08-lc-pld-b-f3 flg-09-mc-gen-b-f4 flg-10-sc-trb-a-f4p"
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
