#!/usr/bin/env python3
"""v2 live driver: round1 fresh, round2+ seeded retries on failures.

Usage: v2-driver.py <exp-dir-name> [--rounds N] [--ids id,id] [--tag tN]
       --tag labels run dirs runs/v2-<tag>-rK (default tag: live)
Retries seed from previous round dir (passed stages skip, failed checks
seed hints, solve re-derives).
"""
import argparse
import json
import re
import subprocess
import sys
import time
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent
FAS = SCRIPTS.parent
REPO = FAS.parents[1]
WHITT = REPO / "target/debug/whitt"
CID = re.compile(r"check-([a-z]{2,4}-\d+)-")


def pick_ids(exp):
    best = {}
    for f in sorted((FAS / exp / "runs").glob("live-r*/check-*.json")):
        m = CID.match(f.name)
        if not m:
            continue
        cid = m.group(1)
        t = f.stat().st_mtime
        if cid not in best or t > best[cid][0]:
            try:
                best[cid] = (t,
                             json.loads(f.read_text()).get("passed") is True)
            except Exception:
                continue
    ps = sorted(c for c, (_, p) in best.items() if p)
    fl = sorted(c for c, (_, p) in best.items() if not p)
    return ps[:2] + fl[:1]


def case_state(rundir, ids):
    st = {}
    for cid in ids:
        st[cid] = False
    for f in (Path(rundir)).glob("check-*.json"):
        m = CID.match(f.name)
        if not m:
            continue
        try:
            if json.loads(f.read_text()).get("passed"):
                st[m.group(1)] = True
        except Exception:
            pass
    return st


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("exp")
    ap.add_argument("--rounds", type=int, default=2)
    ap.add_argument("--ids", default=None)
    ap.add_argument("--tag", default="live")
    a = ap.parse_args()

    ids = a.ids.split(",") if a.ids else pick_ids(a.exp)
    expdir = FAS / a.exp
    prev = None
    for rnd in range(1, a.rounds + 1):
        rd = expdir / "runs" / f"v2-{a.tag}-r{rnd}"
        wf = expdir / "workflows" / f"v2-{a.tag}-r{rnd}.yml"
        import shutil
        if rd.exists():
            shutil.rmtree(rd)
        cmd = ["python3", str(SCRIPTS / "gen-workflow.py"), "--exp", a.exp,
               "--live", "--out", str(wf), "--run-dir", str(rd),
               "--only", ",".join(ids)]
        if prev:
            cmd += ["--seed", str(prev)]
        g = subprocess.run(cmd, capture_output=True, text=True)
        if g.returncode != 0:
            print(f"GEN FAIL r{rnd}: {g.stderr[-200:]}")
            sys.exit(1)
        t0 = time.time()
        e = subprocess.run([str(WHITT), "benchmark", "--workflow", str(wf)],
                           cwd=expdir, capture_output=True, text=True)
        wall = time.time() - t0
        (rd.parent / f"v2-{a.tag}-r{rnd}-engine.log").write_text(
            e.stdout + e.stderr)
        st = case_state(rd, ids)
        npass = sum(st.values())
        print(f"  r{rnd}: {npass}/{len(ids)} pass, {wall:.0f}s, "
              f"exit={e.returncode} — "
              + " ".join(f"{c}:{'P' if p else 'F'}" for c, p in st.items()))
        if npass == len(ids):
            break
        prev = rd
    ok = all(case_state(prev, ids).values()) if prev else False
    sys.exit(0 if ok else 3)


if __name__ == "__main__":
    main()
