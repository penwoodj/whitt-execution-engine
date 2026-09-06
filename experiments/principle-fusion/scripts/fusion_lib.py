#!/usr/bin/env python3
"""Shared library for the principle-fusion experiment (zero LLM).

Provides: digest slicing (E10/P8), check-before-echo harvester (N2),
logprob-entropy confidence router (N3), ledgers, leak-safe failure
hint formatting (N6), proven style prefixes (P1 — byte-exact).
"""
import datetime
import hashlib
import json
import math
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / "experiments/reasoning-enhancer/scripts"))
from check_lib import run_checks  # noqa: E402

# --- P1: byte-exact proven prefixes. NEVER reword (v14c lesson). ---
STYLE_PREFIX = {
    "plan": "PLAN FIRST: list steps, each step one line with its "
            "formula or rule. No answers yet.",
    "cot": "Think step by step through every rule, then answer.",
    "replan": "RE-PLAN: prior plan did not survive the checks. List "
              "new steps, one line each, formulas first. No answers yet.",
    "wait": "Wait. Prior attempt near-miss. Recheck each rule "
            "against your answer. Fix wrong part. Output FINAL "
            "answer only.",
    "verify": "BACKWARD CHECK: test each stated rule against the "
              "answer, one line per rule, pass or fail. Fix any "
              "fail. Then output FINAL answer only.",
    "extract": "From any material above, output ONLY the final "
               "answer in the exact required form.",
    "judge": "You are a blind reviewer. Score the answer against "
             "the task contract only. Output ONLY this JSON: "
             '{"verdict": "pass"} or {"verdict": "fail"}',
}

EXPANSION_MARKER = "and to say the rules one more time"
SPEC_TAIL_MARKER = "before committing, recheck each number"


def word_count(text):
    return len(text.split())


WORKED_MARK = "for orientation only"
CONTRACT_MARK = "then output ONLY this JSON"


def digest(prompt):
    """E10/P8: keep core + output contract, strip expansion boilerplate
    and the worked example.

    Slice at EXPANSION_MARKER (expansion start), drop the worked
    example span (WORKED_MARK -> EXPANSION_MARKER; orientation story,
    not task facts — N8 disjointness keeps it non-confusing when the
    full prompt IS shown to the first solve pass), re-attach from
    CONTRACT_MARK (output shape). DOS-RAG structure preservation:
    order of kept segments is the original document order.
    """
    flat = re.sub(r"\s+", " ", prompt).strip()
    m_at = flat.find(EXPANSION_MARKER)
    if m_at == -1:
        return flat
    w_at = flat.find(WORKED_MARK)
    if w_at != -1 and w_at < m_at:
        core = flat[:w_at].strip()
    else:
        core = flat[:m_at].strip()
    c_at = flat.rfind(CONTRACT_MARK)
    if c_at != -1 and c_at > m_at:
        core += "\n\n" + flat[c_at:]
    return core


def harvest_candidates(prior_text):
    """Yield candidate JSON objects from reasoning text, newest first.

    Reads the TAIL (last 4000 chars — answers live at the end).
    Digit-garble repair: 1_000 -> 1000, 1-000 -> 1000, doubled-word
    dashes collapsed. JSON-anywhere: any balanced {...} span.
    """
    tail = prior_text[-4000:]
    cands = re.findall(r"\{[^{}]*\}", tail)
    for cand in reversed(cands):
        cand = re.sub(r"(\d)[_\-](\d)", r"\1\2", cand)
        cand = re.sub(r"(\w)-\1\b", r"\1", cand)
        yield cand


def harvester(prior_text, checks):
    """N2 check-before-echo: return CLEANED_JSON string iff a
    candidate passes the deterministic checks; else None. The model
    never gets asked to re-emit what already checks green."""
    for cand in harvest_candidates(prior_text):
        try:
            json.loads(cand)
        except Exception:
            continue
        try:
            if run_checks(cand, checks)["passed"]:
                return "CLEANED_JSON:" + cand
        except Exception:
            continue
    return None


def leak_safe_hint(check_json):
    """N6: hint = check id + observed + question. Never embeds the
    expected value. json_exact spec is contract echo (it lives in the
    case prompt itself), values still must be derived."""
    lines = []
    for f in (check_json or {}).get("failures", [])[:6]:
        name = f.get("check", "?")
        detail = f.get("detail", "")[:200]
        lines.append(f"{name}: observed [{detail}] — recheck this "
                     "rule against the task contract")
    return "\n".join(lines)


def entropy_from_logprobs(logprobs):
    """N3: Shannon entropy over first-token logprobs (nats)."""
    if not logprobs:
        return 0.0
    m = max(logprobs)
    ps = [math.exp(x - m) for x in logprobs]
    z = sum(ps)
    if z <= 0:
        return 0.0
    return -sum((p / z) * math.log(p / z + 1e-12) for p in ps)


def conf_route(logprobs, threshold=0.8):
    """N3 router: low entropy -> LIGHT lane, high -> HEAVY."""
    return "HEAVY" if entropy_from_logprobs(logprobs) >= threshold \
        else "LIGHT"


def _append(path, obj):
    p = Path(path)
    p.parent.mkdir(parents=True, exist_ok=True)
    with p.open("a") as f:
        f.write(json.dumps(obj) + "\n")


def ledger_outcome(run_dir, cid, stage, outcome):
    _append(Path(run_dir) / "outcomes.jsonl", {
        "case": cid, "stage": stage, "outcome": outcome,
        "ts": datetime.datetime.utcnow().isoformat() + "Z"})


def ledger_fingerprint(run_dir, cid, stage, prompt_text):
    _append(Path(run_dir) / "fingerprint.jsonl", {
        "case": cid, "stage": stage,
        "sha256": hashlib.sha256(
            f"{cid}|{stage}|{prompt_text}".encode()).hexdigest(),
        "tokens_approx": max(1, len(prompt_text.split()))})


def has_pass(run_dir, cid):
    """Skip-if-won: any passing check for the case."""
    for f in sorted(Path(run_dir).glob(f"check-{cid}-*.json"),
                    key=lambda p: p.stat().st_mtime):
        try:
            if json.loads(f.read_text()).get("passed"):
                return True
        except Exception:
            continue
    return False


def latest_check(run_dir, cid):
    best = None
    latest = None
    for f in sorted(Path(run_dir).glob(f"check-{cid}-*.json"),
                    key=lambda p: p.stat().st_mtime):
        try:
            j = json.loads(f.read_text())
        except Exception:
            continue
        if j.get("passed"):
            best = j
        latest = j
    return best or latest


def two_lane_verdict(det_passed, judge_json):
    """N5: deterministic overrides judge, always. Judge disagreement
    is recorded, never load-bearing."""
    jv = None
    try:
        jv = json.loads(Path(judge_json).read_text()).get("verdict") \
            if judge_json and Path(judge_json).exists() else None
    except Exception:
        jv = None
    return {"det": det_passed, "judge": jv,
            "final": det_passed,
            "judge_disagreed": jv == "fail" and det_passed}
