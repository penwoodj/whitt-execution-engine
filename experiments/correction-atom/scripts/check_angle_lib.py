"""Shared deterministic check library for correction-atom scripts.

All checks read the `deterministic_checks` block from case YAML and apply
them to candidate text. Each check is independent and composable.
"""
import re
import yaml


def count_bullets(text: str) -> int:
    return sum(1 for ln in text.splitlines() if re.match(r"^\s*[-*•]\s+\S", ln))


def count_numbered(text: str) -> int:
    return sum(1 for ln in text.splitlines() if re.match(r"^\s*\d+\.\s+\S", ln))


def word_count(text: str) -> int:
    return len(text.split())


def check_forbidden(text: str, phrases: list) -> list:
    lower = text.lower()
    return [p for p in phrases if p.lower() in lower]


def check_contains_required(text: str, required: list) -> list:
    return [r for r in required if r not in text]


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

    return results
