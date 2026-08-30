#!/usr/bin/env python3
"""end_event_v2.py — lifecycle marker + full run-state reset for v2.

--status init: destructive reset (every v2 artifact pattern) so reruns
never inherit stale trace; appends event=init.
--status pass|fail: appends terminal event after the oracle report.
"""
from __future__ import annotations

import argparse
import glob
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from sh_lib_v2 import trace_append  # noqa: E402

STALE_FILES = ("trace.jsonl", "outcome.json", "report.json")
STALE_GLOBS = ("attempt_*.json", "detection_*.json", "triage_*.json",
               "routing_*.json", "judge_*.json", "heal_*.json")


def reset_run(run_dir: Path) -> None:
    for name in STALE_FILES:
        p = run_dir / name
        if p.exists():
            p.unlink()
    for pattern in STALE_GLOBS:
        for p in glob.glob(str(run_dir / pattern)):
            Path(p).unlink()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--status", required=True, choices=["init", "pass", "fail"])
    args = ap.parse_args()

    run_dir = Path(args.run_dir)
    run_dir.mkdir(parents=True, exist_ok=True)
    if args.status == "init":
        reset_run(run_dir)
        trace_append(run_dir, "init", "step_00_init")
    else:
        trace_append(run_dir, "end", f"end_{args.status}", status=args.status)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
