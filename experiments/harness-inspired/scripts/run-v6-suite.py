#!/usr/bin/env python3
"""v6 suite runner (live mode). Conditional restart: checks zombie count
via zombie-count.sh, restarts Docker ONLY when > 3 (kills the 15s/case
unconditional tax). Spoof mode lives in tests/test_v6_spoof.py.

Usage: python3 scripts/run-v6-suite.py [--cases ha-01,ha-06] [--force-restart]
"""
import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parent.parent
WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"
WF = ROOT / "workflows"
OUT = REPO / "docs/benchmarks/outputs/output"


def zombie_count():
    r = subprocess.run(["sh", str(ROOT / "scripts" / "zombie-count.sh")],
                       capture_output=True, text=True, timeout=10)
    try:
        return int(r.stdout.strip() or "0")
    except ValueError:
        return 99


def maybe_restart(force=False):
    if force or zombie_count() > 3:
        print("  [docker restart: zombie gate]", flush=True)
        subprocess.run(["docker", "restart", "whitt-llama-server"], capture_output=True, timeout=90)
        time.sleep(15)


def run_case(cid, version="v6"):
    yml = WF / f"{version}-{cid}.yml"
    for f in list(OUT.glob("ha-*")) + list(OUT.glob("artifact.*")):
        f.unlink()

    t0 = time.time()
    env = dict(os.environ, WHITT_ZOMBIE_MAX="4")
    r = subprocess.run(
        [WHITT, "benchmark", "--workflow", str(yml), "--output-dir", "./docs/benchmarks/outputs"],
        capture_output=True, text=True, timeout=600, cwd=str(REPO), env=env,
    )
    dt = time.time() - t0

    trace_p = OUT / "ha-trace.jsonl"
    trace = [json.loads(l) for l in trace_p.read_text().splitlines() if l.strip()] if trace_p.is_file() else []
    gates = [f"{t['gate']}={t.get('verdict', '?')}" for t in trace]
    won = any(g.startswith(("check=pass", "fix_1=pass", "fix_2=pass")) for g in gates) \
        and any(g == "final=pass" for g in gates)
    return {"case": cid, "verdict": "PASS" if won else "FAIL",
            "elapsed": round(dt, 1), "gates": gates}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cases", default=",".join(f"ha-{i:02d}" for i in range(1, 11)))
    ap.add_argument("--force-restart", action="store_true")
    ap.add_argument("--version", default="v6", choices=["v6", "v7"])
    args = ap.parse_args()
    cases = [c.strip() for c in args.cases.split(",") if c.strip()]

    print("LIVE MODE — requires LLM permission. Confirm before running real inference.")
    results = []
    for i, cid in enumerate(cases):
        if i > 0:
            maybe_restart(force=args.force_restart)
        print(f"=== {cid} ===", flush=True)
        try:
            r = run_case(cid, version=args.version)
            results.append(r)
            print(f"{r['case']}: {r['verdict']} ({r['elapsed']}s) [{' -> '.join(r['gates'])}]", flush=True)
        except Exception as e:
            results.append({"case": cid, "verdict": "ERROR", "elapsed": 0, "gates": [str(e)[:80]]})
            print(f"{cid}: ERROR {e}", flush=True)

    won = sum(1 for r in results if r["verdict"] == "PASS")
    print(f"\n{args.version} LIVE: {won}/{len(results)}")
    (OUT / f"{args.version}-suite-results.json").write_text(json.dumps(results, indent=2) + "\n")


if __name__ == "__main__":
    main()
