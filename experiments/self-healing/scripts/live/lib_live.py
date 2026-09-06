#!/usr/bin/env python3
"""lib_live.py — shared helpers for live-LLM workflow steps (Phase L).

The spoof kit (sh_lib_v2 / agent_run_v2) stays the source of truth for
thresholds, trace ledger, and failure-profile corruption. This module
adds the raw-text glue the live path needs: model output arrives as raw
.txt files written by save_to hooks; we leniently extract JSON, apply
the deterministic failure injection, and emit the same attempt_<n>.json
contract the detectors already consume.
"""
from __future__ import annotations

import html
import json
import random
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import agent_run_v2 as spoof  # noqa: E402
import sh_lib_v2 as lib  # noqa: E402

VARIANT_ROLES = {
    "fast": "worker_fast",
    "coder": "worker_coder",
    "general": "worker_general",
    "heavy": "heavy_replanner",
}
ROLE_IDS = {"worker_fast": 0, "worker_coder": 1, "worker_general": 2, "heavy_replanner": 3}

FENCE_RE = re.compile(r"```(?:json)?\s*(.*?)```", re.DOTALL)
# Real models occasionally emit spec-verbatim bare tokens as values
# (e.g. "plausible_cause": none-known). Quote them so the artifact parses.
BARE_TOKEN_RE = re.compile(r'(:\s*)([A-Za-z][A-Za-z0-9_\-]*)(\s*[,}\n])')


def _repair_bare_tokens(text: str) -> str:
    def repl(m: re.Match) -> str:
        prefix, token, suffix = m.group(1), m.group(2), m.group(3)
        if token in ("true", "false", "null"):
            return m.group(0)
        return f'{prefix}"{token}"{suffix}'
    return BARE_TOKEN_RE.sub(repl, text)


def _repair_extra_data(text: str) -> str:
    """Splice a premature object close: '{...}],}\n,"more": ...}' -> merge tail.

    Models occasionally close the root object early, then continue adding
    top-level keys. json reports 'Extra data' at the splice point; dropping
    that premature '}' and retrying merges the tail back into the object.
    """
    probe = text
    for _ in range(8):
        try:
            json.loads(probe)
            return probe
        except json.JSONDecodeError as e:
            if e.msg != "Extra data":
                return probe
            rest = probe[e.pos:]
            if not rest[:1] in (",", '"', "}"):
                return probe
            cut = e.pos - 1
            if cut < 0 or probe[cut] != "}":
                return probe
            probe = probe[:cut] + probe[e.pos:]
    return probe


# Models occasionally emit a stray bare number as the first "field" of an
# object (e.g. '{\n      1,\n      "capability": ...') — a numbered-item
# marker leaking into the JSON. Turn it into a harmless string-keyed entry.
STRAY_NUMBER_RE = re.compile(r'(\{\s*)(\d+)(\s*,)(?=\s*")')


def _repair_stray_number(text: str) -> str:
    return STRAY_NUMBER_RE.sub(lambda m: f'{m.group(1)}"item_{m.group(2)}": {m.group(2)}{m.group(3)}', text)


def _strip_line_comments(text: str) -> str:
    """Remove `// line` comments that appear OUTSIDE string values.

    State machine tracks in-string + escape so `https://...` inside strings
    survives; a `//` outside a string drops through end-of-line.
    """
    out = []
    in_string = False
    escaped = False
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if ch == "/" and i + 1 < n and text[i + 1] == "/":
            while i < n and text[i] != "\n":
                i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out)


NUMBER_RE = re.compile(r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?")


def _repair_bare_expressions(text: str) -> str:
    """Quote unquoted expression values (e.g. `"factor": 180/π`).

    String-aware scanner: only rewrites value-position tokens after a
    `:` outside strings; numbers/true/false/null pass through untouched.
    """
    out: list[str] = []
    i = 0
    n = len(text)
    in_string = False
    escaped = False
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if ch == ":":
            out.append(ch)
            i += 1
            while i < n and text[i] in " \t":
                out.append(text[i])
                i += 1
            if i < n and text[i] not in '"{[\n':
                j = i
                while j < n and text[j] not in ",}\n":
                    j += 1
                # Thousands-grouped display numbers (~4,100) put commas
                # inside the intended value; absorb ,NNN groups while the
                # group is exactly three digits followed by a non-digit.
                while j < n and text[j] == "," and j + 4 <= n:
                    nxt = text[j + 1:j + 4]
                    after = text[j + 4:j + 5]
                    if nxt.isdigit() and len(nxt) == 3 and not after.isdigit():
                        j += 4
                        while j < n and text[j] not in ",}\n":
                            j += 1
                    else:
                        break
                raw_val = text[i:j].rstrip()
                if raw_val and not NUMBER_RE.fullmatch(raw_val) and raw_val not in ("true", "false", "null"):
                    out.append(json.dumps(raw_val))
                    i = i + len(raw_val)
            continue
        out.append(ch)
        i += 1
    return "".join(out)


SEMICOLON_SEP_RE = re.compile(r'"\s*;\s*\n(?=\s*")')


def _repair_semicolon_separators(text: str) -> str:
    """Models sometimes end JSON lines with `";` instead of `",`."""
    return SEMICOLON_SEP_RE.sub('",\n', text)


def _escape_string_control_chars(text: str) -> str:
    """Escape raw newlines/tabs inside string values (models emit multiline
    markdown as literal control chars, which JSON forbids)."""
    out: list[str] = []
    in_string = False
    escaped = False
    for ch in text:
        if in_string:
            if escaped:
                escaped = False
                out.append(ch)
            elif ch == "\\":
                escaped = True
                out.append(ch)
            elif ch == '"':
                in_string = False
                out.append(ch)
            elif ch == "\n":
                out.append("\\n")
            elif ch == "\r":
                out.append("\\r")
            elif ch == "\t":
                out.append("\\t")
            else:
                out.append(ch)
            continue
        if ch == '"':
            in_string = True
        out.append(ch)
    return "".join(out)


def _repair_trailing_commas(text: str) -> str:
    """Drop commas outside strings that are followed only by whitespace
    and a closing brace/bracket — models leave empty trailing slots."""
    out = []
    i = 0
    n = len(text)
    in_string = False
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if ch == ",":
            j = i + 1
            while j < n and text[j] in " \t\r\n":
                j += 1
            if j < n and text[j] in "}]":
                i += 1  # skip the comma; closing bracket follows
                continue
        out.append(ch)
        i += 1
    return "".join(out)


def _repair_ellipsis(text: str) -> str:
    """Remove literal '...'/'…' tokens models emit when abbreviating
    repeated structures: standalone between items (`},\\n...\\n{`) is
    dropped; as a value (`"k": ...`) becomes null. String-aware."""
    out = []
    i = 0
    n = len(text)
    in_string = False
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if text.startswith("...", i) or text.startswith("…", i):
            j = i + (3 if text.startswith("...", i) else 1)
            # absorb consecutive ellipses + whitespace between them
            while j < n:
                if text[j] in " \t\r\n" or text.startswith("...", j):
                    j += 3 if text.startswith("...", j) else 1
                else:
                    break
            prev = _last_nonspace(out)
            nxt = text[j] if j < n else ""
            if prev == ":":
                out.append("null")
            elif prev in ",[{:" or nxt in ",}]":
                pass
            else:
                out.append(text[i:i + 3])
            i = j
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def _last_nonspace(out: list) -> str:
    for ch in reversed(out):
        if ch not in " \t\r\n":
            return ch
    return ""


def _repair_unkeyed_strings(text: str) -> str:
    """Objects carrying bare members as entry siblings (models annotating
    list-like content inside {}, or values demoted by object-colon repair):
    `..., "some note",` / `..., 1,` with no following ':' — give each a
    synthesized `"_note_N"` key. Array elements are legit and are left
    alone (bracket-stack aware)."""
    out = []
    i = 0
    n = len(text)
    note = 0
    stack = []
    while i < n:
        ch = text[i]
        if ch == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    break
                j += 1
            j += 1
            k = j
            while k < n and text[k] in " \t\r\n":
                k += 1
            prev = _last_nonspace(out)
            in_object = bool(stack) and stack[-1] == "{"
            follows_colon = k < n and text[k] == ":"
            if in_object and prev in (",", "{") and not follows_colon:
                note += 1
                out.append('"_note_%d": ' % note)
            out.append(text[i:j])
            i = j
            continue
        if ch in "-0123456789" or text.startswith(("true", "false", "null"), i):
            j = i
            if ch == "-":
                j += 1
            while j < n and (text[j].isdigit() or text[j] in ".eE+-"):
                j += 1
            if text.startswith(("true", "false", "null"), i):
                j = i + (4 if text[i] == "t" else (5 if text[i] == "f" else 4))
            k = j
            while k < n and text[k] in " \t\r\n":
                k += 1
            prev = _last_nonspace(out)
            in_object = bool(stack) and stack[-1] == "{"
            follows_colon = k < n and text[k] == ":"
            if in_object and prev in (",", "{") and not follows_colon:
                note += 1
                out.append('"_note_%d": ' % note)
            out.append(text[i:j])
            i = j
            continue
        if ch in "{[":
            stack.append(ch)
        elif ch in "}]" and stack:
            stack.pop()
        out.append(ch)
        i += 1
    return "".join(out)


def _repair_bare_array_elements(text: str) -> str:
    """Quote bare expression tokens used as ARRAY elements (e.g. `[1/3, 1/3]`).

    Bracket-stack scanner: only rewrites tokens in element position inside
    arrays (top-of-stack `[`, directly after `[` or `,`); numbers,
    true/false/null and already-quoted strings pass through.
    """
    out: list[str] = []
    stack: list[str] = []
    i = 0
    n = len(text)
    in_string = False
    escaped = False
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if ch in "{[":
            stack.append(ch)
            out.append(ch)
            i += 1
            continue
        if ch in "}]":
            if stack:
                stack.pop()
            out.append(ch)
            i += 1
            continue
        if ch in ",:":
            out.append(ch)
            i += 1
            continue
        top_array = stack and stack[-1] == "["
        if top_array and ch not in " \t\r\n":
            j = i
            while j < n and text[j] not in ",]}\n":
                j += 1
            raw_val = text[i:j].rstrip()
            if raw_val and not raw_val.startswith('"'):
                if NUMBER_RE.fullmatch(raw_val) or raw_val in ("true", "false", "null"):
                    out.append(raw_val)
                    i += len(raw_val)
                else:
                    out.append(json.dumps(raw_val))
                    i += len(raw_val)
                continue
        out.append(ch)
        i += 1
    return "".join(out)



def _repair_mixed_quotes(text: str) -> str:
    """Closing-quote swap: string opened with `"` but closed with `'`
    directly before a bracket delimiter — common LLM emission. Apostrophes
    followed by commas (possessives mid-sentence) are left untouched to
    avoid false closes; only `'` + ws/newline + `]` or `}` converts."""
    out = []
    in_str = False
    i = 0
    while i < len(text):
        ch = text[i]
        if in_str:
            if ch == '\\':
                out.append(ch)
                if i + 1 < len(text):
                    out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_str = False
                out.append(ch)
                i += 1
                continue
            if ch == "'":
                j = i + 1
                while j < len(text) and text[j] in ' \t\r\n':
                    j += 1
                if j < len(text) and text[j] in ']}':
                    out.append('"')
                    in_str = False
                    i += 1
                    continue
            out.append(ch)
            i += 1
        else:
            if ch == '"':
                in_str = True
            out.append(ch)
            i += 1
    return ''.join(out)


def _repair_array_colon(text: str) -> str:
    """Pseudo-map colon inside an array: `["hard" : ["x"]]` — the model
    emits key/value shape without braces. Demotes the colon to a comma so
    the array stays valid (element shape oddity beats unparseable)."""
    out = []
    stack = []
    in_str = False
    i = 0
    while i < len(text):
        ch = text[i]
        if in_str:
            if ch == '\\' and i + 1 < len(text):
                out.append(ch)
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_str = False
            out.append(ch)
            i += 1
            continue
        if ch == '"':
            in_str = True
            out.append(ch)
            i += 1
            continue
        if ch in '[{':
            stack.append(ch)
        elif ch in ']}':
            if stack:
                stack.pop()
        elif ch == ':' and stack and stack[-1] == '[':
            out.append(',')
            i += 1
            continue
        out.append(ch)
        i += 1
    return ''.join(out)


def _repair_object_colon(text: str) -> str:
    """Colon in object separator position: `"type": "soft": "Team"` — the
    model emits value-then-colon instead of comma. Expectation state machine
    (K=expect key, V=expect value, S=expect separator) over parallel
    bracket/state stacks: a colon while the object frame is in S means a
    completed value was followed by ':', which JSON never allows — demote
    to ',' and let unkeyed-strings mop up."""
    out = []
    brackets = []
    states = []  # per open container: 'K' | 'V' | 'S'
    in_str = False
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if in_str:
            if ch == '\\' and i + 1 < n:
                out.append(ch)
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_str = False
                if states and states[-1] in ('V', 'S'):
                    states[-1] = 'S'
            out.append(ch)
            i += 1
            continue
        if ch == '"':
            in_str = True
            out.append(ch)
            i += 1
            continue
        if ch in '[{':
            brackets.append(ch)
            states.append('V' if ch == '[' else 'K')
        elif ch in ']}':
            if brackets:
                brackets.pop()
                states.pop()
            if states:
                states[-1] = 'S'
        elif ch == ':':
            if brackets and brackets[-1] == '{' and states and states[-1] == 'S':
                out.append(',')
                states[-1] = 'K'
                i += 1
                continue
            if states and states[-1] == 'K':
                states[-1] = 'V'
        elif ch == ',':
            if states:
                states[-1] = 'V' if brackets and brackets[-1] == '[' else 'K'
        out.append(ch)
        i += 1
    return ''.join(out)


def _repair_missing_member_commas(text: str) -> str:
    """Adjacent object members with no separating comma: `"a" "b": 1` or
    `"a" {...}` (newline between). On string-close or container-close with
    an object frame open, peek the next non-ws char; a member starter
    ('"', '{', '[') means the comma was omitted — insert it."""
    out = []
    brackets = []
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if ch == '"':
            j = i + 1
            while j < n:
                if text[j] == '\\' and j + 1 < n:
                    j += 2
                    continue
                if text[j] == '"':
                    break
                j += 1
            out.append(text[i:j + 1])
            i = j + 1
            if brackets and brackets[-1] == '{':
                k = i
                while k < n and text[k] in ' \t\r\n':
                    k += 1
                if k < n and text[k] in '"{[':
                    out.append(',')
            continue
        if ch in '[{':
            brackets.append(ch)
            out.append(ch)
            i += 1
            continue
        if ch in ']}':
            if brackets:
                brackets.pop()
            out.append(ch)
            i += 1
            if brackets and brackets[-1] == '{':
                k = i
                while k < n and text[k] in ' \t\r\n':
                    k += 1
                if k < n and text[k] in '"{[':
                    out.append(',')
            continue
        out.append(ch)
        i += 1
    return ''.join(out)


def _repair_closer_mismatch(text: str) -> str:
    """Swap mismatched closers to match the opener stack: '}' where ']' is
    expected (and vice versa) — models habitually close array-member objects
    with ']'. No-op when the stack is already consistent; valid docs never
    reach this stage (fast path returns them untouched)."""
    out = []
    stack = []
    in_str = False
    esc = False
    for ch in text:
        if in_str:
            out.append(ch)
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
            continue
        if ch == '"':
            in_str = True
            out.append(ch)
            continue
        if ch in "{[":
            stack.append(ch)
            out.append(ch)
            continue
        if ch in "}]":
            want = "]" if stack and stack[-1] == "[" else "}"
            out.append(want if ch != want else ch)
            if stack:
                stack.pop()
            continue
        out.append(ch)
    return ''.join(out)


def _repair_trailing_junk(text: str) -> str:
    """Keep only the first complete JSON value — models append stray
    closers/fragments after a valid root object."""
    try:
        _, end = json.JSONDecoder().raw_decode(text)
        return text[:end]
    except json.JSONDecodeError:
        return text


def _try_json(text: str) -> dict | None:
    """Candidate ladder: two pipelines (raw, html-unescaped), each running
    control-char escape → strip-comments → ellipsis → stray-number →
    semicolon → bare-expression → array-element → trailing-comma →
    unkeyed-string repairs, then bare-token and extra-data tails.

    Fast path first: already-valid JSON is returned untouched — repair
    stages are strictly for invalid input and must never see valid bytes
    (mixed-quotes false-closes on values like "['raw', 'processed']")."""
    for pre in (text, html.unescape(text)):
        try:
            obj = json.loads(pre)
            if isinstance(obj, dict):
                return obj
        except json.JSONDecodeError:
            pass
        base = _repair_mixed_quotes(pre)
        base = _escape_string_control_chars(base)
        base = _repair_closer_mismatch(base)
        base = _repair_array_colon(base)
        base = _repair_missing_member_commas(base)
        base = _repair_object_colon(base)
        base = _repair_ellipsis(base)
        base = _repair_stray_number(base)
        base = _strip_line_comments(base)
        base = _repair_semicolon_separators(base)
        base = _repair_bare_expressions(base)
        base = _repair_bare_array_elements(base)
        base = _repair_trailing_commas(base)
        base = _repair_unkeyed_strings(base)
        for candidate in (_repair_trailing_junk(base), base, _repair_bare_tokens(base),
                          _repair_extra_data(base),
                          _repair_extra_data(_repair_bare_tokens(base))):
            try:
                obj = json.loads(candidate)
                if isinstance(obj, dict):
                    return obj
            except json.JSONDecodeError:
                continue
    return None


def extract_json(text: str) -> dict | None:
    """Lenient JSON extraction from raw model output.

    Tries: fenced block -> whole text -> first '{' to last '}', each with a
    bare-token repair pass. Returns None when nothing parses.
    """
    for candidate in (m.group(1) for m in FENCE_RE.finditer(text)):
        obj = _try_json(candidate)
        if obj is not None:
            return obj
    obj = _try_json(text)
    if obj is not None:
        return obj
    lo, hi = text.find("{"), text.rfind("}")
    if lo != -1 and hi > lo:
        return _try_json(text[lo:hi + 1])
    return None


def read_raw(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def parse_attempt(raw_text: str) -> tuple[dict, bool]:
    """Return (attempt_dict, parsed_flag).

    Unparseable output degrades to a shell dict that detectors will read
    as schema-missing (F2-leaning) — mirrors a real worker rambling
    instead of emitting the contract object.
    """
    obj = extract_json(raw_text)
    if obj is None:
        return {"unparsed_text": raw_text[:4000],
                "_meta": {"confidence": 0.12, "self_consistency": 0.10}}, False
    return obj, True


def ensure_meta(attempt: dict, parsed: bool) -> dict:
    """Guarantee a _meta block with clean-band defaults when the model
    omitted one (live models rarely emit our private _meta contract)."""
    meta = attempt.get("_meta")
    if not isinstance(meta, dict):
        meta = {}
        attempt["_meta"] = meta
    meta.setdefault("confidence", 0.78)
    meta.setdefault("self_consistency", 0.85)
    meta.pop("truncated", None)
    meta.setdefault("tool_errors", 0)
    meta.setdefault("contradiction_pairs", [])
    if not parsed:
        meta["confidence"] = 0.12
        meta["self_consistency"] = 0.10
    return attempt


def corrupt(attempt: dict, cls: str, attempt_no: int, case_id: str) -> dict:
    """Apply the deterministic failure profile via the spoof kit's
    inject() so live-injected signatures match detector expectations
    exactly (same code path as Phase S2). The -persist suffix is a
    schedule (fail every round), not a signature — strip it."""
    rng = random.Random(f"{case_id}:{cls}:{attempt_no}")
    return spoof.inject(cls.replace("-persist", ""), attempt, attempt_no, rng)


def worker_prompt(case: dict, round_no: int, run_dir: Path) -> str:
    """Compose the attempt prompt: round 1 = task verbatim; round >= 2 =
    task + corrective context from the previous heal record (when present —
    a judge-reject round has no heal record yet)."""
    task = case["task"]["prompt"]
    schema = case["task"]["expected_schema"]
    required = list(schema["required"])
    properties = schema.get("properties", {})
    all_required_values_are_strings = bool(required) and all(
        properties.get(key, {}).get("type") == "string" for key in required
    )
    output_contract = [
        "Return exactly one JSON object with top-level keys in this order: %s."
        % ", ".join(required),
    ]
    scalar_format_rules: list[str] = []
    if all_required_values_are_strings:
        count_names = {1: "one", 2: "two", 3: "three", 4: "four"}
        value_count = count_names.get(len(required), str(len(required)))
        scalar_format_rules = [
            "Every top-level value must be a JSON string.",
            "Do not emit nested objects, arrays, numbers, booleans, or null.",
            "Each string value must be concise plain prose, never JSON or escaped JSON.",
            "Never place {, }, [, or ] inside any string value.",
            "Start directly with { and end directly with }.",
            "Use standard JSON double-quoted string values only.",
            "Never use Markdown, code fences, triple quotes, or line breaks inside values.",
            "Each value must be one concise single-line plain sentence.",
            "Keep all %s string values concise enough to finish the response."
            % value_count,
        ]
    prompt_prefix = [*output_contract, *scalar_format_rules, "", "TASK:", task, ""]
    if round_no == 1:
        return "\n".join(prompt_prefix)
    heal_path = run_dir / f"heal_{round_no - 1}.json"
    heal = json.loads(heal_path.read_text()) if heal_path.exists() else None
    lines = [
        "RE-ATTEMPT (round %d). Your previous attempt did not pass." % round_no,
        "",
        *prompt_prefix,
    ]
    if heal is None:
        lines.append(
            "CORRECTIVE CONTEXT: automated checks passed but the auditor "
            "rejected the artifact. Re-emit the complete JSON object with "
            "strict attention to the contract's quality bar."
        )
    else:
        lines.append("CORRECTIVE CONTEXT (round %d heal):" % (round_no - 1))
        lines.append("strategy: %s" % heal.get("strategy", "unknown"))
        if heal.get("strategy_text"):
            lines.append("replanner output:")
            lines.append(str(heal["strategy_text"])[:2400])
        instruction = heal.get("instruction") or heal.get("plan") or ""
        if instruction:
            lines.append("instruction: %s" % instruction)
    lines.append(
        "SCHEMA DISCIPLINE: the JSON artifact's top-level keys must be "
        "EXACTLY %r (plus optional %r only if the contract allows). "
        "Renaming, restructuring, or substituting keys counts as failure "
        "even when the content is complete."
        % (required, schema.get("optional", []))
    )
    lines.append(
        "OUTPUT ANCHORING: emit the required keys FIRST, in the exact order "
        "above, as your opening lines — start the object, write each required "
        "key with its full content before introducing anything else. Do not "
        "invent parallel section names; if a section feels like it deserves "
        "its own key, put it INSIDE the matching required key's value."
    )
    if all_required_values_are_strings:
        lines.extend(["", "FINAL SCALAR OUTPUT OVERRIDE:", *output_contract, *scalar_format_rules])
    lines.append("")
    lines.append("Re-attempt now. Return ONLY the JSON artifact per contract, "
                 "no wrapper object, no commentary.")
    return "\n".join(lines)
