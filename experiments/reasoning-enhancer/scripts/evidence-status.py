#!/usr/bin/env python3
"""v3.0 smart intake gate (tests/test-evidence-status.sh is the contract).

Exit 7 = COMPLETE (all gate tokens in evidence notes -> workflow GWT
{{bookmarks.shell_output.exit_code}} == 7 routes to synth, skipping the
remaining intake chunks). Exit 0 = MORE. Gate tokens: contains_required
entries found verbatim in the chunk files, plus len>=5 alnum-dash parts of
non-verbatim entries that themselves occur verbatim in the chunks. Empty
gate = MORE (safe default: read everything).
"""

import argparse
import json
import re
import sys
from pathlib import Path

import yaml

COMPLETE_EXIT = 7
IDENT_RE = re.compile(r"[A-Za-z0-9-]{5,}")


def chunk_texts(case_path):
    case = yaml.safe_load(Path(case_path).read_text())
    cdir = (Path(case_path).parent / case.get("chunks_dir", "")).resolve()
    n = case.get("chunk_count", 13)
    return case, [(cdir / f"chunk-{i:02d}.txt").read_text()
                  for i in range(1, n + 1)
                  if (cdir / f"chunk-{i:02d}.txt").exists()]


def gate_tokens(required, doc):
    corpus = "\n".join(doc)
    tokens = []
    for tok in required:
        if tok in corpus:
            tokens.append(tok)
            continue
        for part in IDENT_RE.findall(tok):
            if part in corpus:
                tokens.append(part)
    return tokens


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    args = ap.parse_args()
    run = Path(args.run_dir)

    case, doc = chunk_texts(args.case)
    required = ((case.get("success_criteria") or {}).get("deterministic_checks") or {}
                ).get("contains_required") or []

    tokens = gate_tokens(required, doc)
    notes = run.joinpath("notes.txt").read_text() if run.joinpath("notes.txt").exists() else ""
    missing = [t for t in tokens if t not in notes]
    complete = bool(tokens) and not missing

    run.joinpath("evidence-status.json").write_text(json.dumps({
        "case_id": case.get("case_id"),
        "complete": complete,
        "missing": missing,
        "gate_tokens": tokens,
    }, indent=2))
    print(("COMPLETE" if complete else "MORE") + f" gate={len(tokens)} missing={len(missing)}")
    return COMPLETE_EXIT if complete else 0


if __name__ == "__main__":
    sys.exit(main())
