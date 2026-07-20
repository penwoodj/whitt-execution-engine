#!/bin/bash
# Functional test for p06 (jsonlogs). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/jsonlogs.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
cat > "$TMP/app.jsonl" <<'EOF'
{"level": "info", "message": "started"}
{"level": "ERROR", "message": "disk full"}
not json at all
{"level": "warn", "message": "slow"}
{"level": "error", "message": "timeout"}
{"level": "info", "message": "done"}
EOF
OUT=$(python3 "$PY" "$TMP/app.jsonl" 2>&1) || { echo "FUNC_FAIL: exited nonzero: $(echo "$OUT" | head -2)"; exit 1; }
echo "$OUT" | grep -q "^LEVEL error: 2$"   || { echo "FUNC_FAIL: error count"; echo "$OUT"; exit 1; }
echo "$OUT" | grep -q "^LEVEL info: 2$"    || { echo "FUNC_FAIL: info count"; exit 1; }
echo "$OUT" | grep -q "^LEVEL warn: 1$"    || { echo "FUNC_FAIL: warn count"; exit 1; }
echo "$OUT" | grep -q "^ERROR_MSG: disk full$" || { echo "FUNC_FAIL: missing error msg 1"; exit 1; }
echo "$OUT" | grep -q "^ERROR_MSG: timeout$"   || { echo "FUNC_FAIL: missing error msg 2"; exit 1; }
echo "$OUT" | grep -q "^SKIPPED: 1$"       || { echo "FUNC_FAIL: skipped count"; exit 1; }
# missing file: nonzero exit, no traceback
ERR=$(python3 "$PY" "$TMP/nope.jsonl" 2>&1) && { echo "FUNC_FAIL: missing file should be nonzero"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback"; exit 1; }
echo "FUNC_PASS"
