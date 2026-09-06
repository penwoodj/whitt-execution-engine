#!/bin/bash
# Functional test for p14 (logrotate). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
SH=$(ls "$ART"/rotate.sh 2>/dev/null || ls "$ART"/*.sh 2>/dev/null | head -1)
[ -z "$SH" ] && { echo "FUNC_FAIL: no shell artifact"; exit 1; }

TMP=$(mktemp -d)
mkdir -p "$TMP/logs"
head -c 2000 /dev/zero > "$TMP/logs/app.log"      # over threshold
head -c 100  /dev/zero > "$TMP/logs/small.log"    # under threshold
echo prev1 > "$TMP/logs/app.log.1"                 # existing archive to shift

OUT=$(/bin/bash "$SH" "$TMP/logs" 1024 2>&1); EX=$?
echo "$OUT"
[ $EX -eq 0 ] || { echo "FUNC_FAIL: exit=$EX"; exit 1; }
echo "$OUT" | grep -q "^ROTATED: app.log$" || { echo "FUNC_FAIL: ROTATED line"; exit 1; }
echo "$OUT" | grep -q "small.log" && { echo "FUNC_FAIL: small.log should be untouched"; exit 1; }
echo "$OUT" | grep -q "^DONE: rotated=1$" || { echo "FUNC_FAIL: DONE line"; exit 1; }
[ -f "$TMP/logs/app.log" ] || { echo "FUNC_FAIL: new empty app.log missing"; exit 1; }
[ "$(wc -c < "$TMP/logs/app.log" | tr -d ' ')" -eq 0 ] || { echo "FUNC_FAIL: new app.log not empty"; exit 1; }
[ "$(wc -c < "$TMP/logs/app.log.1" | tr -d ' ')" -eq 2000 ] || { echo "FUNC_FAIL: app.log.1 wrong content"; exit 1; }
grep -q prev1 "$TMP/logs/app.log.2" 2>/dev/null || { echo "FUNC_FAIL: old archive not shifted to .2"; exit 1; }
[ "$(wc -c < "$TMP/logs/small.log" | tr -d ' ')" -eq 100 ] || { echo "FUNC_FAIL: small.log modified"; exit 1; }
/bin/bash "$SH" "$TMP/missing" 100 >/dev/null 2>&1 && { echo "FUNC_FAIL: missing dir should exit 1"; exit 1; }
echo "FUNC_PASS"
