#!/usr/bin/env python3
"""Analyzer: score case run.

Reads all angle outputs + check results, computes:
- per-angle: passed?, char_delta vs prior angle, semantic_score (heuristic)
- best_angle: first angle whose deterministic check passed
- corrected_text: output from best_angle (or final if none passed)
- case_passed: bool (did best_angle pass?)
- score_0_100: weighted score for aggregation

Usage: analyze.py <case.yml> <output_dir>
Writes analyze.json to output_dir.
"""
import json
import os
import sys
from pathlib import Path

import yaml


ANGLE_FILES = [
    (1, "after-angle-1.txt", "check-angle-1.json"),
    (2, "after-angle-2.txt", "check-angle-2.json"),
    (3, "after-angle-3.txt", "check-angle-3.json"),
    (4, "after-angle-4.txt", "check-angle-4.json"),
    (5, "after-angle-5.txt", "check-angle-5.json"),
]


def levenshtein(a: str, b: str) -> int:
    if len(a) < len(b):
        return levenshtein(b, a)
    if len(b) == 0:
        return len(a)
    prev = list(range(len(b) + 1))
    for i, ca in enumerate(a):
        cur = [i + 1]
        for j, cb in enumerate(b):
            cur.append(min(prev[j + 1] + 1, cur[j] + 1, prev[j] + (ca != cb)))
        prev = cur
    return prev[-1]


def semantic_heuristic(broken: str, candidate: str, case: dict) -> dict:
    """Heuristic 0-100 score. Penalize: too similar to broken (no fix),
    too different (degenerate rewrite), missing required phrases."""
    broken_clean = broken.strip()
    cand_clean = candidate.strip()
    if not cand_clean:
        return {"score": 0, "reason": "empty"}

    dist = levenshtein(broken_clean.lower(), cand_clean.lower())
    max_len = max(len(broken_clean), len(cand_clean), 1)
    similarity = 1.0 - (dist / max_len)

    if similarity > 0.98:
        return {"score": 10, "reason": f"near-identical to broken ({similarity:.2f})"}

    expected_phrases = case.get("expected_fix_description", "").lower()
    has_forbidden = any(
        p.lower() in cand_clean.lower()
        for p in case.get("deterministic_checks", {}).get("forbidden_phrases", [])
    )
    if has_forbidden:
        return {"score": 20, "reason": "contains forbidden phrase"}

    checks = case.get("deterministic_checks", {})
    bullets = sum(1 for ln in cand_clean.splitlines() if ln.strip().startswith(("-", "*", "•")))
    bmin = checks.get("bullet_count_min", 0)
    bmax = checks.get("bullet_count_max", 9999)
    bullet_ok = bmin <= bullets <= bmax
    wc = len(cand_clean.split())
    wmin = checks.get("min_words", 0)
    wmax = checks.get("max_words", 99999)
    word_ok = wmin <= wc <= wmax

    score = 50
    if bullet_ok:
        score += 25
    if word_ok:
        score += 15
    if 0.3 < similarity < 0.85:
        score += 10

    return {
        "score": min(score, 100),
        "reason": f"bullets_ok={bullet_ok}, words_ok={word_ok}, sim={similarity:.2f}",
    }


def main():
    if len(sys.argv) < 3:
        print("usage: analyze.py <case.yml> <output_dir>", file=sys.stderr)
        sys.exit(2)
    case_path = Path(sys.argv[1])
    outdir = Path(sys.argv[2])

    with open(case_path) as f:
        case = yaml.safe_load(f)

    broken = case.get("broken_output", "")

    angles_data = []
    first_passing = None
    for num, txt_name, chk_name in ANGLE_FILES:
        txt_path = outdir / txt_name
        chk_path = outdir / chk_name
        entry = {"angle": num, "file": txt_name}
        if not txt_path.exists():
            entry["present"] = False
            entry["passed"] = False
            angles_data.append(entry)
            continue
        text = txt_path.read_text()
        entry["present"] = True
        entry["text_excerpt"] = text.strip()[:200]
        entry["char_count"] = len(text)
        entry["word_count"] = len(text.split())
        if chk_path.exists():
            try:
                chk = json.loads(chk_path.read_text())
                entry["passed"] = bool(chk.get("passed", False))
                entry["checks"] = chk.get("checks", {})
            except Exception as e:
                entry["passed"] = False
                entry["check_error"] = str(e)
        else:
            entry["passed"] = False
            entry["checks"] = "missing"
        sem = semantic_heuristic(broken, text, case)
        entry["semantic_score"] = sem["score"]
        entry["semantic_reason"] = sem["reason"]
        if entry["passed"] and first_passing is None:
            first_passing = num
        angles_data.append(entry)

    if first_passing is not None:
        best_text = (outdir / ANGLE_FILES[first_passing - 1][1]).read_text()
        best_angle = first_passing
        case_passed = True
        best_sem = angles_data[first_passing - 1]["semantic_score"]
    else:
        # Fall back to highest semantic score
        ranked = sorted(angles_data, key=lambda a: a.get("semantic_score", 0), reverse=True)
        best_text = (outdir / ANGLE_FILES[ranked[0]["angle"] - 1][1]).read_text() if ranked and ranked[0].get("present") else ""
        best_angle = ranked[0]["angle"] if ranked else None
        case_passed = False
        best_sem = ranked[0].get("semantic_score", 0) if ranked else 0

    result = {
        "case_id": case.get("case_id"),
        "issue_type": case.get("issue_type"),
        "case_passed": case_passed,
        "best_angle": best_angle,
        "best_semantic_score": best_sem,
        "corrected_text": best_text,
        "angles": angles_data,
    }

    out_path = outdir / "analyze.json"
    with open(out_path, "w") as f:
        json.dump(result, f, indent=2)

    print(json.dumps({
        "case_id": result["case_id"],
        "case_passed": result["case_passed"],
        "best_angle": result["best_angle"],
        "best_semantic_score": result["best_semantic_score"],
    }, indent=2))


if __name__ == "__main__":
    main()
