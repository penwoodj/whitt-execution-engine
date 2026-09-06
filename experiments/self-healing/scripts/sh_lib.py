#!/usr/bin/env python3
"""sh_lib.py — shared constants + io for self-healing spoof pipeline.

Phase S (zero-LLM): all functions deterministic. No model contact.
Thresholds register: docs/02-METRICS.md is the human reference; THIS file is code truth.
"""
import json
import os
import time

# ---- Thresholds register (docs/02-METRICS.md) ----
THETA = 0.65            # reliability threshold (paper 2605.06737)
W_C, W_S, W_E = 0.35, 0.35, 0.30   # R = W_C*C + W_S*S + W_E*E
MAX_ATTEMPTS = 3
CONFIDENCE_FLOOR = 0.50
PSC_UNCERTAIN = 0.30
LENGTH_BOUNDS = (50, 5000)

CLASS_EXIT = {"clean": 0, "F1": 1, "F2": 2, "F3": 3, "F4": 4}
EXIT_CLASS = {v: k for k, v in CLASS_EXIT.items()}

CLASS_STRATEGY = {
    "F1": "corrective_prompt",
    "F2": "tool_reselect",
    "F3": "replan",
    "F4": "replan",
}

REFUSAL_KEYWORDS = ["cannot", "unable to", "i can't", "refuse"]
HALLUCINATION_MARKERS = ["source_verified: true", "confirmed_by_upstream: true"]
CONTRADICTION_PAIRS = [("hot>ceiling", "hot<=ceiling"), ("evicted_zero", "evicted_nonzero")]


def read_json(path):
    with open(path) as f:
        return json.load(f)


def write_json(path, obj):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(obj, f, indent=2, sort_keys=True)
        f.write("\n")


def append_trace(run_dir, event, **fields):
    """Append one JSONL trace event. trace.jsonl is the machine-readable execution trace."""
    path = os.path.join(run_dir, "trace.jsonl")
    os.makedirs(run_dir, exist_ok=True)
    rec = {"event": event, "ts": time.time()}
    rec.update(fields)
    with open(path, "a") as f:
        f.write(json.dumps(rec, sort_keys=True) + "\n")


def read_trace(run_dir):
    path = os.path.join(run_dir, "trace.jsonl")
    if not os.path.exists(path):
        return []
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def attempt_dir(run_dir, n):
    return os.path.join(run_dir, "attempt_%d" % n)


def case_path_arg(path):
    return os.path.abspath(path)
