#!/bin/bash
# Functional test for p10 (csv2json). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/csv2json.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/data.csv" <<'EOF'
name,age,score,active,note
alice,30,91.5,true,
bob,25,88,FALSE,hi there
EOF
OUT=$(python3 "$PY" "$TMP/data.csv" 2>&1) || { echo "FUNC_FAIL: exited nonzero"; echo "$OUT" | head -3; exit 1; }
python3 - "$OUT" <<'PYEOF'
import json, sys
rows = json.loads(sys.argv[1])
assert isinstance(rows, list) and len(rows) == 2, f"rows={len(rows)}"
a, b = rows
assert a["name"] == "alice" and isinstance(a["age"], int) and a["age"] == 30, a
assert isinstance(a["score"], float) and a["score"] == 91.5, a
assert a["active"] is True and a["note"] is None, a
assert b["active"] is False and b["note"] == "hi there", b
assert isinstance(b["score"], (int, float)) and b["score"] == 88, b
print("JSON_OK")
PYEOF
[ $? -eq 0 ] || { echo "FUNC_FAIL: json assertions"; exit 1; }
ERR=$(python3 "$PY" "$TMP/nope.csv" 2>&1) && { echo "FUNC_FAIL: missing file exit"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback"; exit 1; }
echo "FUNC_PASS"
