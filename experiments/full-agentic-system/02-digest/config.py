"""Exp 02 — digest. H1: digest (core+contract, worked/expansion cut)
preserves solvability at <=30% of full-prompt tokens across task types.
Fact-position probe (Lost-in-the-Middle): key task digits placed
early / mid / late in the core by idx, so position effect on downstream
digest-based stages is measurable (S37 slice).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_buckets, decode_cid  # noqa: E402

NEEDS = ["E10", "P8"]

_POS_NOTE = {
    0: "the numbers that matter sit early in the event list, hold them "
       "loose anyway, late events can rewrite early conclusions",
    1: "the pivot number sits mid-list, the middle of a list is where "
       "eyes slide past, give it its own pass",
    2: "the deciding number arrives last, nothing before it is final "
       "until the tail is read",
}

_ENG = ["quota", "backoff", "canary", "residency", "preempt", "epistemic"]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"dg-{n:02d}"
                pos = idx % 3
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_POS_NOTE[pos],
                    derivation=f"{engine}/{variant}#{idx} pos={'EML'[pos]}"))
    return cases


SHAPES = {
    "H": [("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("wait", "fmt", 400, "wait"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 500, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"extract": "solve", "wait": "solve", "extract2": "wait"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    # win spread: first solve / after wait / never-in-H (L fallback)
    return scen_buckets(cid, n_h_checked, [5, 2, 1])


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        _, _, idx = decode_cid(c["case_id"], 6, 3, _ENG)
        out[c["case_id"]] = {"fact_position": "EML"[idx % 3]}
    return out
