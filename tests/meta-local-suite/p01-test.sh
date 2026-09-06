#!/bin/bash
# Functional test for p01 (wordfreq). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/wordfreq.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

TMP=$(mktemp -d)
printf 'The quick brown fox. The dog! The fox ran; a quick, quick fox. THE end.\n' > "$TMP/sample.txt"

OUT=$("python3" "$PY" "$TMP/sample.txt" 10 2>/dev/null || python3 "$PY" "$TMP/sample.txt" 2>/dev/null)
echo "$OUT" | head -3
echo "$OUT" | grep -qE '^the: 4$' || { echo "FUNC_FAIL: 'the: 4' not first-style output"; exit 1; }
echo "$OUT" | grep -qE '^quick: 3$' || { echo "FUNC_FAIL: quick count wrong"; exit 1; }

# missing file must not crash with traceback
ERR=$(python3 "$PY" "$TMP/nope.txt" 10 2>&1 || true)
echo "$ERR" | grep -qi "traceback" && { echo "FUNC_FAIL: traceback on missing file"; exit 1; }
echo "FUNC_PASS"
