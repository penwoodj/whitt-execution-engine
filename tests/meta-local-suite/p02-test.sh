#!/bin/bash
# Functional test for p02 (sysreport). Args: $1 = artifacts dir
set -uo pipefail
ART="${1:?artifacts dir}"
SH=$(ls "$ART"/sysreport.sh 2>/dev/null || ls "$ART"/*.sh 2>/dev/null | head -1)
[ -z "$SH" ] && { echo "FUNC_FAIL: no shell artifact"; exit 1; }

TMP=$(mktemp -d)
OUT=$(bash "$SH" "$TMP/report.md" 2>&1)
echo "$OUT" | grep -q "REPORT WRITTEN" || { echo "FUNC_FAIL: no REPORT WRITTEN line (got: $(echo "$OUT" | head -2))"; exit 1; }
[ -s "$TMP/report.md" ] || { echo "FUNC_FAIL: report file empty/missing"; exit 1; }
for sec in "## OS" "## Uptime" "## Disk" "## Memory" "## Top Processes"; do
  grep -q "$sec" "$TMP/report.md" || { echo "FUNC_FAIL: missing section $sec"; exit 1; }
done
echo "FUNC_PASS"
