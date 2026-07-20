#!/usr/bin/env python3
"""Repair a generated deliverable's artifacts using functional-test feedback.

Feeds the original prompt, current artifact files, and the failing test
output to the local model; expects corrected complete files back as fenced
code blocks (each preceded by its `filename.ext`); overwrites the artifacts.

Usage:
    repair-deliverable.py <prompt-file> <artifacts-dir> <failure-log-file>
                          [--judge-url URL] [--model ID]

Exit codes: 0 = artifacts rewritten, 1 = nothing usable returned, 3 = usage.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.request

REPAIR_TEMPLATE = """You are a code repair agent. A generated solution FAILED its functional test. Fix it.

ORIGINAL REQUEST:
{prompt}

CURRENT FILES:
{files}

FUNCTIONAL TEST FAILURE OUTPUT:
{failure}

Instructions:
- Diagnose the failure and produce CORRECTED, COMPLETE versions of any file(s) that must change.
- Keep all required behavior from the original request.
- If a test and its implementation disagree on unspecified behavior, make them consistent.
- TARGET ENVIRONMENT: macOS with /bin/bash 3.2 and BSD userland. Shell scripts must NOT use
  `declare -A`, `ps --sort`, `stat -c`, `readarray/mapfile`, or other bash-4/GNU-only features;
  prefer `wc -c` for sizes, `ps aux | sort -k3 -rn` for CPU sorting; `vm_stat` numbers end with
  a period (strip with tr -d '.'). Python is 3.9, standard library only.
- A script must not abort the whole run because one section fails — degrade gracefully and
  still produce its required success output line.
- For EACH corrected file output exactly: the filename in backticks like `name.ext` on its own line, then ONE fenced code block with the full file content.
- Output nothing else. No explanations."""


def call_model(prompt: str, url: str, model: str) -> str:
    body = {
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 8192,
        "temperature": 0.1,
        "reasoning_effort": "none",
    }
    req = urllib.request.Request(
        url.rstrip("/") + "/v1/chat/completions",
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=1800) as resp:
        data = json.load(resp)
    return data["choices"][0]["message"]["content"]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("prompt_file")
    ap.add_argument("artifacts_dir")
    ap.add_argument("failure_log")
    ap.add_argument("--judge-url", default=os.environ.get("WHITT_JUDGE_URL", "http://localhost:1234"))
    ap.add_argument("--model", default=os.environ.get("WHITT_LMSTUDIO_MODEL", "qwen/qwen3.5-9b"))
    args = ap.parse_args()

    prompt = open(args.prompt_file).read()
    failure = open(args.failure_log).read()[-4000:]

    file_sections = []
    for name in sorted(os.listdir(args.artifacts_dir)):
        path = os.path.join(args.artifacts_dir, name)
        if not os.path.isfile(path) or name.endswith(".txt"):
            continue
        content = open(path, errors="replace").read()
        if len(content) > 12000:
            content = content[:12000] + "\n# ...truncated..."
        file_sections.append(f"`{name}`\n```\n{content}\n```")
    if not file_sections:
        print("no artifact files to repair", file=sys.stderr)
        return 1

    print(f"[repair] asking {args.model} to fix {len(file_sections)} file(s)")
    reply = call_model(
        REPAIR_TEMPLATE.format(prompt=prompt[:6000],
                               files="\n\n".join(file_sections),
                               failure=failure),
        args.judge_url, args.model)

    # parse `filename` + fenced block pairs
    pattern = re.compile(
        r"`([A-Za-z0-9_.-]+\.[A-Za-z0-9]{1,5})`[^\n]*\n+```[A-Za-z0-9_+-]*\n(.*?)```",
        re.DOTALL)
    written = 0
    for name, code in pattern.findall(reply):
        safe = os.path.basename(name)
        path = os.path.join(args.artifacts_dir, safe)
        with open(path, "w") as f:
            f.write(code)
        print(f"[repair] rewrote {safe} ({len(code)} bytes)")
        written += 1

    if written == 0:
        print("[repair] model returned no parseable files", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
