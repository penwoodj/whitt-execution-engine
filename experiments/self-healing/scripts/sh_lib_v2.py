#!/usr/bin/env python3
"""sh_lib_v2.py — shared helpers for self-healing v2 spoof scripts.

v2 adds over v1 (sh_lib.py):
  - budget ledger (llm_call events) with ceiling check
  - routing decisions (model specialization)
  - gate flags (need_replan / need_judge / budget_exhausted)
  - gray-zone acceptance band
Trace event schema: one JSON object per line:
  {"event": str, "step": str, "data": {...}}
"""
from __future__ import annotations

import json
import os
from pathlib import Path

import yaml

# ---- thresholds register (mirrors docs/v2/02-WORKFLOW-V2-DESIGN.md §gates) ----
THETA = 0.65            # reliability gate
EPSILON = 0.05          # gray zone half-width: theta <= R < theta+eps
OMEGA = {"C": 0.35, "S": 0.35, "E": 0.30}
MAX_ATTEMPTS = 3
CONFIDENCE_FLOOR = 0.50
PSC_UNCERTAIN = 0.30
LENGTH_BOUNDS = (50, 5000)
LLM_CALL_CEILING = 6

# exit-code contract (triage_v2)
GATE_CLEAN = 0
GATE_LIGHT = 1
GATE_REPLAN = 2
GATE_FAIL = 3

# route_v2 exit contract (worker variant selection)
ROUTE_FAST = 0
ROUTE_CODER = 1
ROUTE_GENERAL = 2
ROUTE_HEAVY = 3

CLASS_OF_GATE = {GATE_CLEAN: "clean", GATE_LIGHT: "light", GATE_REPLAN: "replan", GATE_FAIL: "fail"}

# model specialization map (docs/v2/02-WORKFLOW-V2-DESIGN §models)
MODELS = {
    "worker_general": "Qwen3-4B-Instruct-2507-Q4_K_M",
    "worker_coder": "Qwen2.5-Coder-3B-Instruct-Q8_0",
    "worker_fast": "Ministral-3-3B-Instruct-2512-Q4_K_M",
    "heavy_replanner": "Qwen3-5-9B-Q4_K_M",
    "judge": "Hermes-2-Pro-Mistral-7B.Q4_K_M",
}

CODE_DOMAINS = {"software-eng", "scientific-compute", "data-analysis"}
CLASS_STRATEGY = {"F1": "corrective_prompt", "F2": "tool_reselect", "F3": "replan", "F4": "replan"}


def load_case(case_path: str) -> dict:
    return yaml.safe_load(Path(case_path).read_text())


def run_dir_of(case_path: str, base: str) -> Path:
    cid = load_case(case_path)["case_id"]
    return Path(base) / cid


def trace_append(run_dir: Path, event: str, step: str, **data) -> None:
    run_dir.mkdir(parents=True, exist_ok=True)
    with (run_dir / "trace.jsonl").open("a") as fh:
        fh.write(json.dumps({"event": event, "step": step, "data": data}) + "\n")


def trace_read(run_dir: Path) -> list[dict]:
    p = run_dir / "trace.jsonl"
    if not p.exists():
        return []
    out = []
    for line in p.read_text().splitlines():
        line = line.strip()
        if line:
            out.append(json.loads(line))
    return out


def reset_run(run_dir: Path) -> None:
    """Destructive: clear run state so stale artifacts never pollute a rerun."""
    import shutil

    if run_dir.exists():
        for child in run_dir.iterdir():
            if child.is_dir():
                shutil.rmtree(child)
            else:
                child.unlink()


def llm_calls_used(run_dir: Path) -> int:
    return sum(1 for e in trace_read(run_dir) if e["event"] == "llm_call")


def budget_ok(run_dir: Path, ceiling: int = LLM_CALL_CEILING) -> bool:
    return llm_calls_used(run_dir) < ceiling


def failed_attempts_of(case: dict) -> list[int]:
    return list(case.get("failure", {}).get("injection_profile", {}).get("attempts_failed", []))


def class_of(case: dict) -> str:
    return case.get("failure", {}).get("class", "clean")


def write_json(run_dir: Path, name: str, obj: dict) -> None:
    run_dir.mkdir(parents=True, exist_ok=True)
    (run_dir / name).write_text(json.dumps(obj, indent=2) + "\n")


def read_json(run_dir: Path, name: str) -> dict:
    return json.loads((run_dir / name).read_text())
