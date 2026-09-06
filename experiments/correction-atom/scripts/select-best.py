#!/usr/bin/env python3
"""Ratchet selector with hard-fail gate.

Selection order:
1. first angle whose deterministic check passed (first_pass)
2. else best partial (most passed subchecks, tie → earliest) — but ONLY
   if its remaining failures are soft (word_count, bullet_count,
   contains_*). Hard failures (prompt_leak, degenerate,
   forbidden_phrases, yaml_parsable) mean shipping the candidate is
   worse than shipping nothing: no final is written and
   mode=hard_fail_input_fallback.

Writes final-corrected.txt + select-best.json. Always exits 0.

Usage: select-best.py <output_dir>
"""
import json
import shutil
import sys
from pathlib import Path

HARD_CHECKS = {"prompt_leak", "degenerate", "forbidden_phrases", "yaml_parsable"}


def load_angle(dirpath: Path, angle: int):
    txt = dirpath / f"after-angle-{angle}.txt"
    chk = dirpath / f"check-angle-{angle}.json"
    if not txt.exists() or not chk.exists():
        return None
    try:
        check = json.loads(chk.read_text())
    except Exception:
        return None
    return {"angle": angle, "text_path": txt, "check": check}


def subchecks_passed(check: dict) -> int:
    return sum(1 for c in check.get("checks", {}).values()
               if isinstance(c, dict) and c.get("passed") is True)


def failed_checks(check: dict) -> dict:
    return {n: c for n, c in check.get("checks", {}).items()
            if isinstance(c, dict) and c.get("passed") is not True}


def main():
    if len(sys.argv) < 2:
        print("usage: select-best.py <output_dir>", file=sys.stderr)
        sys.exit(0)
    dirpath = Path(sys.argv[1])

    candidates = [c for c in (load_angle(dirpath, a) for a in (1, 2, 3, 4, 5)) if c]

    if not candidates:
        print(json.dumps({"selected_angle": None, "mode": "no_outputs", "passed": False}))
        sys.exit(0)

    first_pass = next((c for c in candidates if c["check"].get("passed") is True), None)
    if first_pass:
        chosen, mode = first_pass, "first_pass"
    else:
        chosen = max(candidates, key=lambda c: (subchecks_passed(c["check"]), -c["angle"]))
        hard = HARD_CHECKS & set(failed_checks(chosen["check"]))
        if hard:
            meta = {
                "selected_angle": chosen["angle"],
                "mode": "hard_fail_input_fallback",
                "passed": False,
                "subchecks_passed": subchecks_passed(chosen["check"]),
                "hard_failures": sorted(hard),
            }
            (dirpath / "select-best.json").write_text(json.dumps(meta, indent=2))
            print(json.dumps(meta))
            sys.exit(0)
        mode = "best_partial"

    shutil.copyfile(chosen["text_path"], dirpath / "final-corrected.txt")
    meta = {
        "selected_angle": chosen["angle"],
        "mode": mode,
        "passed": bool(chosen["check"].get("passed")),
        "subchecks_passed": subchecks_passed(chosen["check"]),
    }
    (dirpath / "select-best.json").write_text(json.dumps(meta, indent=2))
    print(json.dumps(meta))


if __name__ == "__main__":
    main()
