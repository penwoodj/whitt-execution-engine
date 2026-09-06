"""Exp 06 — plan. H1: plan-once scaffold raises multi-dependency task
success >=15pp over direct solve (PlanCompiler 92.67 vs 62).

Case design: engine cases with a dependency-chain expansion enumerating
which event feeds which rule (the DAG spoken aloud), plus a trap: one
named dependency is a red herring (a rule that does NOT depend on the
listed event). Correct planning ignores the red herring; truth
unchanged (computed by simulation, red herring is prose only).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, scen_buckets  # noqa: E402

NEEDS = ["P4", "P5"]

_CHAIN = ("dependency chain for tonight, spoken once so the order is "
          "explicit: the totals depend on the final state, the final "
          "state depends on every event applied in arrival order, the "
          "expiry and rollback rules depend on identifying WHICH event "
          "triggers them before applying, and the verdict fields depend "
          "on the totals alone, nothing else. one dependency people "
          "imagine exists: the worked example's totals do NOT feed "
          "tonight's, the example is orientation, its chain is parallel, "
          "never upstream. plan the walk before walking it, then walk "
          "the plan, the plan is not the work but the walk without the "
          "plan re-decides the order mid-stream and mid-stream is where "
          "order errors breed")

_ENG = ["quota", "canary", "backoff", "residency", "preempt", "epistemic"]


def build_cases(E, L):
    cases = []
    n = 0
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"pl-{n:02d}"
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_CHAIN,
                    derivation=f"{engine}/{variant}#{idx}"))
    return cases


SHAPES = {
    "H": [("plan", "think", 1000, "plan"),
          ("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("verify", "think", 2500, "verify"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 500, "cot"),
          ("extract", "fmt", 200, "extract")],
}
UNCHECKED = {"plan", "judge"}
PRIORS = {"extract": "solve", "verify": "extract", "extract2": "verify"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    # plan lane wins spread across depth incl deep verify
    return scen_buckets(cid, n_h_checked, [4, 3, 2, 1], salt="pl")


def EXTRA_SCENARIO(cases):
    return {c["case_id"]: {"scaffold": "chain"} for c in cases}
