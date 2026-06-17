#!/usr/bin/env python3
"""Extract natural language user prompts from OpenCode session DB.

Filters out: system messages, file pastes, logs, YAML content, diffs, short messages.
Outputs clean .md files suitable for meta-workflow baseline testing.

Usage: python3 scripts/extract-prompts.py [OUTPUT_DIR] [MIN_LENGTH]
Default output: docs/plans/meta-workflow-qwen35/test-prompts/real/
Default min length: 1500 chars
"""

import json
import os
import re
import sqlite3
import sys
from datetime import datetime, timezone
from pathlib import Path

DB_PATH = os.environ.get("OPENCODE_DB", os.path.expanduser("~/.local/share/opencode/opencode.db"))

# Patterns that indicate NON-prompt content (system-generated or pasted)
NOISE_PATTERNS = [
    r"^workflow_id:",
    r"^providers:",
    r"^agentic_workflow:",
    r"^<path>",
    r"^<command-instruction>",
    r"^<auto-slash-command>",
    r"^<system-reminder>",
    r"\[SYSTEM DIRECTIVE",
    r"^\[search-mode\]",
    r"^\[analyze-mode\]",
    r"^\s*Compiling ",
    r"^whitt-llama-server",
    r"docker compose",
    r"^Index:",
    r"\.patch",
    r"^\s*warning:",
    r"^\s*error\[",
    r"^   \d+ \|",
    r"^\s*\^\~+",  # rust error carets
    r"^=+$/",  # separator lines
    r"^---$",  # diff separators
    r"^\+\+\+",
    r"^@@",
    r"^diff --git",
    r"data:image/",
    r"data:application/",
    r'"type":\s*"function"',
    r'"parameters":\s*\{',
    r'"\$schema"',
    r"^\s*\".*\":\s*",  # JSON lines
]

COMPILED_NOISE = [re.compile(p, re.MULTILINE) for p in NOISE_PATTERNS]


def is_noise(text: str) -> bool:
    """Check if text matches any noise pattern."""
    for pattern in COMPILED_NOISE:
        if pattern.search(text):
            return True
    # Check ratio of code-like content
    lines = text.split("\n")
    code_lines = sum(1 for l in lines if l.strip().startswith(("-", "+", "*", "|", "#", ">", "{", "}", "[", "]")) or l.strip().endswith((":", ";", ",")))
    if len(lines) > 10 and code_lines / len(lines) > 0.6:
        return True
    return False


def is_agentic(text: str) -> bool:
    """Check if text contains agentic keywords indicating a complex prompt."""
    agentic_keywords = [
        "workflow", "sub-workflow", "build", "implement", "iterate",
        "schema", "plan", "execute", "agent", "hook", "prompt",
        "deconstruct", "translate", "categorize", "assemble",
        "evaluate", "validate", "benchmark", "model", "yaml",
        "rust", "cargo", "test", "deploy", "docker", "llama",
        "qwen", "generate", "pipeline", "loop", "fix",
    ]
    text_lower = text.lower()
    matches = sum(1 for kw in agentic_keywords if kw in text_lower)
    return matches >= 3


def extract_prompts(db_path: str, output_dir: str, min_length: int = 1500):
    """Extract prompts from DB and write to output directory."""
    Path(output_dir).mkdir(parents=True, exist_ok=True)

    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
    cursor = conn.cursor()

    # Query text-type parts from user messages
    query = """
        SELECT DISTINCT
            p.id as part_id,
            p.session_id,
            p.data,
            length(p.data) as data_len
        FROM part p
        JOIN message m ON p.message_id = m.id
        WHERE m.data LIKE '%"role":"user"%'
          AND p.data LIKE '%"type":"text"%'
          AND length(p.data) >= ?
        ORDER BY data_len DESC
        LIMIT 500
    """

    cursor.execute(query, (min_length,))
    rows = cursor.fetchall()

    extracted = []
    count = 0

    for row in rows:
        try:
            data = json.loads(row["data"])
            text = data.get("text", "")
            if not text or len(text) < min_length:
                continue

            # Skip if synthetic/system message
            if data.get("synthetic"):
                continue

            # Skip noise
            if is_noise(text):
                continue

            # Must have agentic content
            if not is_agentic(text):
                continue

            count += 1
            # Generate filename from first meaningful line
            first_line = text.strip().split("\n")[0][:80]
            safe_name = re.sub(r"[^a-zA-Z0-9 -]", "", first_line).strip().replace(" ", "-").lower()[:60]
            if not safe_name:
                safe_name = f"prompt-{count}"
            filename = f"prompt-{count:02d}-{safe_name}.md"
            filepath = os.path.join(output_dir, filename)

            # Write prompt with metadata header
            with open(filepath, "w") as f:
                f.write(f"<!--\n")
                f.write(f"Source Session: {row['session_id']}\n")
                f.write(f"Part ID: {row['part_id']}\n")
                f.write(f"Character Count: {len(text)}\n")
                f.write(f"Extracted: {datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')}\n")
                f.write(f"-->\n\n")
                f.write(text)

            extracted.append({
                "num": count,
                "filename": filename,
                "chars": len(text),
                "session": row["session_id"],
                "preview": first_line,
            })

            if count >= 15:  # Limit to 15 prompts
                break

        except (json.JSONDecodeError, KeyError):
            continue

    conn.close()

    # Write index
    index_path = os.path.join(output_dir, "INDEX.md")
    with open(index_path, "w") as f:
        f.write("# Extracted Prompt Dataset\n\n")
        f.write(f"Extracted from OpenCode session database on {datetime.now(timezone.utc).strftime('%Y-%m-%d')}\n")
        f.write(f"Total prompts: {len(extracted)}\n\n")
        f.write("| # | File | Chars | Session | Preview |\n")
        f.write("|---|------|-------|---------|---------|\n")
        for p in extracted:
            preview = p["preview"][:50].replace("|", "\\|")
            f.write(f"| {p['num']} | {p['filename']} | {p['chars']} | {p['session'][:20]}... | {preview}... |\n")

    print(f"Extracted {len(extracted)} prompts to {output_dir}")
    for p in extracted:
        print(f"  [{p['num']:02d}] {p['filename']} ({p['chars']} chars) — {p['preview'][:60]}")

    return extracted


if __name__ == "__main__":
    output_dir = sys.argv[1] if len(sys.argv) > 1 else "docs/plans/meta-workflow-qwen35/test-prompts/real"
    min_length = int(sys.argv[2]) if len(sys.argv) > 2 else 1500
    extract_prompts(DB_PATH, output_dir, min_length)
