#!/usr/bin/env python3
"""LLM-as-judge automated reviewer — replaces human sampling per user directive.

Per user direction 2026-08-06: 'Automated via LLM-as-judge' replaces the
N=2 human sampling protocol in human-sampling-protocol.md. This script is
the primary reviewer; human sampling is deprecated.

Single-question rubric: 'Does this deliverable accomplish the prompt's stated
objective? Answer PASS / PARTIAL / FAIL with one-sentence justification.'

Calls local Qwen3-5-9B via llama.cpp server (same as workflow engine). Uses
chat completion API at http://localhost:8080/v1/chat/completions.

Usage: llm-judge.py <prompt-file> <deliverable-file>
Output: JSON to stdout: {verdict: PASS|PARTIAL|FAIL, justification: str,
                         elapsed_ms: int, model: str}
Exit: 0 if PASS, 1 if PARTIAL or FAIL, 2 on error
"""

import json
import os
import sys
import time
import urllib.request
import urllib.error
from pathlib import Path

SERVER_URL = os.environ.get("WHITT_LLM_SERVER", "http://localhost:8080")
MODEL_NAME = os.environ.get("WHITT_LLM_MODEL", "Qwen3-5-9B-Q4_K_M")
MAX_TOKENS = 200
TIMEOUT_SEC = 120

JUDGE_PROMPT_TEMPLATE = """You are an objective reviewer evaluating whether a deliverable accomplishes the stated objective.

OBJECTIVE (the prompt that was given):
---
{prompt}
---

DELIVERABLE (what was produced):
---
{deliverable}
---

Answer ONE question: Does the deliverable accomplish the objective's stated goal?

Respond with EXACTLY this format (no other text):
VERDICT: <PASS|PARTIAL|FAIL>
JUSTIFICATION: <one sentence>

PASS = deliverable fully addresses objective
PARTIAL = addresses some aspects but misses key elements
FAIL = does not address objective, off-topic, or refusal
"""


def call_llm(prompt: str) -> dict:
    payload = {
        "model": MODEL_NAME,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": MAX_TOKENS,
        "temperature": 0.1,
        "stream": False,
    }
    body = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        f"{SERVER_URL}/v1/chat/completions",
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=TIMEOUT_SEC) as resp:
        return json.loads(resp.read().decode("utf-8"))


def parse_verdict(raw: str) -> tuple[str, str]:
    verdict = "FAIL"
    justification = "no parseable verdict in response"
    for line in raw.splitlines():
        stripped = line.strip()
        if stripped.upper().startswith("VERDICT:"):
            v = stripped[8:].strip().upper()
            if v in ("PASS", "PARTIAL", "FAIL"):
                verdict = v
        elif stripped.upper().startswith("JUSTIFICATION:"):
            justification = stripped[14:].strip()
    return verdict, justification


def main() -> int:
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <prompt-file> <deliverable-file>", file=sys.stderr)
        return 2

    prompt_path = Path(sys.argv[1])
    deliverable_path = Path(sys.argv[2])

    if not prompt_path.is_file():
        print(json.dumps({"error": f"prompt file missing: {prompt_path}"}))
        return 2
    if not deliverable_path.is_file():
        print(json.dumps({"error": f"deliverable file missing: {deliverable_path}"}))
        return 2

    prompt_text = prompt_path.read_text(encoding="utf-8", errors="replace")
    deliverable_text = deliverable_path.read_text(encoding="utf-8", errors="replace")

    judge_prompt = JUDGE_PROMPT_TEMPLATE.format(
        prompt=prompt_text[:4000],
        deliverable=deliverable_text[:8000],
    )

    t0 = time.monotonic()
    try:
        response = call_llm(judge_prompt)
    except urllib.error.URLError as e:
        print(json.dumps({"error": f"LLM server unreachable: {e}"}))
        return 2
    except Exception as e:
        print(json.dumps({"error": f"LLM call failed: {type(e).__name__}: {e}"}))
        return 2
    elapsed_ms = int((time.monotonic() - t0) * 1000)

    try:
        raw = response["choices"][0]["message"]["content"]
    except (KeyError, IndexError):
        print(json.dumps({"error": "malformed LLM response", "raw": response}))
        return 2

    verdict, justification = parse_verdict(raw)

    result = {
        "verdict": verdict,
        "justification": justification,
        "elapsed_ms": elapsed_ms,
        "model": MODEL_NAME,
        "prompt_file": str(prompt_path),
        "deliverable_file": str(deliverable_path),
        "raw_response": raw,
    }
    print(json.dumps(result, indent=2))
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
