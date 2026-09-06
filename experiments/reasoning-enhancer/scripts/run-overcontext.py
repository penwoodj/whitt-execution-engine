#!/usr/bin/env python3
"""Over-context experiment driver.

Phases:
  baseline — one workflow run: 9B fed each FULL document (38k tokens >
    32k window). Missing answer or failed checks = case FAILS raw (good).
  intake   — chunked runs of <=5 cases (engine cap): 13 intake steps +
    synth r1/[r2]/select per case, one 4B load per run.
  report   — evidence table 9B-alone vs intake workflow.

Markers: results/overcontext/{baseline,intake}-rea-oNN.json
"""

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

REA = Path(__file__).parent.parent
SCRIPTS = REA / "scripts"
RUNNER = SCRIPTS / "run-enhancer.sh"
CASES = REA / "cases" / "overcontext"
OC = REA / "results" / "overcontext"


def sh(cmd, env_extra=None):
    env = None
    if env_extra:
        import os
        env = dict(os.environ)
        env.update(env_extra)
    return subprocess.run(cmd, capture_output=True, text=True, env=env)


def settle(max_wait=180):
    waited = 0
    while waited < max_wait:
        r = subprocess.run(["vmstat", "1", "8"], capture_output=True, text=True)
        total = 0
        for ln in r.stdout.splitlines()[2:]:
            parts = ln.split()
            if len(parts) >= 8:
                try:
                    total += int(parts[6]) + int(parts[7])
                except ValueError:
                    pass
        if total < 20000:
            return waited
        time.sleep(10)
        waited += 10
    return waited


def run_workflow(wf, timeout):
    import os
    t0 = time.time()
    env_extra = {"REA_WORKFLOW": str(wf),
                 "PATH": f"{SCRIPTS / 'shims'}:{os.environ['PATH']}"}
    r = sh(["bash", str(RUNNER), str(CASES / "case-o01.yml"), str(timeout)],
           env_extra=env_extra)
    wall = int(time.time() - t0)
    return r, wall


def harvest(runtime_dir, phase, nums):
    n = 0
    for nn in nums:
        run = runtime_dir / f"o{nn:02d}"
        cid = f"rea-o{nn:02d}"
        marker = OC / f"{phase}-{cid}.json"
        if phase == "baseline":
            ans = run / "baseline-answer.txt"
            chk = run / "check-baseline.json"
            if not ans.exists() or not chk.exists():
                marker.write_text(json.dumps({"case_id": cid, "passed": False,
                                              "reason": "no answer (window exceeded / step failed)"}, indent=2))
                print(f"[oc] {cid}: baseline FAIL — no answer (window exceeded)")
                n += 1
                continue
            c = json.loads(chk.read_text())
            marker.write_text(json.dumps({"case_id": cid, "passed": c["passed"],
                                          "answer": ans.read_text()[:400],
                                          "failures": c["failures"]}, indent=2))
            v = "FAIL (good)" if not c["passed"] else "PASS (NEEDS WORK)"
            print(f"[oc] {cid}: baseline {v}")
            n += 1
        else:
            sb_p = run / "select-best.json"
            if not sb_p.exists():
                continue
            sb = json.loads(sb_p.read_text())
            marker.write_text(json.dumps({"case_id": cid, "mode": sb["mode"],
                                          "passed": sb.get("passed")}, indent=2))
            v = "PASS" if sb.get("passed") else "FAIL"
            print(f"[oc] {cid}: intake {v} — mode={sb['mode']}")
            n += 1
    return n


def phase_baseline():
    OC.mkdir(parents=True, exist_ok=True)
    wf = REA / "workflows" / "oc-baseline.yml"
    gen = sh([sys.executable, str(SCRIPTS / "gen-intake-workflow.py"),
              "--mode", "baseline", "--out", str(wf)])
    print(gen.stdout.strip())
    val = sh([sys.executable, str(SCRIPTS / "validate-rea.py"), str(wf), "--allow-9b", "--batch"])
    if val.returncode != 0:
        print(val.stdout)
        return 1
    print(val.stdout.strip())
    r, wall = run_workflow(wf, 3600)
    print(f"[oc] baseline wall: {wall}s rc={r.returncode}")
    n = harvest(OC / "baseline-runtime", "baseline", range(1, 31))
    print(f"[oc] harvested {n}/30 baselines")
    return 0


def phase_intake(chunk_size=5):
    OC.mkdir(parents=True, exist_ok=True)
    nums = list(range(1, 31))
    pending = [n for n in nums if not (OC / f"intake-rea-o{n:02d}.json").exists()]
    if not pending:
        print("all intake markers present")
    total_wall = 0
    while pending:
        chunk = pending[:chunk_size]
        wf = REA / "workflows" / "oc-intake.yml"
        gen = sh([sys.executable, str(SCRIPTS / "gen-intake-workflow.py"),
                  "--cases", ",".join(map(str, chunk)), "--out", str(wf)])
        print(gen.stdout.strip(), flush=True)
        val = sh([sys.executable, str(SCRIPTS / "validate-rea.py"), str(wf), "--batch"])
        if val.returncode != 0:
            print(val.stdout)
            return 1
        r, wall = run_workflow(wf, 3600)
        total_wall += wall
        print(f"[oc] intake chunk {chunk} wall: {wall}s rc={r.returncode}", flush=True)
        settle()
        harvest(OC / "intake-runtime", "intake", chunk)
        new_pending = [n for n in nums if not (OC / f"intake-rea-o{n:02d}.json").exists()]
        if len(new_pending) == len(pending):
            print("[oc] no progress — stopping to avoid loop")
            return 1
        pending = new_pending
    (OC / "intake-wall.json").write_text(json.dumps({"wall_seconds": total_wall}, indent=2))
    print(f"[oc] all intake done, total wall {total_wall}s")
    return 0


def phase_report():
    rows = []
    for nn in range(1, 31):
        cid = f"rea-o{nn:02d}"
        b = OC / f"baseline-{cid}.json"
        i = OC / f"intake-{cid}.json"
        rows.append((cid,
                     json.loads(b.read_text()) if b.exists() else None,
                     json.loads(i.read_text()) if i.exists() else None))
    bl_fail = it_pass = both = 0
    print(f"{'case':9} {'9B raw':>8} {'intake wf':>9} {'mode':>12}")
    for cid, b, i in rows:
        bfail = bool(b and not b.get("passed"))
        ipass = bool(i and i.get("passed"))
        bl_fail += bfail
        it_pass += ipass
        both += bfail and ipass
        print(f"{cid:9} {'FAIL' if bfail else ('PASS' if b else '—'):>8} "
              f"{'PASS' if ipass else ('FAIL' if i else '—'):>9} "
              f"{i.get('mode', '—') if i else '—':>12}")
    print(f"\n9B-raw fails: {bl_fail}/30 | intake passes: {it_pass}/30 | BOTH: {both}/30")
    wj = OC / "intake-wall.json"
    if wj.exists():
        print(f"intake total wall: {json.loads(wj.read_text())['wall_seconds']}s")
    return 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("phase", choices=["baseline", "intake", "report"])
    ap.add_argument("--chunk-size", type=int, default=5)
    args = ap.parse_args()
    if args.phase == "baseline":
        return phase_baseline()
    if args.phase == "intake":
        return phase_intake(args.chunk_size)
    return phase_report()


if __name__ == "__main__":
    sys.exit(main())
