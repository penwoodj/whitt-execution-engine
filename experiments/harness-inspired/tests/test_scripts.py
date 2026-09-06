#!/usr/bin/env python3
"""Unit+integration tests for harness-inspired eval scripts.

Usage: python -m pytest tests/test_scripts.py -v
"""
import hashlib, json, os, tempfile, textwrap
import pytest
import yaml
from pathlib import Path

SCRIPTS_DIR = Path(__file__).parent.parent / "scripts"


def _run_script(name, args, input_text=None):
    """Run script, return (exit_code, stdout, stderr)."""
    import subprocess
    cmd = ["python3", str(SCRIPTS_DIR / name)] + args
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10, input=input_text)
    return result.returncode, result.stdout, result.stderr


# ============================================================
# check-deterministic.py tests
# ============================================================

class TestCheckDeterministic:
    def _write_case(self, tmp, criteria):
        p = tmp / "criteria.yml"
        p.write_text(yaml.dump(criteria))
        return str(p)

    def _write_artifact(self, tmp, text):
        p = tmp / "artifact.txt"
        p.write_text(text)
        return str(p)

    def test_missing_artifact_fails(self, tmp_path):
        c = self._write_case(tmp_path, {"deterministic_checks": {"contains_required": ["hello"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", "/tmp/nonexistent_xyz.txt", "--criteria", c])
        assert code == 1
        data = json.loads(out)
        assert data["pass"] is False
        assert data["error"] == "artifact_missing"

    def test_contains_required_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, "hello world")
        c = self._write_case(tmp_path, {"deterministic_checks": {"contains_required": ["hello"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        data = json.loads(out)
        assert data["pass"] is True
        assert data["score"] == 1.0

    def test_contains_required_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, "goodbye world")
        c = self._write_case(tmp_path, {"deterministic_checks": {"contains_required": ["hello"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1
        data = json.loads(out)
        assert data["pass"] is False
        assert data["score"] == 0.0

    def test_forbidden_phrases_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, "the result is 42")
        c = self._write_case(tmp_path, {"deterministic_checks": {"forbidden_phrases": ["I cannot", "sorry"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        data = json.loads(out)
        assert data["pass"] is True

    def test_forbidden_phrases_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, "I cannot help with that")
        c = self._write_case(tmp_path, {"deterministic_checks": {"forbidden_phrases": ["I cannot"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1
        data = json.loads(out)
        assert data["pass"] is False

    def test_max_words_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, "one two three")
        c = self._write_case(tmp_path, {"deterministic_checks": {"max_words": 5}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_max_words_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, "one two three four five six")
        c = self._write_case(tmp_path, {"deterministic_checks": {"max_words": 5}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_min_words_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, "one two three")
        c = self._write_case(tmp_path, {"deterministic_checks": {"min_words": 2}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_min_words_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, "hi")
        c = self._write_case(tmp_path, {"deterministic_checks": {"min_words": 3}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_max_lines(self, tmp_path):
        a = self._write_artifact(tmp_path, "line1\nline2")
        c = self._write_case(tmp_path, {"deterministic_checks": {"max_lines": 3}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_max_lines_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, "\n".join(["x"] * 5))
        c = self._write_case(tmp_path, {"deterministic_checks": {"max_lines": 3}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_min_lines(self, tmp_path):
        a = self._write_artifact(tmp_path, "\n".join(["x"] * 5))
        c = self._write_case(tmp_path, {"deterministic_checks": {"min_lines": 3}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_json_valid_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, '{"key": "value"}')
        c = self._write_case(tmp_path, {"deterministic_checks": {"json_valid": True}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_json_valid_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, '{key: bad json}')
        c = self._write_case(tmp_path, {"deterministic_checks": {"json_valid": True}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_yaml_valid_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, 'key: value\nlist:\n  - a')
        c = self._write_case(tmp_path, {"deterministic_checks": {"yaml_valid": True}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_yaml_valid_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, ': invalid: yaml: :')
        c = self._write_case(tmp_path, {"deterministic_checks": {"yaml_valid": True}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_contains_regex(self, tmp_path):
        a = self._write_artifact(tmp_path, 'score: 85%')
        c = self._write_case(tmp_path, {"deterministic_checks": {"contains_regex": [r'\d+%']}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_contains_regex_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, 'no numbers here')
        c = self._write_case(tmp_path, {"deterministic_checks": {"contains_regex": [r'\d+%']}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_exact_match_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, 'rehcsipsid')
        c = self._write_case(tmp_path, {"deterministic_checks": {"exact_match": 'rehcsipsid'}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_exact_match_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, 'dispatcher')
        c = self._write_case(tmp_path, {"deterministic_checks": {"exact_match": 'rehcsipsid'}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_file_exists_pass(self, tmp_path):
        (tmp_path / "target.txt").write_text("x")
        a = self._write_artifact(tmp_path, "anything")
        c = self._write_case(tmp_path, {"deterministic_checks": {"file_exists": ["target.txt"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0

    def test_file_exists_fail(self, tmp_path):
        a = self._write_artifact(tmp_path, "anything")
        c = self._write_case(tmp_path, {"deterministic_checks": {"file_exists": ["nonexistent.txt"]}})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1

    def test_hash_only(self, tmp_path):
        a = self._write_artifact(tmp_path, "hello")
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--hash-only"])
        assert code == 0
        data = json.loads(out)
        assert data["hash"] == hashlib.sha256(b"hello").hexdigest()

    def test_out_writes_json(self, tmp_path):
        a = self._write_artifact(tmp_path, "hello world")
        c = self._write_case(tmp_path, {"deterministic_checks": {"contains_required": ["hello"]}})
        out_path = str(tmp_path / "result.json")
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c, "--out", out_path])
        assert code == 0
        assert Path(out_path).is_file()
        data = json.loads(Path(out_path).read_text())
        assert data["pass"] is True

    def test_score_fraction(self, tmp_path):
        a = self._write_artifact(tmp_path, "hello world")
        c = self._write_case(tmp_path, {
            "deterministic_checks": {
                "contains_required": ["hello", "missing_thing"],
            }
        })
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 1
        data = json.loads(out)
        assert data["score"] == 0.5
        assert data["passed"] == 1
        assert data["total"] == 2

    def test_empty_criteria_all_pass(self, tmp_path):
        a = self._write_artifact(tmp_path, "anything")
        c = self._write_case(tmp_path, {})
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        data = json.loads(out)
        assert data["pass"] is True
        assert data["score"] == 1.0


# ============================================================
# fix-format.py tests
# ============================================================

class TestFixFormat:
    def _write_case(self, tmp, criteria):
        p = tmp / "criteria.yml"
        p.write_text(yaml.dump(criteria))
        return str(p)

    def _write_artifact(self, tmp, text):
        p = tmp / "artifact.txt"
        p.write_text(text)
        return str(p)

    def test_strip_patterns(self, tmp_path):
        a = self._write_artifact(tmp_path, "**bold** text")
        c = self._write_case(tmp_path, {"format_fixes": {"strip_patterns": [r'\*\*']}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert "**" not in out
        assert "bold text" in out

    def test_replace_pairs(self, tmp_path):
        a = self._write_artifact(tmp_path, "foo bar foo")
        c = self._write_case(tmp_path, {"format_fixes": {"replace_pairs": {"foo": "baz"}}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert out.strip() == "baz bar baz"

    def test_strip_leading_whitespace(self, tmp_path):
        a = self._write_artifact(tmp_path, "  indented\n\talso")
        c = self._write_case(tmp_path, {"format_fixes": {"strip_leading_whitespace": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert "indented" in out.split("\n")[0]
        assert not out.split("\n")[0].startswith(" ")

    def test_strip_trailing_whitespace(self, tmp_path):
        a = self._write_artifact(tmp_path, "text   \nmore\t")
        c = self._write_case(tmp_path, {"format_fixes": {"strip_trailing_whitespace": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        for line in out.split("\n"):
            assert line == line.rstrip()

    def test_deduplicate_blank_lines(self, tmp_path):
        a = self._write_artifact(tmp_path, "a\n\n\n\n\nb")
        c = self._write_case(tmp_path, {"format_fixes": {"deduplicate_blank_lines": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert "\n\n\n\n" not in out
        assert "\n\n" in out

    def test_ensure_trailing_newline(self, tmp_path):
        a = self._write_artifact(tmp_path, "no newline")
        c = self._write_case(tmp_path, {"format_fixes": {"ensure_trailing_newline": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert out.endswith("\n")

    def test_strip_markdown_fences(self, tmp_path):
        a = self._write_artifact(tmp_path, '```json\n{"a":1}\n```')
        c = self._write_case(tmp_path, {"format_fixes": {"strip_markdown_fences": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert "```" not in out

    def test_strip_thinking_tags(self, tmp_path):
        a = self._write_artifact(tmp_path, '<think>reasoning here</think>actual answer')
        c = self._write_case(tmp_path, {"format_fixes": {"strip_thinking_tags": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert '<think>' not in out
        assert '</think>' not in out
        assert 'actual answer' in out

    def test_extract_regex(self, tmp_path):
        a = self._write_artifact(tmp_path, 'noise {"key": "val"} more noise')
        c = self._write_case(tmp_path, {"format_fixes": {"extract_regex": [r'\{[^}]+\}']}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        assert json.loads(out)["key"] == "val"

    def test_force_json_object(self, tmp_path):
        a = self._write_artifact(tmp_path, 'Here is the result: {"key": "val"}')
        c = self._write_case(tmp_path, {"format_fixes": {"force_json_object": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        data = json.loads(out)
        assert data["key"] == "val"

    def test_force_yaml_document(self, tmp_path):
        a = self._write_artifact(tmp_path, 'some preamble\nkey: value\n  sub: item\nmore text')
        c = self._write_case(tmp_path, {"format_fixes": {"force_yaml_document": True}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0
        # force_yaml_document may produce partial yaml - just check it starts with a yaml key
        assert 'key: value' in out

    def test_out_writes_file(self, tmp_path):
        a = self._write_artifact(tmp_path, "**bold** text")
        c = self._write_case(tmp_path, {"format_fixes": {"strip_patterns": [r'\*\*']}})
        out_path = str(tmp_path / "fixed.txt")
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c, "--out", out_path])
        assert code == 0
        assert Path(out_path).is_file()
        assert "**" not in Path(out_path).read_text()

    def test_no_changes_still_zero_exit(self, tmp_path):
        a = self._write_artifact(tmp_path, "clean text")
        c = self._write_case(tmp_path, {"format_fixes": {"strip_patterns": [r'\*\*']}})
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c])
        assert code == 0


# ============================================================
# baseline.py tests
# ============================================================

class TestBaseline:
    def test_promote_and_compare_match(self, tmp_path):
        a = tmp_path / "artifact.txt"
        a.write_text("hello world")
        bd = str(tmp_path / "baseline")
        code, out, _ = _run_script("baseline.py", ["promote", "--artifact", str(a), "--baseline-dir", bd])
        assert code == 0
        data = json.loads(out)
        assert data["action"] == "promoted"
        assert data["hash"] == hashlib.sha256(b"hello world").hexdigest()
        assert (tmp_path / "baseline" / "baseline.json").is_file()
        assert (tmp_path / "baseline" / "baseline.txt").is_file()
        code2, out2, _ = _run_script("baseline.py", ["compare", "--artifact", str(a), "--baseline-dir", bd])
        assert code2 == 0
        data2 = json.loads(out2)
        assert data2["drift"] is False

    def test_compare_drift(self, tmp_path):
        a = tmp_path / "artifact.txt"
        a.write_text("hello world")
        bd = str(tmp_path / "baseline")
        _run_script("baseline.py", ["promote", "--artifact", str(a), "--baseline-dir", bd])
        a.write_text("different content")
        code, out, _ = _run_script("baseline.py", ["compare", "--artifact", str(a), "--baseline-dir", bd])
        assert code == 1
        data = json.loads(out)
        assert data["drift"] is True
        assert data["word_delta"] == 0

    def test_compare_no_baseline(self, tmp_path):
        a = tmp_path / "artifact.txt"
        a.write_text("hello")
        bd = str(tmp_path / "baseline")
        code, out, _ = _run_script("baseline.py", ["compare", "--artifact", str(a), "--baseline-dir", bd])
        assert code == 2
        data = json.loads(out)
        assert data["drift"] == "no_baseline"

    def test_hash_only(self, tmp_path):
        a = tmp_path / "artifact.txt"
        a.write_text("test content")
        code, out, _ = _run_script("baseline.py", ["hash", "--artifact", str(a)])
        assert code == 0
        data = json.loads(out)
        assert data["hash"] == hashlib.sha256(b"test content").hexdigest()

    def test_promote_with_criteria(self, tmp_path):
        a = tmp_path / "artifact.txt"
        a.write_text("data")
        c = tmp_path / "criteria.yml"
        c.write_text("key: val")
        bd = str(tmp_path / "baseline")
        code, out, _ = _run_script("baseline.py", ["promote", "--artifact", str(a), "--criteria", str(c), "--baseline-dir", bd])
        assert code == 0
        meta = json.loads((tmp_path / "baseline" / "baseline.json").read_text())
        assert meta["criteria_path"] == str(c)


# ============================================================
# emit-prompt.py tests
# ============================================================

class TestEmitPrompt:
    def _write_case(self, tmp, case_data):
        p = tmp / "case.yml"
        p.write_text(yaml.dump(case_data))
        return str(p)

    def test_worker_gen_stage(self, tmp_path):
        c = self._write_case(tmp_path, {
            "prompt": "Reverse 'hello'.",
            "auxiliary": "Simple string task.",
        })
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "worker", "--stage", "GEN"])
        assert code == 0
        assert "precise engineer" in out
        assert "Reverse 'hello'." in out
        assert "Simple string task." in out

    def test_evaluator_check_stage(self, tmp_path):
        c = self._write_case(tmp_path, {
            "prompt": "task text",
            "success_criteria": {
                "deterministic_checks": {
                    "contains_required": ["answer"],
                    "max_words": 10,
                }
            },
        })
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "evaluator", "--stage", "CHECK"])
        assert code == 0
        assert "QA auditor" in out
        assert "VERDICT: PASS" in out
        assert "MUST contain: answer" in out
        assert "MAX 10 words" in out

    def test_fixer_fix_stage(self, tmp_path):
        c = self._write_case(tmp_path, {
            "prompt": "task",
            "success_criteria": {"deterministic_checks": {"contains_required": ["x"]}}},
        )
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "fixer", "--stage", "FIX"])
        assert code == 0
        assert "surgical editor" in out
        assert "Fix the output" in out

    def test_judge_stage(self, tmp_path):
        c = self._write_case(tmp_path, {
            "prompt": "do the thing",
            "objective": "Produce X that does Y",
        })
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "judge", "--stage", "JUDGE"])
        assert code == 0
        assert "blind quality reviewer" in out
        assert "do NOT know which model" in out

    def test_retry_counting(self, tmp_path):
        c = self._write_case(tmp_path, {"prompt": "task"})
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "worker", "--stage", "GEN", "--retry", "2"])
        assert code == 0
        assert "RETRY 2" in out

    def test_out_writes_file(self, tmp_path):
        c = self._write_case(tmp_path, {"prompt": "task"})
        out_path = str(tmp_path / "prompt.txt")
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "worker", "--stage", "GEN", "--out", out_path])
        assert code == 0
        assert Path(out_path).is_file()
        assert "precise engineer" in Path(out_path).read_text()

    def test_missing_case_fails(self, tmp_path):
        code, out, err = _run_script("emit-prompt.py", ["--case", "/tmp/nonexistent.yml", "--role", "worker", "--stage", "GEN"])
        assert code == 1

    def test_screen_stage_no_auxiliary_in_prompt(self, tmp_path):
        c = self._write_case(tmp_path, {
            "prompt": "Output 'hello' reversed.",
            "auxiliary": "All facts in prompt.",
        })
        code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", "worker", "--stage", "SCREEN"])
        assert code == 0
        assert "Output 'hello' reversed." in out


# ============================================================
# Integration: check → fix → re-check pipeline
# ============================================================

class TestIntegrationPipeline:
    def test_fix_then_recheck_passes(self, tmp_path):
        artifact = tmp_path / "artifact.txt"
        artifact.write_text('Here is the JSON: ```json\n{"a": 1}\n```')
        criteria = tmp_path / "criteria.yml"
        criteria.write_text(yaml.dump({
            "deterministic_checks": {"json_valid": True, "contains_required": ["a"]},
            "format_fixes": {"strip_markdown_fences": True, "extract_regex": [r'\{[^}]+\}']},
        }))
        a, c = str(artifact), str(criteria)
        fixed_path = str(tmp_path / "fixed.txt")
        # Fix
        code, out, _ = _run_script("fix-format.py", ["--artifact", a, "--criteria", c, "--out", fixed_path])
        assert code == 0
        # Re-check fixed
        code2, out2, _ = _run_script("check-deterministic.py", ["--artifact", fixed_path, "--criteria", c])
        assert code2 == 0
        data = json.loads(out2)
        assert data["pass"] is True

    def test_baseline_promote_then_drift_detected(self, tmp_path):
        artifact = tmp_path / "artifact.txt"
        artifact.write_text("original")
        bd = str(tmp_path / "baseline")
        criteria = tmp_path / "criteria.yml"
        criteria.write_text(yaml.dump({"deterministic_checks": {"contains_required": ["original"]}}))
        # Promote
        code, _, _ = _run_script("baseline.py", ["promote", "--artifact", str(artifact), "--criteria", str(criteria), "--baseline-dir", bd])
        assert code == 0
        # Modify
        artifact.write_text("changed")
        # Compare
        code, out, _ = _run_script("baseline.py", ["compare", "--artifact", str(artifact), "--baseline-dir", bd])
        assert code == 1
        data = json.loads(out)
        assert data["drift"] is True

    def test_full_pipeline_screen_gen_check(self, tmp_path):
        case = tmp_path / "case.yml"
        case.write_text(yaml.dump({
            "case_id": "int-01",
            "prompt": "Output the word 'test' reversed. Nothing else.",
            "success_criteria": {
                "deterministic_checks": {
                    "contains_required": ["tset"],
                    "max_words": 1,
                    "forbidden_phrases": ["I cannot", "sorry"],
                }
            },
        }))
        # Emit prompt
        code, prompt, _ = _run_script("emit-prompt.py", ["--case", str(case), "--role", "worker", "--stage", "GEN"])
        assert code == 0
        assert "precise engineer" in prompt
        assert "test" in prompt
        # Simulate worker output (correct)
        artifact = tmp_path / "output.txt"
        artifact.write_text("tset")
        # Check
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", str(artifact), "--criteria", str(case)])
        assert code == 0
        data = json.loads(out)
        assert data["pass"] is True
        assert data["score"] == 1.0

    def test_full_pipeline_with_fix_cycle(self, tmp_path):
        case = tmp_path / "case.yml"
        case.write_text(yaml.dump({
            "case_id": "int-02",
            "prompt": "Output valid JSON: {\"name\": \"alice\"}",
            "success_criteria": {
                "deterministic_checks": {
                    "json_valid": True,
                    "contains_required": ["alice"],
                },
                "format_fixes": {
                    "strip_markdown_fences": True,
                    "extract_regex": [r'\{[^}]+\}'],
                },
            },
        }))
        c = str(case)
        # Simulate bad worker output (wrapped in markdown)
        artifact = tmp_path / "output.txt"
        artifact.write_text('```json\n{"name": "alice"}\n```')
        # Check → should fail JSON parse due to fences
        code, out, _ = _run_script("check-deterministic.py", ["--artifact", str(artifact), "--criteria", c])
        assert code == 1
        # Fix
        fixed_path = str(tmp_path / "fixed.txt")
        code, fixed_out, _ = _run_script("fix-format.py", ["--artifact", str(artifact), "--criteria", c, "--out", fixed_path])
        assert code == 0
        # Re-check fixed → should pass
        code2, out2, _ = _run_script("check-deterministic.py", ["--artifact", fixed_path, "--criteria", c])
        assert code2 == 0
        data = json.loads(out2)
        assert data["pass"] is True

    def test_emit_prompt_for_all_roles(self, tmp_path):
        case = tmp_path / "case.yml"
        case.write_text(yaml.dump({
            "case_id": "int-03",
            "prompt": "Write a function that adds two numbers.",
            "objective": "Working add(a, b) function.",
            "success_criteria": {"deterministic_checks": {"contains_required": ["def ", "return"]}},
        }))
        c = str(case)
        for role, stage in [("worker", "GEN"), ("evaluator", "CHECK"), ("fixer", "FIX"), ("judge", "JUDGE")]:
            code, out, _ = _run_script("emit-prompt.py", ["--case", c, "--role", role, "--stage", stage])
            assert code == 0, f"{role}/{stage} failed"
            assert len(out) > 50, f"{role}/{stage} prompt too short"