#!/usr/bin/env python3
"""agent_run_v2.py — spoof worker LLM attempt with per-class failure injection.

Reads the case's injection profile: attempts listed in attempts_failed
receive the class failure signature; all other attempts produce clean,
schema-conformant output. Writes attempt_<n>.json (the spoofed model
output) plus llm_call budget-ledger events.

Exit 0 always (generation itself succeeded); quality is triage's call.
"""
from __future__ import annotations

import argparse
import json
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import (  # noqa: E402
    MODELS,
    failed_attempts_of,
    load_case,
    read_json,
    run_dir_of,
    trace_append,
    write_json,
)


def clean_output(case: dict, rng: random.Random) -> dict:
    keys = list(case["task"]["expected_schema"]["required"])
    out: dict = {k: f"<{k} content: complete, contract-conformant>" for k in keys}
    out["_meta"] = {
        "confidence": round(rng.uniform(0.72, 0.88), 2),
        "self_consistency": round(rng.uniform(0.80, 0.95), 2),
    }
    return out


def inject(cls: str, base: dict, attempt: int, rng: random.Random) -> dict:
    m = base.setdefault("_meta", {})
    if cls == "F1":
        m["confidence"] = round(rng.uniform(0.42, 0.50), 2)
        m["self_consistency"] = round(rng.uniform(0.50, 0.58), 2)
        base[f"invented_method_{attempt}"] = "storage_facade.read_mirror()"
        base["invented_config_key"] = "iridium.recon.threads"
    elif cls == "F2":
        wrapped = {"document": {k: v for k, v in base.items() if k != "_meta"}}
        keys = list(wrapped["document"].keys())
        if "gap_handling" in wrapped["document"]:
            del wrapped["document"]["gap_handling"]
        wrapped["document"]["validation_invariants"] = [wrapped["document"].get("validation_invariants", "v")]
        wrapped["_meta"] = m
        wrapped["_meta"]["tool_errors"] = 2
        return wrapped
    elif cls == "F3":
        first = next(iter(base))
        if isinstance(base[first], str):
            base[first] = base[first] + " [NOTE: verdict above marked compliant per summary.]"
        base["summary_table_copy"] = "status=breach (contradicts verdict above)"
        m["contradiction_pairs"] = [
            ["verdict=compliant", "table=breach"],
            ["headroom stated 21.4%", "table 12.4%"],
        ]
    elif cls in ("F4", "F4-persist"):
        keys = [k for k in base if k != "_meta"]
        for k in keys[::2]:
            del base[k]
        m["truncated"] = True
        m["upstream_errors"] = [f"artifact truncated at section {attempt} of {len(keys)}"]
    return base


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir-base", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    ap.add_argument("--model-role", default="worker_general",
                    choices=list(MODELS))
    args = ap.parse_args()

    case = load_case(args.case)
    run_dir = run_dir_of(args.case, args.run_dir_base)
    cls = case["failure"]["class"].replace("-persist", "")
    failed = failed_attempts_of(case)

    rng = random.Random(hash((case["case_id"], args.attempt)) & 0xFFFF)
    out = clean_output(case, rng)
    if args.attempt in failed:
        out = inject(cls, out, args.attempt, rng)

    write_json(run_dir, f"attempt_{args.attempt}.json", out)
    trace_append(run_dir, "attempt", f"attempt_{args.attempt}",
                 attempt=args.attempt, model=MODELS[args.model_role],
                 failed=bool(args.attempt in failed))
    trace_append(run_dir, "llm_call", f"attempt_{args.attempt}",
                 model=MODELS[args.model_role], site="worker")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
