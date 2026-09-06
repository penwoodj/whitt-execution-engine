#!/bin/bash
# Functional test for p12 (configmerge). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/configmerge.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/base.json" <<'EOF'
{"a": {"x": 1, "y": 2, "deep": {"k": "old"}}, "list": [1, 2, 3], "keep": true}
EOF
cat > "$TMP/over.json" <<'EOF'
{"a": {"y": 9, "z": 3, "deep": {"k": "new"}}, "list": [5]}
EOF
OUT=$(python3 "$PY" "$TMP/base.json" "$TMP/over.json" 2>&1) || { echo "FUNC_FAIL: exited nonzero"; echo "$OUT" | head -3; exit 1; }
python3 - "$OUT" <<'PYEOF'
import json, sys
m = json.loads(sys.argv[1])
assert m["a"]["x"] == 1 and m["a"]["y"] == 9 and m["a"]["z"] == 3, m
assert m["a"]["deep"]["k"] == "new", m
assert m["list"] == [5], m
assert m["keep"] is True, m
print("MERGE_OK")
PYEOF
[ $? -eq 0 ] || { echo "FUNC_FAIL: merge assertions"; exit 1; }
echo '{bad json' > "$TMP/bad.json"
ERR=$(python3 "$PY" "$TMP/base.json" "$TMP/bad.json" 2>&1) && { echo "FUNC_FAIL: bad json should exit nonzero"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback"; exit 1; }
echo "FUNC_PASS"
