#!/usr/bin/env python3
"""Unit tests for ha-* case integrity, json_exact check, leak-safe feedback,
collector stats, and trace scripts. Run: python3 -m pytest tests/test_agentic_suite.py -v
(or plain: python3 tests/test_agentic_suite.py)
"""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
SCRIPTS = ROOT / "scripts"
CASES = ROOT / "cases"


def run_script(cmd):
    return subprocess.run(
        [sys.executable, str(SCRIPTS / cmd[0])] + cmd[1:],
        capture_output=True, text=True, timeout=30, cwd=str(ROOT),
    )


def write_tmp(text, suffix=".txt"):
    f = tempfile.NamedTemporaryFile(mode="w", suffix=suffix, delete=False)
    f.write(text)
    f.close()
    return Path(f.name)


class TestHaCases:
    def test_ten_cases_exist(self):
        ids = []
        for i in range(1, 11):
            p = CASES / f"ha-{i:02d}.yml"
            assert p.is_file(), f"missing {p}"
            ids.append(p)
        assert len(ids) == 10

    def test_case_structure_valid(self):
        for i in range(1, 11):
            case = yaml.safe_load((CASES / f"ha-{i:02d}.yml").read_text())
            assert case["case_id"] == f"ha-{i:02d}"
            assert case["hops"] in (3, 4, 5, 6)
            assert case["difficulty"] in ("light", "heavy")
            assert len(case["prompt"]) > 200, f"ha-{i:02d} prompt too short"
            dc = case["success_criteria"]["deterministic_checks"]
            assert "json_exact" in dc, f"ha-{i:02d} missing json_exact"
            json.loads(dc["json_exact"])
            assert "forbidden_phrases" in dc
            assert "provenance" in case, f"ha-{i:02d} missing provenance"

    def test_difficulty_split_five_heavy_five_light(self):
        heavies = []
        for i in range(1, 11):
            case = yaml.safe_load((CASES / f"ha-{i:02d}.yml").read_text())
            if case["difficulty"] == "heavy":
                heavies.append(case["hops"])
        assert len(heavies) == 5
        assert all(h >= 5 for h in heavies)

    def test_distractor_present_every_case(self):
        for i in range(1, 11):
            case = yaml.safe_load((CASES / f"ha-{i:02d}.yml").read_text())
            flat = " ".join(case["prompt"].split())
            assert "do not manage" in flat, f"ha-{i:02d} missing distractor"

    def test_expected_json_parseable_and_multitype(self):
        seen_bool = False
        seen_str = False
        for i in range(1, 11):
            case = yaml.safe_load((CASES / f"ha-{i:02d}.yml").read_text())
            expected = json.loads(case["success_criteria"]["deterministic_checks"]["json_exact"])
            assert isinstance(expected, dict)
            if any(isinstance(v, bool) for v in expected.values()):
                seen_bool = True
            if any(isinstance(v, str) for v in expected.values()):
                seen_str = True
        assert seen_bool and seen_str


class TestJsonExactCheck:
    def test_pass_on_exact(self):
        art = write_tmp('{"stages_rerun": 3}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 0
        result = json.loads(r.stdout)
        assert result["pass"] is True

    def test_fail_on_wrong_value(self):
        art = write_tmp('{"stages_rerun": 4}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 1
        result = json.loads(r.stdout)
        assert result["pass"] is False

    def test_fail_on_unparseable(self):
        art = write_tmp("not json at all\n")
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 1

    def test_fail_on_array_not_object(self):
        art = write_tmp("[1, 2, 3]\n")
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 1
        result = json.loads(r.stdout)
        je = [c for c in result["checks"] if c["name"] == "json_exact"][0]
        assert je["pass"] is False

    def test_key_order_insensitive(self):
        art = write_tmp('{"never_judged": 1, "judged": 4}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-02.yml")])
        assert r.returncode == 0

    def test_bool_distinct_from_int(self):
        art = write_tmp('{"spend": 3400, "fixes_skipped": 1, "verified": 1}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-05.yml")])
        assert r.returncode == 1

    def test_out_flag_writes_result(self):
        art = write_tmp('{"stages_rerun": 3}\n')
        out = tempfile.mktemp(suffix=".json")
        run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml"), "--out", out])
        assert Path(out).is_file()
        assert json.loads(Path(out).read_text())["pass"] is True

    def test_cot_reasoning_then_json_passes(self):
        art = write_tmp('Step 1: A won at solve.\nStep 2: D re-runs 3 stages.\nFinal: {"stages_rerun": 3}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 0

    def test_cot_with_wrong_json_fails(self):
        art = write_tmp('Reasoning here.\nFinal: {"stages_rerun": 99}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 1

    def test_last_json_wins_over_earlier(self):
        art = write_tmp('Draft: {"stages_rerun": 5}\nCorrected: {"stages_rerun": 3}\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 0

    def test_no_json_fails(self):
        art = write_tmp('Just text, no braces at all.\n')
        r = run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 1

    def test_missing_artifact_fails(self):
        r = run_script(["check-deterministic.py", "--artifact", "/nonexistent/x.txt", "--criteria", str(CASES / "ha-01.yml")])
        assert r.returncode == 1
        assert "artifact_missing" in r.stdout


class TestLeakSafeFeedback:
    def _fail_result(self):
        art = write_tmp('{"stages_rerun": 4}\n')
        out = tempfile.mktemp(suffix=".json")
        run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml"), "--out", out])
        return Path(out)

    def test_feedback_names_check_but_not_value(self):
        result = self._fail_result()
        r = run_script(["feedback.py", "--result", str(result)])
        assert r.returncode == 0
        assert "json_exact" in r.stdout
        assert '"stages_rerun": 3' not in r.stdout
        assert "stages_rerun" not in r.stdout or "withheld" in r.stdout

    def test_feedback_redacts_leaky_words(self):
        result = self._fail_result()
        r = run_script(["feedback.py", "--result", str(result)])
        for word in ("expected", "golden", "answer"):
            assert word not in r.stdout.lower()

    def test_feedback_all_pass_message(self):
        result = write_tmp(json.dumps({"pass": True, "checks": [{"name": "json_exact", "pass": True}]}), ".json")
        r = run_script(["feedback.py", "--result", str(result)])
        assert "ALL CHECKS PASSED" in r.stdout

    def test_feedback_classifies_format(self):
        result = self._fail_result()
        r = run_script(["feedback.py", "--result", str(result)])
        assert "[FORMAT]" in r.stdout


class TestFeedbackV2:
    def test_feedback_includes_observed_artifact(self):
        result = write_tmp(json.dumps({"pass": False, "checks": [{"name": "json_exact", "pass": False, "evidence": "value or shape mismatch"}]}), ".json")
        art = write_tmp('{"spend": 1700}\n')
        r = run_script(["feedback.py", "--result", str(result), "--artifact", str(art)])
        assert r.returncode == 0
        assert "spend" in r.stdout
        assert "Your output was" in r.stdout

    def test_feedback_never_leaks_expected(self):
        art = write_tmp('{"stages_rerun": 4}\n')
        out = tempfile.mktemp(suffix=".json")
        run_script(["check-deterministic.py", "--artifact", str(art), "--criteria", str(CASES / "ha-01.yml"), "--out", out])
        r = run_script(["feedback.py", "--result", out, "--artifact", str(art)])
        assert '"stages_rerun": 3' not in r.stdout


class TestCollector:
    def _trace(self, lines):
        return write_tmp("\n".join(json.dumps(l) for l in lines) + "\n", ".jsonl")

    def test_depth_zero_pass(self):
        t = self._trace([{"case": "c1", "gate": "check", "verdict": "pass"}, {"case": "c1", "gate": "judge", "verdict": "pass"}])
        r = run_script(["collect.py", "--trace", str(t)])
        stats = json.loads(r.stdout)
        assert stats["cases"] == 1 and stats["pass"] == 1
        assert stats["win_depth"] == {"0": 1}
        assert stats["wasted"] == 0

    def test_depth_two_pass(self):
        t = self._trace([
            {"case": "c1", "gate": "check", "verdict": "fail"},
            {"case": "c1", "gate": "fix_1", "verdict": "fail"},
            {"case": "c1", "gate": "fix_2", "verdict": "pass"},
            {"case": "c1", "gate": "judge", "verdict": "pass"},
        ])
        stats = json.loads(run_script(["collect.py", "--trace", str(t)]).stdout)
        assert stats["win_depth"] == {"2": 1}
        assert stats["wasted"] == 2

    def test_full_fail(self):
        t = self._trace([
            {"case": "c1", "gate": "check", "verdict": "fail"},
            {"case": "c1", "gate": "fix_1", "verdict": "fail"},
            {"case": "c1", "gate": "fix_2", "verdict": "fail"},
            {"case": "c1", "gate": "end_fail", "verdict": "fail"},
        ])
        stats = json.loads(run_script(["collect.py", "--trace", str(t)]).stdout)
        assert stats["fail"] == 1 and stats["pass"] == 0
        assert stats["wasted"] == 3
        assert stats["per_case"]["c1"]["depth"] is None

    def test_multi_case_mixed(self):
        t = self._trace([
            {"case": "a", "gate": "check", "verdict": "pass"},
            {"case": "b", "gate": "check", "verdict": "fail"},
            {"case": "b", "gate": "fix_1", "verdict": "pass"},
            {"case": "b", "gate": "judge", "verdict": "pass"},
        ])
        stats = json.loads(run_script(["collect.py", "--trace", str(t)]).stdout)
        assert stats["cases"] == 2
        assert stats["pass"] == 2
        assert stats["wasted"] == 1

    def test_empty_trace(self):
        t = self._trace([])
        stats = json.loads(run_script(["collect.py", "--trace", str(t)]).stdout)
        assert stats["cases"] == 0

    def test_malformed_lines_skipped(self):
        t = self._trace([{"case": "a", "gate": "check", "verdict": "pass"}])
        with open(t, "a") as f:
            f.write("this is not json\n")
        stats = json.loads(run_script(["collect.py", "--trace", str(t)]).stdout)
        assert stats["cases"] == 1

    def test_out_flag(self):
        t = self._trace([{"case": "a", "gate": "check", "verdict": "pass"}])
        out = tempfile.mktemp(suffix=".json")
        run_script(["collect.py", "--trace", str(t), "--out", out])
        assert json.loads(Path(out).read_text())["cases"] == 1


class TestTraceScripts:
    def test_trace_append(self):
        t = tempfile.mktemp(suffix=".jsonl")
        r = run_script(["trace-append.py", t, "end_fail", "fail"])
        assert r.returncode == 0
        entry = json.loads(Path(t).read_text().strip())
        assert entry["gate"] == "end_fail" and entry["verdict"] == "fail"

    def test_trace_append_case_kv(self):
        t = tempfile.mktemp(suffix=".jsonl")
        r = run_script(["trace-append.py", t, "end_fail", "fail", "case=ha-07"])
        assert r.returncode == 0
        entry = json.loads(Path(t).read_text().strip())
        assert entry["case"] == "ha-07"

    def test_trace_check_with_case_arg(self):
        result = write_tmp(json.dumps({"pass": True}), ".json")
        t = tempfile.mktemp(suffix=".jsonl")
        r = run_script(["trace-check.py", str(result), "check", t, "ha-01"])
        assert r.returncode == 0
        entry = json.loads(Path(t).read_text().strip())
        assert entry["gate"] == "check" and entry["case"] == "ha-01"

    def test_trace_judge_with_case_arg(self):
        v = write_tmp("VERDICT: PASS\nREASON: ok\nSCORE: 9\n")
        t = tempfile.mktemp(suffix=".jsonl")
        r = run_script(["trace-judge.py", str(v), t, "ha-02"])
        assert r.returncode == 0
        entry = json.loads(Path(t).read_text().strip())
        assert entry["gate"] == "judge" and entry["case"] == "ha-02"

    def test_trace_check_pass(self):
        result = write_tmp(json.dumps({"pass": True}), ".json")
        t = tempfile.mktemp(suffix=".jsonl")
        run_script(["trace-check.py", str(result), "check", t])
        entry = json.loads(Path(t).read_text().strip())
        assert entry["gate"] == "check" and entry["verdict"] == "pass"

    def test_trace_check_fail(self):
        result = write_tmp(json.dumps({"pass": False}), ".json")
        t = tempfile.mktemp(suffix=".jsonl")
        run_script(["trace-check.py", str(result), "fix_1", t])
        entry = json.loads(Path(t).read_text().strip())
        assert entry["gate"] == "fix_1" and entry["verdict"] == "fail"

    def test_trace_judge_pass(self):
        v = write_tmp("VERDICT: PASS\nREASON: ok\nSCORE: 9\n")
        t = tempfile.mktemp(suffix=".jsonl")
        run_script(["trace-judge.py", str(v), t])
        entry = json.loads(Path(t).read_text().strip())
        assert entry["gate"] == "judge" and entry["verdict"] == "pass"


class TestScreenValidate:
    def test_valid_ha_case_passes(self):
        r = run_script(["screen-validate.py", str(CASES / "ha-01.yml")])
        assert r.returncode == 0

    def test_missing_file_fails(self):
        r = run_script(["screen-validate.py", "/nonexistent.yml"])
        assert r.returncode != 0

    def test_garbage_yaml_fails(self):
        bad = write_tmp(":::: not yaml [\n", ".yml")
        r = run_script(["screen-validate.py", str(bad)])
        assert r.returncode != 0


if __name__ == "__main__":
    import pytest
    sys.exit(pytest.main([__file__, "-v"]))
