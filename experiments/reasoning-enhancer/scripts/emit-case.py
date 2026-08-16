#!/usr/bin/env python3
"""REA per-step context composer.

Usage (one phase per call; writes composed context to stdout):
  emit-case.py --case C.yml --run-dir D --phase draft-text
  emit-case.py --case C.yml --run-dir D --phase decompose
  emit-case.py --case C.yml --run-dir D --phase validate-subq
  emit-case.py --case C.yml --run-dir D --phase solve
  emit-case.py --case C.yml --run-dir D --phase planverify
  emit-case.py --case C.yml --run-dir D --phase toolverify --out verify.json
  emit-case.py --case C.yml --run-dir D --phase synthesize --round 1|2 [--out F]

Factored-context rule (CoVe, RESEARCH.md #3): solve phase NEVER sees the
draft. synthesize round 2 adds escalation memory (open failures, latest-wins)
plus persona lens. Missing state files degrade to fallbacks, never crash —
mirrors correction-atom emit-state behavior.
"""

import argparse
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import yaml

from check_lib import run_checks

WORD_CAP = 1400
PERSONA_R2 = (
    "You are a skeptical auditor. Before writing the final answer, re-derive "
    "every number and fact from the source material yourself. Distrust the "
    "prior attempt: it already failed the checks listed below."
)


def _cap(text, words=WORD_CAP):
    toks = text.split()
    if len(toks) <= words:
        return text
    return " ".join(toks[:words]) + "\n[...truncated...]"


def _read(path):
    p = Path(path)
    return p.read_text() if p.exists() else None


def _load_json(path):
    raw = _read(path)
    if raw is None:
        return None
    try:
        return json.loads(raw)
    except json.JSONDecodeError:
        return None


def _parse_subquestions(raw):
    """Extract question list from model output: JSON list or numbered lines.
    JSON dict without 'subquestions' = invalid (live run 1: model echoed
    rubric vocabulary as fake JSON; line-parsing its syntax fragments
    wrongly validated it)."""
    if raw is None:
        return None
    try:
        data = json.loads(raw)
        if isinstance(data, dict):
            if isinstance(data.get("subquestions"), list):
                data = data["subquestions"]
            else:
                return None
        if isinstance(data, list) and data and all(isinstance(q, str) for q in data):
            return data
    except json.JSONDecodeError:
        pass
    lines = [re.sub(r"^\s*\d+[.)]\s*", "", ln).strip("- \t")
             for ln in raw.splitlines()]
    lines = [ln for ln in lines
             if 15 <= len(ln) <= 300 and not ln.startswith(("{", '"', "[", "}", "]"))]
    return lines if 3 <= len(lines) <= 5 else None


def open_failures_from(*check_jsons):
    """Aggregate open failures, latest-wins per check (v8 escalation memory)."""
    latest = {}
    for cj in check_jsons:
        if not cj:
            continue
        for f in cj.get("failures", []):
            latest[f["check"]] = f
    return list(latest.values())


def fmt_open_failures(failures):
    if not failures:
        return "none — no known open failures"
    return "\n".join(
        f"- {f['check']}: {f['detail']} (hint: {f['fix_hint']})" for f in failures
    )


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--phase", required=True,
                    choices=["draft-text", "gate-prepare", "decompose",
                             "validate-subq", "solve", "planverify",
                             "toolverify", "synthesize", "baseline",
                             "oc-baseline", "intake", "oc-synth"])
    ap.add_argument("--round", type=int, default=1)
    ap.add_argument("--chunk", type=int, default=1)
    ap.add_argument("--out")
    args = ap.parse_args()

    with open(args.case) as fh:
        case = yaml.safe_load(fh)
    run = Path(args.run_dir)
    prompt = case.get("prompt", "").strip()
    draft = case.get("draft_response", "").strip()
    auxiliary = (case.get("auxiliary") or "").strip()
    checks = (case.get("success_criteria") or {}).get("deterministic_checks") or {}
    out = None

    if args.phase == "draft-text":
        out = draft

    elif args.phase == "baseline":
        out = (
            "TASK:\n" + _cap(prompt) + "\n\n"
            + ("SOURCE MATERIAL:\n" + _cap(auxiliary) + "\n\n" if auxiliary else "")
            + "Answer the task directly and completely."
        )

    elif args.phase == "oc-baseline":
        doc_path = Path(args.case).parent / case["doc_file"]
        doc_text = doc_path.read_text()
        out = (
            "TASK:\n" + prompt + "\n\n"
            + "SOURCE DOCUMENT (attached in full):\n" + doc_text + "\n\n"
            + "Answer the task using the source document."
        )

    elif args.phase == "intake":
        doc_dir = Path(args.case).parent / case["chunks_dir"]
        n = case.get("chunk_count", 13)
        chunk_text = (doc_dir / f"chunk-{args.chunk:02d}.txt").read_text()
        out = (
            "QUESTION TO ANSWER EVENTUALLY:\n" + prompt + "\n\n"
            + f"DOCUMENT CHUNK {args.chunk} OF {n}:\n" + chunk_text + "\n\n"
            + "TASK NOW: extract EVIDENCE LINES.\n"
            + "RULES:\n"
            + "- An EVIDENCE LINE is a line that could help answer the question — it carries a\n"
            + "  distinctive marker (an id like EV-####/ADV-##/TK-###/BG-####, an UPPERCASE tag like\n"
            + "  SPECIAL-EVENT, MIGRATION-NOTICE, CRITICAL-ADVISORY, OVERRUN, DRIFT, DISCOUNT,\n"
            + "  MINUTES decision, EMAIL with a decision/correction) or names the entities the\n"
            + "  question asks about.\n"
            + "- Copy each evidence line VERBATIM — one line each, unchanged.\n"
            + "- Routine operational lines (heartbeats, ordinary alerts, pool settings, ordinary\n"
            + "  emails/minutes/journal lines) are NEVER evidence — ignore them.\n"
            + "- If this chunk has no evidence line, output exactly: NONE\n"
            + "- Output ONLY the evidence lines (or NONE). No preamble. No commentary."
        )

    elif args.phase == "oc-synth":
        notes = _cap((run / "notes.txt").read_text(), words=900) if (run / "notes.txt").exists() else "(none)"
        out = (
            "TASK:\n" + prompt + "\n\n"
            + "EVIDENCE LINES (copied verbatim from the full document — trust them):\n" + notes + "\n\n"
        )
        if args.round == 2:
            r1 = _load_json(run / "check-r1.json")
            fails = open_failures_from(r1) if r1 else []
            out += (
                "PRIOR ATTEMPT FAILED THESE CHECKS (fix ALL):\n"
                + fmt_open_failures(fails) + "\n\n"
                + "You are a skeptical auditor. Re-derive from the notes; distrust the prior attempt.\n\n"
            )
        out += (
            "HARD OUTPUT RULES:\n"
            + ("- " + checks_hint(checks) + "\n" if checks_hint(checks) else "")
            + "- The output IS the deliverable. No preamble. No commentary. No describing the answer."
        )

    elif args.phase == "gate-prepare":
        (run / "draft-response.txt").write_text(draft)
        result = run_checks(draft, checks)
        (run / "check-draft.json").write_text(json.dumps(result, indent=2))
        if result["passed"]:
            (run / "final-answer.txt").write_text(draft)
            (run / "gate-exit.json").write_text(json.dumps({"mode": "early_exit"}))
        print(json.dumps({"passed": result["passed"]}))
        return 0

    elif args.phase == "decompose":
        out = (
            "TASK:\n" + _cap(prompt) + "\n\n"
            + ("SOURCE MATERIAL:\n" + _cap(auxiliary) + "\n\n" if auxiliary else "")
            + "Break this task into 3 to 5 sub-questions that each cover one "
            "fact or step needed for a complete answer.\n"
            + "Sub-questions are plain questions ABOUT THE TASK ABOVE.\n"
            "Output ONLY this JSON shape, filled with real questions about "
            "the task (not the placeholder example):\n"
            '{"subquestions": ["What is the total distance of the planned '
            'route?", "Which two legs share a starting point?"]}'
        )

    elif args.phase == "validate-subq":
        subq = _parse_subquestions(_read(run / "subquestions.txt"))
        ok = subq is not None
        result = {"valid": ok, "count": len(subq) if subq else 0}
        (Path(args.out) if args.out else run / "subq-valid.json").write_text(
            json.dumps(result, indent=2))
        print(json.dumps(result))
        return 0 if ok else 1

    elif args.phase == "solve":
        subq = _parse_subquestions(_read(run / "subquestions.txt"))
        if subq:
            qlist = "\n".join(f"{i+1}. {q}" for i, q in enumerate(subq))
            out = (
                "Answer these sub-questions using the source material.\n\n"
                "SUB-QUESTIONS:\n" + qlist + "\n\n"
                + ("SOURCE MATERIAL:\n" + _cap(auxiliary) + "\n\n" if auxiliary else "")
                + "Answer each on its own numbered line, terse and exact."
            )
        else:
            out = (
                "TASK:\n" + _cap(prompt) + "\n\n"
                + ("SOURCE MATERIAL:\n" + _cap(auxiliary) + "\n\n" if auxiliary else "")
                + "Answer the task directly, terse and exact."
            )

    elif args.phase == "planverify":
        out = (
            "TASK:\n" + _cap(prompt) + "\n\n"
            "DRAFT ANSWER UNDER REVIEW:\n" + _cap(draft) + "\n\n"
            "List 3 to 5 verification questions: each must check ONE claim in "
            "the draft that could be wrong (a number, a count, a missing "
            "requirement). One question per line, numbered."
        )

    elif args.phase == "toolverify":
        draft_check = run_checks(draft, checks)
        suba = _read(run / "subanswers.txt") or ""
        result = {
            "draft_check": draft_check,
            "subanswers_present": bool(suba.strip()),
            "subanswers_words": len(suba.split()),
        }
        target = args.out or str(run / "verify.json")
        Path(target).write_text(json.dumps(result, indent=2))
        print(json.dumps({"draft_passed": draft_check["passed"],
                          "subanswers_present": result["subanswers_present"]}))
        return 0

    elif args.phase == "synthesize":
        subq = _parse_subquestions(_read(run / "subquestions.txt"))
        suba = _read(run / "subanswers.txt")
        verify = _load_json(run / "verify.json")
        draft_failures = open_failures_from((verify or {}).get("draft_check"))
        r1_check = _load_json(run / "check-r1.json")
        parts = []
        parts.append("TASK:\n" + _cap(prompt))
        if auxiliary:
            parts.append("SOURCE MATERIAL (authoritative):\n" + _cap(auxiliary))
        if subq:
            parts.append("SUB-QUESTIONS:\n"
                         + "\n".join(f"{i+1}. {q}" for i, q in enumerate(subq)))
        if suba and suba.strip():
            parts.append("SUB-ANSWERS (from independent solver):\n" + _cap(suba))
        if args.round >= 2:
            parts.append(PERSONA_R2)
            parts.append("PRIOR ATTEMPT (round 1) FAILED THESE CHECKS — fix ALL:\n"
                         + fmt_open_failures(open_failures_from(r1_check)))
        else:
            parts.append("OPEN FAILURES IN THE DRAFT — fix ALL:\n"
                         + fmt_open_failures(draft_failures))
        parts.append("DRAFT ANSWER (may contain errors):\n" + _cap(draft))
        parts.append(
            "HARD OUTPUT RULES (obey exactly):\n"
            "- Write the final answer only. No preamble, no notes, no "
            "mentions of questions, checks, or instructions.\n"
            "- Satisfy every open failure listed above.\n"
            + ("- " + checks_hint(checks) + "\n" if checks_hint(checks) else "")
            + "- Match the requested format from the task."
        )
        out = "\n\n".join(parts)

    if out is None:
        ap.error(f"phase {args.phase} produced no output")
    if args.out:
        Path(args.out).write_text(out)
    print(out)
    return 0


def checks_hint(checks):
    hints = []
    if checks.get("bullet_count_min") or checks.get("bullet_count_max"):
        hints.append(f"use bullet lines '- ' within [{checks.get('bullet_count_min') or 0},"
                     f"{checks.get('bullet_count_max') or 'any'}]")
    if checks.get("yaml_parsable"):
        hints.append("output must parse as YAML")
    if checks.get("numbers_must_sum_to") is not None:
        hints.append(f"amounts must sum to {checks['numbers_must_sum_to']}: write each amount "
                     "exactly ONCE, never restate an amount, never mention the total")
    lo, hi = checks.get("min_words"), checks.get("max_words")
    if lo or hi:
        target = int(hi * 0.75) if hi else None
        band = f"at most {target} words" if target else f"at least {lo} words"
        hints.append(f"word budget {band} (hard window [{lo or 0}, {hi or 'any'}]) — count before writing")
    if checks.get("forbidden_phrases"):
        hints.append("never use these: " + ", ".join(f"'{p}'" for p in checks["forbidden_phrases"]))
    if checks.get("contains_required"):
        hints.append("include exactly (character-for-character): "
                     + ", ".join(f"'{p}'" for p in checks["contains_required"]))
    return "; ".join(hints)


if __name__ == "__main__":
    sys.exit(main())
