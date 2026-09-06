#!/usr/bin/env python3
"""Unit + integration tests for agentic hook scripts (zero LLM).

Covers: stage-emit (fresh/won/priors/failure-hints/caveman/ledgers),
spoof-emit (win/lose/unchecking/skip-if-won/quoted tokens/ledgers),
gen-agentic-native (YAML shape, check shells, prior wiring,
scenario bounds), collect-agentic (win-depth, taxonomy), and the
check-answer integration path against fixture truths.
Run: python3 experiments/reasoning-enhancer-plus/scripts/test_hooks.py
"""
import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
CASES = HERE.parent / "cases/agentic"
FIXTURES = HERE.parent / "fixtures/agentic-fixes.yml"
STAGE_EMIT = HERE / "stage-emit.py"
SPOOF_EMIT = HERE / "spoof-emit.py"
GEN = HERE / "gen-agentic-native.py"
GEN10 = HERE / "gen-agentic-v10.py"
COLLECT = HERE / "collect-agentic.py"
CHECK_ANSWER = REPO / "experiments/reasoning-enhancer/scripts/check-answer.py"

sys.path.insert(0, str(HERE))
sys.path.insert(0, str(REPO / "experiments/reasoning-enhancer/scripts"))


def run_py(script, *args):
    r = subprocess.run([sys.executable, str(script), *map(str, args)],
                       capture_output=True, text=True, timeout=60)
    return r.returncode, r.stdout, r.stderr


def load_case_yamls():
    import yaml
    return {p.name: yaml.safe_load(p.read_text())
            for p in sorted(CASES.glob("case-*.yml"))}


class FixtureIntegrity(unittest.TestCase):
    def test_all_20_cases_have_truths_and_pass_own_checks(self):
        import yaml
        from check_lib import run_checks
        fix = yaml.safe_load(FIXTURES.read_text())
        cases = load_case_yamls()
        self.assertGreaterEqual(len(cases), 20)
        for fname, c in cases.items():
            cid = c["case_id"]
            self.assertIn(cid, fix, f"{cid} missing truth")
            r = run_checks(fix[cid],
                           c["success_criteria"]["deterministic_checks"])
            self.assertTrue(r["passed"],
                            f"{cid} truth fails own checks: {r['failures']}")

    def test_case_shape_contract(self):
        for fname, c in load_case_yamls().items():
            self.assertIn("prompt", c)
            self.assertEqual(c.get("draft_response"), "PLACEHOLDER")
            self.assertIn("rea_plus", c)
            self.assertGreaterEqual(len(c["prompt"].split()), 80)


class StageEmitTests(unittest.TestCase):
    def setUp(self):
        self.td = tempfile.TemporaryDirectory()
        self.rd = Path(self.td.name)
        self.case = CASES / "case-agentic-11.yml"

    def tearDown(self):
        self.td.cleanup()

    def test_fresh_emits_prompt_and_run_ledgers(self):
        rc, out, err = run_py(STAGE_EMIT, "--case", self.case,
                              "--stage", "solve", "--run-dir", self.rd)
        self.assertEqual(rc, 0, err)
        self.assertIn("TASK", out)
        self.assertTrue((self.rd / "outcomes.jsonl").exists())
        self.assertTrue((self.rd / "fingerprint.jsonl").exists())
        o = json.loads((self.rd / "outcomes.jsonl").read_text().splitlines()[0])
        self.assertEqual((o["case"], o["stage"], o["outcome"]),
                         ("agentic-11", "solve", "RUN"))

    def test_won_case_short_circuits_pass(self):
        (self.rd / "check-agentic-11-seed.json").write_text(
            json.dumps({"passed": True}))
        rc, out, _ = run_py(STAGE_EMIT, "--case", self.case,
                            "--stage", "solve", "--run-dir", self.rd)
        self.assertEqual(out.strip(), '"PASS"')

    def test_plan_prior_embedded(self):
        (self.rd / "ans-agentic-11-plan.txt").write_text("plan notes")
        rc, out, _ = run_py(STAGE_EMIT, "--case", self.case,
                            "--stage", "solve", "--prior", "plan",
                            "--run-dir", self.rd)
        self.assertIn("PLANNING NOTES:", out)
        self.assertIn("plan notes", out)
        self.assertIn("TASK REQUIREMENTS:", out)

    def test_failure_hints_injected(self):
        (self.rd / "check-agentic-11-solve.json").write_text(json.dumps({
            "passed": False,
            "failures": [{"check": "json_exact", "detail": "differ",
                          "fix_hint": "re-emit"}]}))
        rc, out, _ = run_py(STAGE_EMIT, "--case", self.case,
                            "--stage", "repair1", "--run-dir", self.rd)
        self.assertIn("PREVIOUS ATTEMPT FAILED", out)
        self.assertIn("json_exact", out)
        self.assertIn("re-emit", out)

    def test_caveman_articles_stripped(self):
        rc, out, _ = run_py(STAGE_EMIT, "--case", self.case,
                            "--stage", "solve", "--run-dir", self.rd)
        self.assertNotIn("the build worker", out.lower())


class SpoofEmitTests(unittest.TestCase):
    def setUp(self):
        self.td = tempfile.TemporaryDirectory()
        self.rd = Path(self.td.name)
        self.case = CASES / "case-agentic-04.yml"
        self.scen = self.rd / "scen.json"
        self.args = ["--case", self.case, "--run-dir", self.rd,
                     "--stage", "solve", "--stage-index", "0",
                     "--stages", "solve,repair1", "--scenario", self.scen]

    def tearDown(self):
        self.td.cleanup()

    def test_lose_writes_failing_check_and_skip_token(self):
        self.scen.write_text(json.dumps({"agentic-04": 1}))
        rc, out, _ = run_py(SPOOF_EMIT, *self.args)
        self.assertEqual(out.strip(), '"SKIP"')
        cj = json.loads(
            (self.rd / "check-agentic-04-solve.json").read_text())
        self.assertFalse(cj["passed"])

    def test_win_writes_truth_and_pass_token(self):
        self.scen.write_text(json.dumps({"agentic-04": 0}))
        rc, out, _ = run_py(SPOOF_EMIT, *self.args)
        self.assertEqual(out.strip(), '"PASS"')
        import yaml
        truth = yaml.safe_load(FIXTURES.read_text())["agentic-04"]
        self.assertEqual(
            (self.rd / "ans-agentic-04-solve.txt").read_text().strip(),
            truth)

    def test_skip_if_won_short_circuits(self):
        (self.rd / "check-agentic-04-seed.json").write_text(
            json.dumps({"passed": True}))
        self.scen.write_text(json.dumps({"agentic-04": 1}))
        rc, out, _ = run_py(SPOOF_EMIT, *self.args)
        self.assertEqual(out.strip(), '"PASS"')

    def test_unchecking_never_wins(self):
        self.scen.write_text(json.dumps({"agentic-04": 0}))
        rc, out, _ = run_py(SPOOF_EMIT, *self.args, "--stage", "plan",
                            "--unchecking")
        self.assertEqual(out.strip(), '"SKIP"')
        self.assertFalse(
            (self.rd / "check-agentic-04-plan.json").exists())

    def test_ledger_written_on_every_path(self):
        self.scen.write_text(json.dumps({"agentic-04": 0}))
        run_py(SPOOF_EMIT, *self.args)
        lines = (self.rd / "outcomes.jsonl").read_text().splitlines()
        self.assertEqual(len(lines), 1)
        self.assertEqual(json.loads(lines[0])["outcome"], "PASS")


class GeneratorTests(unittest.TestCase):
    def setUp(self):
        self.td = tempfile.TemporaryDirectory()
        self.rd = Path(self.td.name)
        self.out = self.rd / "wf.yml"
        self.run_dir = self.rd / "run"

    def tearDown(self):
        self.td.cleanup()

    def _gen(self, technique, spoof=True):
        args = ["--technique", technique, "--out", self.out,
                "--run-dir", self.run_dir]
        if not spoof:
            args.append("--no-spoof")
        rc, out, err = run_py(GEN, *args)
        self.assertEqual(rc, 0, err)
        return self.out.read_text()

    def test_spoof_yaml_has_no_check_shells(self):
        yml = self._gen("dual")
        self.assertNotIn("check-answer.py", yml)
        self.assertIn("spoof-emit.py", yml)

    def test_live_yaml_wires_checks_and_priors(self):
        yml = self._gen("dual", spoof=False)
        self.assertIn("check-answer.py", yml)
        self.assertIn("--prior attempt1,attempt2", yml)
        self.assertNotIn("spoof-emit.py", yml)
        self.assertIn("skip_step: true", yml)

    def test_scenario_bounds_cover_chain_depth(self):
        self._gen("repair")
        scen = json.loads((self.run_dir / "scenario.json").read_text())
        self.assertEqual(len(scen), len(load_case_yamls()))
        for cid, w in scen.items():
            self.assertIn(w, (0, 1, 2), f"{cid} win-depth {w} out of range")

    def test_step_counts_per_technique(self):
        import yaml as Y
        base = len(load_case_yamls())
        for tech, per in (("decompose", 2), ("dual", 3), ("repair", 3)):
            yml = self._gen(tech)
            steps = Y.safe_load(yml)["agentic_workflow"]["steps"]
            self.assertEqual(len(steps), base * per + 1, tech)


class CollectorTests(unittest.TestCase):
    def setUp(self):
        self.td = tempfile.TemporaryDirectory()
        self.rd = Path(self.td.name)
        self.cids = ["agentic-01", "agentic-02", "agentic-03"]

    def tearDown(self):
        self.td.cleanup()

    def _check(self, cid, stage, passed, fails=None, sidx=None):
        rec = {"passed": passed, "failures": fails or []}
        if sidx is not None:
            rec["stage_index"] = sidx
        (self.rd / f"check-{cid}-{stage}.json").write_text(
            json.dumps(rec))

    def test_win_depth_and_taxonomy(self):
        self._check(self.cids[0], "solve", True, sidx=0)
        self._check(self.cids[1], "solve", False,
                    [{"check": "json_exact"}], sidx=0)
        self._check(self.cids[1], "repair1", True, sidx=1)
        self._check(self.cids[2], "solve", False,
                    [{"check": "forbidden_phrases"}], sidx=0)
        self._check(self.cids[2], "repair1", False,
                    [{"check": "contains_required"}], sidx=1)
        self._check(self.cids[2], "repair2", False,
                    [{"check": "contains_required"}], sidx=2)
        (self.rd / "outcomes.jsonl").write_text(
            "\n".join(json.dumps({"case": c, "stage": "solve",
                                  "outcome": "RUN", "ts": "x"})
                      for c in self.cids))
        spec = importlib.util.spec_from_file_location("ca", COLLECT)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        s = mod.collect(self.rd)
        self.assertEqual((s["total"], s["passed"]), (3, 2))
        self.assertEqual(s["win_depths"], {"0": 1, "1": 1})
        self.assertEqual(s["winning_stages"], {"solve": 1, "repair1": 1})
        self.assertEqual(s["failure_taxonomy"],
                         {"FORMAT": 1, "LEAK": 1, "CONTENT": 2})
        self.assertEqual(s["mean_depth"], 0.5)
        self.assertEqual(s["failing_cases"], [self.cids[2]])

    def test_no_checks_returns_none(self):
        spec = importlib.util.spec_from_file_location("ca", COLLECT)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        self.assertIsNone(mod.collect(self.rd))


class V10GeneratorTests(unittest.TestCase):
    def setUp(self):
        self.td = tempfile.TemporaryDirectory()
        self.rd = Path(self.td.name)
        self.out = self.rd / "wf.yml"
        self.run_dir = self.rd / "run"

    def tearDown(self):
        self.td.cleanup()

    def _gen(self, technique, spoof=True):
        args = ["--technique", technique, "--out", self.out,
                "--run-dir", self.run_dir]
        if not spoof:
            args.append("--no-spoof")
        rc, out, err = run_py(GEN10, *args)
        self.assertEqual(rc, 0, err)
        return self.out.read_text()

    def test_difficulty_pruning_step_counts(self):
        import yaml as Y
        base = load_case_yamls()
        n_h = sum(1 for c in base.values()
                  if (c.get("rea_plus") or {}).get("difficulty") == "H")
        n_l = len(base) - n_h
        yml = self._gen("plandispatch")
        steps = Y.safe_load(yml)["agentic_workflow"]["steps"]
        self.assertEqual(len(steps), n_h * 4 + n_l * 2 + 1)

    def test_light_cases_have_no_plan_stage(self):
        yml = self._gen("plandispatch")
        self.assertNotIn("plan_agentic-06", yml)
        self.assertIn("plan_agentic-01", yml)

    def test_live_styles_and_checks_wired(self):
        yml = self._gen("adaptive", spoof=False)
        self.assertIn("--style verify", yml)
        self.assertIn("check-answer.py", yml)
        self.assertIn("skip_step: true", yml)

    def test_scenario_respects_difficulty(self):
        self._gen("samplevote")
        scen = json.loads((self.run_dir / "scenario.json").read_text())
        self.assertEqual(len(scen), len(load_case_yamls()))
        self.assertTrue(all(w in (0, 1, 2) for w in scen.values()))


class StageStyleTests(unittest.TestCase):
    def setUp(self):
        self.td = tempfile.TemporaryDirectory()
        self.rd = Path(self.td.name)
        self.case = CASES / "case-agentic-10.yml"

    def tearDown(self):
        self.td.cleanup()

    def test_style_prefixes_appear(self):
        (self.rd / "ans-agentic-10-solve.txt").write_text("prior attempt")
        for style, marker in [
                ("plan", "PLAN FIRST"),
                ("cot", "Think step by step through every rule"),
                ("wait", "Wait. Prior attempt near-miss"),
                ("verify", "BACKWARD CHECK"),
                ("extract", "Output EXACTLY this JSON object")]:
            rc, out, err = run_py(STAGE_EMIT, "--case", self.case,
                                  "--stage", "solve", "--style", style,
                                  "--prior", "solve" if style in
                                  ("wait", "extract") else "",
                                  "--run-dir", self.rd)
            self.assertEqual(rc, 0, err)
            self.assertIn(marker, out, style)

class CheckAnswerIntegration(unittest.TestCase):
    def test_fixture_truth_passes_real_checker(self):
        import yaml
        fix = yaml.safe_load(FIXTURES.read_text())
        with tempfile.TemporaryDirectory() as td:
            for cid, truth in list(fix.items())[:5]:
                ans = Path(td) / f"{cid}.txt"
                ans.write_text(truth + "\n")
                out = Path(td) / f"{cid}.json"
                r = subprocess.run(
                    [sys.executable, str(CHECK_ANSWER),
                     "--case", CASES / f"case-{cid}.yml",
                     "--text-file", ans, "--out", out],
                    capture_output=True, text=True, timeout=60)
                self.assertEqual(r.returncode, 0,
                                 f"{cid}: {r.stderr[-200:]}")
                self.assertTrue(json.loads(out.read_text())["passed"],
                                cid)


if __name__ == "__main__":
    unittest.main(verbosity=2)
