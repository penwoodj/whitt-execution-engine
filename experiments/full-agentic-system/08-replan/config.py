"""Exp 08 — replan + bounded-retry-escalate. H1: scoped replan after a
check-failed solve recovers >=50% of near-miss cases at <40% token cost
of full re-solve (TDP scoped replan -82% tokens).

Case design: FAULT_PLAN injects 'delta' (subtle near-miss) on the FIRST
solve for most H cases — first attempt is one-field-wrong by
construction. Workflow: solve -> check -> replan (scoped, sees leak-safe
hint) -> solve2 -> extract -> judge. Win depth lands on solve2 for
injected cases (recovery measured), on solve for clean controls.
FAULT_PLAN keys must be built after case ids exist -> build_cases also
exports PLAN via module-level CASE_IDS filled at build time.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, hsh  # noqa: E402

NEEDS = ["N6", "E3"]

_REPLAN_NOTE = ("when the checks come back red, the fix is scoped, not "
                "from the top: re-read only the rule the failure names, "
                "re-walk only the events that rule touches, keep every "
                "green field green, a full re-derive wastes the night "
                "and invites fresh errors into settled ground")

_ENG = ["quota", "canary", "backoff", "residency", "preempt", "epistemic"]

CASE_IDS = []
NEAR_MISS = []   # cids whose first solve is delta-faulted


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"rp-{n:02d}"
                CASE_IDS.append(cid)
                injected = hsh(cid, "rp") % 4 != 3   # 75% near-miss
                if variant == "H" and injected:
                    NEAR_MISS.append(cid)
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_REPLAN_NOTE,
                    derivation=f"{engine}/{variant}#{idx} "
                               f"near_miss={injected}"))
    return cases


SHAPES = {
    "H": [("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("replan", "fmt", 300, "replan"),
          ("solve2", "fmt", 700, "cot"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 500, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"replan", "judge"}
PRIORS = {"extract": "solve", "solve2": "replan,solve",
          "extract2": "solve2"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    if f"{cid}:solve" in FAULT_PLAN:
        # first checked stage fails (fault), win lands on extract2 (idx 2)
        return min(2, n_h_checked - 1)
    return 0 if hsh(cid, "win") % 3 else 1


def _build_fault_plan():
    """Cids are deterministic (engine order x variant x idx), so the
    near-miss set is computable WITHOUT build_cases() having run."""
    plan = {}
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"rp-{n:02d}"
                injected = hsh(cid, "rp") % 4 != 3
                if variant == "H" and injected:
                    plan[f"{cid}:solve"] = "delta"
    return plan


FAULT_PLAN = _build_fault_plan()


def EXTRA_SCENARIO(cases):
    return {c["case_id"]: {"near_miss": c["case_id"] in NEAR_MISS}
            for c in cases}
