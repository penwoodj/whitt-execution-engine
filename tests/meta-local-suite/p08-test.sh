#!/bin/bash
# Functional test for p08 (dirdiff). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
SH=$(ls "$ART"/dirdiff.sh 2>/dev/null || ls "$ART"/*.sh 2>/dev/null | head -1)
[ -z "$SH" ] && { echo "FUNC_FAIL: no shell artifact"; exit 1; }

TMP=$(mktemp -d)
mkdir -p "$TMP/a" "$TMP/b"
echo same > "$TMP/a/keep.txt";   echo same > "$TMP/b/keep.txt"
echo v1   > "$TMP/a/mod.txt";    echo v2   > "$TMP/b/mod.txt"
echo old  > "$TMP/a/gone.txt"
echo new  > "$TMP/b/fresh.txt"

OUT=$(/bin/bash "$SH" "$TMP/a" "$TMP/b" 2>&1); EX=$?
echo "$OUT"
[ $EX -eq 0 ] || { echo "FUNC_FAIL: exit=$EX"; exit 1; }
echo "$OUT" | grep -q "^ADDED: fresh.txt$"   || { echo "FUNC_FAIL: ADDED"; exit 1; }
echo "$OUT" | grep -q "^REMOVED: gone.txt$"  || { echo "FUNC_FAIL: REMOVED"; exit 1; }
echo "$OUT" | grep -q "^CHANGED: mod.txt$"   || { echo "FUNC_FAIL: CHANGED"; exit 1; }
echo "$OUT" | grep -q "keep.txt" && { echo "FUNC_FAIL: unchanged file listed"; exit 1; }
echo "$OUT" | grep -q "^SUMMARY: added=1 removed=1 changed=1$" || { echo "FUNC_FAIL: SUMMARY"; exit 1; }
/bin/bash "$SH" "$TMP/a" "$TMP/missing" >/dev/null 2>&1 && { echo "FUNC_FAIL: missing dir should exit 1"; exit 1; }
echo "FUNC_PASS"
