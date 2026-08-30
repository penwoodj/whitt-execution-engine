#!/usr/bin/env python3
"""Unit tests for v7 scripts: leak-audit, replay-rescore, meter,
check-table extra-row detection, gen-v7 routing + adversarial corruption.
Run: python3 -m pytest tests/test_v7_scripts.py -q
"""
import importlib.util
import json
import sys
import tempfile
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
SCRIPTS = ROOT / "scripts"
CASES = ROOT / "cases"
sys.path.insert(0, str(SCRIPTS))


def load(stem):
    spec = importlib.util.spec_from_file_location(stem, SCRIPTS / f"{stem}.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def tmpdir():
    return Path(tempfile.mkdtemp())


class TestMeter:
    def test_estimate_and_trace(self):
        m = load("meter")
        d = tmpdir()
        art = d / "a.txt"
        art.write_text("x" * 400)
        import subprocess
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "meter.py"), "--artifact", str(art),
             "--stage", "solve", "--trace", str(d / "t.jsonl"), "--case", "ha-01"],
            capture_output=True, text=True)
        assert r.returncode == 0
        assert int(r.stdout.strip()) == 100
        entry = json.loads((d / "t.jsonl").read_text().strip())
        assert entry["gate"] == "meter" and entry["stage"] == "solve"
        assert entry["tokens_est"] == 100 and entry["case"] == "ha-01"

    def test_missing_artifact_zero(self):
        import subprocess
        d = tmpdir()
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "meter.py"), "--artifact", str(d / "nope.txt"),
             "--stage", "fix_1", "--trace", str(d / "t.jsonl")],
            capture_output=True, text=True)
        assert r.returncode == 0
        assert int(r.stdout.strip()) == 0


class TestLeakAudit:
    def test_clean_feedback_passes(self):
        d = tmpdir()
        table = "ENTITY round_1 | \"allowed\"\nENTITY round_2 | \"WRONG\"\nENTITY round_3 | \"allowed\"\n"
        (d / "a.txt").write_text(table)
        import subprocess
        subprocess.run(
            [sys.executable, str(SCRIPTS / "check-table.py"), "--artifact", str(d / "a.txt"),
             "--case", str(CASES / "ha-10.yml"), "--out", str(d / "ha-table-result.json")],
            capture_output=True)
        (d / "ha-cell-feedback.txt").write_text('- round_2 [WRONG]: your value was "WRONG"\n')
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "leak-audit.py"), "--run-dir", str(d),
             "--case", str(CASES / "ha-10.yml")],
            capture_output=True, text=True)
        assert r.returncode == 0
        assert json.loads(r.stdout)["clean"] is True

    def test_leak_detected(self):
        d = tmpdir()
        table = "ENTITY round_1 | \"allowed\"\nENTITY round_2 | \"WRONG\"\nENTITY round_3 | \"allowed\"\n"
        (d / "a.txt").write_text(table)
        import subprocess
        subprocess.run(
            [sys.executable, str(SCRIPTS / "check-table.py"), "--artifact", str(d / "a.txt"),
             "--case", str(CASES / "ha-10.yml"), "--out", str(d / "ha-table-result.json")],
            capture_output=True)
        (d / "ha-cell-feedback.txt").write_text('the answer is "VIOLATION" for round 2\n')
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "leak-audit.py"), "--run-dir", str(d),
             "--case", str(CASES / "ha-10.yml")],
            capture_output=True, text=True)
        assert r.returncode == 1
        rep = json.loads(r.stdout)
        assert rep["clean"] is False
        assert rep["leaks"][0]["entity"] == "round_2"

    def test_weak_numeric_literals_excluded(self):
        d = tmpdir()
        table = "ENTITY case_A | 7\nENTITY case_B | 0\nENTITY case_C | 0\nENTITY case_D | 9\n"
        (d / "a.txt").write_text(table)
        import subprocess
        subprocess.run(
            [sys.executable, str(SCRIPTS / "check-table.py"), "--artifact", str(d / "a.txt"),
             "--case", str(CASES / "ha-01.yml"), "--out", str(d / "ha-table-result.json")],
            capture_output=True)
        (d / "ha-cell-feedback.txt").write_text("- case_A [WRONG]: re-derive, digit 0 appears here\n")
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "leak-audit.py"), "--run-dir", str(d),
             "--case", str(CASES / "ha-01.yml")],
            capture_output=True, text=True)
        assert r.returncode == 0

    def test_json_exact_full_string_is_leak(self):
        d = tmpdir()
        (d / "ha-table-result.json").write_text(json.dumps(
            {"pass": False, "wrong": [{"id": "case_A", "observed": 7}],
             "cells": [{"id": "case_A", "present": True, "match": False, "observed": 7}]}))
        (d / "ha-cell-feedback.txt").write_text(
            'hint: {"stages_rerun": 3} is what the checker wants\n')
        import subprocess
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "leak-audit.py"), "--run-dir", str(d),
             "--case", str(CASES / "ha-01.yml")],
            capture_output=True, text=True)
        assert r.returncode == 1
        assert json.loads(r.stdout)["leaks"][0]["entity"] == "json_exact"


class TestReplayRescore:
    def test_replay_report(self):
        d = tmpdir()
        good = "ENTITY case_A | 0\nENTITY case_B | 0\nENTITY case_C | 0\nENTITY case_D | 3\n"
        bad = "ENTITY case_A | 7\nENTITY case_B | 0\nENTITY case_C | 0\nENTITY case_D | 3\n"
        (d / "artifact.solve.txt").write_text(bad)
        (d / "artifact.fix_1.txt").write_text(good)
        import subprocess
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "replay-rescore.py"), "--run-dir", str(d),
             "--case", str(CASES / "ha-01.yml")],
            capture_output=True, text=True)
        assert r.returncode == 0
        rep = json.loads(r.stdout)
        assert rep["stages"]["solve"]["pass"] is False
        assert rep["stages"]["fix_1"]["pass"] is True
        assert rep["stages"]["fix_2"]["present"] is False
        assert rep["final"]["match"] is True
        assert rep["final"]["value"] == {"stages_rerun": 3}

    def test_empty_run_dir(self):
        d = tmpdir()
        import subprocess
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "replay-rescore.py"), "--run-dir", str(d),
             "--case", str(CASES / "ha-01.yml")],
            capture_output=True, text=True)
        assert r.returncode == 0
        rep = json.loads(r.stdout)
        assert all(not s.get("present") for s in rep["stages"].values())
        assert rep["final"] is None


class TestCheckTableExtra:
    def test_extra_row_fails(self):
        d = tmpdir()
        table = ("ENTITY case_A | 0\nENTITY case_B | 0\nENTITY case_C | 0\n"
                 "ENTITY case_D | 3\nENTITY ghost | 42\n")
        (d / "a.txt").write_text(table)
        import subprocess
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "check-table.py"), "--artifact", str(d / "a.txt"),
             "--case", str(CASES / "ha-01.yml")],
            capture_output=True, text=True)
        assert r.returncode == 1
        rep = json.loads(r.stdout)
        assert rep["extra"] == ["ghost"]

    def test_no_extra_passes(self):
        d = tmpdir()
        table = "ENTITY case_A | 0\nENTITY case_B | 0\nENTITY case_C | 0\nENTITY case_D | 3\n"
        (d / "a.txt").write_text(table)
        import subprocess
        r = subprocess.run(
            [sys.executable, str(SCRIPTS / "check-table.py"), "--artifact", str(d / "a.txt"),
             "--case", str(CASES / "ha-01.yml")],
            capture_output=True, text=True)
        assert r.returncode == 0
        assert json.loads(r.stdout)["extra"] == []


class TestGenV7:
    def test_routing_light_heavy(self):
        g = load("gen-v7")
        for cid, expect in [("ha-01", "m_light"), ("ha-03", "m_light"), ("ha-07", "m_light"),
                            ("ha-04", "m_heavy"), ("ha-05", "m_heavy"), ("ha-09", "m_heavy"),
                            ("ha-10", "m_heavy")]:
            case = yaml.safe_load((CASES / f"{cid}.yml").read_text())
            yml = g.gen_v7(case, spoof=False)
            assert f"models.{expect}" in yml, f"{cid}: expected {expect} ref"
            solve_ref = [l.strip() for l in yml.splitlines()
                         if l.strip().startswith("generative_entity") and "m_judge" not in l][0]
            assert f"models.{expect}" in solve_ref, f"{cid}: solve step wrong ref: {solve_ref}"

    def test_fix2_escalates_to_heavy(self):
        g = load("gen-v7")
        case = yaml.safe_load((CASES / "ha-01.yml").read_text())
        yml = g.gen_v7(case, spoof=False)
        step = yml.split("s03_fix_2:")[1].split("s04_")[0]
        assert "models.m_heavy" in step, "fix_2 must escalate to m_heavy"

    def test_judge_cross_family(self):
        g = load("gen-v7")
        case = yaml.safe_load((CASES / "ha-01.yml").read_text())
        yml = g.gen_v7(case, spoof=False)
        assert "Hermes-2-Pro-Mistral-7B.Q4_K_M" in yml
        assert "VERDICT must be PASS only if all three" in yml

    def test_snapshots_meter_audit_wired(self):
        g = load("gen-v7")
        case = yaml.safe_load((CASES / "ha-05.yml").read_text())
        yml = g.gen_v7(case, spoof=False)
        assert "artifact.solve.txt" in yml and "artifact.fix_1.txt" in yml and "artifact.fix_2.txt" in yml
        assert "meter.py" in yml
        assert "leak-audit.py" in yml
        assert "s08_audit" in yml

    def test_thinking_variant(self):
        g = load("gen-v7")
        case = yaml.safe_load((CASES / "ha-04.yml").read_text())
        yml = g.gen_v7(case, spoof=True, variant="thinking")
        assert "Qwen3-4B-Thinking-2507-Q4_K_M" in yml
        assert "<think>" in yml and "BUDGET" in yml

    def test_adversarial_corruptions_fail_gate(self):
        g = load("gen-v7")
        import subprocess
        for cid in ["ha-01", "ha-03", "ha-05", "ha-10"]:
            case = yaml.safe_load((CASES / f"{cid}.yml").read_text())
            for kind in ("adv_partial", "adv_malformed", "adv_wrongtype", "adv_dup"):
                t = g.corrupt_table(kind, case)
                d = tmpdir()
                (d / "a.txt").write_text(t)
                r = subprocess.run(
                    [sys.executable, str(SCRIPTS / "check-table.py"), "--artifact", str(d / "a.txt"),
                     "--case", str(CASES / f"{cid}.yml")],
                    capture_output=True, text=True)
                assert r.returncode == 1, f"{cid}/{kind}: corruption passed gate"

    def test_scenarios_written(self):
        g = load("gen-v7")
        case = yaml.safe_load((CASES / "ha-02.yml").read_text())
        d = tmpdir()
        g.write_scenarios(case, d)
        for scen in ("wind0", "wind1", "wind2", "lose",
                     "adv_partial", "adv_malformed", "adv_wrongtype", "adv_dup"):
            for stage in ("solve", "fix_1", "fix_2"):
                assert (d / f"ha-02-{stage}-{scen}.txt").is_file(), f"missing {scen}/{stage}"
        assert (d / "ha-02-judge.txt").is_file()
        judge = (d / "ha-02-judge.txt").read_text()
        assert judge.startswith("KEYS: yes")


if __name__ == "__main__":
    import pytest
    sys.exit(pytest.main([__file__, "-q"]))
