#!/bin/bash
# Functional test for p13 (taskgraph). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/taskgraph.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/tasks.json" <<'EOF'
{"tasks": [
  {"id": "A", "duration": 3, "deps": []},
  {"id": "B", "duration": 2, "deps": ["A"]},
  {"id": "C", "duration": 4, "deps": ["A"]},
  {"id": "D", "duration": 1, "deps": ["B", "C"]}
]}
EOF
OUT=$(python3 "$PY" "$TMP/tasks.json" 2>&1) || { echo "FUNC_FAIL: exited nonzero"; echo "$OUT" | head -3; exit 1; }
echo "$OUT"
ORDER=$(echo "$OUT" | grep '^ORDER: ' | sed 's/^ORDER: //')
[ -n "$ORDER" ] || { echo "FUNC_FAIL: no ORDER line"; exit 1; }
# validate topological: A before B, A before C, B and C before D
python3 - "$ORDER" <<'PYEOF'
import sys
order = sys.argv[1].split()
assert sorted(order) == ["A", "B", "C", "D"], order
i = {t: n for n, t in enumerate(order)}
assert i["A"] < i["B"] and i["A"] < i["C"] and i["B"] < i["D"] and i["C"] < i["D"], order
print("TOPO_OK")
PYEOF
[ $? -eq 0 ] || { echo "FUNC_FAIL: invalid topological order [$ORDER]"; exit 1; }
echo "$OUT" | grep -q "^CRITICAL_PATH_LENGTH: 8$" || { echo "FUNC_FAIL: critical path (expect 8)"; exit 1; }
# cycle detection
cat > "$TMP/cycle.json" <<'EOF'
{"tasks": [
  {"id": "A", "duration": 1, "deps": ["B"]},
  {"id": "B", "duration": 1, "deps": ["A"]}
]}
EOF
python3 "$PY" "$TMP/cycle.json" >/dev/null 2>&1 && { echo "FUNC_FAIL: cycle should exit nonzero"; exit 1; }
echo "FUNC_PASS"
