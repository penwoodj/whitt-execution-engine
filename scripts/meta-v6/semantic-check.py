#!/usr/bin/env python3
"""Semantic alignment cross-validator for meta-workflow outputs.

Addresses Goodhart's Law in parity-check.sh:
  - C2 "well-formed" was shallow grep
  - C4 "substantive" was byte/line count (lorem ipsum passes)
  - C5 refusals was fixed 5-pattern regex (paraphrase evasion)
  - No semantic alignment check existed

This script scores deliverable alignment with prompt objective via 3 heuristics:
  1. Keyword overlap: extract content words from prompt, check >=30% appear in deliverable
  2. Structure match: prompt asks for list/code/summary -> check deliverable has matching shape
  3. Refusal paraphrase: broader pattern set than parity-check.sh C5

Scope: minimal, fast (<1s per check). LLM-as-judge reserved for human sampling protocol.

Usage: semantic-check.py <prompt-file> <deliverable-file>
Output: JSON to stdout: {"score": N, "max": 10, "keyword_overlap": float,
                         "structure_match": bool, "refusal_broad_count": int,
                         "reasons": [str, ...]}
Exit: 0 if score >= 6 (PASS), 1 otherwise (FAIL)
"""

import json
import re
import sys
from pathlib import Path

STOPWORDS = frozenset({
    "a", "an", "the", "and", "or", "but", "is", "are", "was", "were",
    "be", "been", "being", "have", "has", "had", "do", "does", "did",
    "will", "would", "could", "should", "may", "might", "must",
    "to", "of", "in", "on", "at", "by", "for", "with", "about", "against",
    "between", "into", "through", "during", "before", "after", "above", "below",
    "from", "up", "down", "out", "off", "over", "under", "again", "further",
    "then", "once", "here", "there", "when", "where", "why", "how",
    "all", "each", "few", "more", "most", "other", "some", "such",
    "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very",
    "s", "t", "can", "just", "don", "should", "now", "i", "you", "he", "she",
    "it", "we", "they", "me", "him", "her", "us", "them",
    "what", "which", "who", "whom", "this", "that", "these", "those",
    "am", "as", "if", "because", "while", "of", "the",
    "use", "using", "used", "get", "got", "make", "made", "go", "went",
    "one", "two", "also", "very", "via", "per", "etc",
})

REFUSAL_PATTERNS_BROAD = [
    r"\bI cannot\b", r"\bI can't\b", r"\bI'm unable\b", r"\bI am unable\b",
    r"\bI don't have access\b", r"\bas an AI\b", r"\bas a language model\b",
    r"\bI would need to (?:read|access|see)\b",
    r"\bunfortunately,? I(?:'m| am)? (?:can't|cannot|unable)\b",
    r"\bexceeds? my (?:current )?capabilities\b",
    r"\bnot able to (?:provide|help|assist)\b",
    r"\boutside (?:the scope of |my )?(?:what I|that I)\b",
    r"\bbeyond (?:what|my) (?:I|capabilities|training)\b",
    r"\bI(?:'m| am)?:? sorry,? (?:but )?I (?:can't|cannot|am unable)\b",
    r"\bI (?:don't|do not) have (?:the ability|access)\b",
    r"\bthat(?:'s| is) (?:not|beyond) (?:something )?I\b",
    r"\bI'd need to (?:read|access|see|examine)\b",
    r"\bI (?:would|will) need (?:access|the actual|to read)\b",
    r"\bcan't actually\b",
    r"\bI don't (?:actually )?have\b",
]


def extract_content_words(text: str) -> set[str]:
    """Extract lowercase content words (length > 3, not stopwords)."""
    tokens = re.findall(r"[a-zA-Z][a-zA-Z_-]{2,}", text.lower())
    return {t for t in tokens if t not in STOPWORDS and len(t) > 3}


def detect_prompt_structure(prompt: str) -> list[str]:
    """Detect structural requirements implied by prompt language."""
    structures = []
    lower = prompt.lower()
    if re.search(r"\blist\b|\bbenefits\b|\bsteps\b|\bitems\b|\boptions\b", lower):
        structures.append("list")
    if re.search(r"\bcode\b|\bfunction\b|\bclass\b|\bimplement\b|\bscript\b|\bprogram\b", lower):
        structures.append("code")
    if re.search(r"\bsummary\b|\bsummarize\b|\bbrief\b|\babstract\b", lower):
        structures.append("summary")
    if re.search(r"\bplan\b|\bstrategy\b|\bapproach\b|\broadmap\b", lower):
        structures.append("plan")
    if re.search(r"\bcompar(?:e|ison)\b|\bversus\b|\bvs\.?\b|\bdifference\b", lower):
        structures.append("comparison")
    if re.search(r"\banalyz(?:e|is)\b|\bbreakdown\b|\bevaluat(?:e|ion)\b", lower):
        structures.append("analysis")
    return structures


def check_structure_match(deliverable: str, structures: list[str]) -> bool:
    """Verify deliverable exhibits at least one expected structural element."""
    if not structures:
        return True
    for s in structures:
        if s == "list":
            if re.search(r"^(?:\s*[-*+]\s|\s*\d+\.\s)", deliverable, re.MULTILINE):
                return True
        elif s == "code":
            if re.search(r"^```|^\s{4,}\w|^def |^func |^class |^import ", deliverable, re.MULTILINE):
                return True
        elif s == "summary":
            if re.search(r"^#+\s", deliverable, re.MULTILINE) or len(deliverable.split("\n\n")) >= 3:
                return True
        elif s == "plan":
            if re.search(r"^#+\s|^\s*(?:phase|stage|step)\s*\d+", deliverable, re.MULTILINE | re.IGNORECASE):
                return True
        elif s == "comparison":
            if re.search(r"\|.*\|.*\|^\s*vs\.?\s", deliverable, re.MULTILINE):
                return True
        elif s == "analysis":
            if re.search(r"^#+\s|because|therefore|consequently", deliverable, re.MULTILINE | re.IGNORECASE):
                return True
    return False


def count_broad_refusals(deliverable: str) -> int:
    """Count matches against broadened refusal pattern set."""
    count = 0
    for pattern in REFUSAL_PATTERNS_BROAD:
        matches = re.findall(pattern, deliverable, re.IGNORECASE)
        count += len(matches)
    return count


def score_deliverable(prompt: str, deliverable: str) -> dict:
    """Score semantic alignment. Returns dict with score 0-10 and reasons."""
    prompt_words = extract_content_words(prompt)
    deliverable_lower = deliverable.lower()
    matched = sum(1 for w in prompt_words if w in deliverable_lower)
    overlap_ratio = (matched / len(prompt_words)) if prompt_words else 0.0

    structures = detect_prompt_structure(prompt)
    structure_ok = check_structure_match(deliverable, structures)

    refusal_count = count_broad_refusals(deliverable)

    score = 0
    reasons = []

    if overlap_ratio >= 0.30:
        score += 4
        reasons.append(f"keyword_overlap={overlap_ratio:.2f} PASS (>=0.30)")
    elif overlap_ratio >= 0.15:
        score += 2
        reasons.append(f"keyword_overlap={overlap_ratio:.2f} PARTIAL (>=0.15)")
    else:
        reasons.append(f"keyword_overlap={overlap_ratio:.2f} FAIL (<0.15)")

    if structure_ok:
        score += 3
        reasons.append(f"structure_match PASS (expected={structures})")
    else:
        reasons.append(f"structure_match FAIL (expected={structures})")

    if refusal_count == 0:
        score += 3
        reasons.append("refusal_broad=0 PASS")
    elif refusal_count <= 1:
        score += 1
        reasons.append(f"refusal_broad={refusal_count} PARTIAL")
    else:
        reasons.append(f"refusal_broad={refusal_count} FAIL")

    if overlap_ratio < 0.10:
        score = min(score, 3)
        reasons.append("HARD_CAP: keyword_overlap < 0.10 — likely off-topic")

    return {
        "score": score,
        "max": 10,
        "keyword_overlap": round(overlap_ratio, 3),
        "structure_match": structure_ok,
        "structures_expected": structures,
        "refusal_broad_count": refusal_count,
        "reasons": reasons,
    }


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

    result = score_deliverable(prompt_text, deliverable_text)
    result["prompt_file"] = str(prompt_path)
    result["deliverable_file"] = str(deliverable_path)
    result["verdict"] = "PASS" if result["score"] >= 6 else "FAIL"

    print(json.dumps(result, indent=2))
    return 0 if result["score"] >= 6 else 1


if __name__ == "__main__":
    sys.exit(main())
