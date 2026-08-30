#!/usr/bin/env python3
"""v6 spoof battery: 10 cases x 4 scenarios through the REAL engine,
zero LLM inference. Asserts gate sequences, final JSON, judge/end_fail
exclusivity, and zero-inference.

Run: python3 tests/test_v6_spoof.py  (or pytest)
"""
import json
import os
import subprocess
import sys
import time
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parent.parent
WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"
GEN = ROOT / "scripts" / "gen-v6.py"
WF = ROOT / "workflows"
SPOOF_OUT = REPO / "docs/benchmarks/outputs/output/spoof-v6"

EXPECT_SEQ = {
    "wind0": ["check=pass", "final=pass", "judge=pass"],
    "wind1": ["check=fail", "fix_1=pass", "final=pass", "judge=pass"],
    "wind2": ["check=fail", "fix_1=fail", "fix_2=pass", "final=pass", "judge=pass"],
    "lose": ["check=fail", "fix_1=fail", "fix_2=fail", "end_fail=fail"],
}


def run_engine(yml_path):
    env = dict(os.environ, WHITT_ZOMBIE_MAX="4")
    return subprocess.run(
        [WHITT, "benchmark", "--workflow", str(yml_path), "--output-dir", "./docs/benchmarks/outputs"],
        capture_output=True, text=True, timeout=120, cwd=str(REPO), env=env,
    )


def read_trace():
    p = SPOOF_OUT / "ha-trace.jsonl"
    if not p.is_file():
        return []
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]


def clean_outputs():
    for f in SPOOF_OUT.glob("ha-*"):
        f.unlink()


def gates_of(trace):
    return [f"{t['gate']}={t['verdict']}" for t in trace]


def run_one(cid, scen):
    subprocess.run(
        [sys.executable, str(GEN), "--case", cid, "--spoof", "--scenario", scen],
        capture_output=True, text=True, cwd=str(REPO), check=True,
    )
    clean_outputs()
    yml = WF / f"v6-{cid}-spoof.yml"
    t0 = time.time()
    r = run_engine(yml)
    dt = time.time() - t0

    trace = read_trace()
    gates = gates_of(trace)
    final_p = SPOOF_OUT / "ha-final.json"
    final = json.loads(final_p.read_text()) if final_p.is_file() else None
    case = yaml.safe_load((ROOT / "cases" / f"{cid}.yml").read_text())
    expected_final = json.loads(case["success_criteria"]["deterministic_checks"]["json_exact"])

    checks = {
        "seq": gates == EXPECT_SEQ[scen],
        "final": (final == expected_final) if scen != "lose" else (final is None),
        "no_judge_on_lose": not (scen == "lose" and any(g.startswith("judge") for g in gates)),
        "no_endfail_on_win": not (scen != "lose" and any(g.startswith("end_fail") for g in gates)),
        "zero_inference": "output truncated" not in r.stderr and "inference" not in r.stderr.lower(),
        "engine_ok": r.returncode == 0 or "Benchmark run failed" not in r.stderr,
    }
    ok = all(checks.values())
    label = "PASS" if ok else "FAIL"
    bad = [k for k, v in checks.items() if not v]
    print(f"  {label} {cid}/{scen} ({dt:.1f}s) gates={' -> '.join(gates) if gates else 'NONE'}"
          + (f" BAD={bad}" if bad else ""))
    return ok


def main():
    cases = [f"ha-{i:02d}" for i in range(1, 11)]
    scenarios = ["wind0", "wind1", "wind2", "lose"]
    total = passed = 0
    for cid in cases:
        for scen in scenarios:
            total += 1
            try:
                passed += run_one(cid, scen)
            except Exception as e:
                print(f"  FAIL {cid}/{scen} EXC: {e}")
    print(f"\nSPOOF BATTERY: {passed}/{total} green")
    sys.exit(0 if passed == total else 1)


if __name__ == "__main__":
    main()
