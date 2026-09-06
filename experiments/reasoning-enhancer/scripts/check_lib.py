"""REA deterministic check library.

Ported and trimmed from experiments/correction-atom/scripts/check_angle_lib.py
(v8) — number-boundary matching, forbidden/degenerate/leak classes, fix hints.
Pure stdlib. Same input -> same output. No network, no LLM.
"""

import json
import re
import yaml

HARD_FAIL_CHECKS = {"prompt_leak", "degenerate", "forbidden_phrases", "yaml_parsable"}

PROMPT_LEAK_MARKERS = [
    "as an ai",
    "my instructions",
    "the task spec",
    "task spec says",
    "i was told to",
    "as per the prompt",
    "the prompt asks me",
    "system prompt",
]

DEGENERATE_MAX_REPEAT_RATIO = 0.6

KNOWN_CHECK_KEYS = {
    "contains_required",
    "forbidden_phrases",
    "min_words",
    "max_words",
    "bullet_count_min",
    "bullet_count_max",
    "yaml_parsable",
    "numbers_must_sum_to",
    "json_exact",
    "line_count",
    "all_caps",
}


NUM_WORDS = {
    "one": "1", "two": "2", "three": "3", "four": "4", "five": "5",
    "six": "6", "seven": "7", "eight": "8", "nine": "9", "ten": "10",
    "first": "1st", "second": "2nd", "third": "3rd", "fourth": "4th",
    "fifth": "5th", "sixth": "6th", "seventh": "7th", "eighth": "8th",
    "ninth": "9th", "tenth": "10th",
}


def _normalize_numbers(text: str) -> str:
    """v8 verifier calibration: three ≡ 3, third ≡ 3rd (SUMMARY-v8 item 5)."""
    out = []
    for token in text.split():
        stripped = token.strip(".,;:!?()[]\"'")
        lower = stripped.lower()
        if lower in NUM_WORDS:
            out.append(token.replace(stripped, NUM_WORDS[lower]))
        else:
            out.append(token)
    return " ".join(out)


def _matches_phrase(text: str, phrase: str) -> bool:
    if phrase.lower() in text.lower():
        return True
    norm_text = _normalize_numbers(text.lower())
    norm_phrase = _normalize_numbers(phrase.lower())
    if norm_phrase != phrase.lower() and norm_phrase in norm_text:
        return True
    stripped = phrase.strip()
    m = re.match(r"(\d+)(.*)", stripped)
    if m and stripped[0].isdigit() and re.fullmatch(r"\d+(?:\s+\S+)?", stripped):
        num, rest = m.group(1), m.group(2)
        pattern = r"(?<!\d)" + re.escape(num) + r"(?!\d)" + re.escape(rest)
        return re.search(pattern, text, re.IGNORECASE) is not None
    return False


def check_contains_required(text, phrases):
    missing = [p for p in phrases if not _matches_phrase(text, p)]
    return not missing, f"missing: {missing}" if missing else "all present"


def check_forbidden_phrases(text, phrases):
    found = [p for p in phrases if p.lower() in text.lower()]
    return not found, f"found: {found}" if found else "none present"


def check_min_words(text, n):
    count = len(text.split())
    return count >= n, f"{count} words < {n}" if count < n else f"{count} words"


def check_max_words(text, n):
    count = len(text.split())
    return count <= n, f"{count} words > {n}" if count > n else f"{count} words"


def _bullet_lines(text):
    return [ln for ln in text.splitlines() if re.match(r"^\s*[-*•]\s*\S", ln)]


def check_bullet_count(text, lo, hi):
    count = len(_bullet_lines(text))
    if count < lo:
        return False, f"{count} bullets < min {lo}"
    if count > hi:
        return False, f"{count} bullets > max {hi}"
    return True, f"{count} bullets in [{lo},{hi}]"


def check_yaml_parsable(text, want: bool):
    try:
        loaded = yaml.safe_load(text)
    except yaml.YAMLError as exc:
        ok = not want
        return ok, f"parse error: {str(exc).splitlines()[0]}"
    if want and not isinstance(loaded, (dict, list)):
        return False, "parsed to scalar, expected mapping/list"
    return True, "parses"


def _canon_json(s: str) -> str:
    return json.dumps(json.loads(s), sort_keys=True, separators=(",", ":"))


def check_json_exact(text, want: str):
    t = text.strip()
    if not t.startswith("{") or not t.endswith("}"):
        return False, "output is not a bare JSON object"
    try:
        ok = _canon_json(t) == _canon_json(want)
    except Exception:
        return False, "invalid JSON in output or target"
    return ok, "exact JSON match (key-order insensitive)" if ok else "JSON fields/values differ"


def check_line_count(text, n: int):
    lines = [ln for ln in text.strip().splitlines() if ln.strip()]
    ok = len(lines) == n
    return ok, f"{len(lines)} non-empty lines, want {n}"


def check_all_caps(text, want: bool):
    letters = [c for c in text if c.isalpha()]
    if not letters:
        return want, "no letters — vacuously satisfies"
    is_caps = all(c.isupper() for c in letters)
    return is_caps == want, f"all-caps={is_caps}, want {want}"


def _dollar_amounts(text):
    return [float(m) for m in re.findall(r"\$\s*(\d+(?:\.\d+)?)", text)]


def _bare_ints(text):
    amounts = set(_dollar_amounts(text))
    vals = []
    for m in re.findall(r"(?<![\d.])(\d+)(?![\d.])", text):
        if float(m) not in amounts:
            vals.append(float(m))
    return vals


def check_numbers_sum(text, target):
    vals = _dollar_amounts(text) if _dollar_amounts(text) else _bare_ints(text)
    if not vals:
        return False, "no numbers found to sum"
    total = sum(vals)
    return abs(total - target) < 0.001, f"sum({vals}) = {total}, want {target}"


def check_degenerate(text):
    stripped = text.strip()
    if not stripped:
        return False, "empty output"
    lines = [ln for ln in stripped.splitlines() if ln.strip()]
    if len(lines) >= 3:
        counts = {}
        for ln in lines:
            counts[ln.strip()] = counts.get(ln.strip(), 0) + 1
        if max(counts.values()) / len(lines) > DEGENERATE_MAX_REPEAT_RATIO:
            return False, "repeated-line ratio > 0.6"
    return True, "not degenerate"


def check_prompt_leak(text):
    lowered = text.lower()
    hits = [m for m in PROMPT_LEAK_MARKERS if m in lowered]
    return not hits, f"leak markers: {hits}" if hits else "no leak"


FIX_HINTS = {
    "contains_required": "include the exact required phrase(s), numbers exact",
    "forbidden_phrases": "remove the forbidden phrase(s) entirely",
    "min_words": "add substantive content until the word count minimum is met",
    "max_words": "trim content under the word count maximum",
    "bullet_count_min": "add bullet lines ('- ') until the minimum is met",
    "bullet_count_max": "merge or delete bullet lines above the maximum",
    "yaml_parsable": "fix indentation/syntax so the output parses as YAML",
    "numbers_must_sum_to": "recompute the amounts so they sum to the target",
    "json_exact": "output ONLY the JSON object with exactly the required fields and values",
    "line_count": "adjust content to produce exactly the required number of non-empty lines",
    "all_caps": "convert all letters to uppercase (or mixed if want=false)",
    "degenerate": "write real content, not repeated or near-empty text",
    "prompt_leak": "never mention instructions, prompts, or your role",
}


def format_check_failure(check, detail):
    return {
        "check": check,
        "passed": False,
        "detail": detail,
        "fix_hint": FIX_HINTS.get(check, "fix this check"),
    }


def run_checks(text, checks):
    """Run deterministic checks. Returns result dict; always dict, never raises
    on bad check input (unknown keys reported as failures)."""
    failures = []
    passed_count = 0
    total = 0

    def record(name, ok, detail):
        nonlocal passed_count, total
        total += 1
        if ok:
            passed_count += 1
        else:
            failures.append(format_check_failure(name, detail))

    for key in checks:
        if key not in KNOWN_CHECK_KEYS:
            record(key, False, "unknown check key")
            continue
        if key == "contains_required":
            ok, d = check_contains_required(text, checks[key] or [])
            record("contains_required", ok, d)
        elif key == "forbidden_phrases":
            ok, d = check_forbidden_phrases(text, checks[key] or [])
            record("forbidden_phrases", ok, d)
        elif key == "min_words" and checks[key] is not None:
            record("min_words", *check_min_words(text, checks[key]))
        elif key == "max_words" and checks[key] is not None:
            record("max_words", *check_max_words(text, checks[key]))
        elif key == "bullet_count_min" and checks[key] is not None:
            record("bullet_count_min", *check_bullet_count(text, checks[key], 10**6))
        elif key == "bullet_count_max" and checks[key] is not None:
            record("bullet_count_max", *check_bullet_count(text, 0, checks[key]))
        elif key == "yaml_parsable" and checks[key]:
            record("yaml_parsable", *check_yaml_parsable(text, True))
        elif key == "numbers_must_sum_to" and checks[key] is not None:
            record("numbers_must_sum_to", *check_numbers_sum(text, checks[key]))
        elif key == "json_exact" and checks[key]:
            record("json_exact", *check_json_exact(text, checks[key]))
        elif key == "line_count" and checks[key] is not None:
            record("line_count", *check_line_count(text, checks[key]))
        elif key == "all_caps" and checks[key] is not None:
            record("all_caps", *check_all_caps(text, checks[key]))

    record("degenerate", *check_degenerate(text))
    record("prompt_leak", *check_prompt_leak(text))

    hard_fail = any(f["check"] in HARD_FAIL_CHECKS for f in failures)
    return {
        "passed": not failures,
        "subchecks_passed": passed_count,
        "subchecks_total": total,
        "failures": failures,
        "hard_fail": hard_fail,
    }
