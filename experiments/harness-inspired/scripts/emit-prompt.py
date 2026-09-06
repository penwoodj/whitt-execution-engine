#!/usr/bin/env python3
"""Prompt emitter: generates role-framed prompts for each workflow step.

Usage: emit-prompt.py --case YAML --role worker|evaluator|fixer|judge --stage SCREEN|GEN|CHECK|FIX|JUDGE
       emit-prompt.py --case YAML --role worker --stage GEN --retry 2
Output: prompt text to stdout, exit 0=ready, 1=error.
"""
import argparse, json, os, sys, yaml
from pathlib import Path

ROLE_FRAMES = {
    "worker": {
        "voice": "precise engineer",
        "rules": [
            "Output ONLY what was asked. No preamble. No commentary.",
            "No meta-discussion about the task. Just do it.",
            "If uncertain, state your best answer. Never refuse.",
        ],
    },
    "evaluator": {
        "voice": "QA auditor",
        "rules": [
            "You evaluate output against explicit criteria.",
            "Output exactly ONE line: VERDICT: PASS or VERDICT: FAIL.",
            "If FAIL, second line: REASON: one sentence explanation.",
            "No other output. No preamble.",
        ],
    },
    "fixer": {
        "voice": "surgical editor",
        "rules": [
            "Fix the specific problems identified. Nothing else.",
            "Preserve all correct content. Change only what's broken.",
            "Output the complete fixed artifact. No diff, no commentary.",
            "Do NOT add explanations about what you changed.",
        ],
    },
    "judge": {
        "voice": "blind quality reviewer",
        "rules": [
            "You are a quality judge. You do NOT know which model produced this.",
            "Rate the output: VERDICT: PASS (meets standard) or VERDICT: FAIL (below standard).",
            "If FAIL: REASON: specific what's missing or wrong.",
            "One verdict only. No negotiation.",
        ],
    },
}


def build_prompt(case, role, stage, retry=0):
    prompt_text = case.get("prompt", "")
    auxiliary = case.get("auxiliary", "")
    criteria = case.get("success_criteria", {})
    frame = ROLE_FRAMES.get(role, ROLE_FRAMES["worker"])

    parts = [f"Role: {frame['voice']}."]
    parts.extend(f"{r}" for r in frame["rules"])

    if retry > 0:
        parts.append(f"RETRY {retry}. Previous attempt failed. Improve quality.")

    if stage == "CHECK":
        checks = criteria.get("deterministic_checks", {})
        parts.append("Evaluate this output against these criteria:")
        if checks.get("contains_required"):
            parts.append(f"MUST contain: {', '.join(checks['contains_required'])}")
        if checks.get("forbidden_phrases"):
            parts.append(f"MUST NOT contain: {', '.join(checks['forbidden_phrases'])}")
        if checks.get("max_words"):
            parts.append(f"MAX {checks['max_words']} words.")
        if checks.get("min_words"):
            parts.append(f"MIN {checks['min_words']} words.")
        parts.append("Output: VERDICT: PASS or VERDICT: FAIL")
    elif stage == "JUDGE":
        objective = case.get("objective", prompt_text)
        parts.append(f"Task objective: {objective}")
        parts.append("Does the output accomplish this objective? VERDICT: PASS or VERDICT: FAIL")
    elif stage == "FIX":
        parts.append(f"Fix the output. Original criteria: {json.dumps(criteria.get('deterministic_checks', {}), indent=None)[:200]}")
    else:
        if auxiliary:
            parts.append(f"Context: {auxiliary}")
        parts.append(f"Task: {prompt_text}")

    return "\n".join(parts)


def main():
    p = argparse.ArgumentParser(description="Prompt emitter")
    p.add_argument("--case", required=True, help="Case YAML path")
    p.add_argument("--role", required=True, choices=["worker", "evaluator", "fixer", "judge"])
    p.add_argument("--stage", default="GEN", choices=["SCREEN", "GEN", "CHECK", "FIX", "JUDGE"])
    p.add_argument("--retry", type=int, default=0)
    p.add_argument("--out", help="Write prompt to file instead of stdout")
    args = p.parse_args()

    if not Path(args.case).is_file():
        print(f"ERROR: case not found: {args.case}", file=sys.stderr)
        sys.exit(1)

    case = yaml.safe_load(Path(args.case).read_text()) or {}
    prompt = build_prompt(case, args.role, args.stage, args.retry)

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(prompt)

    print(prompt)
    sys.exit(0)


if __name__ == "__main__":
    main()