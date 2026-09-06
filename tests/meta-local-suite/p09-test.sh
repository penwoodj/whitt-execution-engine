#!/bin/bash
# Functional test for p09 (passgen). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
PY=$(ls "$ART"/passgen.py 2>/dev/null || ls "$ART"/*.py 2>/dev/null | head -1)
[ -z "$PY" ] && { echo "FUNC_FAIL: no python artifact"; exit 1; }

P1=$(python3 "$PY" --chars 24) || { echo "FUNC_FAIL: --chars errored"; exit 1; }
P2=$(python3 "$PY" --chars 24)
[ "${#P1}" -eq 24 ] || { echo "FUNC_FAIL: length ${#P1} != 24"; exit 1; }
[ "$P1" != "$P2" ] || { echo "FUNC_FAIL: two runs identical"; exit 1; }
echo "$P1" | grep -qE '^[A-Za-z0-9!@#$%^&*]+$' || { echo "FUNC_FAIL: charset [$P1]"; exit 1; }

W=$(python3 "$PY" --words 4) || { echo "FUNC_FAIL: --words errored"; exit 1; }
echo "$W" | grep -qE '^[a-z]+-[a-z]+-[a-z]+-[a-z]+$' || { echo "FUNC_FAIL: passphrase shape [$W]"; exit 1; }

python3 "$PY" >/dev/null 2>&1; [ $? -eq 2 ] || { echo "FUNC_FAIL: no-mode should exit 2"; exit 1; }
python3 "$PY" --chars 10 --words 3 >/dev/null 2>&1; [ $? -eq 2 ] || { echo "FUNC_FAIL: both-modes should exit 2"; exit 1; }
python3 "$PY" --chars -5 >/dev/null 2>&1; [ $? -eq 2 ] || { echo "FUNC_FAIL: negative N should exit 2"; exit 1; }
grep -q "import secrets" "$PY" || { echo "FUNC_FAIL: must use secrets module"; exit 1; }
echo "FUNC_PASS"
