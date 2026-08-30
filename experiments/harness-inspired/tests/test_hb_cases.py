import json
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
SCRIPTS = ROOT / "scripts"
CASES = ROOT / "cases"
sys.path.insert(0, str(SCRIPTS))

from aggregate import aggregate


def load(cid):
    return yaml.safe_load((CASES / f"{cid}.yml").read_text())


class TestHbCases:
    def test_twenty_cases_exist(self):
        for i in range(1, 21):
            assert (CASES / f"hb-{i:02d}.yml").is_file(), f"missing hb-{i:02d}"

    def test_word_count_1500_plus(self):
        for i in range(1, 21):
            d = load(f"hb-{i:02d}")
            wc = len(d["prompt"].split())
            assert wc >= 1500, f"hb-{i:02d} only {wc} words"

    def test_distractor_present(self):
        for i in range(1, 21):
            flat = " ".join(load(f"hb-{i:02d}")["prompt"].split())
            assert "do not manage" in flat, f"hb-{i:02d} missing distractor"

    def test_json_exact_parses_to_dict(self):
        for i in range(1, 21):
            dc = load(f"hb-{i:02d}")["success_criteria"]["deterministic_checks"]
            expected = json.loads(dc["json_exact"])
            assert isinstance(expected, dict)
            assert "forbidden_phrases" in dc

    def test_aggregate_reproduces_json_exact(self):
        for i in range(1, 21):
            d = load(f"hb-{i:02d}")
            expected = json.loads(d["success_criteria"]["deterministic_checks"]["json_exact"])
            ents = {e["id"]: e["answer"] for e in d["v6"]["entities"]}
            assert aggregate(d, ents) == expected, f"hb-{i:02d} aggregate mismatch"

    def test_difficulty_split_ten_ten(self):
        heavy = [i for i in range(1, 21) if load(f"hb-{i:02d}")["difficulty"] == "heavy"]
        assert len(heavy) == 10
        hops = [load(f"hb-{i:02d}")["hops"] for i in heavy]
        assert all(h >= 5 for h in hops)

    def test_hops_range(self):
        for i in range(1, 21):
            assert load(f"hb-{i:02d}")["hops"] in (3, 4, 5, 6)

    def test_provenance_present(self):
        for i in range(1, 21):
            assert load(f"hb-{i:02d}").get("provenance"), f"hb-{i:02d} no provenance"

    def test_entities_have_rule_restating_questions(self):
        for i in range(1, 21):
            d = load(f"hb-{i:02d}")
            for e in d["v6"]["entities"]:
                assert len(e["question"]) > 60, f"hb-{i:02d} entity {e['id']} question too thin"

    def test_domains_differ_from_ha(self):
        ha_cats = {load(f"ha-{i:02d}")["category"] for i in range(1, 11)}
        for i in range(1, 21):
            assert load(f"hb-{i:02d}")["category"] not in ha_cats
