import json
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
CASES = ROOT / "cases"
SCRIPTS = ROOT / "scripts"

sys.path.insert(0, str(SCRIPTS))
from aggregate import aggregate  # noqa: E402


def load(cid):
    return yaml.safe_load((CASES / f"{cid}.yml").read_text())


class TestHcCaseIntegrity:
    def test_exactly_100_cases(self):
        ids = sorted(p.stem for p in CASES.glob("hc-*.yml"))
        assert len(ids) == 100, f"got {len(ids)}"
        assert "hc-01" in ids and "hc-100" in ids and "hc-99" in ids

    def test_word_bounds_every_case(self):
        for i in range(1, 101):
            p = load(f"hc-{i:02d}")["prompt"]
            wc = len(p.split())
            assert 1000 <= wc <= 2000, f"hc-{i:02d}: {wc} words"

    def test_distractor_every_case(self):
        for i in range(1, 101):
            flat = " ".join(load(f"hc-{i:02d}")["prompt"].split())
            assert "do not manage" in flat, f"hc-{i:02d}"

    def test_recheck_tail_every_case(self):
        for i in range(1, 101):
            flat = " ".join(load(f"hc-{i:02d}")["prompt"].split())
            assert "before committing" in flat and "recheck each number" in flat, f"hc-{i:02d}"

    def test_json_exact_parses_every_case(self):
        for i in range(1, 101):
            je = load(f"hc-{i:02d}")["success_criteria"]["deterministic_checks"]["json_exact"]
            d = json.loads(je)
            assert isinstance(d, dict) and d

    def test_aggregate_reproduces_every_case(self):
        for i in range(1, 101):
            c = load(f"hc-{i:02d}")
            answers = {e["id"]: e["answer"] for e in c["v6"]["entities"]}
            want = json.loads(c["success_criteria"]["deterministic_checks"]["json_exact"])
            got = aggregate(c, answers)
            assert got == want, f"hc-{i:02d}: {got} != {want}"

    def test_difficulty_split_60_40(self):
        heavies = [i for i in range(1, 101) if load(f"hc-{i:02d}")["difficulty"] == "heavy"]
        assert len(heavies) == 40, f"got {len(heavies)}"

    def test_hops_range(self):
        for i in range(1, 101):
            h = load(f"hc-{i:02d}")["hops"]
            assert 3 <= h <= 6, f"hc-{i:02d}: {h}"

    def test_provenance_present(self):
        for i in range(1, 101):
            c = load(f"hc-{i:02d}")
            assert c.get("provenance", "").strip(), f"hc-{i:02d}"

    def test_style_lowercase_openers(self):
        for i in range(1, 101):
            first = load(f"hc-{i:02d}")["prompt"].lstrip()[:1]
            assert first.islower(), f"hc-{i:02d}: opener not lowercase"

    def test_variation_distinct_openers(self):
        firsts = []
        for i in range(1, 101):
            p = load(f"hc-{i:02d}")["prompt"].lstrip()
            firsts.append(" ".join(p.split()[:3]))
        assert len(set(firsts)) >= 12, f"only {len(set(firsts))} distinct opener triples"

    def test_task_core_complete(self):
        for i in range(1, 101):
            tc = load(f"hc-{i:02d}")["task_core"]
            assert tc.get("rules") and tc.get("facts") and tc.get("output_spec"), f"hc-{i:02d}"
            je = load(f"hc-{i:02d}")["success_criteria"]["deterministic_checks"]["json_exact"]
            for k in json.loads(je):
                assert k in tc["output_spec"], f"hc-{i:02d}: {k} missing from output_spec"

    def test_contract_matches_truth(self):
        for i in range(1, 101):
            c = load(f"hc-{i:02d}")
            truth = json.loads(c["success_criteria"]["deterministic_checks"]["json_exact"])
            prompt = c["prompt"]
            env = {}
            for key, spec in c["contract"].items():
                formula = spec["formula"]
                for lit in spec["literals"]:
                    m = re.search(lit["regex"], prompt)
                    assert m, f"hc-{i:02d}.{key}: literal {lit['name']} not in prompt"
                    if m.groups():
                        v = m.group(1)
                        if v is None:
                            v = lit["value"]
                        elif lit.get("map"):
                            v = lit["map"].get(v, v)
                        else:
                            try:
                                v = int(v)
                            except ValueError:
                                try:
                                    v = float(v)
                                except ValueError:
                                    v = str(v)
                    else:
                        v = lit["value"]
                    env[lit["name"]] = v
                got = eval(formula, {"__builtins__": {}, "min": min, "max": max, "abs": abs, "int": int, "float": float, "bool": bool, "str": str, "sum": sum, "len": len, "round": round}, env)  # noqa: S307
                want = truth[key]
                assert got == want, f"hc-{i:02d}.{key}: {got} != {want}"
                env[key] = got

    def test_no_domain_overlap_with_ha_hb(self):
        ha_hb_cats = set()
        for p in list(CASES.glob("ha-*.yml")) + list(CASES.glob("hb-*.yml")):
            ha_hb_cats.add(yaml.safe_load(p.read_text())["category"])
        hc_cats = {load(f"hc-{i:02d}")["category"] for i in range(1, 101)}
        assert not (hc_cats & ha_hb_cats), hc_cats & ha_hb_cats


if __name__ == "__main__":
    import pytest
    sys.exit(pytest.main([__file__, "-v"]))
