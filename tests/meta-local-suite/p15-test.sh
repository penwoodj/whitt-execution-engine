#!/bin/bash
# Functional test for p15 (locreport). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/locreport.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
mkdir -p "$TMP/src/sub"
printf 'a\nb\nc\n'       > "$TMP/src/one.rs"     # 3 lines
printf 'x\ny\n'          > "$TMP/src/sub/two.rs" # 2 lines
printf 'not rust\n'      > "$TMP/src/readme.md"

OUT=$(python3 "$PY" "$TMP/src" 2>&1) || { echo "FUNC_FAIL: exited nonzero"; echo "$OUT" | head -3; exit 1; }
echo "$OUT"
echo "$OUT" | grep -q '| File | Lines |'   || { echo "FUNC_FAIL: header"; exit 1; }
echo "$OUT" | grep -qE '\| one\.rs \| 3 \|' || { echo "FUNC_FAIL: one.rs row"; exit 1; }
echo "$OUT" | grep -qE '\| sub/two\.rs \| 2 \|' || { echo "FUNC_FAIL: sub/two.rs row"; exit 1; }
echo "$OUT" | grep -qE '\| TOTAL \| 5 \|'  || { echo "FUNC_FAIL: TOTAL 5"; exit 1; }
echo "$OUT" | grep -q "readme" && { echo "FUNC_FAIL: non-rs file included"; exit 1; }
ERR=$(python3 "$PY" "$TMP/missing" 2>&1) && { echo "FUNC_FAIL: missing dir exit"; exit 1; }
echo "$ERR" | grep -qi traceback && { echo "FUNC_FAIL: traceback"; exit 1; }
echo "FUNC_PASS"
