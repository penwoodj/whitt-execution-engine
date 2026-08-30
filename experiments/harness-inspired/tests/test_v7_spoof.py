#!/usr/bin/env python3
"""v7 spoof battery: 10 cases x (4 win/lose + 4 adversarial) scenarios +
thinking-variant smoke through the REAL engine, zero LLM inference.

Asserts: gate sequences, final JSON == json_exact, per-stage snapshots,
token meter entries, leak-audit trace, replay-rescore consistency.

Run: python3 tests/test_v7_spoof.py  (or pytest -q)
"""
import json
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parent.parent
WHITT = "/home/jon/code/whitt-execution-engine/target/release/whitt"
GEN = ROOT / "scripts" / "gen-v7.py"
SPOOF_OUT = REPO / "docs/benchmarks/outputs/output/spoof-v7"
REPLAY = ROOT / "scripts" / "replay-rescore.py"

WIN_LOSE_SEQ = {
    "wind0": ["check=pass", "final=pass", "judge=pass", "audit=pass"],
    "wind1": ["check=fail", "fix_1=pass", "final=pass", "judge=pass", "audit=pass"],
    "wind2": ["check=fail", "fix_1=fail", "fix_2=pass", "final=pass", "judge=pass", "audit=pass"],
    "lose": ["check=fail", "fix_1=fail", "fix_2=fail", "end_fail=fail", "audit=pass"],
}
ADV_SEQ = ["check=fail", "fix_1=pass", "final=pass", "judge=pass", "audit=pass"]
CASES = [f"ha-{i:02d}" for i in range(1, 11)]


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
    for f in SPOOF_OUT.glob("artifact.*"):
        f.unlink()


def gates_of(trace):
    return [f"{t['gate']}={t['verdict']}" for t in trace if t["gate"] != "meter"]


def gen(cid, scen, variant="std"):
    subprocess.run(
        [sys.executable, str(GEN), "--case", cid, "--spoof", "--scenario", scen, "--variant", variant],
        capture_output=True, text=True, cwd=str(REPO), check=True,
    )


def case_yaml(cid):
    import yaml
    return yaml.safe_load((ROOT / "cases" / f"{cid}.yml").read_text())


def run_one(cid, scen, variant="std"):
    gen(cid, scen, variant)
    clean_outputs()
    yml = ROOT / "workflows" / f"v7-{cid}{'-thinking' if variant == 'thinking' else ''}-spoof.yml"
    t0 = time.time()
    r = run_engine(yml)
    dt = time.time() - t0
    trace = read_trace()
    return r, trace, dt


def check_run(cid, scen, expected_gates, variant="std"):
    r, trace, dt = run_one(cid, scen, variant)
    assert r.returncode == 0, f"{cid}/{scen}: engine failed: {r.stderr[-400:]}"
    got = gates_of(trace)
    assert got == expected_gates, f"{cid}/{scen}: gates {got} != {expected_gates}"
    assert any(t["gate"] == "audit" and t["verdict"] == "pass" for t in trace), \
        f"{cid}/{scen}: missing audit=pass trace"
    meters = [t for t in trace if t["gate"] == "meter"]
    expected_meters = sum(1 for g in expected_gates if g.startswith(("check", "fix")))
    assert len(meters) == expected_meters, \
        f"{cid}/{scen}: meter entries {len(meters)} != {expected_meters}"
    assert all("tokens_est" in m and "stage" in m for m in meters)
    for stage in ("solve",) if scen != "wind0" else ("solve",):
        pass
    snapshots = sorted(p.name for p in SPOOF_OUT.glob("artifact.*"))
    n_expected_stages = len([g for g in expected_gates if g.startswith(("check", "fix"))])
    assert len(snapshots) == n_expected_stages, \
        f"{cid}/{scen}: snapshots {snapshots} != {n_expected_stages} stages"
    if scen != "lose":
        final = json.loads((SPOOF_OUT / "ha-final.json").read_text())
        exact = json.loads(case_yaml(cid)["success_criteria"]["deterministic_checks"]["json_exact"])
        assert final == exact, f"{cid}/{scen}: final {final} != {exact}"
    return dt


def test_full_battery():
    total = 0.0
    for cid in CASES:
        for scen, seq in WIN_LOSE_SEQ.items():
            total += check_run(cid, scen, seq)
        for scen in ("adv_partial", "adv_malformed", "adv_wrongtype", "adv_dup"):
            total += check_run(cid, scen, ADV_SEQ)
    print(f"battery: 80 runs, {total:.1f}s engine time")


def test_thinking_variant_structure():
    dt = check_run("ha-04", "wind0", WIN_LOSE_SEQ["wind0"], variant="thinking")
    print(f"thinking smoke: {dt:.1f}s")


def test_replay_rescore_wind2():
    r, trace, _ = run_one("ha-09", "wind2")
    assert r.returncode == 0
    rep = subprocess.run(
        [sys.executable, str(REPLAY), "--run-dir", str(SPOOF_OUT),
         "--case", str(ROOT / "cases" / "ha-09.yml")],
        capture_output=True, text=True, cwd=str(REPO), check=True,
    )
    report = json.loads(rep.stdout)
    assert report["stages"]["solve"]["pass"] is False
    assert report["stages"]["fix_1"]["pass"] is False
    assert report["stages"]["fix_2"]["pass"] is True
    assert report["final"]["match"] is True
    assert report["final"]["value"] == json.loads(
        case_yaml("ha-09")["success_criteria"]["deterministic_checks"]["json_exact"])


def test_audit_detects_injected_leak():
    run_one("ha-10", "lose")
    leak_fb = SPOOF_OUT / "ha-cell-feedback.txt"
    leak_fb.write_text('- round_2 [WRONG]: it should be "VIOLATION"\n')
    import yaml
    audit = ROOT / "scripts" / "leak-audit.py"
    r = subprocess.run(
        [sys.executable, str(audit), "--run-dir", str(SPOOF_OUT),
         "--case", str(ROOT / "cases" / "ha-10.yml")],
        capture_output=True, text=True, cwd=str(REPO),
    )
    assert r.returncode == 1, "injected leak not detected"
    rep = json.loads(r.stdout)
    assert rep["leaks"] and rep["leaks"][0]["entity"] == "round_2"


if __name__ == "__main__":
    test_full_battery()
    test_thinking_variant_structure()
    test_replay_rescore_wind2()
    test_audit_detects_injected_leak()
    print("ALL V7 SPOOF TESTS PASSED")
