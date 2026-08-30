#!/usr/bin/env python3
"""agent_run.py — SPOOF the worker model's task attempt (Phase S, zero-LLM).

Emits the output the worker model (Qwen3-4B-Instruct-2507) WOULD have produced,
with deterministic failure injection per the case's fail_attempts list.

Usage: agent_run.py --case <case.yml> --run-dir <dir> --attempt <N>
Exit: 0 always (injection is simulation, not error)
"""
import argparse
import sys
import os

import yaml

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sh_lib


def build_output(case, attempt):
    """Deterministic per-class output contract (docs/04-CASE-SUITE-SPEC.md)."""
    cls = case["failure"]["class"]
    injected = attempt in case["failure"]["fail_attempts"]

    if not injected:
        return {
            "attempt": attempt,
            "text": '{"status": "ok", "evicted": 130, "retained": 58}',
            "confidence": 0.90,
            "psc": 0.90,
            "tool_calls": [{"tool": "cache_api.sweep_stats", "status": "ok"}],
            "reasoning_markers": [],
            "upstream_errors": [],
            "injected_failure": None,
        }

    if cls == "F1":
        return {
            "attempt": attempt,
            "text": '{"status": "ok", "evicted": 130, "retained": 9999, "source_verified": true}',
            "confidence": 0.42,
            "psc": 0.55,
            "tool_calls": [{"tool": "cache_api.sweep_stats", "status": "ok"}],
            "reasoning_markers": [],
            "upstream_errors": [],
            "injected_failure": "F1",
        }
    if cls == "F2":
        return {
            "attempt": attempt,
            "text": '{"status": "ok"}',
            "confidence": 0.75,
            "psc": 0.70,
            "tool_calls": [
                {"tool": "cache_api.sweep_stats", "status": "ok"},
                {"tool": "cache_api.dupe_scan", "status": "error", "error": "connection refused"},
            ],
            "reasoning_markers": [],
            "upstream_errors": [],
            "injected_failure": "F2",
        }
    if cls == "F3":
        return {
            "attempt": attempt,
            "text": '{"status": "ok", "evicted": 130, "retained": 58}',
            "confidence": 0.60,
            "psc": 0.33,
            "tool_calls": [{"tool": "cache_api.sweep_stats", "status": "ok"}],
            "reasoning_markers": ["hot>ceiling", "hot<=ceiling"],
            "upstream_errors": [],
            "injected_failure": "F3",
        }
    if cls == "F4":
        return {
            "attempt": attempt,
            "text": '{"status": "ok", "evicted": 130, "retained"',
            "confidence": 0.55,
            "psc": 0.60,
            "tool_calls": [{"tool": "cache_api.sweep_stats", "status": "ok"}],
            "reasoning_markers": [],
            "upstream_errors": [{"step": "upstream_digest", "error": "timeout"}],
            "injected_failure": "F4",
        }
    raise ValueError("unknown class %r" % cls)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--attempt", type=int, required=True)
    args = ap.parse_args()

    with open(args.case) as f:
        case = yaml.safe_load(f)

    out = build_output(case, args.attempt)
    adir = sh_lib.attempt_dir(args.run_dir, args.attempt)
    sh_lib.write_json(os.path.join(adir, "output.json"), out)
    sh_lib.append_trace(
        args.run_dir, "attempt", attempt=args.attempt,
        injected_failure=out["injected_failure"],
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
