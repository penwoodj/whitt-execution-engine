#!/usr/bin/env python3
"""Select 3 cases per exp (2 pass + 1 fail), gen v2 live test ymls, validate."""
import json
import re
import subprocess
import sys
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
REPO = FAS.parents[1]
EXPS = [f"{i:02d}-{n}" for i, n in enumerate([
    "typed-envelope", "digest", "cache-addressing", "intent-classify",
    "gather", "plan", "solve", "replan", "sample-diverse",
    "execute-observe", "verify-blind", "shape-library", "synthesis",
    "budget-inflation"], 1)]
CID = re.compile(r"check-([a-z]{2,4}-\d+)-")


def pick(exp):
    best = {}
    for f in sorted((FAS / exp / "runs").glob("live-r*/check-*.json")):
        m = CID.match(f.name)
        if not m:
            continue
        cid = m.group(1)
        t = f.stat().st_mtime
        if cid not in best or t > best[cid][0]:
            try:
                best[cid] = (t, json.loads(f.read_text()).get("passed") is True)
            except Exception:
                continue
    ps = sorted(c for c, (_, p) in best.items() if p)
    fl = sorted(c for c, (_, p) in best.items() if not p)
    return ps[:2] + fl[:1], len(ps), len(fl)


def main():
    sel = {}
    for exp in EXPS:
        ids, np_, nf = pick(exp)
        sel[exp] = ids
        print(f"{exp:<22} pass={np_} fail={nf} picked={ids}")
    ok = True
    for exp, ids in sel.items():
        if len(ids) < 3:
            print(f"SKIP {exp}: only {len(ids)} cases")
            continue
        rd = FAS / exp / "runs" / "v2-live-test"
        out = FAS / exp / "workflows" / "v2-live-test.yml"
        r = subprocess.run(
            ["python3", str(SCRIPTS / "gen-workflow.py"), "--exp", exp,
             "--live", "--out", str(out), "--run-dir", str(rd),
             "--only", ",".join(ids)], capture_output=True, text=True)
        if r.returncode != 0:
            print(f"GEN FAIL {exp}: {r.stderr[-300:]}")
            ok = False
            continue
        v = subprocess.run(
            ["python3", str(REPO / "scripts/meta-v6/validate-workflow.py"),
             str(out)], capture_output=True, text=True)
        verdict = "VALID" if v.returncode == 0 else f"INVALID: {v.stdout[-200:]}"
        print(f"  gen: {r.stdout.strip()} -> {verdict}")
        if v.returncode != 0:
            ok = False
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
