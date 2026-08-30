#!/usr/bin/env python3
"""Unit tests for v6 scripts: table_lib, check-table, aggregate, failflag,
table-feedback, spoof-write. Run: python3 -m pytest tests/test_v6_scripts.py -v
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


class TestTableLib:
    def test_parse_simple_rows(self):
        sys.path.insert(0, str(SCRIPTS))
        from table_lib import parse_table
        t = 'ENTITY a | 3\nENTITY b | "x"\nENTITY c | true\n'
        d = parse_table(t)
        assert d == {"a": 3, "b": "x", "c": True}

    def test_parse_object_value(self):
        sys.path.insert(0, str(SCRIPTS))
        from table_lib import parse_table
        d = parse_table('ENTITY c1 | {"final": "FAIL", "judge_calls": 0}\n')
        assert d["c1"] == {"final": "FAIL", "judge_calls": 0}

    def test_parse_garbage_value_kept_as_string(self):
        sys.path.insert(0, str(SCRIPTS))
        from table_lib import parse_table
        d = parse_table("ENTITY a | not json\n")
        assert d["a"] == "not json"

    def test_parse_ignores_non_entity_lines(self):
        sys.path.insert(0, str(SCRIPTS))
        from table_lib import parse_table
        d = parse_table("Reasoning here.\nENTITY a | 1\nMore text\n")
        assert d == {"a": 1}

    def test_parse_bare_rows_no_entity_prefix(self):
        sys.path.insert(0, str(SCRIPTS))
        from table_lib import parse_table
        t = "case_A | 0\ncase_B | 2\ncase_C | 2\ncase_D | 3\n"
        d = parse_table(t)
        assert d == {"case_A": 0, "case_B": 2, "case_C": 2, "case_D": 3}

    def test_explicit_ids_tracks_prefix(self):
        sys.path.insert(0, str(SCRIPTS))
        from table_lib import explicit_ids
        t = "ENTITY a | 1\nb | 2\nENTITY ghost | 3\n"
        assert explicit_ids(t) == {"a", "ghost"}


class TestCheckTable:
    def test_correct_table_passes(self):
        art = write_tmp("ENTITY case_A | 0\nENTITY case_B | 0\nENTITY case_C | 0\nENTITY case_D | 3\n")
        r = run_script(["check-table.py", "--artifact", str(art), "--case", str(CASES / "ha-01.yml")])
        assert r.returncode == 0
        assert json.loads(r.stdout)["pass"] is True

    def test_missing_row_fails_and_names_id(self):
        art = write_tmp("ENTITY case_A | 0\nENTITY case_B | 0\nENTITY case_C | 0\n")
        r = run_script(["check-table.py", "--artifact", str(art), "--case", str(CASES / "ha-01.yml")])
        assert r.returncode == 1
        res = json.loads(r.stdout)
        assert res["pass"] is False
        assert res["missing"] == ["case_D"]

    def test_wrong_value_fails_shows_observed_not_expected(self):
        art = write_tmp("ENTITY case_A | 99\nENTITY case_B | 0\nENTITY case_C | 0\nENTITY case_D | 3\n")
        r = run_script(["check-table.py", "--artifact", str(art), "--case", str(CASES / "ha-01.yml")])
        res = json.loads(r.stdout)
        assert res["wrong"][0]["observed"] == 99
        assert "expected" not in json.dumps(res["wrong"][0])

    def test_float_and_object_values(self):
        good = 'ENTITY tech_X | {"passes": 3, "mean": 1.5}\nENTITY tech_Y | {"passes": 4, "mean": 0.75}\nENTITY tech_Z | {"passes": 3, "mean": 1.75}\n'
        art = write_tmp(good)
        r = run_script(["check-table.py", "--artifact", str(art), "--case", str(CASES / "ha-03.yml")])
        assert r.returncode == 0

    def test_missing_artifact(self):
        r = run_script(["check-table.py", "--artifact", "/nonexistent/x", "--case", str(CASES / "ha-01.yml")])
        assert r.returncode == 1
        assert "artifact_missing" in r.stdout

    def test_only_flag_filters(self):
        art = write_tmp("ENTITY case_D | 3\n")
        r = run_script(["check-table.py", "--artifact", str(art), "--case", str(CASES / "ha-01.yml"), "--only", "case_D"])
        assert r.returncode == 0


class TestAggregate:
    def test_all_ten_cases_aggregate_to_json_exact(self):
        sys.path.insert(0, str(SCRIPTS))
        from aggregate import aggregate
        ok = 0
        for i in range(1, 11):
            cid = f"ha-{i:02d}"
            case = yaml.safe_load((CASES / f"{cid}.yml").read_text())
            expected = json.loads(case["success_criteria"]["deterministic_checks"]["json_exact"])
            observed = {e["id"]: e["answer"] for e in case["v6"]["entities"]}
            final = aggregate(case, observed)
            if final == expected:
                ok += 1
            else:
                print(f"  {cid}: {final} != {expected}")
        assert ok == 10

    def test_cli_writes_output(self):
        case = CASES / "ha-01.yml"
        tr = write_tmp(json.dumps({"pass": True, "answers": {"case_A": 0, "case_B": 0, "case_C": 0, "case_D": 3}}), ".json")
        out = tempfile.mktemp(suffix=".json")
        r = run_script(["aggregate.py", "--case", str(case), "--table-result", str(tr), "--out", out])
        assert r.returncode == 0
        assert json.loads(Path(out).read_text()) == {"stages_rerun": 3}


class TestFailFlag:
    def test_pass_table_exit0(self):
        tr = write_tmp(json.dumps({"pass": True}), ".json")
        assert run_script(["failflag.py", "--result", str(tr)]).returncode == 0

    def test_fail_table_exit1(self):
        tr = write_tmp(json.dumps({"pass": False}), ".json")
        assert run_script(["failflag.py", "--result", str(tr)]).returncode == 1

    def test_missing_file_exit1(self):
        assert run_script(["failflag.py", "--result", "/nonexistent.json"]).returncode == 1

    def test_final_check_fail_exit1(self):
        tr = write_tmp(json.dumps({"pass": True}), ".json")
        fc = write_tmp(json.dumps({"pass": False}), ".json")
        r = run_script(["failflag.py", "--result", str(tr), "--final-check", str(fc)])
        assert r.returncode == 1


class TestTableFeedback:
    def test_names_missing_entity_and_question(self):
        res = write_tmp(json.dumps({"pass": False, "missing": ["case_D"], "wrong": []}), ".json")
        r = run_script(["table-feedback.py", "--result", str(res), "--case", str(CASES / "ha-01.yml")])
        assert "case_D" in r.stdout
        assert "never passed" in r.stdout

    def test_wrong_shows_observed_hides_expected(self):
        res = write_tmp(json.dumps({"pass": False, "missing": [], "wrong": [{"id": "case_A", "observed": 99}]}), ".json")
        r = run_script(["table-feedback.py", "--result", str(res), "--case", str(CASES / "ha-01.yml")])
        assert "99" in r.stdout
        assert "withheld" in r.stdout

    def test_all_pass_message(self):
        res = write_tmp(json.dumps({"pass": True, "missing": [], "wrong": []}), ".json")
        r = run_script(["table-feedback.py", "--result", str(res), "--case", str(CASES / "ha-01.yml")])
        assert "ALL TABLE CELLS PASSED" in r.stdout


class TestSpoofWrite:
    def test_writes_canned_text(self):
        src = write_tmp("ENTITY x | 1\n")
        dst = tempfile.mktemp(suffix=".txt")
        r = run_script(["spoof-write.py", "--artifact", dst, "--text-file", str(src)])
        assert r.returncode == 0
        assert Path(dst).read_text() == "ENTITY x | 1\n"


if __name__ == "__main__":
    import pytest
    sys.exit(pytest.main([__file__, "-v"]))
