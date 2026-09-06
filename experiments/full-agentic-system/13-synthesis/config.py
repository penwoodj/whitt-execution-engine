"""Exp 13 — synthesis (orchestrator-workers merge). Two independent
sub-answers exist; a synthesis stage must merge into one contract without
conflict-blindness. One worked example (engine A) orients; engine B's
rules are stated in core only (worker isolation, S53).

Combo engines: (quota,backoff), (canary,residency), (preempt,epistemic)
x 2 variants x 3 idx = 36 cases. FAULT_PLAN injects worker_b delta on
1/3 (conflict band) — synthesis must not blind-merge a wrong part.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from fas_cases import hsh, scen_buckets, scen_mod  # noqa: E402
from fas_lib import (CONTRACT_MARK, WORKED_MARK, assemble_prompt,  # noqa: E402
                     digest, word_count)

_PAIRS = [("quota", "backoff"), ("canary", "residency"),
          ("preempt", "epistemic")]
_PAD = (
    "margin note, because short pages invite skimming: the scaffold above "
    "is not decoration, each sentence names a discipline that exists "
    "because a night went wrong without it, the traps enumerated are the "
    "exact traps, the checklist is the exact order, and the contract at "
    "the close is the exact shape the parser enforces, nothing in this "
    "note changes any number or rule, it only insists the whole page be "
    "read as written before the walk begins")


def _combo_case(E, L, eng_a, eng_b, variant, idx, cid):
    ga = E.gen(eng_a, variant, idx)
    gb = E.gen(eng_b, variant, idx)
    truth = {"part_a": ga["truth"], "part_b": gb["truth"]}
    core = (ga["core"].rstrip() + "\n\n"
            "second matter, handled by a different crew on the same night, "
            "with its own rules stated from scratch:\n\n"
            + gb["core"].rstrip())
    contract = ga["contract"].rstrip().rstrip(".") + \
        ", and the second half exactly " + gb["contract"].rstrip().rstrip(".")
    header = (f"night desk {cid}: two crews worked, two ledgers to close, "
              f"one report to file")
    contract_line = (CONTRACT_MARK + ", one object, key part_a carrying "
                     f"the {eng_a} fields and key part_b the {eng_b} "
                     f"fields: {contract}")
    prompt = assemble_prompt(
        {"core": core, "worked": ga["worked"]},
        header, contract_line,
        aux=None,
        extra_expansion=(
            "two crews, one report: the parts are independent, neither "
            "borrows numbers from the other, a disagreement inside one "
            "part is settled inside that part alone, and the filed object "
            "must carry both parts complete, never one standing in for "
            "the other"))
    while word_count(prompt) < 1010:
        prompt += "\n\n" + _PAD
    digs = sorted(set(ga["task_digits"]) | set(gb["task_digits"]))
    return {
        "case_id": cid, "prompt": prompt, "truth": truth,
        "checks": {"json_exact": json.dumps(truth)},
        "task_digits": digs,
        "derivation": f"{eng_a}+{eng_b}/{variant}#{idx}",
        "rea_plus": {"engine": f"{eng_a}+{eng_b}", "variant": variant,
                     "idx": idx, "pair": [eng_a, eng_b]},
        "needs": ["P5", "P9"], "tkeys": "part_a,part_b",
    }


def build_cases(E, L):
    cases = []
    n = 0
    for eng_a, eng_b in _PAIRS:
        for variant in ("H", "L"):
            for idx in range(5):
                n += 1
                cases.append(_combo_case(
                    E, L, eng_a, eng_b, variant, idx, f"sy-{n:02d}"))
    return cases


SHAPES = {
    "H": [("split", "fmt", 250, "plan"),
          ("worker_a", "fmt", 600, "cot"),
          ("extract_a", "fmt", 200, "extract"),
          ("worker_b", "fmt", 600, "cot"),
          ("extract_b", "fmt", 200, "extract"),
          ("resolve", "fmt", 400, "replan"),
          ("solve_b2", "fmt", 600, "cot"),
          ("extract_b2", "fmt", 200, "extract"),
          ("merge", "fmt", 700, "cot"),
          ("extract_s", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
    "L": [("solve", "fmt", 700, "cot"),
          ("extract", "fmt", 200, "extract"),
          ("judge", "think", 300, "judge")],
}
UNCHECKED = {"split", "judge"}
PRIORS = {"extract_a": "worker_a", "extract_b": "worker_b",
          "resolve": "extract_b", "solve_b2": "resolve",
          "extract_b2": "solve_b2,worker_b,resolve",
          "merge": "extract_a,extract_b2",
          "extract_s": "merge"}
CHECK_HOOK_STAGES = {"extract_a", "extract_b", "extract_b2", "extract_s"}
FINAL_CHECK_STAGE = "extract_s"


def SCENARIO(cid, n_h_checked):
    # conflict band 1/3: worker_b wrong until resolve->solve_b2 re-derives
    if hsh(cid, "syconf") % 3 == 0:
        return min(5, n_h_checked - 1)
    return scen_buckets(cid, n_h_checked, [2, 2], salt="sy")


_CONFLICT = {}


def _build_fault_plan():
    n = 0
    for eng_a, eng_b in _PAIRS:
        for variant in ("H", "L"):
            for idx in range(5):
                n += 1
                cid = f"sy-{n:02d}"
                if hsh(cid, "syconf") % 3 == 0:
                    _CONFLICT[f"{cid}:worker_b"] = "delta"


_build_fault_plan()
FAULT_PLAN = _CONFLICT


def EXTRA_SCENARIO(cases):
    out = {}
    for c in cases:
        cid = c["case_id"]
        out[cid] = {"conflict": f"{cid}:worker_b" in FAULT_PLAN,
                    "pair": "+".join(c["rea_plus"]["pair"])}
    return out
