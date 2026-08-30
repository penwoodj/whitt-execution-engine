#!/usr/bin/env python3
"""Parse ENTITY table rows from artifact text.

Row format: `ENTITY <id> | <json value>` (one per line, json value may be
number, string, bool, or compact object). Returns {id: value}.
The `ENTITY` keyword is optional: `<id> | <json value>` also accepted
(models reliably emit the id-pipe-value shape but often drop the keyword).
"""
import json
import re

ROW_RE = re.compile(
    r"^\s*(?:ENTITY\s+)?([A-Za-z0-9_]+)\s*\|\s*(.+?)\s*$", re.MULTILINE
)
COLON_RE = re.compile(
    r"^\s*(?:ENTITY\s+)?([A-Za-z0-9_]+)\s*:\s*(\S.*?)\s*$", re.MULTILINE
)


def parse_table(text):
    answers = {}
    explicit = set()
    text = re.sub(r"(?<=\S)\s+(ENTITY\s+)", r"\n\1", text)
    for m in ROW_RE.finditer(text):
        eid, raw = m.group(1), m.group(2)
        if m.group(0).lstrip().startswith("ENTITY"):
            explicit.add(eid)
        try:
            answers[eid] = json.loads(raw)
        except (json.JSONDecodeError, ValueError):
            answers[eid] = raw
    if len(answers) < 2:
        for m in COLON_RE.finditer(text):
            eid, raw = m.group(1), m.group(2)
            if eid.lower() in ("entity", "id"):
                continue
            try:
                answers[eid] = json.loads(raw)
            except (json.JSONDecodeError, ValueError):
                answers[eid] = raw
    return answers


def explicit_ids(text):
    ids = set()
    for m in ROW_RE.finditer(text):
        if m.group(0).lstrip().startswith("ENTITY"):
            ids.add(m.group(1))
    return ids
