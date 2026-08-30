"""Exp 12 — shape-library. H1: hash-matched shape reuse produces >=90%
of from-scratch quality at <=30% generation cost (Agent Primitives
Knowledge Pool; Nexus plan library).

Case design: cases grouped into shape families (engine = shape id).
First occurrence of a shape in the run = NOVEL (must generate full
chain); later occurrences = REPEAT (shape-matched, short chain suffices
— the shape carries the procedure). Traps: near-shape cases where the
engine matches but variant differs (H vs L same engine) — a naive
shape key would wrongly reuse the H procedure for the L case; correct
key includes variant. EXTRA_SCENARIO tags novelty per case. Workflow:
unchecked 'match' stage emits the shape id (reuse surface), then H or
short-R (repeat) chain.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import std_case, hsh  # noqa: E402

NEEDS = ["P9", "E6"]

_SHAPE_NOTE = ("tonight resembles shifts you have run before: same "
               "rule family, same kind of walk. when the shape matches, "
               "borrow the procedure, not the numbers — the procedure "
               "is the part that transfers, the numbers never do. when "
               "the shape almost matches but one rule differs, that "
               "difference is the whole difference, treat the night as "
               "new, because a borrowed procedure with one wrong step "
               "walks confidently in the wrong direction")

_ENG = ["quota", "canary", "backoff", "residency", "preempt", "epistemic"]


def build_cases(E, L):
    cases = []
    n = 0
    seen = set()
    for engine in _ENG:
        for variant in ("H", "L"):
            for idx in range(3):
                n += 1
                cid = f"sl-{n:02d}"
                shape = f"{engine}:{variant}"
                novel = shape not in seen
                seen.add(shape)
                cases.append(std_case(
                    E, L, engine, variant, idx, cid, NEEDS,
                    extra_expansion=_SHAPE_NOTE,
                    derivation=f"{engine}/{variant}#{idx} "
                               f"shape={shape} novel={novel}"))
    return cases


SHAPES = {
    "H": [("match", "fmt", 30, "extract"),
          ("plan", "think", 900, "plan"),
          ("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("verify", "think", 2000, "verify"),
          ("extract2", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("match", "fmt", 30, "extract"),
          ("solve", "fmt", 450, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
}
UNCHECKED = {"match", "plan", "judge"}
PRIORS = {"extract": "solve", "verify": "extract",
          "extract2": "verify"}
CHECK_HOOK_STAGES = {"extract", "extract2"}


def SCENARIO(cid, n_h_checked):
    # novel cases (first of each engine:variant) must walk the full
    # chain (deep win); repeats win early (shape carry)
    nn = int(cid.split("-")[1])
    idx_in_shape = (nn - 1) % 3
    if idx_in_shape == 0:
        return min(n_h_checked - 1, 3)
    return idx_in_shape % 2


def EXTRA_SCENARIO(cases):
    out = {}
    seen = set()
    for c in cases:
        rp = c.get("rea_plus") or {}
        shape = f"{rp.get('engine', '?')}:{rp.get('difficulty', '?')}"
        novel = shape not in seen
        seen.add(shape)
        out[c["case_id"]] = {"shape_id": shape, "novel": novel}
    return out
