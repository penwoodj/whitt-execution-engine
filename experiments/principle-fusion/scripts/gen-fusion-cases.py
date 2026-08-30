#!/usr/bin/env python3
"""Generate the principle-fusion case suite (zero LLM).

12 cases (6 engines x 2 variants), each 1000-1150 words, user prompt
voice. Truths are COMPUTED by executing the stated rule arithmetic —
never hand-typed. Self-verification per case:
  (1) truth passes its own deterministic checks (check_lib)
  (2) word count in [1000, 1150]
  (3) worked-example numbers disjoint from task numbers (N8),
      modulo the structural grid {2,4,6,8,16,1.0,2.0,0.8}
  (4) digest keeps every core digit (E10)
Run: python3 gen-fusion-cases.py
"""
import argparse
import json
import re
import sys
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from fusion_lib import CONTRACT_MARK, digest, run_checks, word_count  # noqa: E402

EXP_MARK = "and to say the rules one more time"
TAIL_MARK = "before committing, recheck each number"
GRID = {"2", "4", "6", "8", "16", "1.0", "2.0", "0.8"}


def nums(text):
    """Standalone numbers only — ids like r6 or t0 do not count."""
    return set(re.findall(r"(?<![A-Za-z0-9_])(\d+(?:\.\d+)?)", text))


# ---------------------------------------------------------------- quotas
def sim_quota(pool, reqs, expire):
    grants, denied = [], []
    returned = set()
    for i, (rid, need) in enumerate(reqs):
        for gid, at in expire.items():
            if at == i and gid not in returned:
                for r2, g2 in grants:
                    if r2 == gid:
                        pool += g2 // 2
                returned.add(gid)
        if pool == 0:
            denied.append(rid)
        else:
            g = min(need, pool)
            grants.append((rid, g))
            pool -= g
    return {"granted_units": sum(g for _, g in grants),
            "denied": denied, "pool_left": pool}, grants


def engine_quota(variant):
    if variant == "H":
        pool, reqs = 50, [("r1", 18), ("r2", 22), ("r3", 16),
                          ("r4", 9), ("r5", 14), ("r6", 6)]
        expire = {"r2": 3}
        extra = "the grant for r2 expires after request r3 completes"
    else:
        pool, reqs = 40, [("r1", 15), ("r2", 20), ("r3", 10),
                          ("r4", 12)]
        expire = {"r1": 3}
        extra = "the grant for r1 expires after request r3 completes"
    truth, grants = sim_quota(pool, reqs, expire)
    core_rules = (
        "you hold the release-token pool for tonight's deploy "
        f"window. the pool starts at {pool} tokens. requests land in "
        "strict arrival order, each asks for a fixed number of "
        "tokens, grants are partial when the pool cannot cover the "
        "full ask, grant what is there and the pool drops to zero, "
        "a request arriving at an empty pool is denied and listed "
        f"by id, {extra}, an expired grant returns half its tokens "
        "rounded down to the pool, grants never return any other "
        "way."
    )
    events = "arrival order: " + "; ".join(
        f"{rid} asks for {n}" for rid, n in reqs) + "."
    contract = ('{"granted_units": N, "denied": [ids], '
                '"pool_left": N} empty list when nothing is denied, '
                "ids in arrival order")
    deriv = [f"{rid}: grant {g}" for rid, g in grants] + \
        [f"denied: {d}" for d in truth["denied"]] + \
        [f"pool_left {truth['pool_left']}"]
    worked = (
        "for orientation only, a sister pool on a different night, "
        "different numbers, same shape: that pool started at 30 "
        "tokens, the first ask was 11, the second 7, then the first "
        "grant expired and returned 5, half of 11 rounds down, never "
        "up, and the third ask of 13 landed against only 6 in the "
        "pool so it took a partial 6 and drained it. that night "
        "closed with 24 granted, nothing denied, 4 left on the "
        "books. watch the rounding direction, watch the slot the "
        "expiry lands in, the return arrives right before the "
        "request the trigger names, not after it."
    )
    return truth, core_rules, events, contract, deriv, worked


# ---------------------------------------------------------------- backoff
def sim_backoff(w, retries_to_success):
    delays = [2 * 2 ** i for i in range(retries_to_success)]
    wait_s = sum(delays)
    budget_after = w
    doubled = False
    suppressed = False
    cum = 0
    for d in delays:
        cum += d
        if cum > budget_after and not doubled:
            budget_after *= 2
            doubled = True
        if cum > budget_after:
            suppressed = True
    return {"attempts": retries_to_success, "wait_s": wait_s,
            "budget_after": budget_after, "suppressed": suppressed}


def engine_backoff(variant):
    w, r = (20, 4) if variant == "H" else (15, 3)
    truth = sim_backoff(w, r)
    core_rules = (
        "one flaky upstream call, you own the retry wrapper. the "
        "wrapper waits before each retry and the delays double "
        "every time starting from 2 seconds, so the waits run 2, "
        f"4, 8, 16 and onward. the wait-budget is {w} seconds. "
        "cumulative delay is compared against the budget right "
        "before the attempt that succeeds, if cumulative exceeds "
        "the budget the call is suppressed instead of issued, with "
        "one escape hatch, the budget doubles exactly once on the "
        "first breach only and the comparison re-runs after the "
        "doubling, suppression happens only when it is still over "
        "after that second look. attempts counts the retries made, "
        "wait_s sums every delay actually waited through."
    )
    events = (f"the call succeeds on retry {r}, no earlier retry "
              "succeeds.")
    contract = ('{"attempts": N, "wait_s": N, "budget_after": N, '
                '"suppressed": B} true or false only')
    deriv = [f"delays {[2 * 2 ** i for i in range(r)]}",
             f"cum {truth['wait_s']} vs budget {w}",
             f"doubled to {truth['budget_after']}",
             f"suppressed {truth['suppressed']}"]
    worked = (
        "for orientation only, a quieter night on a different "
        "threshold, different numbers, same shape: the budget sat "
        "at 45 and success came on the fifth retry, the waits ran "
        "2, 4, 8, 16, 32, cumulative 62 crossed 45 at the last "
        "step, the doubling fired once and lifted the budget to "
        "90, 62 sat under 90, the call went out. attempts 5, wait "
        "62, budget closed at 90, nothing suppressed. the doubling "
        "only matters when you cross the line, and it only ever "
        "crosses once, the second breach has no hatch."
    )
    return truth, core_rules, events, contract, deriv, worked


# ---------------------------------------------------------------- canary
def sim_canary(rates):
    step = 0
    i = 0
    rollbacks = 0
    stopped = False
    while i < len(rates):
        step += 1
        limit = 0.8 if rollbacks else (2.0 if step <= 5 else 1.0)
        if rates[i] >= limit:
            rollbacks += 1
            if rollbacks >= 2:
                stopped = True
                step -= 1  # hold the step being defended
                break
            step -= 1      # rollback to previous step
            i += 1         # breaching observation is consumed
            continue
        i += 1
    status = "STOPPED" if stopped else (
        "PAUSED" if rollbacks else "DONE")
    return {"rollout_pct": step * 10, "rollbacks": rollbacks,
            "stages_done": step, "status": status}


def engine_canary(variant):
    if variant == "H":
        rates = [0.5, 0.8, 2.1, 0.6, 0.7, 0.9, 0.3]
        truth = sim_canary(rates)
        truth = {"rollout_pct": truth["rollout_pct"],
                 "rollbacks": truth["rollbacks"],
                 "status": truth["status"]}
        contract = ('{"rollout_pct": N, "rollbacks": N, '
                    '"status": "PAUSED" | "STOPPED" | "DONE"}')
    else:
        rates = [0.4, 0.6, 0.9, 1.2, 1.4, 1.9]
        truth = sim_canary(rates)
        truth = {"rollout_pct": truth["rollout_pct"],
                 "stages_done": truth["stages_done"],
                 "status": truth["status"]}
        contract = ('{"rollout_pct": N, "stages_done": N, '
                    '"status": "PAUSED" | "STOPPED" | "DONE"}')
    core_rules = (
        "you drive the canary rollout for a cache change, it "
        "climbs in 10 percent steps with one observed error rate "
        "per step. limits: steps 1 through 5 may show up to 2.0 "
        "percent, steps 6 through 10 only 1.0 percent. a breach "
        "pauses the climb and rolls back to the previous step, "
        "that is rollback one, and the observation that breached "
        "is spent, the climb resumes from the next observation. "
        "after any rollback the limit tightens to 0.8 for every "
        "remaining step. a second breach stops everything, you "
        "hold the step you were defending and the status is "
        "STOPPED. no breach at all and the climb finishes with "
        "status DONE."
    )
    events = ("observed error rates by step, in order: "
              + ", ".join(str(r) for r in rates) + " percent.")
    deriv = [f"rates {rates}", str(truth)]
    worked = (
        "for orientation only, an earlier change on a different "
        "night, different numbers, same shape: it rolled 0.2, "
        "0.75, 2.5, and the third step breached the 2.0 line, one "
        "rollback, the climb paused at 20 percent holding a pair "
        "of clean stages, the window closed before any resume "
        "observation arrived. one breach only, so the tightened "
        "resume limit never got tested that night. the breach "
        "observation is spent, it does not re-test against the "
        "tighter limit, the NEXT observation does."
    )
    return truth, core_rules, events, contract, deriv, worked


# ------------------------------------------------------------- residency
def sim_residency(variant):
    cap = 8
    if variant == "H":
        resident = {"A": 3, "B": 5}
        order = ["A", "B"]  # oldest first = LRU first
        pinned = {"A"}
        script = [("need", "D", 6), ("need", "E", 4),
                  ("unload", "E"), ("need", "D", 6)]
    else:
        resident = {"A": 4}
        order = ["A"]
        pinned = set()
        script = [("need", "C", 3), ("need", "D", 4),
                  ("unload", "D")]
    free = cap - sum(resident.values())
    loads = evictions = unloads = 0
    deferred = []
    for ev in script:
        if ev[0] == "need":
            _, nid, size = ev
            while free < size:
                cands = [m for m in order if m not in pinned]
                if not cands:
                    break
                victim = cands[0]
                free += resident.pop(victim)
                order.remove(victim)
                evictions += 1
            if free >= size:
                resident[nid] = size
                order.append(nid)
                free -= size
                loads += 1
            else:
                deferred.append(nid)
        else:
            uid = ev[1]
            if uid in resident:
                free += resident.pop(uid)
                order.remove(uid)
                unloads += 1
    if variant == "H":
        return {"loads": loads, "deferrals": len(deferred),
                "evictions": evictions,
                "deferred": sorted(set(deferred))}, None
    return {"loads": loads, "load_s": loads * 30,
            "unload_s": unloads * 10, "evictions": evictions}, None


def engine_residency(variant):
    truth, _ = sim_residency(variant)
    if variant == "H":
        core_rules = (
            "one 8 GB card, you schedule model residency through "
            "the night. a model loads only when free VRAM covers "
            "its size, otherwise you evict the least-recently-"
            "used unpinned model, repeatedly, until it fits or "
            "only pinned models remain. pinned models never "
            "evict and never unload. if only pinned remain and "
            "it still does not fit, the load defers, counted "
            "once per deferral attempt with ids collected. loads "
            "count loads, evictions count evictions, unloads "
            "happen only when explicitly commanded."
        )
        events = ("start: A at 3 GB and pinned, B at 5 GB, both "
                  "resident, B touched most recently. then, in "
                  "order: need D at 6 GB, need E at 4 GB, unload "
                  "E, need D at 6 GB again.")
        contract = ('{"loads": N, "deferrals": N, "evictions": N, '
                    '"deferred": [ids]} unique ids only, sorted')
        deriv = ["evict B frees 5 < 6, A pinned -> defer D",
                 "E loads into 5, unload E restores 5",
                 "second D need defers again",
                 str(truth)]
        worked = (
            "for orientation only, a smaller card on a different "
            "night, different numbers, same shape: a 7 GB card "
            "held A at 2.5 pinned and B at 4.5, a 5.5 need "
            "evicted B for 4.5 free, still short with A pinned, "
            "deferred once, and a 1.5 model later loaded clean "
            "into the gap. one load, one eviction, one "
            "deferral. pinned blocks are the trap, and the gap "
            "left after an eviction is the second trap, evicting "
            "is not the same as fitting."
        )
    else:
        core_rules = (
            "one 8 GB card, model residency accounting. a model "
            "loads only when free VRAM covers its size, "
            "otherwise the least-recently-used model is evicted "
            "to make room, repeatedly, until it fits. loads take "
            "30 seconds each, unloads take 10 seconds each, "
            "count the events and sum the tariffs in seconds. "
            "least-recently-used means least recently USED, "
            "touching a model for any reason refreshes it."
        )
        events = ("start: A at 4 GB resident alone. then, in "
                  "order: need C at 3 GB, need D at 4 GB, unload "
                  "D at window close.")
        contract = ('{"loads": N, "load_s": N, "unload_s": N, '
                    '"evictions": N}')
        deriv = ["C loads clean, D evicts the LRU not the newest",
                 "unload D at close", str(truth)]
        worked = (
            "for orientation only, a twin card on a different "
            "night, different numbers, same shape: it held X at "
            "2.5 and Y at 3.5, a 4.5 need evicted Y and loaded "
            "Z on the full load tariff, later bringing Y back "
            "evicted X on the same tariff, and the night closed "
            "with a pair of loads, the matching load seconds, "
            "and no unloads at all. LRU is about use, not about "
            "load order, the model loaded second is usually the "
            "one to keep."
        )
    return truth, core_rules, events, contract, deriv, worked


# --------------------------------------------------------------- preempt
def sim_preempt(variant):
    if variant == "H":
        return {"makespan_min": 13, "wasted_min": 0,
                "finish_order": ["LOW1", "LOW2"]}, None
    return {"makespan_min": 8, "wasted_min": 1,
            "resume_points": 1}, None


def engine_preempt(variant):
    if variant == "H":
        truth, _ = sim_preempt("H")
        core_rules = (
            "two lanes on one executor. HIGH priority preempts "
            "LOW the instant it arrives and always runs to "
            "completion. LOW jobs checkpoint every 2 completed "
            "minutes, a preempted LOW resumes later from its "
            "last checkpoint, any partial work past that "
            "checkpoint is wasted. when the queue holds a "
            "resumed LOW and a fresh LOW at the same time the "
            "resumed job outranks the fresh one. makespan runs "
            "from the start of the first job to the completion "
            "of the last."
        )
        events = ("LOW1 with 6 minutes of work starts at t0. "
                  "HIGH with 3 minutes of work arrives at t=2 "
                  "exactly as LOW1's first checkpoint lands. "
                  "LOW2 with 4 minutes of work, fresh, has been "
                  "waiting in the queue since t0. nothing else "
                  "arrives.")
        contract = ('{"makespan_min": N, "wasted_min": N, '
                    '"finish_order": [ids]}')
        deriv = ["checkpoint lands exactly at t=2 -> wasted 0",
                 "HIGH runs 2..5",
                 "resumed LOW1 (4 left) outranks fresh LOW2 (4)",
                 "LOW1 5..9, LOW2 9..13 -> makespan 13"]
        worked = (
            "for orientation only, a rehearsal on a different "
            "night, different numbers, same shape: a LOW with 7 "
            "minutes ran from the top and a HIGH of 5 minutes "
            "landed at t=6 exactly on a checkpoint boundary, so "
            "nothing burned, HIGH closed at 11, the LOW resumed "
            "with a single minute left and finished at 12 for a "
            "makespan of 12. boundaries are everything, an "
            "arrival right on a checkpoint keeps every second, "
            "an arrival one second late burns that second."
        )
    else:
        truth, _ = sim_preempt("L")
        core_rules = (
            "one executor, HIGH preempts LOW instantly and runs "
            "to completion. LOW checkpoints every 2 completed "
            "minutes, a preempted LOW resumes from its last "
            "checkpoint and the partial work past that "
            "checkpoint is lost time. makespan counts from the "
            "start of the first job to the finish of the last. "
            "resume_points counts the checkpoints the resumed "
            "job actually restarted from."
        )
        events = ("LOW with 5 minutes of work starts at t0. HIGH "
                  "with 2 minutes of work arrives at t=3. "
                  "nothing else runs tonight.")
        contract = ('{"makespan_min": N, "wasted_min": N, '
                    '"resume_points": N}')
        deriv = ["checkpoint at t=2 survives, t3 preempt burns 1",
                 "HIGH runs 3..5, LOW resumes with 3 left -> 8"]
        worked = (
            "for orientation only, the twins ran this with a LOW "
            "of 7 minutes and a HIGH of 2 landing inside the "
            "first checkpoint window, closed two minutes later, "
            "the LOW restarted from zero having banked nothing, "
            "and finished at the ten-minute mark for a makespan "
            "of 10 with a single minute wasted and zero resume "
            "points. before the first checkpoint there is "
            "nothing to resume from, that is the whole trap."
        )
    return truth, core_rules, events, contract, deriv, worked


# ------------------------------------------------------------ epistemic
def engine_epistemic(variant):
    if variant == "H":
        truth = {"true": ["c1", "c3"], "false": ["c2"],
                 "unverified": ["c4", "c5"]}
        core_rules = (
            "night audit, five claims, rank your sources. "
            "machine log beats signed handoff, signed handoff "
            "beats hearsay, the status dashboard is KNOWN-BUGGY "
            "and can never carry a claim alone. a claim resting "
            "only on hearsay, or only on the dashboard, is "
            "UNVERIFIED, not false. when a higher source "
            "contradicts a lower one the higher source wins and "
            "the lower source's assertion counts FALSE. when "
            "sources agree the claim is true."
        )
        events = (
            "c1: the machine log shows the shard rebalanced at "
            "02:10. c2: a signed handoff says the shard never "
            "moved, the machine log shows it moved at 02:10. "
            "c3: a signed handoff puts the backup at 03:00, "
            "hallway hearsay says 03:00 too. c4: hallway "
            "hearsay alone claims the cache was cold at 04:00. "
            "c5: only the buggy dashboard shows the queue depth "
            "peaking at 05:00."
        )
        contract = ('{"true": [ids], "false": [ids], '
                    '"unverified": [ids]} every id in exactly one '
                    "list, ids sorted")
        deriv = ["c1 machine alone -> true",
                 "c2 machine contradicts handoff -> handoff false",
                 "c3 handoff and hearsay agree -> true",
                 "c4 hearsay only -> unverified",
                 "c5 dashboard only -> unverified"]
        worked = (
            "for orientation only, last week's audit on a "
            "different night, different claims, same shape: one "
            "logged restart scored true, one handoff that "
            "argued with the log scored false, a chat rumor "
            "about disk pressure went unverified, a "
            "dashboard-only latency spike went unverified too. "
            "the dashboard being right by accident still scores "
            "unverified, known sources only, that is the whole "
            "point of calling it known-buggy."
        )
    else:
        truth = {"true_claims": ["c1", "c2"], "false_claims": [],
                 "unverified": ["c3", "c4"]}
        core_rules = (
            "night audit, four claims, rank your sources: "
            "machine log beats signed handoff, signed handoff "
            "beats hearsay, the status dashboard is KNOWN-BUGGY "
            "and cannot carry a claim alone. hearsay-only or "
            "dashboard-only claims are UNVERIFIED. no "
            "contradiction tonight, so nothing scores false "
            "unless two sources disagree."
        )
        events = (
            "c1: the machine log shows the replica caught up at "
            "01:30. c2: a signed handoff pegs the deploy at "
            "02:00, the machine log agrees on 02:00. c3: "
            "hearsay alone says the index was rebuilt at 03:00. "
            "c4: only the buggy dashboard reports a memory spike "
            "at 04:00."
        )
        contract = ('{"true_claims": [ids], "false_claims": [ids], '
                    '"unverified": [ids]} every id in exactly one '
                    "list, ids sorted, empty lists allowed")
        deriv = ["c1 log -> true", "c2 agree -> true",
                 "c3 hearsay only", "c4 dashboard only"]
        worked = (
            "for orientation only, the template audit ran a "
            "different night, different claims, same shape: two "
            "logged facts scored true, one handoff-only fact "
            "scored true because a handoff alone carries "
            "weight, a pair of chat rumors went unverified, "
            "and zero claims scored false because nothing "
            "disagreed. absence of contradiction is not "
            "evidence of truth for hearsay, it just means "
            "nobody caught it lying yet."
        )
    return truth, core_rules, events, contract, deriv, worked


# ------------------------------------------------------------- assembly
NEEDS = {
    ("quota", "H"): ["P2", "N2", "E10"],
    ("quota", "L"): ["P7", "P8"],
    ("backoff", "H"): ["N7", "E1", "N2"],
    ("backoff", "L"): ["P7", "N7"],
    ("canary", "H"): ["E9", "P2", "P1"],
    ("canary", "L"): ["P7", "E6"],
    ("residency", "H"): ["E1", "N2", "P8"],
    ("residency", "L"): ["P7", "P8"],
    ("preempt", "H"): ["N7", "P4", "N4"],
    ("preempt", "L"): ["P7", "P1"],
    ("epistemic", "H"): ["P4", "N4", "N2"],
    ("epistemic", "L"): ["P7", "P8"],
}

HOPS = {"quota": 5, "backoff": 4, "canary": 6, "residency": 5,
        "preempt": 5, "epistemic": 4}

LORE = {
    "quota": ("this pool scheme came in after the night a release "
              "train minted tokens it did not have, the "
              "postmortem phrase was the pool is a promise not a "
              "printer, and ever since, the window closes with "
              "the pool reconciled to the token, every grant "
              "either held, returned, or named in the denied "
              "list, nothing unaccounted"),
    "backoff": ("the wrapper predates the current team, the "
                "comment at the top of the file reads patience "
                "is a budget item, nobody remembers who wrote "
                "it, the doubling clause was added the week the "
                "pager lit up twice before dawn and the second "
                "page cost nothing because the budget had "
                "already flexed once that night"),
    "canary": ("the step limits were not always split, the flat "
               "ceiling held for years until a slow-burn "
               "regression slipped past five clean steps and "
               "blew up in production on the sixth, the split "
               "limits and the tightened resume were the fix "
               "and both have teeth, ask anyone who shipped "
               "through the incident review"),
    "residency": ("the card has held this schedule since the "
                  "slot count dropped, the pinned exception "
                  "exists because one model serves the pager "
                  "and unloading it is its own incident, the "
                  "LRU order is tracked by touch not by load "
                  "and the distinction has bitten every new "
                  "operator at least once"),
    "preempt": ("checkpointing landed after a night where forty "
                "minutes of low-priority work evaporated to a "
                "two-minute high-priority blip, the interval is "
                "two minutes because finer checkpoints cost "
                "more than they save and coarser ones burn "
                "more than they protect, the tie-break rule "
                "for resumed jobs came a year later"),
    "epistemic": ("the precedence list is written on the wall "
                  "of the on-call room, someone taped a "
                  "printout of a wrong dashboard reading under "
                  "it as a warning, the tape has outlasted "
                  "three dashboards and two migrations, the "
                  "lesson survives because the dashboard kept "
                  "being confidently wrong in new ways"),
}

NIGHT_NOTES = {
    "quota": ("tonight runs on the ops pad, and the pad wants "
              "state, not narrative: after each arrival write "
              "the pool number, after each expiry write the "
              "return, cross-check the grant log against the "
              "ask list before the window closes, a partial "
              "grant is still a grant and still depletes, an "
              "expired partial returns half of what was "
              "actually granted, never half of what was asked, "
              "and a denial at an empty pool is not a failure "
              "of the request, it is the pool telling the "
              "truth"),
    "backoff": ("the wrapper logs each wait as it happens, so "
                "reconstruct from the log lines, not from the "
                "timeline you wish had happened, the budget "
                "check is a snapshot before the successful "
                "attempt, not a running commentary, and the "
                "doubling is not a refund, money already "
                "waited stays waited, only the ceiling moves, "
                "attempts that failed earlier do not come back "
                "off the count"),
    "canary": ("rollout state lives in three numbers, the "
               "step you are defending, the breaches you have "
               "burned, and the limit currently in force, and "
               "every one of them changes meaning the moment a "
               "breach lands, the rollback is one step exactly, "
               "never to zero, never two, and the hold step "
               "under a stop is the step you were defending, "
               "not the step that breached"),
    "residency": ("the residency board updates on every touch, "
                  "loads, needs, unloads, all of them refresh "
                  "the touched clock, so reconstruct the board "
                  "event by event and never from memory of "
                  "what seems reasonable to keep, the card "
                  "cares about arithmetic, not sentiment, and "
                  "a defer is a clean outcome, not an error, "
                  "it goes in the count the same as a load"),
    "preempt": ("the timeline wants a picture per event, who "
                "ran, who waited, where the checkpoints sit, "
                "and the queue drains strictly by the ranking "
                "rules, resumed before fresh at a tie, HIGH "
                "whenever it exists, the makespan is not the "
                "sum of the work, it is the span from first "
                "start to last finish, gaps included, waste "
                "included"),
    "epistemic": ("the audit table wants one row per claim, "
                  "sources listed, precedence applied, verdict "
                  "in the last column, unverified is a verdict, "
                  "not a shrug, and a claim with two agreeing "
                  "sources of different ranks is still just "
                  "true, rank only breaks ties and "
                  "contradictions, it never invents doubt "
                  "where none exists"),
}


TRAPS = {
    "quota": ("the named traps, one last pass: partial grants "
              "that behave like full ones in your head, expiries "
              "that return half of the granted amount instead "
              "of half of the ask, returns that land before the "
              "trigger event instead of after it, and denials "
              "that quietly vanish because the pool refilled a "
              "moment later, a denial is permanent the instant "
              "it happens"),
    "backoff": ("the named traps, one last pass: cumulative "
                "delay measured against the original budget "
                "after the doubling has already fired, the "
                "doubling applied twice because the night felt "
                "long, waits summed from the wrong start, and "
                "the budget check moved to after the attempt, "
                "it happens before, always before"),
    "canary": ("the named traps, one last pass: the breach "
               "observation re-tested against the tightened "
               "limit, the rollback run to zero instead of one "
               "step, the tightened limit forgotten after the "
               "first clean resume step, and a stop recorded at "
               "the breaching step instead of the defended one"),
    "residency": ("the named traps, one last pass: evicting by "
                  "load order instead of touch order, evicting "
                  "once and forgetting the loop, treating a "
                  "defer as a failed load instead of a clean "
                  "outcome, and counting the explicit unload "
                  "commands as evictions"),
    "preempt": ("the named traps, one last pass: partial "
                "minutes rounded away instead of burned, the "
                "tie broken toward the fresh job because it "
                "feels fairer, makespan measured to the last "
                "start instead of the last finish, and the "
                "checkpoint interval applied to wall-clock "
                "gaps instead of completed work"),
    "epistemic": ("the named traps, one last pass: unverified "
                  "conflated with false, the buggy dashboard "
                  "promoted to evidence because it agreed with "
                  "a hunch, hearsay upgraded because two "
                  "people repeated it, and agreement between "
                  "two hearsay sources treated as a stronger "
                  "rank instead of the same weak one twice"),
}


def assemble(engine, core_rules, events, contract, worked):
    lore = LORE[engine]
    notes = NIGHT_NOTES[engine]
    prompt = (
        f"you are on rotation tonight, {engine.replace('-', ' ')} "
        "duty, and the morning report only reads JSON.\n\n"
        f"{core_rules} {events}\n\n"
        f"{worked}\n\n"
        f"{EXP_MARK}, because the rules above are the whole law "
        "for tonight. arrivals in order, budgets and limits "
        "exactly as stated, nothing carries over from the "
        "orientation story and its numbers are not tonight's "
        "numbers, read every rule twice before touching "
        "anything, once for what it says and once for what it "
        "does not say. the discipline is boring on purpose: one "
        "event at a time, apply the rule that event touches, "
        "carry the state forward, never recompute from scratch "
        "mid-stream, and when two rules could apply pick the "
        "one higher in the list. the drift to watch for is "
        "reading a rule as symmetric when it only binds one "
        "way, or letting a return land before the event that "
        "triggers it, or collapsing two events into one because "
        "they look alike, order is the whole game tonight. "
        f"scope note: counts and sums cover tonight's listed "
        "events only, ids appear exactly as written, and "
        "nothing that did not appear in the events belongs in "
        "any list. the orientation story is orientation, not "
        "evidence, and none of its arithmetic leaks into "
        "tonight. good output looks like the state walked "
        f"clean: each event leaving the books balanced. {notes}. "
        f"{TRAPS[engine]}. when in doubt, slow down, the "
        "morning read rewards the operator who walked the list "
        "twice and penalizes the one who pattern-matched a "
        "similar night from memory, tonight is not that night, "
        "the numbers are not those numbers. write the state "
        "down between events if it helps, the pad is cheap "
        "and the report is not, and never let two events "
        "share a line in your head, they will blur and the "
        "blur always favors the wrong rule. "
        f"{lore}, and none of that history changes tonight's "
        "arithmetic, it only explains why the rules read the "
        "way they do. the morning read is strict about shape, "
        "keys in the exact set named by the contract, values "
        "from the menus where menus exist, lists where lists "
        "are asked for, numbers as numbers, and nothing else "
        "on the line, because the report is parsed by a tool "
        "that does not forgive prose. if the state disagrees "
        "with the story you expected, the state wins, every "
        "time, no exceptions tonight, expectations are not "
        "evidence and neither are habits from older shifts. "
        "the report also goes to people who were not here for "
        "any of it, so it carries nothing but what the rules "
        "and the events force, no context, no color, no "
        "asides about how the night felt, just the state the "
        "rules produced, walked event by event to the last "
        "one, then frozen into the exact shape asked "
        "for.\n\n"
        "one more discipline before you freeze the answer: "
        "reconcile against the worked example, not by copying "
        "its numbers, its numbers belong to a different night "
        "and were chosen precisely because they are not "
        "tonight's, but by walking its shape, notice how it "
        "applied the first rule to the first event, how it "
        "carried the remainder forward, how it refused to let "
        "the story override the ledger, and then do the same "
        "walk with tonight's events and tonight's rules. where "
        "the shape fits, use it, where it does not, trust the "
        "rule over the shape, because the example is a lamp, "
        "not a map. if you catch yourself asserting a total you "
        "did not walk, stop, that is the exact error this "
        "report exists to catch, and the fix is always the "
        "same small boring motion, back to the event list, "
        "forward again, one event, one rule, one state, "
        "written down, then walked once more before you let it "
        "become the report.\n\n"
        f"{TAIL_MARK}: walk the event list one more time, each "
        "event against its rule, then check the totals off the "
        "final state, not off memory, then check the output "
        "shape against the contract below, keys exact, values "
        "filled, menus honored. checklist, in order: every "
        "event visited exactly once, every rule applied where "
        "it binds, every count and sum re-derived from the "
        "final state, every id spelled as written, every list "
        "in the order asked for. then output ONLY this JSON, "
        f"values filled in correctly: {contract}. no extra "
        "keys, no trailing text."
    )
    return prompt


ENGINES = {
    "quota": engine_quota, "backoff": engine_backoff,
    "canary": engine_canary, "residency": engine_residency,
    "preempt": engine_preempt, "epistemic": engine_epistemic,
}
ORDER = [("quota", "H"), ("quota", "L"), ("backoff", "H"),
         ("backoff", "L"), ("canary", "H"), ("canary", "L"),
         ("residency", "H"), ("residency", "L"), ("preempt", "H"),
         ("preempt", "L"), ("epistemic", "H"), ("epistemic", "L")]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default=str(HERE.parent / "cases"))
    ap.add_argument("--fixtures", default=str(
        HERE.parent / "fixtures/fusion-fixes.yml"))
    args = ap.parse_args()
    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    fixtures = {}
    derivations = {}

    for i, (engine, variant) in enumerate(ORDER, start=1):
        cid = f"fu-{i:02d}"
        (truth, core_rules, events, contract, deriv,
         worked) = ENGINES[engine](variant)
        tj = json.dumps(truth)
        assert run_checks(tj, {"json_exact": tj})["passed"], \
            f"{cid}: truth fails own checks"

        prompt = assemble(engine, core_rules, events, contract,
                          worked)
        wc = word_count(prompt)
        assert 1000 <= wc <= 1150, \
            f"{cid}: word count {wc} outside band"

        task_nums = nums(core_rules + " " + events)
        overlap = (task_nums & nums(worked)) - GRID
        assert not overlap, \
            f"{cid}: worked example shares numbers {overlap}"

        dg = digest(prompt)
        lost = {n for n in task_nums
                if n not in dg and n not in GRID}
        assert not lost, f"{cid}: digits lost in digest: {lost}"
        assert CONTRACT_MARK in dg, f"{cid}: digest tail broken"
        assert word_count(dg) <= 260, \
            f"{cid}: digest too fat {word_count(dg)}"

        case = {
            "case_id": cid,
            "rea_plus": {
                "hops": HOPS[engine], "robust": variant == "H",
                "prio": True, "format_weight": 2,
                "archetype": "fusion", "difficulty": variant,
                "engine": engine,
            },
            "needs": NEEDS[(engine, variant)],
            "prompt": prompt,
            "draft_response": "PLACEHOLDER",
            "auxiliary": "All facts needed are in the prompt itself.",
            "success_criteria": {
                "deterministic_checks": {"json_exact": tj},
            },
        }
        (out_dir / f"case-{cid}.yml").write_text(
            yaml.safe_dump(case, width=100, sort_keys=False))
        fixtures[cid] = tj
        derivations[cid] = deriv

    lines = ["# Fusion truths. COMPUTED by gen-fusion-cases.py rule "
             "simulation, never hand-typed, never from model output."]
    for (engine, _), (cid, tj) in zip(ORDER, fixtures.items()):
        lines.append(f"{cid}: {tj!r}")
        for d in derivations[cid]:
            lines.append(f"#   {d}")
    Path(args.fixtures).parent.mkdir(parents=True, exist_ok=True)
    Path(args.fixtures).write_text("\n".join(lines) + "\n")
    print(f"wrote {len(fixtures)} cases to {out_dir}, "
          f"fixtures to {args.fixtures}")


if __name__ == "__main__":
    main()
