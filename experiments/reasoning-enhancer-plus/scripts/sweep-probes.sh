#!/usr/bin/env bash
# Full 10-model sweep via skill orchestrator (resume-safe, cooldowns).
# Smallest-portion-first: smoke one model before this.
set -uo pipefail
python3 ~/.config/opencode/skills/rea-model-probe/probe.py --sweep "$@"
