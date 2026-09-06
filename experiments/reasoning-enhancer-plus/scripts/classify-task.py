#!/usr/bin/env python3
"""Deterministic keyword classifier for REA+ v2 routing.

Reads a case YAML, scores 6 specialization channels from keyword
weights (derived from 35-model probe failure/success data), prints
JSON: {"category": "...", "model": "...", "scores": {...}}.
Exit 0 always; unknown -> generalist channel.
"""
import json
import re
import sys
from pathlib import Path

import yaml

CHANNELS = {
    "fmt": {
        "model": "Qwen3-4B-Instruct-2507-Q4_K_M",
        "keywords": {
            "json": 3, "yaml": 3, "only this": 3, "output only": 3,
            "exactly n keys": 2, "no extra keys": 3, "envelope": 2,
            "bullet": 2, "line_count": 2, "single line": 2,
            "nothing else": 2, "all caps": 2, "verbatim": 3,
            "emit only": 3, "bare json": 3, "total_pages": 2,
            "next_page": 2, "per_page": 2,
        },
    },
    "pln": {
        "model": "gemma-3n-E4B-it-Q4_K_M",
        "keywords": {
            "schedule": 3, "priority": 3, "plan": 2, "order": 2,
            "allocate": 3, "queue": 2, "drain": 3, "worker": 2,
            "makespan": 3, "wait": 1, "capacity": 2, "throughput": 2,
            "priority lanes": 3, "evict": 2, "lru": 3,
        },
    },
    "logic": {
        "model": "Bonsai-27B-Q1_0",
        "keywords": {
            "if": 1, "iff": 3, "only if": 2, "clause": 3,
            "first match wins": 3, "policy": 3, "gate": 2,
            "grant": 2, "deny": 2, "rollback": 2, "verdict": 2,
            "escalat": 2, "and/or": 2, "except": 2, "unless": 2,
            "rules": 2, "condition": 2,
        },
    },
    "derive": {
        "model": "Bonsai-27B-Q1_0",
        "keywords": {
            "how many": 2, "compute": 2, "calculate": 2,
            "percent": 2, "average": 2, "sum": 1, "remainder": 3,
            "ceil": 3, "split evenly": 2, "total profit": 2,
            "per day": 2, "rate": 1, "discount": 2, "doubles": 2,
            "cubic meters": 1, "in dollars": 2,
        },
    },
    "audit": {
        "model": "nvidia_Orchestrator-8B-Q5_K_L",
        "keywords": {
            "audit": 3, "review": 2, "verify": 2, "spot the": 3,
            "wrong": 2, "error": 2, "bug": 2, "inconsistenc": 3,
            "find the": 2, "check whether": 2, "missing": 2,
            "delegate": 2, "orchestrat": 3, "which agent": 2,
        },
    },
    "general": {
        "model": "Qwen3-4B-Hivemind-Inst-Hrtic-Ablit-Uncensored-Q4_K_M-imat",
        "keywords": {},
    },
}

def classify(text):
    t = text.lower()
    scores = {}
    for ch, spec in CHANNELS.items():
        s = sum(w for kw, w in spec["keywords"].items() if kw in t)
        scores[ch] = s
    best = max(scores, key=lambda k: scores[k])
    if scores[best] == 0:
        best = "general"
    return best, scores


def main():
    src = sys.stdin.read() if len(sys.argv) < 2 else Path(sys.argv[1]).read_text()
    try:
        case = yaml.safe_load(src) or {}
        text = " ".join(filter(None, [
            case.get("prompt", ""), case.get("auxiliary", "")]))
    except yaml.YAMLError:
        text = src
    best, scores = classify(text)
    print(json.dumps({
        "category": best,
        "model": CHANNELS[best]["model"],
        "scores": scores,
    }))


if __name__ == "__main__":
    main()
