"""Shared deterministic check library for correction-atom scripts.

All checks read the `deterministic_checks` block from case YAML and apply
them to candidate text. Each check is independent and composable.
"""
import re
import yaml


PROMPT_LEAK_MARKERS = [
    "text under review",
    "emit corrected",
    "persona:",
    "prior attempt",
    "lens:",
    "recognize:",
    "fix rules:",
    "task spec + auxiliary",
    "first word must match",
    "inputs:",
]


def check_prompt_leak(text: str) -> list:
    lower = text.lower()
    return [m for m in PROMPT_LEAK_MARKERS if m in lower]


STOPWORDS = {
    "a", "an", "the", "and", "or", "of", "to", "in", "on", "for", "with",
    "is", "are", "was", "were", "be", "been", "by", "as", "at", "it",
    "this", "that", "these", "those", "each", "per", "from", "into",
    "after", "before", "below", "above", "more", "than", "then", "when",
}


NUM_WORDS = {
    "one": "1", "two": "2", "three": "3", "four": "4", "five": "5",
    "six": "6", "seven": "7", "eight": "8", "nine": "9", "ten": "10",
    "first": "1st", "second": "2nd", "third": "3rd", "fourth": "4th",
    "fifth": "5th", "sixth": "6th", "seventh": "7th", "eighth": "8th",
    "ninth": "9th", "tenth": "10th",
}


def normalize_numbers(text: str) -> str:
    out = []
    for token in text.split():
        stripped = token.strip(".,;:!?()[]\"'")
        lower = stripped.lower()
        if lower in NUM_WORDS:
            out.append(token.replace(stripped, NUM_WORDS[lower]))
        else:
            out.append(token)
    return " ".join(out)


def check_contains_required(text: str, required: list) -> list:
    norm_text = normalize_numbers(text.lower())
    missing = []
    for r in required:
        norm_r = normalize_numbers(r.lower())
        if r in text or norm_r in norm_text:
            continue
        missing.append(r)
    return missing


def format_check_failure(name: str, c: dict) -> str:
    parts = [name]
    for k in ("value", "found", "missing"):
        if k in c and c[k]:
            parts.append(f"{k}={c[k]}")
    if c.get("error"):
        parts.append(f"error={str(c['error'])[:120]}")
    return ": ".join(parts[:2]) if len(parts) > 1 else name


def fix_hint(name: str, c: dict) -> str:
    if name == "bullet_count":
        lo, hi = c.get("min"), c.get("max")
        target = f"exactly {lo}" if lo == hi else f"{lo}-{hi}"
        return f"restructure as {target} lines each starting with '-'"
    if name == "word_count":
        return f"rewrite so total words fall in range {c.get('min', 0)}-{c.get('max', 99999)} using only source content"
    if name == "forbidden_phrases":
        return f"delete these phrases entirely, with no greeting or closing chatter: {c.get('found', [])}"
    if name == "prompt_leak":
        return "output ONLY the deliverable itself — no instructions, no labels, no meta text from any prompt"
    if name == "contains_required":
        return f"add the missing required content: {c.get('missing', [])}"
    if name == "contains_any":
        return "include at least one of the required options from the task spec"
    if name == "yaml_parsable":
        return "output valid YAML only — no code fences, no prose, no commentary"
    if name == "degenerate":
        return "rewrite using real sentences built from the source material — no repeated tokens, no invented vocabulary"
    return "fix this check"


def content_words(text: str) -> set:
    return {w for w in re.findall(r"[a-z0-9]+", text.lower())
            if w not in STOPWORDS and len(w) > 1}


def check_degenerate(text: str, case: dict) -> dict:
    """Reject word salad and ungrounded invention.

    A candidate passes only if it looks like natural text AND (when the case
    provides source material) reuses source vocabulary — real corrections
    copy content, garbage shares none.
    """
    reasons = []
    words = text.split()
    stripped = text.strip()
    lines = [l for l in text.splitlines() if l.strip()]
    table_lines = sum(1 for l in lines if l.strip().startswith("|"))

    if not stripped:
        return {"passed": False, "reasons": ["empty output"]}

    if any(len(re.sub(r"[^a-zA-Z0-9]", "", w)) > 35 for w in words):
        reasons.append("long_token: word longer than 35 chars (degenerate run-on)")

    if words:
        alpha_ratio = sum(c.isalnum() or c.isspace() for c in text) / len(text)
        if alpha_ratio < 0.5:
            reasons.append(f"low_alpha_ratio: {alpha_ratio:.2f} (symbol soup)")

        if len(words) >= 20 and table_lines < 3:
            uniq = len({w.lower() for w in words}) / len(words)
            if uniq < 0.35:
                reasons.append(f"low_unique_word_ratio: {uniq:.2f} (repetition loop)")

    source_text = " ".join(
        part for part in (case.get("auxiliary", ""), case.get("broken_output", ""))
        if part)
    src = content_words(source_text)
    if len(src) >= 10:
        cand = content_words(text)
        if cand:
            grounding = len(cand & src) / len(cand)
            if grounding < 0.5:
                reasons.append(
                    f"ungrounded: only {grounding:.0%} of content words appear in source")

    return {"passed": not reasons, "reasons": reasons}


def count_bullets(text: str) -> int:
    return sum(1 for ln in text.splitlines() if re.match(r"^\s*[-*•]\s+\S", ln))


def count_numbered(text: str) -> int:
    return sum(1 for ln in text.splitlines() if re.match(r"^\s*\d+\.\s+\S", ln))


def word_count(text: str) -> int:
    return len(text.split())


def check_forbidden(text: str, phrases: list) -> list:
    lower = text.lower()
    return [p for p in phrases if p.lower() in lower]


def check_contains_any(text: str, options: list) -> list:
    return [o for o in options if o in text]


def check_yaml_parsable(text: str) -> tuple:
    try:
        stripped = text.strip()
        if stripped.startswith("```"):
            lines = stripped.splitlines()
            if lines and lines[0].startswith("```"):
                lines = lines[1:]
            if lines and lines[-1].startswith("```"):
                lines = lines[:-1]
            stripped = "\n".join(lines)
        else:
            import re
            fence_match = re.search(r"```(?:ya?ml)?\s*\n(.*?)```", stripped, re.DOTALL)
            if fence_match:
                stripped = fence_match.group(1)
        yaml.safe_load(stripped)
        return True, None
    except Exception as e:
        return False, str(e)


def run_checks_for_angle(angle_num: int, case: dict, text: str) -> dict:
    """Apply deterministic checks from case to text. Returns result dict.

    `angle_num` is recorded for traceability but does not filter checks —
    all configured checks apply on every call. The angle prompts themselves
    determine scope; this verifier just confirms structural invariants.
    """
    checks = case.get("deterministic_checks", {})
    results = {"angle": angle_num, "passed": True, "checks": {}}

    if "bullet_count_min" in checks or "bullet_count_max" in checks:
        bullets = count_bullets(text)
        bmin, bmax = checks.get("bullet_count_min", 0), checks.get("bullet_count_max", 9999)
        ok = bmin <= bullets <= bmax
        results["checks"]["bullet_count"] = {"value": bullets, "min": bmin, "max": bmax, "passed": ok}
        results["passed"] = results["passed"] and ok

    if "max_words" in checks or "min_words" in checks:
        wc = word_count(text)
        wmin, wmax = checks.get("min_words", 0), checks.get("max_words", 99999)
        ok = wmin <= wc <= wmax
        results["checks"]["word_count"] = {"value": wc, "min": wmin, "max": wmax, "passed": ok}
        results["passed"] = results["passed"] and ok

    forbidden = checks.get("forbidden_phrases", [])
    if forbidden:
        hits = check_forbidden(text, forbidden)
        results["checks"]["forbidden_phrases"] = {"found": hits, "passed": len(hits) == 0}
        results["passed"] = results["passed"] and len(hits) == 0

    required = checks.get("contains_required", [])
    if required:
        missing = check_contains_required(text, required)
        results["checks"]["contains_required"] = {"missing": missing, "passed": len(missing) == 0}
        results["passed"] = results["passed"] and len(missing) == 0

    any_of = checks.get("contains_any", [])
    if any_of:
        found = check_contains_any(text, any_of)
        results["checks"]["contains_any"] = {"found": found, "passed": len(found) > 0}
        results["passed"] = results["passed"] and len(found) > 0

    if checks.get("yaml_parsable", False):
        ok, err = check_yaml_parsable(text)
        results["checks"]["yaml_parsable"] = {"passed": ok, "error": err}
        results["passed"] = results["passed"] and ok

    leaks = check_prompt_leak(text)
    results["checks"]["prompt_leak"] = {"found": leaks, "passed": len(leaks) == 0}
    results["passed"] = results["passed"] and len(leaks) == 0

    deg = check_degenerate(text, case)
    results["checks"]["degenerate"] = deg
    results["passed"] = results["passed"] and deg["passed"]

    return results
