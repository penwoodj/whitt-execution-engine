"""Exp 11 — verify-blind + det-check (two-lane). H1: independent-lane
verify catches >=50% of planted answer errors the det json_exact lane
misses (det checks shape+exact truth; planted errors that are
json-valid but derived wrong need the reasoning lane).

Case design: FAULT 'delta' planted on first extract for most H cases:
the harvested JSON is well-formed but one field is subtly wrong —
json_exact FAILS it (det lane catches exact mismatches), and the
planted-error class where det passes but reasoning would flag is
simulated via judge_disagree (blind lane 10%). Win depth = verify
stage. Controls: clean cases win early.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, hsh, decode_cid  # noqa: E402

NEEDS = ["P10", "N5"]

_VER_NOTE = ("the reviewer who wrote the answer is the wrong reviewer "
             "of the answer: verification is a blind lane, it sees the "
             "contract and the answer, not the labor that produced it, "
             "and it checks each rule against each field with no stake "
             "in the answer being right. disagreement between lanes is "
             "recorded, and the deterministic lane wins ties, always, "
             "because it is the lane that cannot be persuaded")

_ENG = ["quota", "canary", "backoff", "residency", "preempt", "epistemic"]


def _planted(cid, variant):
    if variant != "H":
        return False
    return hsh(cid, "vb") % 3 != 2   # ~2/3 planted


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"vb-{n:02d}"
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_VER_NOTE,
                    derivation=f"{engine}/{variant}#{idx} "
                               f"planted={_planted(cid, variant)}"))
    return cases


SHAPES = {
    "H": [("solve", "fmt", 600, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("verify", "think", 3000, "verify"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"extract": "solve", "verify": "extract",
          "extract2": "verify"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    nn = int(cid.split("-")[1])
    variant = "H" if (nn - 1) // 3 % 2 == 0 else "L"
    if _planted(cid, variant):
        # extract(0) tainted; verify(1) catches; extract2(2) wins
        return min(2, n_h_checked - 1)
    return 0 if hsh(cid, "w") % 2 else 1


def _build_fault_plan():
    plan = {}
    for nn in range(1, 37):
        cid = f"vb-{nn:02d}"
        variant = "H" if (nn - 1) // 3 % 2 == 0 else "L"
        if _planted(cid, variant):
            plan[f"{cid}:extract"] = "delta"
    return plan


FAULT_PLAN = _build_fault_plan()


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        _, variant, _ = decode_cid(c["case_id"], 6, 3, _ENG)
        planted = _planted(c["case_id"], variant)
        out[c["case_id"]] = {
            "planted": planted,
            "judge_disagree": planted and hsh(c["case_id"], "jd") % 4 == 0}
    return out
