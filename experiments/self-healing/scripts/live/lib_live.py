#!/usr/bin/env python3
"""lib_live.py — shared helpers for live-LLM workflow steps (Phase L).

The spoof kit (sh_lib_v2 / agent_run_v2) stays the source of truth for
thresholds, trace ledger, and failure-profile corruption. This module
adds the raw-text glue the live path needs: model output arrives as raw
.txt files written by save_to hooks; we leniently extract JSON, apply
the deterministic failure injection, and emit the same attempt_<n>.json
contract the detectors already consume.
"""
from __future__ import annotations

import json
import random
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import agent_run_v2 as spoof  # noqa: E402
import sh_lib_v2 as lib  # noqa: E402

VARIANT_ROLES = {
    "fast": "worker_fast",
    "coder": "worker_coder",
    "general": "worker_general",
    "heavy": "heavy_replanner",
}
ROLE_IDS = {"worker_fast": 0, "worker_coder": 1, "worker_general": 2, "heavy_replanner": 3}

FENCE_RE = re.compile(r"```(?:json)?\s*(.*?)```", re.DOTALL)


def extract_json(text: str) -> dict | None:
    """Lenient JSON extraction from raw model output.

    Tries: fenced block -> whole text -> first '{' to last '}'.
    Returns None when nothing parses.
    """
    for candidate in (m.group(1) for m in FENCE_RE.finditer(text)):
        try:
            obj = json.loads(candidate)
            if isinstance(obj, dict):
                return obj
        except json.JSONDecodeError:
            continue
    try:
        obj = json.loads(text)
        if isinstance(obj, dict):
            return obj
    except json.JSONDecodeError:
        pass
    lo, hi = text.find("{"), text.rfind("}")
    if lo != -1 and hi > lo:
        try:
            obj = json.loads(text[lo:hi + 1])
            if isinstance(obj, dict):
                return obj
        except json.JSONDecodeError:
            pass
    return None


def read_raw(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def parse_attempt(raw_text: str) -> tuple[dict, bool]:
    """Return (attempt_dict, parsed_flag).

    Unparseable output degrades to a shell dict that detectors will read
    as schema-missing (F2-leaning) — mirrors a real worker rambling
    instead of emitting the contract object.
    """
    obj = extract_json(raw_text)
    if obj is None:
        return {"unparsed_text": raw_text[:4000],
                "_meta": {"confidence": 0.12, "self_consistency": 0.10}}, False
    return obj, True


def ensure_meta(attempt: dict, parsed: bool) -> dict:
    """Guarantee a _meta block with clean-band defaults when the model
    omitted one (live models rarely emit our private _meta contract)."""
    meta = attempt.get("_meta")
    if not isinstance(meta, dict):
        meta = {}
        attempt["_meta"] = meta
    meta.setdefault("confidence", 0.78)
    meta.setdefault("self_consistency", 0.85)
    meta.pop("truncated", None)
    meta.setdefault("tool_errors", 0)
    meta.setdefault("contradiction_pairs", [])
    if not parsed:
        meta["confidence"] = 0.12
        meta["self_consistency"] = 0.10
    return attempt


def corrupt(attempt: dict, cls: str, attempt_no: int, case_id: str) -> dict:
    """Apply the deterministic failure profile via the spoof kit's
    inject() so live-injected signatures match detector expectations
    exactly (same code path as Phase S2)."""
    rng = random.Random(f"{case_id}:{cls}:{attempt_no}")
    return spoof.inject(cls, attempt, attempt_no, rng)


def worker_prompt(case: dict, round_no: int, run_dir: Path) -> str:
    """Compose the attempt prompt: round 1 = task verbatim; round >= 2 =
    task + corrective context from the previous heal record (when present —
    a judge-reject round has no heal record yet)."""
    task = case["task"]["prompt"]
    if round_no == 1:
        return task
    heal_path = run_dir / f"heal_{round_no - 1}.json"
    heal = json.loads(heal_path.read_text()) if heal_path.exists() else None
    lines = [
        "RE-ATTEMPT (round %d). Your previous attempt did not pass." % round_no,
        "",
        "TASK CONTRACT (verbatim):",
        task,
        "",
    ]
    if heal is None:
        lines.append(
            "CORRECTIVE CONTEXT: automated checks passed but the auditor "
            "rejected the artifact. Re-emit the complete JSON object with "
            "strict attention to the contract's quality bar."
        )
    else:
        lines.append("CORRECTIVE CONTEXT (round %d heal):" % (round_no - 1))
        lines.append("strategy: %s" % heal.get("strategy", "unknown"))
        if heal.get("strategy_text"):
            lines.append("replanner output:")
            lines.append(str(heal["strategy_text"])[:2400])
        instruction = heal.get("instruction") or heal.get("plan") or ""
        if instruction:
            lines.append("instruction: %s" % instruction)
    lines.append("")
    lines.append("Re-attempt now. Return ONLY the JSON artifact per contract, "
                 "no wrapper object, no commentary.")
    return "\n".join(lines)
