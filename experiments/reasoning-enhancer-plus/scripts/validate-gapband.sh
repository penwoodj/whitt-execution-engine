#!/bin/bash
# Gap-band dual validation per docs/04-CASE-SUITE-SPEC.md:
#   1) old REA v1.4.3 cascade must FAIL (>=2 subchecks) every case
#   2) >=1 picked specialist must PASS every case direct
# Emits results/stage2-direct/GAPBAND-MATRIX.md + cases-to-rework list.
# Usage: validate-gapband.sh [--limit N] [--old-only|--direct-only]
#   [--models "A,B,C"]  (defaults to MODEL-PICKS winners)
set -uo pipefail

REPO_ROOT="/home/jon/.local/share/opencode/worktree/b3a0edc5d199ed0b05278bdc34894bc7a8b0e833/nimble-harbor"
EXP="${REPO_ROOT}/experiments/reasoning-enhancer-plus"
REA="${REPO_ROOT}/experiments/reasoning-enhancer"
CASES_DIR="${EXP}/cases/stage2-train"
OUTROOT="${EXP}/results/stage2-direct"
LIMIT=""
MODE="all"
MODELS=""

while [ $# -gt 0 ]; do
  case "$1" in
    --limit) LIMIT="$2"; shift 2 ;;
    --old-only) MODE="old"; shift ;;
    --direct-only) MODE="direct"; shift ;;
    --models) MODELS="$2"; shift 2 ;;
    *) echo "unknown arg $1"; exit 1 ;;
  esac
done

if [ -z "${MODELS}" ]; then
  MODELS=$(python3 - <<'PY'
import re, pathlib
t = pathlib.Path("experiments/reasoning-enhancer-plus/results/MODEL-PICKS.md").read_text()
rows = re.findall(r"^\| (FMT|DRV|LOG|PLN|AUD) \| (\S+) \|", t, re.M)
seen = []
for _, m in rows:
    if m not in seen:
        seen.append(m)
print(",".join(seen))
PY
)
fi
IFS=',' read -ra PICKS <<< "${MODELS}"
echo "[gapband] specialists: ${PICKS[*]}"

CASES=$(ls "${CASES_DIR}"/case-*.yml | sort)
if [ -n "${LIMIT}" ]; then
  CASES=$(echo "${CASES}" | head -"${LIMIT}")
fi

mkdir -p "${OUTROOT}"
OLD_DIR="${OUTROOT}/old-workflow"
mkdir -p "${OLD_DIR}"

run_old() {
  local CASE="$1"
  local CID
  CID=$(python3 -c "import yaml,sys;print(yaml.safe_load(open(sys.argv[1]))['case_id'])" "${CASE}")
  local DEST="${OLD_DIR}/${CID}"
  [ -f "${DEST}/metrics.json" ] && { echo "[old] ${CID} cached"; return 0; }
  local TS_OUT
  TS_OUT=$(bash "${REA}/scripts/run-enhancer.sh" "${CASE}" 300 2>&1 | tail -5)
  local LATEST
  LATEST=$(ls -dt "${REA}/results/"*-"${CID}" 2>/dev/null | head -1)
  if [ -z "${LATEST}" ]; then
    echo "[old] ${CID} NO RUN DIR: ${TS_OUT}"
    echo '{"error":"no-run-dir"}' > "${DEST}/metrics.json"
    return 1
  fi
  cp -r "${LATEST}/." "${DEST}/" 2>/dev/null || true
  echo "[old] ${CID} done ($(basename "${LATEST}"))"
}

run_direct() {
  local M="$1"
  echo "[direct] ${M}"
  bash "${EXP}/scripts/run-stage2-model.sh" "${M}" 2>&1 | tail -2
}

if [ "${MODE}" != "direct" ]; then
  for C in ${CASES}; do run_old "${C}"; done
fi
if [ "${MODE}" != "old" ]; then
  for M in "${PICKS[@]}"; do run_direct "${M}"; done
fi

python3 - "${OUTROOT}" "${OLD_DIR}" "${CASES_DIR}" "${PICKS[*]}" <<'PY'
import json, sys, re
from pathlib import Path
import yaml

outroot, old_dir, cases_dir = (Path(p) for p in sys.argv[1:4])
picks = sys.argv[4].split()

direct = {}
for m in picks:
    s = outroot / m / "summary.json"
    if s.exists():
        for it in json.loads(s.read_text()).get("items", []):
            direct.setdefault(it["case_id"], {})[m] = bool(it["passed"])

old = {}
for d in sorted(old_dir.glob("*/")):
    cid = d.name
    sb = d / "select-best.json"
    ck = None
    if sb.exists():
        try:
            sel = json.loads(sb.read_text()).get("selected")
            if sel in (1, 2, "r1", "r2"):
                ck = d / f"check-r{str(sel).lstrip('r')}.json"
        except Exception:
            pass
    if ck is None:
        cand = sorted(d.glob("check-r*.json"))
        ck = cand[-1] if cand else None
    if ck and ck.exists():
        try:
            j = json.loads(ck.read_text())
            fails = (j.get("subchecks_total") or 0) - (j.get("subchecks_passed") or 0)
            old[cid] = {"fails": fails, "passed": bool(j.get("passed"))}
        except Exception:
            old[cid] = {"fails": -1, "passed": None}
    else:
        old[cid] = {"fails": -1, "passed": None}

lines = ["| case | old-fails | " + " | ".join(m[:18] for m in picks) + " | verdict |",
         "|---|---|" + "---|" * (len(picks) + 1)]
rework = []
valid = 0
for p in sorted(Path(cases_dir).glob("case-*.yml")):
    cid = yaml.safe_load(p.read_text())["case_id"]
    o = old.get(cid, {"fails": -1, "passed": None})
    drow = direct.get(cid, {})
    old_ok = not o["passed"]
    any_pass = any(drow.get(m, False) for m in picks)
    verdict = "VALID" if (old_ok and any_pass) else "REWORK"
    if verdict == "VALID":
        valid += 1
    else:
        why = []
        if not old_ok:
            why.append(f"old(passed={o['passed']},fails={o['fails']})")
        if not any_pass:
            why.append("no-specialist-pass")
        rework.append(f"{cid}: {', '.join(why)}")
    marks = " | ".join("PASS" if drow.get(m) else ("fail" if m in drow else "—")
                       for m in picks)
    lines.append(f"| {cid} | {o['fails']} | {marks} | {verdict} |")

md = outroot / "GAPBAND-MATRIX.md"
md.write_text(
    "# Gap-band dual-validation matrix\n\n"
    f"Specialists: {', '.join(picks)}\n\n"
    + "\n".join(lines)
    + f"\n\n**{valid} VALID** / {len(lines)-2} total\n\n"
    + "## Cases to rework\n"
    + ("\n".join(f"- {r}" for r in rework) if rework else "- none"))
print(md.read_text().splitlines()[-3] if rework else "all valid")
print(f"[gapband] {valid} valid — matrix: {md}")
PY
