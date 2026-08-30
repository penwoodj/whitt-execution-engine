"""Outcome/fingerprint ledgers shared by agentic gate scripts.

Writes one JSONL line per gate invocation so runs are auditable and
collectable without parsing engine logs (AGENTIC-PHASE.md L2/L3).
"""
import datetime
import hashlib
import json
from pathlib import Path


def _append(path, obj):
    p = Path(path)
    p.parent.mkdir(parents=True, exist_ok=True)
    with p.open("a") as f:
        f.write(json.dumps(obj) + "\n")


def ledger_outcome(run_dir, cid, stage, outcome):
    _append(Path(run_dir) / "outcomes.jsonl", {
        "case": cid, "stage": stage, "outcome": outcome,
        "ts": datetime.datetime.utcnow().isoformat() + "Z"})


def ledger_fingerprint(run_dir, cid, stage, prompt_text):
    _append(Path(run_dir) / "fingerprint.jsonl", {
        "case": cid, "stage": stage,
        "sha256": hashlib.sha256(
            f"{cid}|{stage}|{prompt_text}".encode()).hexdigest(),
        "tokens_approx": max(1, len(prompt_text.split()))})
