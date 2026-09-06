#!/usr/bin/env python3
"""Per-experiment config helpers: standard case wrapper + scenario fns."""
import hashlib
import json
import re

NUM_RE = re.compile(r"(?<![A-Za-z0-9_])(\d+(?:\.\d+)?)")

_PAD = (
    "margin note, because short pages invite skimming: the scaffold above "
    "is not decoration, each sentence names a discipline that exists "
    "because a night went wrong without it, the traps enumerated are the "
    "exact traps, the checklist is the exact order, and the contract at "
    "the close is the exact shape the parser enforces, nothing in this "
    "note changes any number or rule, it only insists the whole page be "
    "read as written before the walk begins")


def hsh(cid, salt=""):
    return int(hashlib.sha256(f"{cid}{salt}".encode()).hexdigest()[:8], 16)


def std_case(E, L, engine, variant, idx, cid, needs,
             extra_expansion="", aux=None, header=None,
             task_digits=None, truth_override=None, derivation=None,
             core_override=None):
    """Wrap an engine case into the gen-cases.py contract."""
    g = E.gen(engine, variant, idx)
    if core_override is not None:
        g = dict(g, core=core_override)
    hdr = header or (f"you are on rotation tonight, {engine} duty, "
                     "and the morning report only reads JSON.")
    # CONTRACT_MARK must open the contract sentence (digest re-attaches
    # from its rfind), engine contracts carry the bare JSON shape.
    contract_line = (f"{L.CONTRACT_MARK}, values filled in correctly: "
                     f"{g['contract']}")
    prompt = L.assemble_prompt(g, hdr, contract_line, aux=aux,
                               extra_expansion=extra_expansion)
    while L.word_count(prompt) < 1010:
        prompt = L.assemble_prompt(
            g, hdr, contract_line, aux=aux,
            extra_expansion=(extra_expansion + " " + _PAD).strip())
    truth = g["truth"] if truth_override is None else truth_override
    td = g["task_digits"] if task_digits is None else task_digits
    return {
        "case_id": cid,
        "prompt": prompt,
        "truth": truth,
        "checks": {"json_exact": json.dumps(truth)},
        "task_digits": td,
        "derivation": derivation or f"{engine}/{variant}#{idx} sim",
        "rea_plus": {"hops": 4 + idx % 3, "robust": variant == "H",
                     "prio": True, "format_weight": 2,
                     "archetype": "fas", "difficulty": variant,
                     "engine": engine},
        "needs": list(needs),
        "tkeys": g["tkeys"],
    }


def decode_cid(cid, n_engines, idx_count, engines):
    """Reverse the build loop: engine -> variant(H/L) -> idx.

    Cases numbered 1..N in order: for engine in engines: for variant in
    (H, L): for idx in range(idx_count). Returns (engine, variant, idx).
    """
    nn = int(cid.split("-")[1])
    per_engine = 2 * idx_count
    ei = (nn - 1) // per_engine
    rem = (nn - 1) % per_engine
    variant = "H" if rem < idx_count else "L"
    idx = rem % idx_count
    return engines[ei], variant, idx


def scen_mod(cid, n, salt=""):
    """Deterministic win depth: h % n (clamped to n-1)."""
    return min(hsh(cid, salt) % max(1, n), max(0, n - 1))


def scen_buckets(cid, n, buckets, salt=""):
    """Map hash into weighted depth buckets, clamped to n-1.

    buckets: list of relative weights, index = win depth.
    """
    if n <= 0:
        return 0
    total = sum(buckets)
    pick = hsh(cid, salt) % total
    acc = 0
    for depth, w in enumerate(buckets):
        acc += w
        if pick < acc:
            return min(depth, n - 1)
    return n - 1
