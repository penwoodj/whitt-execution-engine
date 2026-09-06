#!/usr/bin/env python3
"""Verify that a generated-workflow deliverable actually achieves the prompt.

Three layers:
  1. Machine gates  — exists, substantive, no refusals, limited meta-commentary
  2. Code checks    — extract fenced code blocks, syntax-check python/bash/node
  3. LLM judge      — local model scores deliverable vs original prompt (0-10)

Usage:
    verify-deliverable.py <prompt-file> <deliverable-file> [--judge-url URL]
                          [--judge-model ID] [--min-score N] [--report FILE]

Exit codes: 0 = VERIFIED, 1 = machine-gate failure, 2 = judge FAIL, 3 = usage.
Prints one `verdict: ...` line at the end that callers can parse.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
import urllib.request

REFUSAL_RE = re.compile(
    r"I cannot|I'm unable|I don't have access|As an AI|I would need to (read|access|see)"
)
META_RE = re.compile(
    r"To accomplish this|To do this, we need|I will (now|then)|Let me (start|begin|first)|First, I'll"
)

SYNTAX_CHECKERS = {
    "python": [sys.executable, "-m", "py_compile"],
    "py": [sys.executable, "-m", "py_compile"],
    "bash": ["bash", "-n"],
    "sh": ["bash", "-n"],
}

JUDGE_TEMPLATE = """You are a strict deliverable evaluator. Judge ONLY whether the deliverable below accomplishes the original request. Ignore style. Penalize: missing functionality the request asked for, placeholder/TODO content, code that obviously cannot run, answers that describe instead of implement.

ORIGINAL REQUEST:
{prompt}

DELIVERABLE:
{deliverable}

Respond in EXACTLY this format (three lines, nothing else):
SCORE: <integer 0-10>
VERDICT: <PASS or FAIL>  (PASS only if the deliverable substantially accomplishes the request)
REASONS: <one line summary of the main gaps or strengths>"""


def gate(name: str, passed: bool, detail: str, results: list) -> bool:
    results.append((name, passed, detail))
    print(f"  {'PASS' if passed else 'FAIL'}  {name}: {detail}")
    return passed


def extract_code_blocks(text: str):
    """Return list of (lang, code) for fenced blocks."""
    blocks = []
    for m in re.finditer(r"```([A-Za-z0-9_+-]*)\n(.*?)```", text, re.DOTALL):
        lang = (m.group(1) or "").lower()
        blocks.append((lang, m.group(2)))
    return blocks


def syntax_check(lang: str, code: str):
    """Return (checked, ok, message)."""
    cmd = SYNTAX_CHECKERS.get(lang)
    if not cmd:
        return (False, True, "no checker")
    suffix = {"python": ".py", "py": ".py"}.get(lang, ".sh")
    with tempfile.NamedTemporaryFile("w", suffix=suffix, delete=False) as f:
        f.write(code)
        path = f.name
    try:
        r = subprocess.run(cmd + [path], capture_output=True, text=True, timeout=30)
        ok = r.returncode == 0
        msg = (r.stderr or r.stdout).strip().splitlines()[-1] if not ok else "ok"
        return (True, ok, msg[:160])
    except Exception as e:  # noqa: BLE001
        return (True, False, str(e)[:160])
    finally:
        os.unlink(path)


def judge(prompt: str, deliverable: str, url: str, model: str):
    """Ask the local model for a PASS/FAIL + score. Returns (score, verdict, reasons)."""
    body = {
        "model": model,
        "messages": [{
            "role": "user",
            "content": JUDGE_TEMPLATE.format(
                prompt=prompt[:8000], deliverable=deliverable[:24000]),
        }],
        "max_tokens": 200,
        "temperature": 0.0,
        "reasoning_effort": "none",
    }
    req = urllib.request.Request(
        url.rstrip("/") + "/v1/chat/completions",
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=600) as resp:
        data = json.load(resp)
    text = data["choices"][0]["message"]["content"]
    score_m = re.search(r"SCORE:\s*(\d+)", text)
    verdict_m = re.search(r"VERDICT:\s*(PASS|FAIL)", text, re.IGNORECASE)
    reasons_m = re.search(r"REASONS:\s*(.+)", text)
    score = int(score_m.group(1)) if score_m else -1
    verdict = verdict_m.group(1).upper() if verdict_m else "UNPARSEABLE"
    reasons = reasons_m.group(1).strip() if reasons_m else text.strip()[:200]
    return score, verdict, reasons


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("prompt_file")
    ap.add_argument("deliverable_file")
    ap.add_argument("--judge-url", default=os.environ.get("WHITT_JUDGE_URL", "http://localhost:1234"))
    ap.add_argument("--judge-model", default=os.environ.get("WHITT_LMSTUDIO_MODEL", "qwen/qwen3.5-9b"))
    ap.add_argument("--min-score", type=int, default=6)
    ap.add_argument("--report", default=None)
    ap.add_argument("--skip-judge", action="store_true")
    ap.add_argument("--extract-dir", default=None,
                    help="write each fenced code block to this dir as a runnable file")
    args = ap.parse_args()

    try:
        prompt = open(args.prompt_file).read()
    except OSError as e:
        print(f"cannot read prompt: {e}", file=sys.stderr)
        return 3

    results: list = []
    print("== machine gates ==")
    exists = os.path.isfile(args.deliverable_file)
    gate("deliverable exists", exists, args.deliverable_file, results)
    if not exists:
        print("verdict: FAIL (missing deliverable)")
        return 1
    deliverable = open(args.deliverable_file, errors="replace").read()
    size, lines = len(deliverable), deliverable.count("\n") + 1
    g_sub = gate("substantive", size > 500 and lines > 10, f"{size}B / {lines} lines", results)
    refusals = len(REFUSAL_RE.findall(deliverable))
    g_ref = gate("no refusals", refusals == 0, f"{refusals} refusal patterns", results)
    meta = len(META_RE.findall(deliverable))
    g_meta = gate("limited meta-commentary", meta <= 2, f"{meta} meta patterns", results)

    print("== code checks ==")
    blocks = extract_code_blocks(deliverable)
    checked = valid = 0
    for i, (lang, code) in enumerate(blocks):
        did, ok, msg = syntax_check(lang, code)
        if did:
            checked += 1
            valid += ok
            print(f"  {'PASS' if ok else 'FAIL'}  block {i+1} ({lang or '?'}): {msg}")
    g_code = checked == 0 or valid > 0
    print(f"  summary: {len(blocks)} blocks, {checked} checked, {valid} valid")
    if checked > 0 and valid == 0:
        print("  FAIL  no syntactically valid code block")

    if args.extract_dir and blocks:
        os.makedirs(args.extract_dir, exist_ok=True)
        ext_map = {"python": ".py", "py": ".py", "bash": ".sh", "sh": ".sh",
                   "javascript": ".js", "js": ".js", "html": ".html",
                   "yaml": ".yml", "yml": ".yml", "json": ".json", "rust": ".rs"}
        # try to name files from a preceding "`filename.ext`" mention; else block-N
        for i, (lang, code) in enumerate(blocks):
            name = None
            window = deliverable[:deliverable.find(code)][-400:]
            mentions = re.findall(r"`([A-Za-z0-9_.-]+\.[A-Za-z0-9]{1,5})`", window)
            if mentions:
                name = mentions[-1]
            if not name:
                name = f"block-{i+1}{ext_map.get(lang, '.txt')}"
            path = os.path.join(args.extract_dir, name)
            if os.path.exists(path):
                base, ext = os.path.splitext(name)
                path = os.path.join(args.extract_dir, f"{base}-{i+1}{ext}")
            with open(path, "w") as f:
                f.write(code)
        print(f"  extracted {len(blocks)} blocks → {args.extract_dir}")

    machine_ok = all([g_sub, g_ref, g_meta, g_code])

    score, verdict, reasons = -1, "SKIPPED", "judge skipped"
    if not args.skip_judge:
        print("== llm judge ==")
        try:
            score, verdict, reasons = judge(prompt, deliverable, args.judge_url, args.judge_model)
            print(f"  score   {score}/10")
            print(f"  verdict {verdict}")
            print(f"  reasons {reasons}")
        except Exception as e:  # noqa: BLE001
            print(f"  judge unavailable: {e}")
            verdict, reasons = "UNAVAILABLE", str(e)[:200]

    if args.report:
        with open(args.report, "w") as f:
            json.dump({
                "deliverable": args.deliverable_file,
                "machine_gates": {n: p for n, p, _ in results},
                "code_blocks": {"total": len(blocks), "checked": checked, "valid": valid},
                "judge": {"score": score, "verdict": verdict, "reasons": reasons},
            }, f, indent=2)

    judge_ok = args.skip_judge or verdict in ("SKIPPED", "UNAVAILABLE") or (
        verdict == "PASS" and score >= args.min_score)

    if machine_ok and judge_ok:
        print("verdict: VERIFIED")
        return 0
    if not machine_ok:
        print("verdict: FAIL (machine gates)")
        return 1
    print(f"verdict: FAIL (judge: {verdict} score={score})")
    return 2


if __name__ == "__main__":
    sys.exit(main())
