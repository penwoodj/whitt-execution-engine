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


HD_ARCHETYPES = {
    "quota_proration", "tiered_postage", "ballot_quorum", "disk_quota_conv",
    "escrow_ladder", "per_diem_holds", "multi_wallet", "rebate_tiers",
    "ring_rotation_2cyc", "shift_carousel", "canary_percent",
    "redrive_decay", "backoff_scale", "poison_mix",
    "standby_cascade", "overbook_bump", "room_split",
    "triage_2hop", "cold_divert", "customs_tier",
    "sla_pause_clock", "window_overlap_dedupe", "accrual_diff",
    "label_surgery", "checksum_weighted",
}


class TestHdCaseIntegrity:
    def test_exactly_100_cases(self):
        ids = sorted(p.stem for p in CASES.glob("hd-*.yml"))
        assert len(ids) == 100, f"got {len(ids)}"
        assert "hd-01" in ids and "hd-99" in ids and "hd-100" in ids

    def test_all_25_archetypes_4_variants(self):
        from collections import Counter
        arches = Counter()
        variants = Counter()
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            arches[c["archetype"]] += 1
            variants[(c["archetype"], c.get("variant"))] += 1
        assert set(arches) == HD_ARCHETYPES, set(arches) ^ HD_ARCHETYPES
        assert all(v == 4 for v in arches.values()), "each archetype needs exactly 4 cases"
        assert all(v == 1 for v in variants.values()), "each (archetype, variant) exactly once"

    def test_word_bounds_every_case(self):
        for i in range(1, 101):
            p = load(f"hd-{i:02d}")["prompt"]
            wc = len(p.split())
            assert 1000 <= wc <= 2000, f"hd-{i:02d}: {wc} words"

    def test_distractor_every_case(self):
        for i in range(1, 101):
            flat = " ".join(load(f"hd-{i:02d}")["prompt"].split())
            assert "do not manage" in flat, f"hd-{i:02d}"

    def test_recheck_tail_every_case(self):
        for i in range(1, 101):
            flat = " ".join(load(f"hd-{i:02d}")["prompt"].split())
            assert "before committing" in flat and "recheck each number" in flat, f"hd-{i:02d}"

    def test_json_exact_parses_every_case(self):
        for i in range(1, 101):
            je = load(f"hd-{i:02d}")["success_criteria"]["deterministic_checks"]["json_exact"]
            d = json.loads(je)
            assert isinstance(d, dict) and len(d) >= 3, f"hd-{i:02d}: truth too thin"

    def test_entity_count_5_or_more(self):
        for i in range(1, 101):
            ents = load(f"hd-{i:02d}")["v6"]["entities"]
            assert len(ents) >= 5, f"hd-{i:02d}: only {len(ents)} entities (hardness floor)"

    def test_derived_entity_share(self):
        """Extra-hard contract: at least 2 entity cells per case must be
        DERIVED (answer != any literal appearing verbatim in the fact lines)."""
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            facts_text = " ".join(c["task_core"]["facts"])
            derived = 0
            for e in c["v6"]["entities"]:
                ans = str(e["answer"]).lower()
                if not re.search(rf"(?<![\w.]){re.escape(ans)}(?![\w.])", facts_text.lower()):
                    derived += 1
            assert derived >= 2, f"hd-{i:02d}: only {derived} derived cells (need >=2)"

    def test_aggregate_reproduces_every_case(self):
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            answers = {e["id"]: e["answer"] for e in c["v6"]["entities"]}
            want = json.loads(c["success_criteria"]["deterministic_checks"]["json_exact"])
            got = aggregate(c, answers)
            assert got == want, f"hd-{i:02d}: {got} != {want}"

    def test_difficulty_split(self):
        heavies = [i for i in range(1, 101) if load(f"hd-{i:02d}")["difficulty"] == "heavy"]
        assert 45 <= len(heavies) <= 60, f"got {len(heavies)} heavies"

    def test_hops_range(self):
        for i in range(1, 101):
            h = load(f"hd-{i:02d}")["hops"]
            assert 4 <= h <= 7, f"hd-{i:02d}: {h}"

    def test_boundary_convention_varies(self):
        """Variants of an archetype must not all share one boundary word."""
        from collections import defaultdict
        conv = defaultdict(set)
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            flat = " ".join(c["prompt"].split())
            marks = set()
            if "at-or-over" in flat or "at or over" in flat:
                marks.add("inclusive")
            if "strictly past" in flat or "strictly beyond" in flat:
                marks.add("strict")
            conv[c["archetype"]].update(marks)
        varied = sum(1 for a, m in conv.items() if len(m) >= 2)
        assert varied >= 8, f"only {varied} archetypes vary boundary conventions"

    def test_provenance_present(self):
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            assert c.get("provenance", "").strip(), f"hd-{i:02d}"

    def test_style_lowercase_openers(self):
        for i in range(1, 101):
            first = load(f"hd-{i:02d}")["prompt"].lstrip()[:1]
            assert first.islower(), f"hd-{i:02d}: opener not lowercase"

    def test_variation_distinct_openers(self):
        firsts = []
        for i in range(1, 101):
            p = load(f"hd-{i:02d}")["prompt"].lstrip()
            firsts.append(" ".join(p.split()[:3]))
        assert len(set(firsts)) >= 12, f"only {len(set(firsts))} distinct opener triples"

    def test_no_opener_overlap_with_hc(self):
        def first3(prefix, n):
            out = set()
            for i in range(1, n + 1):
                p = load(f"{prefix}-{i:02d}")["prompt"].lstrip()
                out.add(" ".join(p.split()[:5]))
            return out
        overlap = first3("hd", 100) & first3("hc", 100)
        assert not overlap, f"hd reuses hc openers: {sorted(overlap)[:5]}"

    def test_task_core_complete(self):
        for i in range(1, 101):
            tc = load(f"hd-{i:02d}")["task_core"]
            assert tc.get("rules") and tc.get("facts") and tc.get("output_spec"), f"hd-{i:02d}"
            je = load(f"hd-{i:02d}")["success_criteria"]["deterministic_checks"]["json_exact"]
            for k in json.loads(je):
                assert k in tc["output_spec"], f"hd-{i:02d}: {k} missing from output_spec"

    def test_unit_spec_on_derived_cells(self):
        """EVERY derived entity question must state the answer unit or shape
        (unit-drift guard: indices say number/index, bools say true or false,
        strings say exact string, numerics name their unit)."""
        markers = ("mib", "ms", "cents", "minutes", "usd", "pages", "seats",
                   "bytes", "dollars", "percent", "kg", "letters", "users",
                   "messages", "patients", "units", "milliseconds", "promotions", "passengers", "attendees", "count", "how many", "number",
                   "index", "true or false", "destination", "label", "ledger",
                   "exact string", "characters", "summed", "modulo")
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            facts_text = " ".join(c["task_core"]["facts"])
            for e in c["v6"]["entities"]:
                ans = str(e["answer"]).lower()
                if re.search(rf"(?<![\w.]){re.escape(ans)}(?![\w.])", facts_text.lower()):
                    continue  # copy cell, unit stated in its source fact
                q = e["question"].lower()
                assert any(u in q for u in markers), \
                    f"hd-{i:02d}: derived cell {e['id']} lacks unit/shape spec: {q[:80]}"

    def test_contract_matches_truth(self):
        for i in range(1, 101):
            c = load(f"hd-{i:02d}")
            truth = json.loads(c["success_criteria"]["deterministic_checks"]["json_exact"])
            prompt = c["prompt"]
            env = {}
            for key, spec in c["contract"].items():
                formula = spec["formula"]
                for lit in spec["literals"]:
                    if lit["regex"] is None:
                        env[lit["name"]] = lit["value"]
                        continue
                    m = re.search(lit["regex"], prompt)
                    assert m, f"hd-{i:02d}.{key}: literal {lit['name']} not in prompt"
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
                assert got == want, f"hd-{i:02d}.{key}: {got} != {want}"
                env[key] = got

    def test_no_domain_overlap_with_prior_suites(self):
        prior = set()
        for p in list(CASES.glob("ha-*.yml")) + list(CASES.glob("hb-*.yml")) + list(CASES.glob("hc-*.yml")):
            prior.add(yaml.safe_load(p.read_text()).get("domain", ""))
        for i in range(1, 101):
            d = load(f"hd-{i:02d}")["domain"]
            assert d not in prior, f"hd-{i:02d} domain overlaps prior suite: {d}"


if __name__ == "__main__":
    import pytest
    sys.exit(pytest.main([__file__, "-v"]))
