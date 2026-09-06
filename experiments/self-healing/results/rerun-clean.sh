#!/bin/bash
set -u
cd /home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor
W=target/release/whitt
LOGD=experiments/self-healing/results/whitt-logs
WHITT_ZOMBIE_MAX=8 $W benchmark --workflow experiments/self-healing/workflows/generated-live-v2/sh-live-v2-da-gen-a-clean.yml --output-dir $LOGD > $LOGD/da-gen-a-clean-r2.log 2>&1
echo "da-gen-a-clean EXIT=$?"
