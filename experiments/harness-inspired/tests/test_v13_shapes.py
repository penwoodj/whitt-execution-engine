import json
import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
SCRIPTS = ROOT / "scripts"
CASES = ROOT / "cases"
sys.path.insert(0, str(SCRIPTS))

from table_lib import parse_table


class TestV13Parser:
    def test_inline_entity_rows_split(self):
        t = 'ENTITY e1 | 0 ENTITY e2 | 2 ENTITY p1 | 1'
        d = parse_table(t)
        assert d == {"e1": 0, "e2": 2, "p1": 1}

    def test_colon_rows_parsed_when_no_pipes(self):
        t = "sr: 8\nws: 4\nvc: 0\ntc: 30\n"
        d = parse_table(t)
        assert d == {"sr": 8, "ws": 4, "vc": 0, "tc": 30}

    def test_colon_entity_prefixed(self):
        t = "ENTITY sr: 8\nENTITY ws: 4\n"
        d = parse_table(t)
        assert d == {"sr": 8, "ws": 4}

    def test_pipe_rows_win_over_colon(self):
        t = "a | 1\nb | 2\nnote: ignore me\n"
        d = parse_table(t)
        assert d == {"a": 1, "b": 2}

    def test_prose_with_colons_ignored_when_pipe_rows_present(self):
        t = "Step 1: compute.\nshipped | 2\nrolled | 1\n"
        d = parse_table(t)
        assert d == {"shipped": 2, "rolled": 1}

    def test_colon_string_value(self):
        t = 'hr: "ring-1"\nab: true\n'
        d = parse_table(t)
        assert d == {"hr": "ring-1", "ab": True}


class TestV13CaseShapes:
    def test_specimen_numeric_codes(self):
        for v in range(4):
            cid = 20 + v * 25
            case = yaml.safe_load((CASES / f"hc-{cid:02d}.yml").read_text())
            ents = {e["id"]: e for e in case["v6"]["entities"]}
            assert "xl" in ents
            for i in range(1, 5):
                assert ents[f"p{i}"]["answer"] in (0, 1), f"hc-{cid} p{i} not numeric"
                assert ents[f"b{i}"]["answer"] in (1, 2, 3), f"hc-{cid} b{i} not coded"

    def test_payroll_numeric_holds(self):
        for v in range(4):
            cid = 6 + v * 25
            case = yaml.safe_load((CASES / f"hc-{cid:02d}.yml").read_text())
            ents = {e["id"]: e for e in case["v6"]["entities"]}
            for i in range(1, 5):
                assert ents[f"h{i}"]["answer"] in (0, 1), f"hc-{cid} h{i} not numeric"
                assert "OVER the hold threshold" in ents[f"h{i}"]["question"]

    def test_cache_sweep_extraction_ids(self):
        for v in range(4):
            cid = 5 + v * 25
            case = yaml.safe_load((CASES / f"hc-{cid:02d}.yml").read_text())
            ids = {e["id"] for e in case["v6"]["entities"]}
            assert {"h1", "h2", "h3", "c1", "c2", "c3", "d"} <= ids, f"hc-{cid} ids {ids}"

    def test_v13_workflow_exists_all_100(self):
        W = ROOT / "workflows"
        for i in range(1, 101):
            assert (W / f"v13-hc-{i:02d}.yml").is_file(), f"missing v13-hc-{i:02d}"

    def test_v13_solve_prompt_numeric_directives(self):
        yml = yaml.safe_load((ROOT / "workflows" / "v13-hc-20.yml").read_text())
        p = yml["agentic_workflow"]["steps"]["s01_solve"]["prompt"]
        assert "answer 1 for complete or 0 for incomplete" in p
        assert "1 for cold, 2 for room, or 3 for frozen" in p


if __name__ == "__main__":
    import pytest
    sys.exit(pytest.main([__file__, "-v"]))
