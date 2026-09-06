"""Exp 07 — solve (baseline control). Establishes the control-group
numbers every other atom compares against (S46 waterfall base tier).

Case design: pure engine cases, no extra scaffolding beyond the shared
expansion. Token sweep: H solve gets 800 max_tokens, L gets 400 — the
budget axis of the control. Win-depth here = intrinsic difficulty map.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_buckets  # noqa: E402

NEEDS = ["P1", "P7"]

_ENG = ["quota", "backoff", "canary", "residency", "preempt", "epistemic"]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(4):
                n += 1
                cid = f"sv-{n:02d}"
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    derivation=f"{engine}/{variant}#{idx}"))
    return cases


SHAPES = {
    "H": [("solve", "fmt", 800, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 400, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"judge"}
PRIORS = {"extract": "solve"}
CHECK_HOOK_STAGES = {"extract"}


def SCENARIO(cid, n_h_checked):
    return scen_buckets(cid, n_h_checked, [6, 2], salt="sv")


def EXTRA_SCENARIO(cases):
    return {c["case_id"]: {"control": True} for c in cases}
