#!/usr/bin/env python3
"""Unit tests for the principle-fusion hook scripts (zero LLM).

Covers fusion_lib primitives (digest, harvester, leak-safe hints,
entropy router, two-lane verdict, skip-if-won) and the three hook
CLIs (spoof-emit, stage-emit, fusion_conf) via subprocess against
the REAL generated case fu-01 and its truth fixture.

Run: python3 test_hooks.py   (from this scripts/ dir)
"""
import json
import math
import re
import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent

from fusion_lib import (CONTRACT_MARK, STYLE_PREFIX, WORKED_MARK,
                        conf_route, digest, entropy_from_logprobs,
                        harvest_candidates, harvester,
                        leak_safe_hint, two_lane_verdict)

CASE = EXP / "cases/case-fu-01.yml"
FIXTURES = EXP / "fixtures/fusion-fixes.yml"

FAILS = []


def check(name, cond, detail=""):
    if cond:
        print(f"  ok  {name}")
    else:
        print(f" FAIL {name} {detail}")
        FAILS.append(name)


def run_cli(script, args, cwd=HERE):
    r = subprocess.run(
        [sys.executable, str(HERE / script)] + [str(a) for a in args],
        capture_output=True, text=True, cwd=cwd, timeout=60)
    return r.returncode, r.stdout.strip(), r.stderr.strip()


def case_data():
    return yaml.safe_load(CASE.read_text())


def truth_for(cid):
    return yaml.safe_load(FIXTURES.read_text())[cid]


# ---------- fusion_lib primitives ----------

def test_digest():
    c = case_data()
    d = digest(c["prompt"])
    check("digest cuts worked example",
          WORKED_MARK not in d)
    check("digest keeps output contract",
          CONTRACT_MARK in d)
    check("digest <= 260 words", len(d.split()) <= 260,
          f"got {len(d.split())}")
    task_nums = ["50", "18", "22", "16", "9", "14", "6"]
    missing = [n for n in task_nums
               if not re.search(rf"(?<![A-Za-z0-9_]){n}"
                                rf"(?![0-9])", d)]
    check("digest preserves task digits", not missing,
          f"missing {missing}")


def test_harvester():
    truth = truth_for("fu-01")
    ctx = {"json_exact": truth}
    noisy = ("reasoning... partial grants... maybe "
             + truth.replace("61", "6_1")
             + "\nfinal answer above")
    got = harvester(noisy, ctx)
    check("harvester repairs digit garble + check-before-echo",
          got == "CLEANED_JSON:" + truth, repr(got))
    check("harvester rejects garbage",
          harvester("no json here", ctx) is None)
    cands = list(harvest_candidates("x {\"a\": 1} y {\"b\": 2}"))
    check("harvest newest-first",
          cands == ['{"b": 2}', '{"a": 1}'], repr(cands))


def test_leak_safe_hint():
    cj = {"failures": [{
        "check": "json_exact",
        "detail": 'expected granted_units 61 pool_left 0',
        "fix_hint": "recompute pool"}]}
    hint = leak_safe_hint(cj)
    check("hint names check + observed", "json_exact" in hint
          and "61" in hint)
    check("hint poses question not answer",
          "recheck this rule" in hint)
    check("hint empty on no failures", leak_safe_hint({}) == "")


def test_entropy_router():
    check("deterministic dist -> 0 entropy",
          entropy_from_logprobs([0.0, -1e9, -1e9]) < 0.01)
    uniform = [0.0, 0.0, 0.0]
    check("uniform 3-way -> ln3",
          abs(entropy_from_logprobs(uniform) - math.log(3)) < 1e-6)
    check("peaked -> LIGHT",
          conf_route([0.0, -20.0, -20.0]) == "LIGHT")
    check("uniform -> HEAVY",
          conf_route(uniform) == "HEAVY")
    check("empty -> LIGHT (0 entropy)",
          conf_route([]) == "LIGHT")


def test_two_lane():
    with tempfile.TemporaryDirectory() as td:
        jp = Path(td) / "judge.json"
        jp.write_text('{"verdict": "fail"}')
        v = two_lane_verdict(True, jp)
        check("det overrides judge fail", v["final"] is True
              and v["judge_disagreed"] is True)
        jp.write_text('{"verdict": "pass"}')
        v = two_lane_verdict(True, jp)
        check("agree pass", v["final"] is True
              and v["judge_disagreed"] is False)
        v = two_lane_verdict(False, None)
        check("no judge file -> det only",
              v["final"] is False and v["judge"] is None)


def test_style_prefixes():
    for st in ("plan", "cot", "replan", "wait", "verify", "extract",
               "judge"):
        check(f"style prefix exists [{st}]",
              st in STYLE_PREFIX and STYLE_PREFIX[st].strip())


# ---------- spoof-emit CLI ----------

def test_spoof_emit():
    truth = truth_for("fu-01")
    with tempfile.TemporaryDirectory() as td:
        scen = Path(td) / "scenario.json"
        # win at index 0
        scen.write_text(json.dumps({"fu-01": 0}))
        rc, out, err = run_cli("spoof-emit.py", [
            "--case", CASE, "--stage", "solve", "--stage-index", "0",
            "--stages", "solve,extract", "--run-dir", td,
            "--scenario", scen, "--truths", FIXTURES])
        check("spoof win0 prints PASS", out == '"PASS"', out + err)
        check("spoof win0 wrote truth ans",
              (Path(td) / "ans-fu-01-solve.txt").read_text().strip()
              == truth)
        cj = json.loads(
            (Path(td) / "check-fu-01-solve.json").read_text())
        check("spoof win0 check passed=True", cj["passed"] is True)

    with tempfile.TemporaryDirectory() as td:
        scen = Path(td) / "scenario.json"
        scen.write_text(json.dumps({"fu-01": 3}))
        rc, out, err = run_cli("spoof-emit.py", [
            "--case", CASE, "--stage", "solve", "--stage-index", "0",
            "--stages", "solve,extract", "--run-dir", td,
            "--scenario", scen, "--truths", FIXTURES])
        check("spoof pre-win prints SKIP", out == '"SKIP"')
        cj = json.loads(
            (Path(td) / "check-fu-01-solve.json").read_text())
        check("spoof pre-win check passed=False",
              cj["passed"] is False)
        check("spoof pre-win failures non-empty",
              bool(cj["failures"]))

    with tempfile.TemporaryDirectory() as td:
        # already won -> PASS fast
        (Path(td) / "check-fu-01-other.json").write_text(
            json.dumps({"passed": True}))
        scen = Path(td) / "scenario.json"
        scen.write_text(json.dumps({"fu-01": 3}))
        rc, out, err = run_cli("spoof-emit.py", [
            "--case", CASE, "--stage", "solve", "--stage-index", "0",
            "--stages", "solve,extract", "--run-dir", td,
            "--scenario", scen, "--truths", FIXTURES])
        check("spoof skip-if-won prints PASS", out == '"PASS"')

    with tempfile.TemporaryDirectory() as td:
        scen = Path(td) / "scenario.json"
        scen.write_text(json.dumps({"fu-01": 0}))
        rc, out, err = run_cli("spoof-emit.py", [
            "--case", CASE, "--stage", "plan", "--stage-index", "0",
            "--stages", "solve,extract", "--run-dir", td,
            "--scenario", scen, "--truths", FIXTURES,
            "--unchecking"])
        check("spoof scaffold prints SKIP", out == '"SKIP"')
        check("spoof scaffold ans written",
              "scaffold" in (Path(td) / "ans-fu-01-plan.txt"
                             ).read_text())

    with tempfile.TemporaryDirectory() as td:
        (Path(td) / "check-fu-01-solve.json").write_text(
            json.dumps({"passed": True}))
        scen = Path(td) / "scenario.json"
        scen.write_text(json.dumps({
            "fu-01": 0, "fu-01::judge_disagree": True}))
        rc, out, err = run_cli("spoof-emit.py", [
            "--case", CASE, "--stage", "judge", "--stage-index", "0",
            "--stages", "solve,extract", "--run-dir", td,
            "--scenario", scen, "--truths", FIXTURES,
            "--unchecking", "--judge"])
        check("spoof judge prints SKIP (never gates)",
              out == '"SKIP"')
        v = json.loads((Path(td) / "ans-fu-01-judge.txt"
                        ).read_text())
        check("spoof judge disagreement verdict=fail",
              v == {"verdict": "fail"}, repr(v))


# ---------- stage-emit CLI ----------

def test_stage_emit_extract():
    truth = truth_for("fu-01")
    with tempfile.TemporaryDirectory() as td:
        (Path(td) / "ans-fu-01-solve.txt").write_text(
            "let me think... the pool drains... final:\n" + truth)
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "extract", "--style",
            "extract", "--run-dir", td, "--prior", "solve"])
        check("extract echoes CLEANED_JSON",
              out == "CLEANED_JSON:" + truth, out + err)

    with tempfile.TemporaryDirectory() as td:
        (Path(td) / "ans-fu-01-solve.txt").write_text(
            "no json at all, just words")
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "extract", "--style",
            "extract", "--run-dir", td, "--prior", "solve"])
        check("extract fallback SOLUTION TEXT",
              out.startswith("SOLUTION TEXT:"), out[:80])
        check("extract fallback embeds json contract form",
              "Output EXACTLY this JSON object" in out)

    with tempfile.TemporaryDirectory() as td:
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "extract", "--style",
            "extract", "--run-dir", td, "--prior", "solve"])
        check("extract without prior prints SKIP", out == '"SKIP"')


def test_stage_emit_solve_and_judge():
    c = case_data()
    with tempfile.TemporaryDirectory() as td:
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "solve", "--style", "cot",
            "--run-dir", td])
        check("first solve sees FULL prompt (worked included)",
              WORKED_MARK in out and "starts at 50" in out
              and "r6" in out, err[:200])

    with tempfile.TemporaryDirectory() as td:
        (Path(td) / "ans-fu-01-solve.txt").write_text(
            '{"granted_units": 55, "denied": [], "pool_left": 5}')
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "solve2", "--style", "cot",
            "--run-dir", td, "--prior", "replan,solve"])
        check("prior-mode uses digest (worked example cut)",
              WORKED_MARK not in out)
        check("prior-mode embeds prior attempt",
              "ATTEMPT (solve)" in out)

    with tempfile.TemporaryDirectory() as td:
        # failed check -> leak-safe hint block
        (Path(td) / "ans-fu-01-solve.txt").write_text(
            '{"granted_units": 55}')
        (Path(td) / "check-fu-01-solve.json").write_text(json.dumps({
            "passed": False,
            "failures": [{"check": "json_exact",
                          "detail": "missing keys",
                          "fix_hint": "emit all keys"}]}))
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "wait", "--style", "wait",
            "--run-dir", td, "--prior", "solve2"])
        check("failed check triggers hint block",
              "PREVIOUS ATTEMPT FAILED" in out)
        check("hint is leak-safe (no expected values)",
              "recheck this rule" in out)

    with tempfile.TemporaryDirectory() as td:
        (Path(td) / "ans-fu-01-solve.txt").write_text(
            '{"granted_units": 55, "denied": [], "pool_left": 5}')
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "judge", "--style", "judge",
            "--run-dir", td])
        check("judge sees contract digest",
              "TASK CONTRACT" in out and CONTRACT_MARK in out)
        check("judge sees answer under review",
              "ANSWER UNDER REVIEW" in out)
        check("judge blind: truth value 61 absent", "61" not in out)
        check("judge verdict instruction present",
              '"verdict"' in out)

    with tempfile.TemporaryDirectory() as td:
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "solve", "--style", "cot",
            "--run-dir", td])
        (Path(td) / "check-fu-01-x.json").write_text(
            json.dumps({"passed": True}))
        rc, out, err = run_cli("stage-emit.py", [
            "--case", CASE, "--stage", "solve", "--style", "cot",
            "--run-dir", td])
        check("stage-emit skip-if-won prints PASS",
              out == '"PASS"')


# ---------- fusion_conf CLI ----------

def test_fusion_conf():
    with tempfile.TemporaryDirectory() as td:
        lp = Path(td) / "probe.json"
        lp.write_text(json.dumps([0.0, 0.0, 0.0]))
        rc, out, err = run_cli("fusion_conf.py", [
            "--run-dir", td, "--case", CASE,
            "--logprobs-file", lp])
        check("conf uniform -> HEAVY", out == '"HEAVY"', out + err)
        conf = json.loads((Path(td) / "conf-fu-01.json").read_text())
        check("conf artifact records source=probe",
              conf["source"] == "probe" and conf["lane"] == "HEAVY")
    with tempfile.TemporaryDirectory() as td:
        lp = Path(td) / "probe.json"
        lp.write_text(json.dumps([0.0, -20.0, -20.0]))
        rc, out, err = run_cli("fusion_conf.py", [
            "--run-dir", td, "--case", CASE,
            "--logprobs-file", lp])
        check("conf peaked -> LIGHT", out == '"LIGHT"')
    with tempfile.TemporaryDirectory() as td:
        rc, out, err = run_cli("fusion_conf.py", [
            "--run-dir", td, "--case", CASE,
            "--logprobs-file", Path(td) / "missing.json",
            "--default", "LIGHT"])
        check("conf missing probe -> default LIGHT",
              out == '"LIGHT"')
        conf = json.loads((Path(td) / "conf-fu-01.json").read_text())
        check("conf artifact records source=default",
              conf["source"] == "default")


def main():
    for fn in (test_digest, test_harvester, test_leak_safe_hint,
               test_entropy_router, test_two_lane,
               test_style_prefixes, test_spoof_emit,
               test_stage_emit_extract, test_stage_emit_solve_and_judge,
               test_fusion_conf):
        print(fn.__name__)
        fn()
    print()
    if FAILS:
        print(f"{len(FAILS)} FAILURES: {FAILS}")
        return 1
    print("ALL TESTS PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
