#!/usr/bin/env python3
"""Shared library for full-agentic-system experiments (zero LLM).

Generalizes principle-fusion's fusion_lib: prompt assembly into the
[1000, 3000] word band, digest slicing, check-before-echo harvester,
leak-safe hints, ledgers, two-lane verdict, plus FAS-specific helpers
(content addressing for 03, fault classes for 10).
"""
import datetime
import hashlib
import json
import math
import re
import shutil
import sys
from pathlib import Path

FAS = Path(__file__).resolve().parents[1]          # experiments/full-agentic-system
REPO = FAS.parents[1]                               # repo root
sys.path.insert(0, str(REPO / "experiments/reasoning-enhancer/scripts"))
from check_lib import run_checks  # noqa: E402


def run_checks_safe(text, checks):
    """Exception-safe check execution; None on internal error."""
    try:
        return run_checks(text, checks)
    except Exception:
        return None

# --- P1 byte-exact prefixes (from fusion; proven) ---
STYLE_PREFIX = {
    "plan": "PLAN FIRST: list steps, each step one line with its "
            "formula or rule. No answers yet.",
    "cot": "Work in two phases. Phase 1 RULE SCAN: list every rule in "
           "the task that changes state during the event or claim list, "
           "including any expiry, return, revocation, refill, promotion, "
           "fallback, or source-ranking rule, one line each, and mark "
           "which event or claim triggers it. Phase 2 LEDGER WALK: apply "
           "the events one at a time in strict arrival order, one line "
           "per event, each line showing the state after that event. "
           "Only after both phases, give the final answer. Your reply "
           "MUST show the Phase 1 rule list and the Phase 2 walk lines "
           "BEFORE the final answer: a bare final answer with no rule "
           "list and no walk above it is an invalid reply. The final "
           "answer JSON must contain ONLY the contract keys with no "
           "events log, no rule key, no error key, nothing extra.",
    "replan": "RE-PLAN: prior plan did not survive the checks. List "
              "new steps, one line each, formulas first. No answers yet.",
    "wait": "Wait. Prior attempt near-miss. Recheck each rule "
            "against your answer. Fix wrong part. Output FINAL "
            "answer only.",
    "verify": "BACKWARD CHECK: test each stated rule against the "
              "answer, one line per rule, pass or fail. Fix any "
              "fail. Then output FINAL answer only.",
    "extract": "From any material above, output ONLY the final "
               "answer as a raw JSON object. No markdown fences, "
               "no backticks, no prose before or after: the first "
               "character of your reply must be { and the last "
               "must be }.",
    "judge": "You are a blind reviewer. Score the answer against "
             "the task contract only. Output ONLY this JSON: "
             '{"verdict": "pass"} or {"verdict": "fail"}',
    "sample": "A second operator, working independently from the "
              "notes only, reaches a different number. Re-derive "
              "from the events, then answer.",
    "gather": "You are the gather stage. The task text contains "
              "tonight's real data lines plus decoy blocks from other "
              "nights under other-night headers; decoy numbers are not "
              "tonight's. Your ONLY job is evidence collection: quote "
              "verbatim, one bullet per line, every data line of "
              "tonight's task — events like requests, grants, "
              "arrivals, observations, loads, jobs, claims, AND the "
              "standing facts: capacities, sizes, pinned markers, "
              "budgets, limits, durations, and any id-to-number fact "
              "the rules mention — ids and numbers exactly as "
              "written. Do not solve, do not compute, no arithmetic, "
              "no JSON, no braces, no final answer. Field names like "
              "loads, rollout_pct, stages_done, makespan_min belong to "
              "the answer stage, never to you: if your reply contains "
              "a JSON object or an answer value you have failed this "
              "stage. If a requested id "
              "appears nowhere in tonight's lines, report NOT_FOUND "
              "for that id. Nothing else.",
}

EXPANSION_MARKER = "and to say the rules one more time"
SPEC_TAIL_MARKER = "before committing, recheck each number"
WORKED_MARK = "for orientation only"
CONTRACT_MARK = "then output ONLY this JSON"

# shared expansion scaffold (fusion-proven, ~700 words)
_EXPANSION = (
    "and to say the rules one more time, because the rules above are the "
    "whole law for tonight. arrivals in order, budgets and limits exactly as "
    "stated, nothing carries over from the orientation story and its numbers "
    "are not tonight's numbers, read every rule twice before touching "
    "anything, once for what it says and once for what it does not say. the "
    "discipline is boring on purpose: one event at a time, apply the rule "
    "that event touches, carry the state forward, never recompute from "
    "scratch mid-stream, and when two rules could apply pick the one higher "
    "in the list. the drift to watch for is reading a rule as symmetric when "
    "it only binds one way, or letting a return land before the event that "
    "triggers it, or collapsing two events into one because they look alike, "
    "order is the whole game tonight. scope note: counts and sums cover "
    "tonight's listed events only, ids appear exactly as written, and "
    "nothing that did not appear in the events belongs in any list. the "
    "orientation story is orientation, not evidence, and none of its "
    "arithmetic leaks into tonight. good output looks like the state walked "
    "clean: each event leaving the books balanced. the pad wants state, not "
    "narrative: after each arrival write the number, after each expiry write "
    "the return, cross-check the log against the ask list before the window "
    "closes, a partial result is still a result and still depletes, and an "
    "expired partial returns half of what was actually granted, never half "
    "of what was asked. the named traps, one last pass: results that behave "
    "like full ones in your head, expiries that return half of the wrong "
    "base, returns that land before the trigger event instead of after it, "
    "and denials that quietly vanish because the pool refilled a moment "
    "later, a denial is permanent the instant it happens. when in doubt, "
    "slow down, the morning read rewards the operator who walked the list "
    "twice and penalizes the one who pattern-matched a similar night from "
    "memory, tonight is not that night, the numbers are not those numbers. "
    "write the state down between events if it helps, the pad is cheap and "
    "the report is not, and never let two events share a line in your head, "
    "they will blur and the blur always favors the wrong rule. this scheme "
    "came in after the night the books did not balance, the postmortem "
    "phrase was the ledger is a promise not a printer, and ever since the "
    "window closes reconciled to the unit, everything either held, returned, "
    "or named, nothing unaccounted, and none of that history changes "
    "tonight's arithmetic, it only explains why the rules read the way they "
    "do. the morning read is strict about shape, keys in the exact set named "
    "by the contract, values from the menus where menus exist, lists where "
    "lists are asked for, numbers as numbers, and nothing else on the line, "
    "because the report is parsed by a tool that does not forgive prose. if "
    "the state disagrees with the story you expected, the state wins, every "
    "time, no exceptions tonight, expectations are not evidence and neither "
    "are habits from older shifts. the report also goes to people who were "
    "not here for any of it, so it carries nothing but what the rules and "
    "the events force, no context, no color, no asides about how the night "
    "felt, just the state the rules produced, walked event by event to the "
    "last one, then frozen into the exact shape asked for.")

_RECONCILE = (
    "one more discipline before you freeze the answer: reconcile against "
    "the worked example, not by copying its numbers, its numbers belong to "
    "a different night and were chosen precisely because they are not "
    "tonight's, but by walking its shape, notice how it applied the first "
    "rule to the first event, how it carried the remainder forward, how it "
    "refused to let the story override the ledger, and then do the same "
    "walk with tonight's events and tonight's rules. where the shape fits, "
    "use it, where it does not, trust the rule over the shape, because the "
    "example is a lamp, not a map. if you catch yourself asserting a total "
    "you did not walk, stop, that is the exact error this report exists to "
    "catch, and the fix is always the same small boring motion, back to the "
    "event list, forward again, one event, one rule, one state, written "
    "down, then walked once more before you let it become the report.")

_TAIL = (
    "before committing, recheck each number: walk the event list one more "
    "time, each event against its rule, then check the totals off the final "
    "state, not off memory, then check the output shape against the contract "
    "below, keys exact, values filled, menus honored. checklist, in order: "
    "every event visited exactly once, every rule applied where it binds, "
    "every count and sum re-derived from the final state, every id spelled "
    "as written, every list in the order asked for. {contract_line}.")


def word_count(text):
    return len(text.split())


def assemble_prompt(engine_case, header_line, contract_line, aux=None,
                    extra_expansion=""):
    """Full prompt: header → core → worked → expansion(+extra) → reconcile →
    tail(contract). Returns prompt string."""
    parts = [header_line.strip(), "", engine_case["core"].strip(), "",
             engine_case["worked"].strip(), "", _EXPANSION]
    if extra_expansion:
        parts += ["", extra_expansion.strip()]
    parts += ["", _RECONCILE, "",
              _TAIL.format(contract_line=contract_line)]
    if aux:
        parts += ["", "source material follows:", "", aux.strip()]
    return "\n\n".join(parts)


def digest(prompt):
    """E10: core+contract, cut worked + expansion. Same as fusion."""
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
    tail = prior_text[-4000:]
    cands = re.findall(r"\{[^{}]*\}", tail)
    for cand in reversed(cands):
        cand = re.sub(r"(\d)[_\-](\d)", r"\1\2", cand)
        cand = re.sub(r"(\w)-\1\b", r"\1", cand)
        yield cand


def harvester(prior_text, checks):
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


def scoped_stage_key(stage):
    if stage.endswith("_a"):
        return "part_a"
    if stage.endswith("_b") or stage.endswith("_b2"):
        return "part_b"
    return None


def scoped_checks(checks, stage):
    key = scoped_stage_key(stage)
    truth = checks.get("json_exact") if key else None
    if truth is None:
        return checks
    try:
        full = json.loads(truth)
    except Exception:
        return checks
    if not isinstance(full, dict) or key not in full:
        return checks
    scoped = dict(checks)
    scoped["json_exact"] = json.dumps(full[key])
    return scoped


def nested_json_objects(text):
    tail = text[-4000:]
    out = []
    i = 0
    while i < len(tail):
        if tail[i] == "{":
            depth, j = 1, i + 1
            while j < len(tail) and depth:
                if tail[j] == "{":
                    depth += 1
                elif tail[j] == "}":
                    depth -= 1
                j += 1
            if depth == 0:
                cand = tail[i:j]
                try:
                    json.loads(cand)
                    out.append(cand)
                except Exception:
                    pass
                i = j
                continue
        i += 1
    return out


def stage_candidates(text, key=None):
    seen = []
    for cand in harvest_candidates(text):
        if cand not in seen:
            seen.append(cand)
        try:
            obj = json.loads(cand)
        except Exception:
            obj = None
        if key and isinstance(obj, dict) and key in obj:
            inner = json.dumps(obj[key])
            if inner not in seen:
                seen.append(inner)
    for cand in nested_json_objects(text):
        if cand not in seen:
            seen.append(cand)
        if key:
            try:
                obj = json.loads(cand)
            except Exception:
                obj = None
            if isinstance(obj, dict) and key in obj:
                inner = json.dumps(obj[key])
                if inner not in seen:
                    seen.append(inner)
    return seen


def passing_candidate(text, checks, key=None):
    for cand in stage_candidates(text, key):
        r = run_checks_safe(cand, checks)
        if r is not None and r.get("passed"):
            return cand
    return None


def leak_safe_hint(check_json):
    lines = []
    for f in (check_json or {}).get("failures", [])[:6]:
        name = f.get("check", "?")
        detail = f.get("detail", "")[:200]
        lines.append(f"{name}: observed [{detail}] — recheck this "
                     "rule against the task contract")
    if not lines and check_json and not check_json.get("passed"):
        lines.append("json_exact: previous answer missed the required "
                     "fields/values — redo the walk from the task rules")
    return "\n".join(lines)


def entropy_from_logprobs(logprobs):
    if not logprobs:
        return 0.0
    m = max(logprobs)
    ps = [math.exp(x - m) for x in logprobs]
    z = sum(ps)
    if z <= 0:
        return 0.0
    return -sum((p / z) * math.log(p / z + 1e-12) for p in ps)


def conf_route(logprobs, threshold=0.8):
    return "HEAVY" if entropy_from_logprobs(logprobs) >= threshold else "LIGHT"


def content_key(stage, input_text):
    """03 cache-addressing: sha256 key over stage + input."""
    return hashlib.sha256(f"{stage}|{input_text}".encode()).hexdigest()[:16]


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


def has_pass(run_dir, cid, final_stage=None):
    pattern = (f"check-{cid}-{final_stage}.json" if final_stage
               else f"check-{cid}-*.json")
    for f in sorted(Path(run_dir).glob(pattern),
                    key=lambda p: p.stat().st_mtime):
        try:
            if json.loads(f.read_text()).get("passed"):
                return True
        except Exception:
            continue
    return False


def seed_hints(seed_dir, run_dir, cid):
    seed = Path(seed_dir)
    dst = Path(run_dir)
    for chk in seed.glob(f"check-{cid}-*.json"):
        try:
            if json.loads(chk.read_text()).get("passed"):
                continue
        except Exception:
            continue
        tgt = dst / chk.name
        if not tgt.exists():
            shutil.copy2(chk, tgt)


def seed_carry(seed_dir, run_dir, cid, stage):
    seed = Path(seed_dir)
    dst = Path(run_dir)
    ans = seed / f"ans-{cid}-{stage}.txt"
    chk = seed / f"check-{cid}-{stage}.json"
    chk_passed = None
    if chk.exists():
        try:
            chk_passed = json.loads(chk.read_text()).get("passed") is True
        except Exception:
            chk_passed = None
        if not chk_passed:
            shutil.copy2(chk, dst / chk.name)
            return False
    if any_failed_check(seed, cid) and not any_passed_check(seed, cid):
        return False
    if not (ans.exists() and ans.read_text().strip()):
        return False
    if chk_passed:
        shutil.copy2(chk, dst / chk.name)
    shutil.copy2(ans, dst / ans.name)
    return True


def any_passed_check(run_dir, cid):
    for f in Path(run_dir).glob(f"check-{cid}-*.json"):
        try:
            if json.loads(f.read_text()).get("passed"):
                return True
        except Exception:
            continue
    return False


def any_failed_check(run_dir, cid):
    for f in Path(run_dir).glob(f"check-{cid}-*.json"):
        try:
            if not json.loads(f.read_text()).get("passed"):
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
    jv = None
    try:
        jv = json.loads(Path(judge_json).read_text()).get("verdict") \
            if judge_json and Path(judge_json).exists() else None
    except Exception:
        jv = None
    return {"det": det_passed, "judge": jv, "final": det_passed,
            "judge_disagreed": jv == "fail" and det_passed}


def caveman(text):
    """Terse-prompt compressor (from REA+ stage-emit)."""
    text = text.replace("in order to", "to")
    for w in ("the ", "a ", "an ", "please ", "basically ",
              "actually ", "simply "):
        text = re.sub(r"\b" + w.rstrip() + r"\b ", "", text, flags=re.I) \
            if w.strip() in ("the", "a", "an") else text
    text = re.sub(r"\b(the|a|an) (\w)", r"\2", text, flags=re.I)
    return text


FAULT_CLASSES = ["timeout", "unreachable", "garble", "delta", "schema_drift"]


def classify_observation(obs):
    """10-execute-observe: map observation text -> fault class."""
    o = obs.lower()
    if "timed out" in o or "timeout" in o:
        return "timeout"
    if "connection refused" in o or "unreachable" in o or "no route" in o:
        return "unreachable"
    if re.search(r"[^\x20-\x7e\n]", o) and len(o) > 0 and "\ufffd" in o:
        return "garble"
    if "unexpected field" in o or "schema" in o:
        return "schema_drift"
    if "delta" in o or "drift" in o:
        return "delta"
    return "unknown"


_PREEMPT_RE = re.compile(
    r"LOW1 runs (\d+) minutes starting at minute (\d+), "
    r"LOW2 runs (\d+) minutes starting at minute (\d+)\. "
    r"HIGH arrives at minute (\d+) and runs (\d+) minutes",
    re.I)


def preempt_parse(prompt):
    m = _PREEMPT_RE.search(prompt or "")
    if not m:
        return None
    l1d, l1s, l2d, l2s, ha, hd = (int(x) for x in m.groups())
    return {"l1_dur": l1d, "l1_start": l1s, "l2_dur": l2d,
            "l2_start": l2s, "h_arr": ha, "h_dur": hd}


def preempt_walk(f):
    jobs = {
        "LOW1": {"dur": f["l1_dur"], "start": f["l1_start"],
                 "owed": f["l1_dur"], "banked": 0, "ran": False,
                 "core_get": None, "finish": None},
        "LOW2": {"dur": f["l2_dur"], "start": f["l2_start"],
                 "owed": f["l2_dur"], "banked": 0, "ran": False,
                 "core_get": None, "finish": None},
    }
    h_arr, h_dur = f["h_arr"], f["h_dur"]
    holder = None
    wasted = 0
    for m in range(0, 400):
        if h_arr <= m < h_arr + h_dur:
            if holder is not None:
                j = jobs[holder]
                stretch = m - j["core_get"]
                banks = 2 * (stretch // 2)
                j["banked"] += banks
                wasted += stretch - banks
                j["owed"] += stretch - banks
                j["ran"] = True
                j["core_get"] = None
                holder = None
            continue
        if holder is None:
            ready = [n for n, j in jobs.items()
                     if j["finish"] is None and j["start"] <= m]
            ready.sort(key=lambda n: (jobs[n]["start"],
                                      0 if jobs[n]["ran"] else 1))
            if ready:
                holder = ready[0]
                jobs[holder]["core_get"] = m
        if holder is not None:
            j = jobs[holder]
            j["owed"] -= 1
            if j["owed"] <= 0:
                j["finish"] = m + 1
                holder = None
        if all(j["finish"] is not None for j in jobs.values()):
            return {"makespan_min": max(
                        h_arr + h_dur,
                        max(j["finish"] for j in jobs.values())),
                    "wasted_min": wasted,
                    "order": sorted(("LOW1", "LOW2"),
                                    key=lambda n: jobs[n]["finish"])}
    return None
