#!/usr/bin/env python3
"""Append-only evidence keeper for intake steps (SEGMENT+ Evidence
precision + bounded budget). v2.0 rewrite-memory diluted needles with
abstractions; v2.1 keeps verbatim evidence lines only, appended in
document order, FIFO-capped. 'NONE' outputs append nothing. Exit 0
always.

Usage: intake-merge.py --run-dir D [--max-words 900]
"""

import argparse
import re
import sys
from pathlib import Path


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--max-words", type=int, default=900)
    args = ap.parse_args()
    run = Path(args.run_dir)

    raw = (run / "intake-last.txt").read_text() if (run / "intake-last.txt").exists() else ""
    ts_led = re.compile(r"^\d{1,2}:\d{2}(:\d{2})?\s")
    lines = []
    for ln in raw.splitlines():
        s = re.sub(r"^\s*(?:[-*•]|\d+[.)])\s*", "", ln.strip())
        if not s or s.upper().startswith("NONE") or s.upper().startswith("EVIDENCE"):
            continue
        if "DOCUMENT CHUNK" in s or s.startswith("TASK NOW"):
            continue
        if ts_led.match(s):
            continue
        lines.append(s)
    notes_p = run / "notes.txt"
    prior = notes_p.read_text().splitlines() if notes_p.exists() else []
    merged = prior + lines
    seen = set()
    deduped = [l for l in merged if not (l in seen or seen.add(l))]
    words = sum(len(l.split()) for l in deduped)
    while words > args.max_words and deduped:
        words -= len(deduped[0].split())
        deduped = deduped[1:]
    if deduped:
        notes_p.write_text("\n".join(deduped) + "\n")
    print(f"evidence lines={len(deduped)} words={words if deduped else 0} +{len(lines)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
