#!/bin/bash
# scripts/meta-v6/parity-check.sh
# HONEST parity validator (replaces fraudulent cycle-3 script)
#
# Cycle-3 script gave false PASS for YAML parses + file >100 bytes.
# This script enforces REAL validation: deliverable exists, real content,
# no refusals, no meta-commentary, live execution evidence.
#
# Usage: parity-check.sh <workflow.yml> <deliverable-path> [exec-dir]

set -uo pipefail

WORKFLOW="${1:-}"
DELIVERABLE="${2:-}"
EXEC_DIR="${3:-}"

if [ -z "$WORKFLOW" ] || [ ! -f "$WORKFLOW" ]; then
  echo "Usage: $0 <workflow.yml> <deliverable-path> [exec-dir]"
  exit 1
fi

if [ -z "$DELIVERABLE" ]; then
  echo "FAIL: deliverable path required"
  exit 1
fi

TOTAL=0
MAX=50
FAILS=()

echo "=== HONEST Parity Check ==="
echo "Workflow: $WORKFLOW"
echo "Deliverable: $DELIVERABLE"
echo ""

# C1: YAML parses (5 pts)
echo -n "C1 YAML parses: "
if python3 -c "import yaml; yaml.safe_load(open('$WORKFLOW'))" 2>/dev/null; then
  echo "PASS (5/5)"; TOTAL=$((TOTAL+5))
else
  echo "FAIL (0/5)"; FAILS+=("C1 yaml-parse")
fi

# C2: Workflow has generative_entity + steps (5 pts)
echo -n "C2 workflow well-formed: "
if grep -q "generative_entity" "$WORKFLOW" && grep -q "steps:" "$WORKFLOW"; then
  echo "PASS (5/5)"; TOTAL=$((TOTAL+5))
else
  echo "FAIL (0/5)"; FAILS+=("C2 workflow-shape")
fi

# C3: Deliverable EXISTS (10 pts — critical)
echo -n "C3 deliverable exists: "
if [ -f "$DELIVERABLE" ]; then
  SIZE=$(stat -c %s "$DELIVERABLE")
  echo "PASS (10/10, size=${SIZE}B)"; TOTAL=$((TOTAL+10))
else
  echo "FAIL (0/10)"; FAILS+=("C3 deliverable-missing")
fi

# C4: Deliverable has real content (10 pts)
echo -n "C4 deliverable substantive: "
if [ -f "$DELIVERABLE" ]; then
  SIZE=$(stat -c %s "$DELIVERABLE")
  LINES=$(wc -l < "$DELIVERABLE")
  if [ "$SIZE" -gt 500 ] && [ "$LINES" -gt 10 ]; then
    echo "PASS (10/10, ${LINES} lines, ${SIZE}B)"; TOTAL=$((TOTAL+10))
  else
    echo "FAIL (0/10, too small: ${LINES}L/${SIZE}B)"; FAILS+=("C4 size=${SIZE}")
  fi
else
  echo "SKIP (no file)"; FAILS+=("C4 no-file")
fi

# C5: NO refusals in deliverable (10 pts — critical)
echo -n "C5 no refusals: "
if [ -f "$DELIVERABLE" ]; then
  REFUSALS=$(grep -cE "I cannot|I'm unable|I don't have access|As an AI|I would need to (read|access|see)" "$DELIVERABLE" 2>/dev/null || true)
  REFUSALS=${REFUSALS:-0}
  if [ "$REFUSALS" -eq 0 ]; then
    echo "PASS (10/10)"; TOTAL=$((TOTAL+10))
  else
    echo "FAIL (0/10, ${REFUSALS} refusal patterns)"; FAILS+=("C5 refusals=${REFUSALS}")
  fi
else
  echo "SKIP (no file)"; FAILS+=("C5 no-file")
fi

# C6: NO meta-commentary (5 pts)
echo -n "C6 no meta-commentary: "
if [ -f "$DELIVERABLE" ]; then
  META=$(grep -cE "To accomplish this|To do this, we need|I will (now|then)|Let me (start|begin|first)|First, I'll|Step [0-9]+:" "$DELIVERABLE" 2>/dev/null || true)
  META=${META:-0}
  if [ "$META" -le 2 ]; then
    echo "PASS (5/5, ${META} matches)"; TOTAL=$((TOTAL+5))
  else
    echo "FAIL (0/5, ${META} meta patterns)"; FAILS+=("C6 meta=${META}")
  fi
else
  echo "SKIP (no file)"; FAILS+=("C6 no-file")
fi

# C7: Live execution evidence (5 pts)
echo -n "C7 execution evidence: "
if [ -n "$EXEC_DIR" ] && [ -d "$EXEC_DIR" ]; then
  if ls "$EXEC_DIR"/workflow.log >/dev/null 2>&1 || ls "$EXEC_DIR"/benchmark.log >/dev/null 2>&1 || ls "$EXEC_DIR"/logs/*.log >/dev/null 2>&1; then
    echo "PASS (5/5)"; TOTAL=$((TOTAL+5))
  else
    echo "FAIL (0/5, no logs)"; FAILS+=("C7 no-logs")
  fi
else
  echo "SKIP (no exec-dir)"
fi

# C8: Markdown fences stripped from deliverable (bonus, not required)
echo -n "C8 fences stripped: "
if [ -f "$DELIVERABLE" ]; then
  FENCES=$(grep -cE '^\`\`\`' "$DELIVERABLE" || echo 0)
  if [ "$FENCES" -eq 0 ]; then
    echo "PASS (+5 bonus)"; TOTAL=$((TOTAL+5))
  else
    echo "NOTE (${FENCES} fences present, no bonus)"
  fi
else
  echo "SKIP"
fi

echo ""
echo "=== Score: ${TOTAL}/${MAX} ==="

# Verdict
if [ ${#FAILS[@]} -gt 0 ]; then
  echo ""
  echo "FAILURES:"
  for f in "${FAILS[@]}"; do
    echo "  - $f"
  done
fi

echo ""
# Critical gates: C3 (exists), C5 (no refusals), C4 (substantive) MUST pass
if [ "$TOTAL" -ge 40 ] && [ -f "$DELIVERABLE" ]; then
  if grep -qE "I cannot|I'm unable|I don't have access" "$DELIVERABLE"; then
    echo "VERDICT: FAIL (refusals present despite score)"
    exit 1
  fi
  SIZE=$(stat -c %s "$DELIVERABLE")
  if [ "$SIZE" -lt 500 ]; then
    echo "VERDICT: FAIL (deliverable too small despite score)"
    exit 1
  fi
  echo "VERDICT: PASS"
  exit 0
else
  echo "VERDICT: FAIL"
  exit 1
fi
