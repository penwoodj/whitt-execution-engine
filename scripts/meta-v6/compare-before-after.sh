#!/bin/bash
# scripts/meta-v6/compare-before-after.sh
# Side-by-side diff of deliverables: pre-fix vs post-fix.
#
# "Before" = existing cycle-3 results in
#   docs/plans/meta-workflow-parity/cycle-3-results/prompt-XX/exec/
# "After" = new run in <new-exec-dir>.
#
# Compares:
#   - Step count (how many steps executed)
#   - Output sizes (per-step)
#   - Refusal pattern counts
#   - parity-check.sh scores
#
# Outputs JSON to stdout.
#
# Usage: compare-before-after.sh <prompt-num> <new-exec-dir>

set -uo pipefail

N="${1:?usage: $0 <prompt-num> <new-exec-dir>}"
NEW_EXEC="${2:?usage: $0 <prompt-num> <new-exec-dir>}"
REPO="/home/jon/code/whitt-execution-engine"
BEFORE_DIR="${REPO}/docs/plans/meta-workflow-parity/cycle-3-results/prompt-${N}"

if [ ! -d "$BEFORE_DIR" ]; then
  echo "{\"error\":\"no before dir for P${N}\",\"before\":null,\"after\":null}"
  exit 0
fi

if [ ! -d "$NEW_EXEC" ]; then
  echo "{\"error\":\"no after dir: $NEW_EXEC\",\"before\":null,\"after\":null}"
  exit 1
fi

# Step counts
before_steps=$(ls "${BEFORE_DIR}/exec/output"/step_*.txt 2>/dev/null | wc -l || echo 0)
after_steps=$(ls "${NEW_EXEC}/output"/step_*.txt 2>/dev/null | wc -l || echo 0)

# Total size
before_size=$(du -sb "${BEFORE_DIR}/exec/output" 2>/dev/null | awk '{print $1}')
after_size=$(du -sb "${NEW_EXEC}/output" 2>/dev/null | awk '{print $1}')
[ -z "$before_size" ] && before_size=0
[ -z "$after_size" ] && after_size=0

# Refusal counts (whole output dir)
REFUSAL_RE='I cannot|I'"'"'m unable|I don'"'"'t have access|As an AI|cannot access|cannot read|do not have access'

before_refusals=$(grep -rE "$REFUSAL_RE" "${BEFORE_DIR}/exec/output" 2>/dev/null | wc -l)
[ -z "$before_refusals" ] && before_refusals=0
after_refusals=$(grep -rE "$REFUSAL_RE" "${NEW_EXEC}/output" 2>/dev/null | wc -l)
[ -z "$after_refusals" ] && after_refusals=0

# parity-check scores if score files exist
before_score=""
if [ -f "${BEFORE_DIR}/exec-score.txt" ]; then
  before_score=$(grep -oE "TOTAL: [0-9]+/[0-9]+" "${BEFORE_DIR}/exec-score.txt" | head -1 | grep -oE "^[0-9]+")
fi
after_score=""
if [ -f "${NEW_EXEC}/../exec-score.txt" ]; then
  after_score=$(grep -oE "TOTAL: [0-9]+/[0-9]+" "${NEW_EXEC}/../exec-score.txt" | head -1 | grep -oE "^[0-9]+")
fi

# Per-step size comparison
python3 -c "
import os, json, sys
before_dir = '${BEFORE_DIR}/exec/output'
after_dir  = '${NEW_EXEC}/output'

def step_files(d):
    out = {}
    if not os.path.isdir(d):
        return out
    # Walk recursively; pick up step_*.* anywhere under exec tree
    for root, dirs, files in os.walk(d):
        for f in files:
            if f.startswith('step_') or f == 'deliverable.md':
                try:
                    rel = os.path.relpath(os.path.join(root, f), d)
                    out[rel] = os.path.getsize(os.path.join(root, f))
                except OSError:
                    out[rel] = 0
    return out

before = step_files(before_dir)
after  = step_files(after_dir)

all_files = sorted(set(list(before.keys()) + list(after.keys())))
per_step = []
for f in all_files:
    per_step.append({
        'file': f,
        'before_size': before.get(f, 0),
        'after_size': after.get(f, 0),
        'delta': after.get(f, 0) - before.get(f, 0),
    })

print(json.dumps({'per_step': per_step}))
"
