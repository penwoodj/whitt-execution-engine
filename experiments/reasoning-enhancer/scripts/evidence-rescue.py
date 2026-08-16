#!/usr/bin/env python3
"""v3.0 cascade r3 prompt builder (tests/test-evidence-rescue.sh is the
contract). Reads check-r1/r2 failures, greps the chunk files for the missing
tokens (verbatim, then len>=5 parts), prints a targeted-rescue prompt to
stdout (--out also writes it). Exit 0 always.
"""

import argparse
import ast
import importlib.util
import json
import re
import sys
from pathlib import Path

import yaml

HERE = Path(__file__).parent
spec = importlib.util.spec_from_file_location("emit_case", HERE / "emit-case.py")
assert spec is not None and spec.loader is not None
emit_case = importlib.util.module_from_spec(spec)
spec.loader.exec_module(emit_case)

MISSING_RE = re.compile(r"missing:\s*(\[.*?\])")
IDENT_RE = re.compile(r"[A-Za-z0-9-]{5,}")
MAX_LINES = 40
MAX_WORDS = 600


def missing_tokens(run):
    found = []
    for name in ("check-r1.json", "check-r2.json"):
        p = run / name
        if not p.exists():
            continue
        for f in json.loads(p.read_text()).get("failures", []):
            m = MISSING_RE.search(f.get("detail", ""))
            if m:
                try:
                    found.extend(str(x) for x in ast.literal_eval(m.group(1)))
                except (ValueError, SyntaxError):
                    pass
    seen = set()
    return [t for t in found if not (t in seen or seen.add(t))]


def rescue_lines(tokens, case_path):
    case = yaml.safe_load(Path(case_path).read_text())
    cdir = (Path(case_path).parent / case.get("chunks_dir", "")).resolve()
    n = case.get("chunk_count", 13)
    chunks = [(i, (cdir / f"chunk-{i:02d}.txt").read_text())
              for i in range(1, n + 1)
              if (cdir / f"chunk-{i:02d}.txt").exists()]
    lines, seen = [], set()
    probes = []
    for tok in tokens:
        probes.append(tok)
        probes.extend(p for p in IDENT_RE.findall(tok) if p != tok)
    for _, text in chunks:
        for ln in text.splitlines():
            s = ln.strip()
            if not s or s in seen:
                continue
            if any(p in ln for p in probes):
                seen.add(s)
                lines.append(s)
    while lines and (len(lines) > MAX_LINES or sum(len(x.split()) for x in lines) > MAX_WORDS):
        lines.pop()
    return case, lines


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--out")
    args = ap.parse_args()
    run = Path(args.run_dir)

    case = yaml.safe_load(Path(args.case).read_text())
    checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}
    required = checks.get("contains_required") or []
    tokens = missing_tokens(run) or list(required)
    case, lines = rescue_lines(tokens, args.case)

    out = (
        "TASK:\n" + case.get("prompt", "") + "\n\n"
        + "TARGETED RESCUE EVIDENCE (verbatim lines from the full document containing\n"
        + "the still-missing facts — trust them):\n"
        + ("\n".join(lines) if lines else "(no direct hits; rely on the rules below)") + "\n\n"
        + "STILL-OPEN FAILURES (the answer MUST fix all of these):\n"
        + "- include exactly (character-for-character): " + ", ".join(f"'{t}'" for t in required) + "\n\n"
        + "HARD OUTPUT RULES:\n"
        + ("- " + emit_case.checks_hint(checks) + "\n" if emit_case.checks_hint(checks) else "")
        + "- The output IS the deliverable. No preamble. No commentary."
    )
    if args.out:
        Path(args.out).write_text(out + "\n")
    print(out)
    return 0


if __name__ == "__main__":
    sys.exit(main())
