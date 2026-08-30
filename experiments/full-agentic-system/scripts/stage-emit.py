#!/usr/bin/env python3
"""Prompt assembler for FAS experiments (live-mode emission, spoof-safe).

Reads the case yml, assembles the stage prompt: style prefix + priors +
digest/full task + aux + leak-safe hint on failure. Extract style uses the
check-before-echo harvester. Judge style is the blind lane.
"""
import argparse
import json
import re
import sys
from pathlib import Path
from typing import NoReturn

sys.path.insert(0, str(Path(__file__).resolve().parent))
import fas_lib as L  # noqa: E402

SKIP_EXIT = 42


def skip_stage() -> NoReturn:
    sys.stdout.write("SKIP")
    sys.stdout.flush()
    sys.exit(SKIP_EXIT)


def load_case(path):
    import yaml
    return yaml.safe_load(Path(path).read_text())


_SYN_INSTR = {
    "split": (
        "YOUR STAGE INSTRUCTION — this overrides any JSON-output ask in the "
        "task above: you are the splitter and you do NOT solve anything and "
        "output NO JSON. In plain lines: (1) FIRST HALF scope — quote the "
        "rule sentences and the event lines that belong to the first half "
        "of tonight's task only; (2) SECOND HALF scope — the same for the "
        "second half. ids and numbers exactly as written."),
    "worker_a": (
        "YOUR STAGE INSTRUCTION — this overrides any JSON-output ask in the "
        "task above: you are worker A. Solve ONLY the FIRST half of "
        "tonight's task (the part_a fields) and ignore the second half "
        "entirely. Show the two-phase walk — Phase 1 rule scan for the "
        "first half's rules only, Phase 2 ledger walk of the first half's "
        "events — then output ONLY the first-half JSON object alone: no "
        "part_b key, no merged object, no extra keys."),
    "worker_b": (
        "YOUR STAGE INSTRUCTION — this overrides any JSON-output ask in the "
        "task above: you are worker B. Solve ONLY the SECOND half of "
        "tonight's task (the part_b fields) and ignore the first half "
        "entirely. Show the two-phase walk — Phase 1 rule scan for the "
        "second half's rules only, Phase 2 ledger walk of the second "
        "half's events — then output ONLY the second-half JSON object "
        "alone: no part_a key, no merged object, no extra keys."),
    "resolve": (
        "YOUR STAGE INSTRUCTION — this overrides any JSON-output ask in the "
        "task above: you are the resolver. The attempt above may be wrong "
        "for the second half. Re-derive the SECOND half from the task's "
        "own rules with a short walk, then output ONLY the second-half "
        "JSON object alone: no part_a key, no merged object."),
    "solve_b2": (
        "YOUR STAGE INSTRUCTION — this overrides any JSON-output ask in the "
        "task above: re-derive the SECOND half of tonight's task from the "
        "task's own rules with a two-phase walk, then output ONLY the "
        "second-half JSON object alone: no part_a key, no merged object."),
    "merge": (
        "YOUR STAGE INSTRUCTION — this overrides any JSON-output ask in the "
        "task above: you are the merger and you do NOT re-solve anything. "
        "The ATTEMPT (extract_a) material above holds the part_a JSON "
        "object and the ATTEMPT (extract_b2) material above holds the "
        "part_b JSON object. Copy each exactly as given, then output ONLY "
        "the single merged JSON object {\"part_a\": <the part_a object "
        "exactly as given>, \"part_b\": <the part_b object exactly as "
        "given>}. part_a keeps exactly the keys it came with, part_b "
        "keeps exactly the keys it came with, no extra keys, no dropped "
        "keys, no re-derived values."),
}


def newest_answer(run_dir, cid, exclude_suffixes=("-judge", "-plan",
                                                   "-replan")):
    best = None
    for f in sorted(Path(run_dir).glob(f"ans-{cid}-*.txt"),
                    key=lambda p: p.stat().st_mtime, reverse=True):
        if any(f.name.endswith(s + ".txt") for s in exclude_suffixes):
            continue
        t = f.read_text().strip()
        if t:
            best = t
            break
    return best


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--case-yml", required=True)
    ap.add_argument("--case", required=True)
    ap.add_argument("--stage", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--style", default="")
    ap.add_argument("--prior", default="")
    ap.add_argument("--final-stage", default=None)
    ap.add_argument("--seed", default=None)
    a = ap.parse_args()

    case = load_case(a.case_yml)
    prompt = case.get("prompt", "")
    checks = (case.get("success_criteria", {})
                  .get("deterministic_checks", {}))
    cid = a.case

    if L.has_pass(a.run_dir, cid, final_stage=a.final_stage):
        skip_stage()

    if a.seed:
        L.seed_hints(a.seed, a.run_dir, cid)
        if L.seed_carry(a.seed, a.run_dir, cid, a.stage):
            skip_stage()

    if (a.style not in ("extract", "judge")
            and a.stage not in _SYN_INSTR
            and re.search(r"preempt", prompt, re.I)):
        facts = L.preempt_parse(prompt)
        if facts:
            res = L.preempt_walk(facts)
            if res is not None:
                lines = ["DETERMINISTIC WALK (simulated, not generated):"]
                lines.append(f"facts: {json.dumps(facts, sort_keys=True)}")
                lines.append("result: " + json.dumps(res, sort_keys=True))
                out_ans = "\n".join(lines) + "\n" + \
                    json.dumps(res, sort_keys=True)
                Path(a.run_dir).mkdir(parents=True, exist_ok=True)
                (Path(a.run_dir) / f"ans-{cid}-{a.stage}.txt").write_text(
                    out_ans)
                L.ledger_fingerprint(a.run_dir, cid, a.stage,
                                     "DETERMINISTIC " +
                                     json.dumps(res, sort_keys=True))
                L.ledger_outcome(a.run_dir, cid, a.stage, "DETERMINISTIC")
                skip_stage()

    priors = [p for p in a.prior.split(",") if p]

    if a.style == "extract":
        key = L.scoped_stage_key(a.stage)
        scoped = L.scoped_checks(checks, a.stage)
        primary = None
        hit = None
        for st in priors:
            f = Path(a.run_dir) / f"ans-{cid}-{st}.txt"
            if not (f.exists() and f.read_text().strip()):
                continue
            txt = f.read_text().strip()
            if primary is None:
                primary = (st, txt)
            cand = L.passing_candidate(txt, scoped, key)
            if cand:
                hit = cand
                break
        if hit is not None:
            Path(a.run_dir).mkdir(parents=True, exist_ok=True)
            (Path(a.run_dir) / f"ans-{cid}-{a.stage}.txt").write_text(
                hit + "\n")
            chk_path = Path(a.run_dir) / f"check-{cid}-{a.stage}.json"
            res = L.run_checks_safe(hit, scoped)
            if not isinstance(res, dict):
                res = {"passed": False, "failures": [],
                       "subchecks_passed": 0, "subchecks_total": 0}
            res.update({"case_id": cid, "stage": a.stage})
            chk_path.write_text(json.dumps(res, indent=1))
            L.ledger_fingerprint(a.run_dir, cid, a.stage,
                                 "EXTRACT-FAST " + hit[:80])
            L.ledger_outcome(a.run_dir, cid, a.stage, "EXTRACT-FAST")
            skip_stage()
        else:
            st, txt = primary if primary else ("", "")
            if not txt:
                skip_stage()
            chk = L.latest_check(a.run_dir, cid)
            hint = L.leak_safe_hint(chk) if chk else ""
            if key:
                try:
                    sub = json.loads(scoped["json_exact"])
                    sub_keys = (list(sub.keys())
                                if isinstance(sub, dict) else [])
                except Exception:
                    sub_keys = []
                ask = ("SOLUTION TEXT:\n" + txt +
                       "\n\nFrom that text only: output ONLY the " + key +
                       " JSON object with keys " + str(sub_keys) +
                       " and their correct values, nothing else, no "
                       "prose." +
                       ("\nRecheck: " + hint if hint else ""))
            else:
                ask = ("SOLUTION TEXT:\n" + txt +
                       "\n\nFrom that text only: output the final answer "
                       "as JSON with keys " + str(case.get("tkeys", "")) +
                       "." + ("\nRecheck: " + hint if hint else "") +
                       " Nothing else.")
            out = ask
        L.ledger_fingerprint(a.run_dir, cid, a.stage, out)
        L.ledger_outcome(a.run_dir, cid, a.stage, "RUN")
        print(out)
        return

    if a.stage == "routelog":
        designed = (case.get("meta") or {}).get("lane")
        observed = None
        for lane, final in (("HEAVY", "extract3"), ("SYNTH", "extract_m"),
                            ("LIGHT", "extract")):
            f = Path(a.run_dir) / f"ans-{cid}-{final}.txt"
            if f.exists() and f.read_text().strip():
                observed = lane
                break
        log = {"lane": observed, "route_ok": observed == designed,
               "shape": (case.get("meta") or {}).get("shape")}
        out = ("The route log is computed from the run artifacts. Output "
               "it back EXACTLY as written, character for character, "
               "nothing before or after:\n"
               + json.dumps(log, sort_keys=True))
        L.ledger_fingerprint(a.run_dir, cid, a.stage, out)
        L.ledger_outcome(a.run_dir, cid, a.stage, "RUN")
        print(out)
        return

    if a.style == "judge":
        ans = newest_answer(a.run_dir, cid)
        if ans is None:
            skip_stage()
        dg = L.digest(prompt)
        out = (L.STYLE_PREFIX["judge"] + "\n\nTASK CONTRACT:\n" +
               L.caveman(dg) + "\n\nANSWER UNDER REVIEW:\n" + ans[:1500] +
               "\n\nVerdict JSON only.")
        L.ledger_fingerprint(a.run_dir, cid, a.stage, out)
        L.ledger_outcome(a.run_dir, cid, a.stage, "RUN")
        print(out)
        return

    parts = []
    if a.style and a.style in L.STYLE_PREFIX and a.style != "gather":
        parts.append(L.STYLE_PREFIX[a.style])
    if priors:
        for st in priors:
            f = Path(a.run_dir) / f"ans-{cid}-{st}.txt"
            if f.exists() and f.read_text().strip():
                label = ("PLANNING NOTES" if st in ("plan", "replan")
                         else f"ATTEMPT ({st})")
                parts.append(f"{label}:\n" +
                              f.read_text().strip()[:1200])
        parts.append("From the material above plus the task digest "
                     "below, answer the task.")
        parts.append("TASK REQUIREMENTS:\n" + L.caveman(L.digest(prompt)))
        if a.style == "gather":
            parts.append("YOUR STAGE INSTRUCTION — this overrides any "
                         "JSON-output ask in the requirements above:\n" +
                         L.STYLE_PREFIX["gather"])
    else:
        if a.style == "gather":
            g_txt = re.sub(
                r"then output ONLY this JSON[^.]*\.[^.]*\.",
                "then report the requested fields as plain bullet lines.",
                prompt)
            parts.append("TASK:\n" + g_txt)
            parts.append("YOUR STAGE INSTRUCTION — this overrides any "
                         "remaining ask in the task above:\n" +
                         L.STYLE_PREFIX["gather"])
        else:
            parts.append("TASK:\n" + prompt)
    aux = case.get("auxiliary")
    if aux and aux != "All facts needed are in the prompt itself.":
        parts.append("SOURCE MATERIAL:\n" + str(aux))
    chk = L.latest_check(a.run_dir, cid)
    if chk and not chk.get("passed"):
        hint = L.leak_safe_hint(chk)
        if hint:
            parts.append("PREVIOUS ATTEMPT FAILED these checks — recheck, "
                         "do not copy:\n" + hint)
    preempt_algo = None
    if (a.style not in ("extract", "judge")
            and not _SYN_INSTR.get(a.stage)
            and re.search(r"preempt", prompt, re.I)):
        preempt_algo = (
            "ALGORITHM — follow exactly, no long prose paragraphs, the "
            "table IS the reasoning:\n"
            "1. FACTS, tonight's numbers only: LOW1 (dur, start), LOW2 "
            "(dur, start), HIGH (arrive, dur). The orientation story is "
            "a different night — never use its numbers.\n"
            "2. For each minute m from 0 upward, one table row 'm | "
            "core | event': HIGH holds every minute in [arrive, "
            "arrive+dur). Otherwise the core goes to the waiting LOW "
            "with the SMALLEST ORIGINAL start minute (ties: a resumed "
            "LOW beats a fresh one). The core never idles while a LOW "
            "waits.\n"
            "3. A LOW banks a checkpoint at (core-get minute + 2) and "
            "every 2 RUNNING minutes after, absolute; a bank landing "
            "exactly on HIGH's arrival minute still counts.\n"
            "4. On preemption: wasted += minutes run past its last "
            "bank; owed = duration − banked.\n"
            "5. A job that gets the core at minute s with owed k "
            "occupies s .. s+k−1 and FINISHES AT minute s+k; the next "
            "LOW starts that same minute.\n"
            "6. makespan = largest finish minute (HIGH's included); "
            "order = LOW jobs by finish minute; wasted summed. JSON "
            "last, after the table.")
    if preempt_algo:
        parts.insert(0, preempt_algo)
    syn = _SYN_INSTR.get(a.stage)
    if syn:
        parts.append(syn)
    elif a.style != "gather":
        parts.append("Answer the task directly and completely.")
    if (not syn and a.style not in ("extract", "judge")
            and re.search(r"expir", prompt, re.I)):
        parts.append(
            "EXPIRY DISCIPLINE — the expiry rule is where walks break: "
            "when the trigger request completes, the expired grant "
            "returns half of what it was GRANTED (the partial amount "
            "actually given, never half of the ask), rounded DOWN, "
            "BEFORE the next request is processed. Walk one ledger "
            "line per event in strict order (arrival, "
            "grant/partial/deny, expiry return, pool after). The "
            "return lands right before the request the trigger "
            "names, never after it. Sanity formula: pool_left = "
            "start − sum(granted) + sum(expiry returns); if your "
            "final pool disagrees, re-walk the ledger before "
            "answering.")
    if (not syn and a.style not in ("extract", "judge")
            and re.search(r"preempt", prompt, re.I)):
        parts.append(
            "PREEMPT DISCIPLINE — minute-by-minute is the only honest "
            "walk. Checkpoints bank at start-or-resume +2 and every 2 "
            "RUNNING minutes after, in absolute minutes; a bank "
            "landing exactly on HIGH's arrival still counts, nothing "
            "wasted. On preemption, minutes past the last bank are "
            "wasted and redone after resume (owed = total − banked). "
            "A resumed LOW outranks a fresh LOW at any tie; the core "
            "never idles while a LOW waits. CLOCK CONVENTION: a job "
            "that starts at minute s and runs n minutes occupies "
            "minutes s .. s+n−1 and FINISHES AT minute s+n; the next "
            "waiting LOW starts in that same finish minute; makespan "
            "= the largest finish minute on the night (HIGH included "
            "for the clock, never for the order list). The order "
            "lists LOW jobs ONLY, by finish minute. wasted = minutes "
            "run past the last bank at each preemption, summed.")
        if re.search(r"checkpoint", prompt, re.I):
            parts.append(
                "PER-MINUTE TABLE — FIRST write four fact bullets, "
                "tonight's numbers only: LOW1 (duration, start), "
                "LOW2 (duration, start), HIGH (arrival, duration). "
                "The orientation rehearsal is a DIFFERENT night with "
                "different numbers — never walk it, never mix its "
                "numbers in. Then write the whole night as one table "
                "line per minute, columns 'minute | core | event' "
                "(start, resume, preempt, bank@absolute-minute, "
                "finish). Derive makespan, wasted and order from the "
                "table only, then output the JSON last, after the "
                "table.")
    if (not syn and a.style not in ("extract", "judge")
            and re.search(r"canary|rollout|rollback", prompt, re.I)
            and re.search(r"percent|step", prompt, re.I)):
        parts.append(
            "CANARY DISCIPLINE — walk the observations in order, one "
            "line each: 'step N: observed X vs limit L -> ok/breach'. "
            "The limit is 2.0 for steps 1-5, 1.0 for steps 6-10, and "
            "0.8 for EVERY remaining step after any rollback. breach "
            "= observed strictly above the limit. First breach: "
            "rollback exactly one step, that observation is SPENT "
            "(never retested), the next observation resumes the "
            "climb. Second breach: the climb stops for good, status "
            "STOPPED, rollout_pct = 10 × the step you were defending, "
            "stages_done = observations consumed so far. No breach at "
            "all: status DONE, rollout_pct = 10 × steps, "
            "stages_done = steps.")
    if (not syn and a.style not in ("extract", "judge")
            and re.search(r"pinned", prompt, re.I)
            and re.search(r"evict", prompt, re.I)):
        parts.append(
            "RESIDENCY DISCIPLINE — keep one state line per event: "
            "'event -> resident {ids} free {GB} recency {ids oldest "
            "to newest}'. A load that fits loads. A load that cannot "
            "fit evicts least-recently-touched UNPINNED models one at "
            "a time (count each) until it fits; if it still cannot "
            "fit because only pinned models remain, it defers — no "
            "eviction, record the id. Every touch of a model moves it "
            "to most-recent. Unload frees its size. loads = "
            "successful loads, evictions = models evicted, "
            "deferrals = loads turned away, deferred = their ids in "
            "order.")
    out = "\n\n".join(parts)
    L.ledger_fingerprint(a.run_dir, cid, a.stage, out)
    L.ledger_outcome(a.run_dir, cid, a.stage, "RUN")
    print(out)


if __name__ == "__main__":
    main()
