#!/usr/bin/env python3
"""Format fixer: applies deterministic corrections to LLM output.
Reads artifact, applies fix rules from criteria YAML, writes fixed version.

Usage: fix-format.py --artifact FILE --criteria YAML --out FILE
Output: fixed text to stdout, written to --out if specified.
"""
import argparse, json, re, sys, yaml
from pathlib import Path


def apply_fixes(text, criteria):
    fc = criteria.get("format_fixes", {})
    original = text

    for pat in fc.get("strip_patterns", []):
        text = re.sub(pat, "", text)

    for old, new in fc.get("replace_pairs", {}).items():
        text = text.replace(old, new)

    if fc.get("strip_leading_whitespace"):
        lines = text.splitlines()
        text = "\n".join(l.lstrip() for l in lines)

    if fc.get("strip_trailing_whitespace"):
        lines = text.splitlines()
        text = "\n".join(l.rstrip() for l in lines)

    if fc.get("deduplicate_blank_lines"):
        text = re.sub(r"\n{3,}", "\n\n", text)

    if fc.get("ensure_trailing_newline"):
        text = text.rstrip() + "\n"

    if fc.get("strip_markdown_fences"):
        text = re.sub(r"^```\w*\n?", "", text, flags=re.MULTILINE)
        text = re.sub(r"\n?```\s*$", "", text, flags=re.MULTILINE)

    if fc.get("strip_thinking_tags"):
        text = re.sub(r"<think>.*?</think>", "", text, flags=re.DOTALL)
        text = re.sub(r"</?reasoning>", "", text)

    for pat in fc.get("extract_regex", []):
        m = re.search(pat, text, re.DOTALL)
        if m:
            text = m.group(0) if m.lastindex is None else m.group(1)

    if fc.get("force_json_object"):
        text = text.strip()
        if not text.startswith("{"):
            start = text.find("{")
            if start >= 0:
                end = text.rfind("}") + 1
                text = text[start:end]

    if fc.get("force_yaml_document"):
        text = text.strip()
        if not (text.startswith("---") or re.match(r"^[a-zA-Z_][a-zA-Z0-9_]*:", text)):
            lines = text.splitlines()
            yaml_lines = [l for l in lines if re.match(r"^[a-zA-Z_][a-zA-Z0-9_]*:", l) or l.startswith("  ") or l.startswith("- ")]
            if yaml_lines:
                text = "\n".join(yaml_lines)

    changed = text != original
    return text, changed


def main():
    p = argparse.ArgumentParser(description="Format fixer")
    p.add_argument("--artifact", required=True)
    p.add_argument("--criteria", required=True)
    p.add_argument("--out", help="Output path for fixed text")
    args = p.parse_args()

    text = Path(args.artifact).read_text(encoding="utf-8", errors="replace")
    criteria = {}
    if Path(args.criteria).is_file():
        criteria = yaml.safe_load(Path(args.criteria).read_text()) or {}
    if "format_fixes" not in criteria and "success_criteria" in criteria:
        criteria = criteria["success_criteria"]

    fixed, changed = apply_fixes(text, criteria)

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(fixed, encoding="utf-8")

    print(fixed)
    sys.exit(0 if not changed else 0)


if __name__ == "__main__":
    main()