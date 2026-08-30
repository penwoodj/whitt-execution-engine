#!/usr/bin/env python3
"""Authoring engine for the agentic-100 suite.

25 category engines x 4 variants = 100 cases in the user's prompt
voice. Every truth is computed by the same arithmetic the prompt
states (rule engines below), never invented. Writes:
  cases/agentic100/case-a1-<cat>-<v>.yml
  fixtures/agentic100-fixes.yml (with inline derivations)
  docs/AGENTIC100-MANIFEST.md (reference index)
"""
import json
import math
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
EXP = HERE.parent
OUT = EXP / "cases/agentic100"
FIX = EXP / "fixtures/agentic100-fixes.yml"
MAN = EXP / "docs/AGENTIC100-MANIFEST.md"

OPENERS = [
    "you're on call tonight and the numbers below are all yours to own",
    "the runbook is explicit, so follow it exactly and don't improvise",
    "a messy morning just landed on your desk and it needs reconciling by hand",
    "reconciliation time - read every rule before touching a single number",
]
DISTRACTORS = [
    "a dashboard service you don't manage renders all of this continuously and changes none of it",
    "a cost-reporting crawler you don't manage tallies everything for finance and touches nothing",
    "a search indexer you don't manage archived three snapshots of this and can't be trusted for counts",
    "a compliance scanner you don't manage timestamps everything independently and changes no outcomes",
]
CHECK_TAIL = ('before committing, recheck each number against its rule, '
              'then output ONLY this json with exactly these keys: ')


def dump_truth(obj):
    return json.dumps(obj, separators=(", ", ": "))


def expand(vi, engine, body, truth_obj):
    """Voice expansion: truth-neutral blocks in the user's register."""
    keys = list(truth_obj.keys())
    first = f'"{keys[0]}"'
    lore = LORE.get(engine, LORE_DEFAULT)
    ex_a, ex_b = WORKED_EX[vi % len(WORKED_EX)]
    p1 = (
        "and to say the rules one more time in different words, "
        "because this is the part people get wrong when they think "
        "they've got it: the rules above are not suggestions and "
        "they don't bend under pressure - when two of them seem to "
        "conflict, read both again, because one of them always "
        "specifies the tiebreak or the precedence and the apparent "
        "conflict is just the test. nobody is watching over your "
        "shoulder here, the numbers are what they are, and a "
        "careful walk through the rules beats a confident skim "
        "every single time. if a rule says count something, count "
        "exactly that thing, not the thing next to it, not the "
        "thing it sounds like, the thing it says.")
    p2 = (
        "a wrinkle that almost matters and deliberately doesn't: a "
        "neighboring team ran a similar exercise yesterday with "
        "different numbers and got a different answer, which is "
        "exactly what should happen and proves nothing about today; "
        "and a draft of this task circulating in chat has one "
        "transposed digit in it that people keep quoting at each "
        "other - the text above is the authority, the chat draft "
        "is not, and if you catch yourself reconciling toward the "
        "draft, stop. opinions in the channel about what the "
        "answer should be are noise; the rules and the numbers "
        "above are the entire universe.")
    p3 = (
        "for calibration, a worked example with different numbers "
        "entirely - pattern, not data: " + ex_a + " treat it as a "
        "shape for how the counting walks forward, never as "
        "numbers to reuse. " + ex_b + " the example exists to fix "
        "the arithmetic shape in your head; the numbers in the "
        "task above are the only ones that count for the answer.")
    p4 = (
        "before computing, run the discipline: name what each rule "
        "governs before touching a number; propagate forward "
        "through the rules only, not by vibes; respect the stated "
        "conventions over your instincts even where the "
        "conventions feel slightly off, because the convention is "
        "part of the task; don't invent events, thresholds, or "
        "partial credits the text never contains; and don't delete "
        "things the text does contain just because they feel "
        "adjacent to something else. guard the boundaries of "
        "every count: know exactly where each window opens and "
        "closes before you sum anything, because off-by-one "
        "boundaries are where careful readers lose more points "
        "than careless ones. if two counts seem to want the same "
        "number, that's coincidence, not a shortcut.")
    p5 = (
        "and a drift warning, because every task like this has one "
        "canonical wrong answer people converge on: if you catch "
        "yourself writing a value because it felt close, or "
        "because it matched the worked example, or because a "
        "number from the events section leaked into " + first + " "
        "where it doesn't belong - stop, back up to the rule that "
        "governs that key, and recompute from the rule. the wrong "
        "answer always sounds right in your head for about two "
        "seconds; the rule is what still sounds right after ten.")
    p6 = (
        "a note on authority and what you may lean on, because "
        "these tasks always carry more voices than facts: the "
        "events and rules above are the only evidence. anything "
        "that sounds like a summary, a dashboard, a recollection, "
        "a neighboring team's take, or a chat draft inherits "
        "nothing - it borrows credibility it never earned, and "
        "each of those voices is usually one revision behind the "
        "truth or one bug ahead of it. when a number in the "
        "events feels surprising, surprise is not evidence "
        "against it; when a rule feels harsh, harshness is not a "
        "reason to soften it in your arithmetic. read once for "
        "the rules, once for the events, once more for the "
        "boundaries, and only then pick up the pen.")
    p7 = (
        "and rehearse the whole thing twice, because once is how "
        "errors survive: first pass, name every rule and bind it "
        "to the count it governs, out loud if you have to; second "
        "pass, walk the events in order and place each one on the "
        "timeline before anything is summed. then the boundaries: "
        "every window's open and close named exactly, every "
        "tiebreak applied in the order written rather than the "
        "order that feels natural, and every rejected, denied, "
        "skipped, or suppressed thing counted as rejected, denied, "
        "skipped, or suppressed - not silently dropped, not "
        "quietly rescued, and definitely not re-labeled because "
        "the label felt unkind. half the wrong answers to tasks "
        "like this are right numbers attached to the wrong "
        "events, and the other half are right events summed "
        "against the wrong boundaries, so the rehearsal is not "
        "ceremony - it is the actual work. if any step of the "
        "rehearsal makes you hesitate, that hesitation is the "
        "finding: go back to that step and recompute it from its "
        "rule instead of pushing through, because pushing "
        "through is how a two-second wrong answer becomes a "
        "committed one.")
    p8 = (
        "one final note on scope, because thorough agents "
        "overreach: answer exactly what is asked, in the exact "
        "shape asked, with exactly the keys asked for - no "
        "commentary, no reasoning transcript, no extra keys for "
        "context, no markdown fences. a correct number in the "
        "wrong shape fails this task just as dead as a wrong "
        "number, and the checking is mechanical, so give the "
        "machine exactly what it asked for and nothing else.")
    blocks = [body.strip(), p1, lore, p2, p3, p4, p5, p6, p7, p8]
    return "\n\n".join(blocks)


def case(cid, cat, diff, hops, body, truth_obj, derivation, vi=0):
    prompt = (expand(vi, cat, body, truth_obj) + "\n\n" + CHECK_TAIL
              + dump_truth_keys(truth_obj) + ".")
    doc = {
        "case_id": cid,
        "rea_plus": {"hops": hops, "robust": True,
                     "prio": diff == "H", "format_weight": 2,
                     "archetype": "agentic", "difficulty": diff,
                     "provenance": derivation},
        "prompt": prompt,
        "draft_response": "PLACEHOLDER",
        "auxiliary": "All facts needed are in the prompt itself.",
        "success_criteria": {"deterministic_checks": {
            "json_exact": dump_truth(truth_obj)}},
    }
    (OUT / f"case-{cid}.yml").write_text(
        yaml.safe_dump(doc, sort_keys=False, width=78))
    return cid, dump_truth(truth_obj), derivation


def dump_truth_keys(obj):
    ks = list(obj.keys())
    return "{" + ", ".join(f'"{k}": ...' for k in ks) + "}"


def framed(vi, rules, events, ask):
    o = OPENERS[vi % 4]
    d = DISTRACTORS[(vi + 1) % 4]
    return (f"{o}. {rules}\n\n{events}\n\n{d}. {ask}")


LORE_DEFAULT = (
    "a note on where these rules come from, because it helps you "
    "hold them: every clause above was added after somebody got "
    "burned, the tiebreaks exist because two teams deadlocked once, "
    "and the 'exactly' wording exists because a near-miss reading "
    "of an older draft caused an outage that the postmortem is "
    "still annoyed about. the rules are battle scars, not bureaucracy.")

LORE = {
    "backoff": "the delay-cap clause exists because a storm once "
        "stacked 900-second waits behind a hung endpoint; the "
        "'delays before each retry' phrasing exists because two "
        "auditors counted call-time into the wait and both were "
        "wrong.",
    "ratelimit": "the 'strict rolling window' wording exists "
        "because a fixed-window limiter once let double traffic "
        "through at the boundary and nobody noticed for a week.",
    "canary": "the 'paused step is not completed' clause exists "
        "because a rollout dashboard once painted a parked step "
        "green and an executive read it as done.",
    "cache": "the 'no refresh on read' clause exists because an "
        "older cache refreshed on access and hid a TTL bug for a "
        "month.",
    "sched": "the 2-minute rejection threshold exists because a "
        "late-finish penalty once cost more than the job's entire "
        "value; the cooldown clause exists because back-to-back "
        "runs cooked a worker in june.",
    "resume": "the skip-and-flag behavior exists because a stuck "
        "step once blocked a nightly run until morning.",
    "flag": "the severity filter exists because a severity-1 "
        "looking severity-3 once tripped a kill switch on a "
        "featureship friday and the postmortem banned counting "
        "anything but the rule.",
    "budget": "the promotion order exists because a doc once went "
        "straight to the expensive model and finance noticed.",
    "timeline": "the template definitions exist because three "
        "postmortems once reported three different MTTRs for the "
        "same incident.",
    "semver": "the 'strictly below' clause exists because a "
        "caret-range habit once let a major bump through.",
    "idem": "the dedup ledger exists because a console once showed "
        "three charges where the bank showed one and support "
        "argued with the bank.",
    "migration": "the retry-once rule exists because a flaky "
        "network once burned five retries on one chunk while the "
        "rest of the migration waited.",
    "cascade": "the per-layer budget phrasing exists because a "
        "global timeout once masked which layer actually hung.",
    "quota": "the banking rules exist because two teams once "
        "handshake-lent banked storage and finance made it a "
        "policy; the arrival-order drain exists because someone "
        "tried to split requests to stretch the pool in may.",
    "counterfactual": "the single-branch-point rule exists because "
        "an earlier counterfactual quietly edited two things and "
        "the review argued for a week about which edit mattered.",
    "epistemic": "the precedence ladder exists because an audit "
        "once trusted a pretty dashboard over a machine log and "
        "the VP's office still brings it up.",
    "paging": "the escalation release-then-issue rule exists "
        "because a double-count once convinced on-call they were "
        "out of slots during a quiet tuesday.",
    "freeze": "the never-starts clause exists because a deploy "
        "that would have crossed the freeze by forty seconds "
        "once started anyway and met the freeze halfway.",
    "checkpoint": "the wasted-minutes-count rule exists because "
        "an accountant once billed the crash gap as free.",
    "residency": "the unload-after-each-job rule exists because a "
        "left-resident model once starved the next job's load and "
        "the schedule silently degraded.",
    "weighted": "the remainder-to-first rule exists because three "
        "teams once argued over a single leftover unit for a "
        "week.",
    "cron": "the same-minute definition exists because two jobs "
        "'overlapping' across a second boundary once spawned a "
        "duplicate alert storm.",
    "logs": "the heartbeat exclusion exists because heartbeat "
        "noise once drowned the real error count in a status "
        "report.",
    "invalidation": "the transitive-poison rule exists because a "
        "half-invalidated chain once shipped a stale binary with "
        "fresh tests.",
    "queue": "the no-partial-credit rule exists because a lane "
        "once got credit for half a message and the count never "
        "reconciled after.",
}

WORKED_EX = [
    ("a three-item tally under a cap of 7 with items of 3, 2, and "
     "5 arriving in that order admits 3 then 2 then rejects 5, "
     "landing the count at 5 with one rejection.",
     "notice how the cap is checked against the running total at "
     "each arrival, not against the item alone."),
    ("a window that opens at the first tick and closes on "
     "confirmation spans exactly the distance between those two "
     "events, never the distance plus the observation delay.",
     "the measurement convention, not the physics, defines the "
     "window."),
    ("a pool of 12 drained by requests of 5, 5, and 5 in arrival "
     "order grants the first two, denies the third, and leaves "
     "2 sitting unused because partial grants don't exist.",
     "the remainder is not a failure, it is just unspendable."),
]

ENGINES = []


def engine(name, diff, hops, fn):
    ENGINES.append((name, diff, hops, fn))


# E1 backoff ------------------------------------------------------------
def e_backoff():
    variants = [(3, False), (4, False), (5, False), (7, True)]
    for vi, (k, capped) in enumerate(variants):
        if not capped:
            wait = 2 ** k - 2
            cap_hit = "false"
            truth = {"attempts": k, "wait_s": wait, "cap_hit": False}
            ev = (f"this run succeeded on attempt {k} and no delay ever "
                  f"reached the cap")
        else:
            delays = [min(2 ** i, 30) for i in range(1, k)]
            wait = sum(delays)
            truth = {"attempts": k, "wait_s": wait, "cap_hit": True}
            cap_hit = "true"
            ev = (f"this run succeeded on attempt {k}, and the backoff "
                  f"doubled past the cap twice before getting there")
        rules = ("your retry client doubles backoff from 2 seconds, caps "
                 "any single delay at 30 seconds, gives up after 8 "
                 "attempts, and counts only delays that elapse before "
                 "each retry - the call times themselves don't count "
                 "toward the wait")
        ask = (f"report attempts, total seconds waited, and whether the "
               f"cap was ever hit ({cap_hit} is not the answer, that's "
               f"just whether any single delay got capped)")
        yield framed(vi, rules, ev, ask), truth, (
            f"backoff; delays before retries 1..{k-1}: "
            f"{'+'.join(str(min(2**i,30)) for i in range(1,k))} "
            f"= {truth['wait_s']}s; cap_hit {truth['cap_hit']}")


# E2 rate limit ---------------------------------------------------------
def e_ratelimit():
    for vi, (cap, active, burst) in enumerate(
            [(80, 65, 20), (100, 40, 30), (60, 55, 10), (50, 20, 35)]):
        allowed = min(burst, cap - active)
        truth = {"allowed": allowed, "rejected": burst - allowed,
                 "active": cap}
        rules = (f"the limiter allows {cap} requests per strict rolling "
                 f"60-minute window and rejects excess instantly, no "
                 f"queueing, nothing carries over")
        ev = (f"the window currently holds {active} active requests and "
              f"a fresh burst of exactly {burst} arrives now")
        ask = ("compute how many the burst gets through, how many get "
               "rejected, and where active settles after it lands")
        yield framed(vi, rules, ev, ask), truth, (
            f"rolling window; headroom {cap}-{active} = {cap-active}; "
            f"burst {burst} -> allowed {allowed}, rejected "
            f"{burst-allowed}; settles at cap {cap}")


# E3 canary -------------------------------------------------------------
def e_canary():
    for vi, rates in enumerate([[0.4, 0.8, 1.2, None],
                                [0.2, 0.5, 0.7, 0.9],
                                [1.1, None, None, None],
                                [0.3, 0.6, 0.9, 1.4]]):
        steps = [5, 25, 50, 100]
        viol = next((i for i, r in enumerate(rates) if r is None
                     or r >= 1.0), None)
        if viol is None:
            truth = {"rollout_pct": 100, "stages_done": 4,
                     "status": "COMPLETE"}
        else:
            truth = {"rollout_pct": steps[viol], "stages_done": viol,
                     "status": "PAUSED"}
        meas = ", ".join(
            (f"{r} percent" if r is not None else "not measured")
            for r in rates)
        rules = ("the rollout steps 5, 25, 50, then 100 percent of "
                 "traffic and pauses the moment any step measures "
                 "error at or above 1.0 percent - a completed step is "
                 "one that finished clean, the paused step parks "
                 "traffic but doesn't count as done")
        ev = f"measured rates came in at: {meas}"
        ask = ("report where the rollout sits, how many stages "
               "finished clean, and whether it's PAUSED or COMPLETE")
        yield framed(vi, rules, ev, ask), truth, (
            f"canary; violation at stage {viol}; "
            f"parked {truth['rollout_pct']}% with "
            f"{truth['stages_done']} done")


# E4 cache --------------------------------------------------------------
def e_cache():
    for vi, (ttl, flush, readT, rewrite) in enumerate(
            [(300, 350, 400, ["C"]), (600, 700, 800, ["B", "C"]),
             (300, 200, 280, []), (900, 950, 1000, ["A"])]):
        keys = ["A", "B", "C"]
        wt = {"A": 100, "B": 220, "C": 300}
        hits = len(rewrite)
        truth = {"hits": hits, "misses": 3 - hits}
        wtxt = ", ".join(f"key {k} written at second {wt[k]}" for k in keys)
        rtxt = (", ".join("key " + k for k in rewrite) + " rewritten right "
                "after the flush") if rewrite else "nothing rewritten"
        rules = (f"TTL is exactly {ttl} seconds with no refresh on read, "
                 f"and a flush invalidates everything present at the "
                 f"moment it runs")
        ev = (f"{wtxt}; an operator flushed everything at second "
              f"{flush}; {rtxt}; you read all three keys at second "
              f"{readT}")
        ask = "count hits and misses across the three reads"
        yield framed(vi, rules, ev, ask), truth, (
            f"cache; flush {flush} clears all; rewritten "
            f"{rewrite or 'nothing'} fresh at read {readT} "
            f"(< ttl {ttl}); hits {hits}")


# E5 scheduling (enumerated permutations) ------------------------------
def e_sched():
    varsets = [
        [("J1", 6, 10), ("J2", 3, 8), ("J3", 4, 12)],
        [("J1", 5, 12), ("J2", 4, 6), ("J3", 6, 20)],
        [("J1", 8, 16), ("J2", 2, 5), ("J3", 3, 9)],
        [("J1", 4, 9), ("J2", 7, 15), ("J3", 4, 8)],
    ]
    import itertools
    for vi, jobs in enumerate(varsets):
        names = [j[0] for j in jobs]
        best = None
        for perm in itertools.permutations(jobs):
            t = 0
            met = 0
            rejected = []
            for nm, ln, due in perm:
                end = t + ln
                if end - due >= 2:
                    rejected.append(nm)
                    continue
                if end <= due:
                    met += 1
                t = end + 1
            order_key = tuple(names.index(p[0]) for p in perm)
            if best is None or met > best[0] or (
                    met == best[0] and order_key < best[2]):
                best = (met, perm, order_key, tuple(rejected))
        met, perm, _, rejected = best
        rej = sorted(rejected)
        order = [p[0] for p in perm if p[0] not in rej]
        truth = {"order": order, "met": met, "rejected": rej}
        jtxt = "; ".join(f"{nm} takes {ln} minutes, due at minute {due}"
                         for nm, ln, due in jobs)
        rules = ("one worker, mandatory 1-minute cooldown between jobs "
                 "that the clock does not pause for; maximize deadlines "
                 "met, ties broken by the order that runs earlier-"
                 "submitted jobs first (J1 before J2 before J3); any "
                 "job that would miss its deadline by 2 minutes or "
                 "more gets rejected outright and never starts, while "
                 "a miss under 2 minutes still runs and just counts "
                 "as a miss")
        ev = f"all three landed at once: {jtxt}"
        ask = ("evaluate every order, then report the executed order, "
               "deadlines met, and any rejections")
        yield framed(vi, rules, ev, ask), truth, (
            f"scheduling; enumerated all 6 orders -> best {order} "
            f"meets {met}; rejected {rej} (would-miss >= 2min rule)")


def e_resume():
    for vi, (n, k) in enumerate([(10, 7), (8, 4), (12, 9), (6, 3)]):
        truth = {"executed": n + 1, "flagged": 1, "finished": True}
        rules = ("the retry policy grants a failed step at most 2 more "
                 "attempts, then skips and flags it while the workflow "
                 "runs every remaining step to the end")
        ev = (f"a {n}-step workflow crashed after step {k-1} committed, "
              f"with step {k} having burned 2 failed attempts already; "
              f"this morning's resume burned both extra attempts on "
              f"step {k}, flagged and skipped it, and ran the rest "
              f"clean to the end")
        ask = ("count stage-execution events across both runs - each "
               "attempt counts as one event - plus flags and whether "
               "it finished")
        yield framed(vi, rules, ev, ask), truth, (
            f"resume; night {k-1} events + 2 retries + {n-k} tail "
            f"= {n+1} events; flagged 1; finished true")


# E7 flag kill ----------------------------------------------------------
def e_flag():
    for vi, (orgs, pct, sev2, thr) in enumerate(
            [(600, 20, 2, 3), (400, 25, 3, 3), (900, 10, 2, 2),
             (250, 40, 1, 4)]):
        truth = {"kill": sev2 >= thr, "enabled_orgs": orgs * pct // 100}
        rules = (f"the kill switch trips only on {thr} or more "
                 f"severity-2 incidents inside one hour - other "
                 f"severities never count, doubled dashboard display "
                 f"never counts")
        ev = (f"the flag covers {pct} percent of {orgs} accounts and "
              f"this hour produced {sev2} severity-2 and one "
              f"severity-3")
        ask = ("decide the switch state and the enabled account count")
        yield framed(vi, rules, ev, ask), truth, (
            f"flag gate; {sev2} sev2 vs threshold {thr} -> kill "
            f"{truth['kill']}; {pct}% of {orgs} = "
            f"{truth['enabled_orgs']}")


# E8 budget routing -----------------------------------------------------
def e_budget():
    for vi, (docs, costA, f, costB, legal, costC) in enumerate(
            [(27, 2, 4, 5, 2, 9), (40, 1, 6, 3, 3, 6),
             (30, 2, 3, 4, 2, 7), (50, 1, 5, 2, 5, 4)]):
        spend = docs * costA + f * costB + legal * costC
        truth = {"a_docs": docs, "b_docs": f, "c_docs": legal,
                 "spend_k": spend}
        rules = ("every doc routes to A first, exactly A's failures "
                 "promote to B, legal-flagged docs (never overlapping "
                 "A's failures) go to C, and the plan holds only if "
                 "the total fits the budget - it does")
        ev = (f"{docs} docs at {costA}k each on A; {f} failed and "
              f"promote to B at {costB}k each; {legal} carry the legal "
              f"flag for C at {costC}k each")
        ask = ("report routing counts and total spend")
        yield framed(vi, rules, ev, ask), truth, (
            f"budget routing; {docs}x{costA} + {f}x{costB} + "
            f"{legal}x{costC} = {spend}k")


# E9 timeline -----------------------------------------------------------
def e_timeline():
    for vi, (d, s, r, b) in enumerate([("14:00", "14:23", "14:40", "14:55"),
                                       ("09:00", "09:31", "10:00", "10:12"),
                                       ("22:00", "22:41", "23:00", "23:07"),
                                       ("11:00", "11:12", "11:20", "11:35")]):
        def mins(a, bb):
            (ah, am), (bh, bm) = map(int, a.split(":")), map(int, bb.split(":"))
            return (bh * 60 + bm) - (ah * 60 + am)
        truth = {"detect_min": mins(d, s), "mttr_min": mins(r, b),
                 "impact_min": mins(s, b)}
        rules = ("the postmortem template defines detection as "
                 "deploy-to-spike minutes, MTTR as rollback-to-baseline "
                 "minutes, total impact as spike-to-baseline minutes - "
                 "template definitions verbatim, not the colloquial "
                 "meanings, and two chat messages quoting rounded "
                 "times are not the authority")
        ev = (f"deploy {d}, error spike {s}, rollback {r}, baseline "
              f"restored {b}")
        ask = "compute the three template numbers"
        yield framed(vi, rules, ev, ask), truth, (
            f"timeline; detect {truth['detect_min']}, mttr "
            f"{truth['mttr_min']}, impact {truth['impact_min']}")


# E10 semver ------------------------------------------------------------
def e_semver():
    for vi, (xlo, xhi, zlo, bump) in enumerate(
            [(2, 4, 3.5, 4.1), (1, 5, 2, 4.9), (3, 6, 3, 6.2),
             (2, 3, 4, 2.5)]):
        broken = []
        if not (xlo <= bump < xhi):
            broken.append("X")
        if bump < zlo:
            broken.append("Z")
        truth = {"safe": not broken, "broken": sorted(broken)}
        rules = (f"package X declares Y >= {xlo} and strictly < {xhi}; "
                 f"package Z declares Y >= {zlo}; safe only if every "
                 f"declared range holds, broken packages name "
                 f"themselves sorted alphabetically, and cosmetic "
                 f"transitive warnings never affect ranges")
        ev = f"the environment currently pins Y green and the proposal bumps it to {bump}"
        ask = "decide safety and the broken list"
        yield framed(vi, rules, ev, ask), truth, (
            f"semver; bump {bump} vs X[{xlo},{xhi}) Z>={zlo} -> "
            f"broken {sorted(broken) or 'none'}")


# E11 idempotency -------------------------------------------------------
def e_idem():
    for vi, (kam, n, mam) in enumerate([(49, 3, 25), (120, 4, 60),
                                        (75, 2, 30), (19, 5, 81)]):
        truth = {"charged": kam + mam, "deduped": n - 1}
        rules = ("the server dedupes by request id, charging once per "
                 "unique id; refunds are out of scope and the console "
                 "rendering duplicate line items is display, not "
                 "ledger")
        ev = (f"request K (carrying {kam} dollars) was submitted "
              f"{n} times in a flapping storm and unrelated request M "
              f"({mam} dollars, different cart) went through once")
        ask = "compute total charged and submissions absorbed by dedup"
        yield framed(vi, rules, ev, ask), truth, (
            f"idempotency; K once {kam} + M {mam} = {kam+mam}; "
            f"absorbed {n-1}")


# E12 migration ---------------------------------------------------------
def e_migration():
    for vi, (recs, c, fail, recovered) in enumerate(
            [(200, 25, {3, 7}, {7}), (120, 20, {2}, set()),
             (300, 50, {2, 4, 6}, {4}), (160, 40, {3}, {3})]):
        chunks = recs // c
        dead = sorted(fail - recovered)
        truth = {"processed": recs - len(dead) * c,
                 "deadletter": len(dead) * c,
                 "retries": len(fail)}
        rules = (f"{recs} records split into equal chunks of {c}, one "
                 f"chunk at a time; on chunk failure retry that chunk "
                 f"exactly once, then dead-letter it and continue to "
                 f"the end")
        ftxt = ", ".join(f"chunk {i}" for i in sorted(fail))
        rtxt = (", ".join("chunk " + str(i) for i in sorted(recovered))
                + " recovered") if recovered else "none recovered"
        ev = f"first-pass failures: {ftxt}; on retry: {rtxt}"
        ask = ("tally processed records, dead-lettered records, and "
               "retries used - the external reconciler that claims "
               "dead letters later stays out of your counts")
        yield framed(vi, rules, ev, ask), truth, (
            f"migration; {chunks} chunks; dead {dead} -> "
            f"{len(dead)*c} records; retries {len(fail)}")


# E13 timeout cascade ---------------------------------------------------
def e_cascade():
    for vi, (abud, batt, cap) in enumerate([(5, 2, 15), (4, 2, 12),
                                            (6, 3, 20), (3, 1, 8)]):
        b_total = 2 * batt
        a_to = 1 if b_total > abud else 0
        retry_total = 2 * b_total
        if retry_total <= cap:
            latency, supp = retry_total, False
        else:
            latency, supp = b_total, True
        truth = {"b_timeouts": 2, "a_timeouts": a_to,
                 "latency_s": latency, "suppressed": supp}
        rules = (f"A gives B {abud} seconds total; B gives C "
                 f"{batt} seconds per attempt and retries C exactly "
                 f"once; A retries a failed whole call exactly once "
                 f"with its budget doubled; ops caps total "
                 f"caller-observed latency at {cap} seconds and "
                 f"suppresses any attempt that would cross it")
        ev = "C hung completely, so B burned both attempts, and the retry failed identically"
        ask = ("count B's timed-out attempts, A's own timeouts, the "
               "latency the caller observed, and whether anything got "
               "suppressed")
        yield framed(vi, rules, ev, ask), truth, (
            f"cascade; B 2x{batt} = {b_total} vs A {abud} -> a_to "
            f"{a_to}; retry total {retry_total} vs cap {cap} -> "
            f"latency {latency}, suppressed {supp}")


# E14 quota banking (simulated) ----------------------------------------
def e_quota():
    teams = ["atlas", "borealis", "cascade"]
    varsets = [
        ({"used": 70, "flag": 20}, {"used": 100, "flag": 0},
         {"used": 40, "flag": 45}, 50,
         [("borealis", 40), ("atlas", 15), ("cascade", 30)]),
        ({"used": 80, "flag": 10}, {"used": 90, "flag": 5},
         {"used": 50, "flag": 30}, 40,
         [("atlas", 25), ("cascade", 20)]),
        ({"used": 60, "flag": 30}, {"used": 100, "flag": 0},
         {"used": 30, "flag": 55}, 60,
         [("cascade", 35), ("borealis", 20), ("atlas", 10)]),
        ({"used": 100, "flag": 0}, {"used": 70, "flag": 25},
         {"used": 80, "flag": 15}, 30,
         [("borealis", 30), ("atlas", 30)]),
    ]
    for vi, (a, b, c, pool, reqs) in enumerate(varsets):
        cfg = dict(zip(teams, [a, b, c]))
        banks = {t: cfg[t]["flag"] for t in teams}
        granted = {t: 0 for t in teams}
        denied = []
        left = pool
        for t, amt in reqs:
            if amt <= left:
                granted[t] = amt
                left -= amt
            else:
                denied.append(t)
        caps = {f"{t[:3]}_cap": 100 + banks[t] + granted[t]
                for t in teams}
        truth = dict(caps)
        truth["pool_left"] = left
        truth["denied"] = sorted(denied)
        usetxt = "; ".join(f"{t} used {cfg[t]['used']} TB and flagged "
                           f"{cfg[t]['flag']} of its unused"
                           for t in teams)
        reqtxt = "; ".join(f"{t} requested {amt} TB"
                           for t, amt in reqs)
        rules = ("100 TB base each, base never rolls over; flagged "
                 "unused banks into exactly next month then expires if "
                 "unused; banks are per-team with no transfers; the "
                 f"overflow pool holds {pool} TB, grants one-shot "
                 "requests strictly in arrival order each in full "
                 "until empty, no splitting, denials cost nothing and "
                 "touch nothing")
        ev = (f"last month: {usetxt}. this morning's queue in arrival "
              f"order: {reqtxt}")
        ask = ("compute each team's capacity for the coming month "
               "(base + bank + granted overflow), the pool's remainder, "
               "and the denied list alphabetical by proper name")
        yield framed(vi, rules, ev, ask), truth, (
            f"quota; banks {banks}; pool {pool} drains -> granted "
            f"{granted}, denied {sorted(denied)}, left {left}")


# E15 counterfactual ----------------------------------------------------
def e_counterfactual():
    common_rules = ("the bug came from flag F going live and everything "
                    "downstream cascades from that single point: "
                    "removing F at moment T removes the cascade from T "
                    "onward instantly, mitigation is downstream of the "
                    "bug so it never fires at nothing, rollbacks exist "
                    "only as responses to the bug, and everything not "
                    "downstream of F replays identically; the elevated "
                    "window is spike-to-zero per the reliability team's "
                    "convention - no spike, no window, full stop")
    variants = [(20, 30, 0, 2), (15, 30, 0, 2), (25, 30, 0, 2),
                (35, 30, 5, 2)]
    for vi, (offT, spikeT, elev, deps) in enumerate(variants):
        truth = {"elevated_min": elev, "m1_runs": 0, "rollbacks": 0,
                 "deploys_total": deps}
        ev = (f"actual timeline: D-200 ships with F live at t=0; F "
              f"promoted to 100 percent at t=10; spike hits at t={spikeT}; "
              f"mitigation M1 applied t=40; rollback R1 turns F off at "
              f"t=80; errors zero by t=90; unrelated deploy D-201 ships "
              f"at t=120")
        ask = (f"counterfactual: F turns OFF at t={offT}. report "
               f"elevated minutes under the convention, whether M1 "
               f"ever runs, rollbacks executed, and deploys t=0..120 "
               f"inclusive in the counterfactual")
        if elev:
            deriv = (f"counterfactual; off at {offT} AFTER spike "
                     f"{spikeT} -> window {spikeT}..{offT} = {elev}; "
                     f"no M1 (bug gone before t=40); deploys {deps}")
        else:
            deriv = (f"counterfactual; off at {offT} precedes spike "
                     f"{spikeT} -> no window, no M1, no rollback; "
                     f"deploys {deps}")
        yield framed(vi, common_rules, ev, ask), truth, deriv


# E16 epistemic ---------------------------------------------------------
def e_epistemic():
    # pattern of verdicts for claims 1..5; sources fixed: log >
    # signed handoff > hearsay/dashboard(unreliable)
    pats = [("T", "F", "F", "U", "T"), ("T", "T", "F", "U", "F"),
            ("T", "F", "T", "U", "T"), ("F", "T", "F", "U", "T")]
    for vi, pat in enumerate(pats):
        t = pat.count("T"); f = pat.count("F"); u = pat.count("U")
        truth = {"true": t, "false": f, "unverified": u}
        rules = ("machine log is authoritative for deploys, rollbacks, "
                 "freezes; the signed handoff is authoritative for "
                 "paging; unsigned lines are hearsay with zero weight; "
                 "the dashboard doubles every count and is "
                 "authoritative for nothing - a claim is TRUE if the "
                 "highest-precedence covering source supports it, "
                 "FALSE if it contradicts it, UNVERIFIED only if "
                 "nothing authoritative covers it, and you don't fall "
                 "back to rescue it")
        ev = ("log: deploy D-9 shipped 13:05, rollback RB-2 executed "
              "15:10, freeze began 16:00; signed handoff: primary "
              "paged 13:20, confirmed rollback 15:12, no secondary "
              "page listed anywhere authoritative; one unsigned line "
              "claims a secondary page; dashboard shows pages sent: 2 "
              "and peak error 6.2 percent")
        claims = {
            "T": ("deploy shipped at 13:05 (log confirms)",
                  "freeze began at 16:00 (log confirms)",
                  "rollback executed 15:10 (log confirms)"),
            "F": ("no rollbacks occurred (log contradicts)",
                  "both on-calls were paged (signed handoff lists "
                  "primary only)", "primary paged at 13:25 (signed "
                  "says 13:20)"),
            "U": ("peak error hit 6.2 percent (dashboard-only "
                  "numbering)", "pages sent: 2 (doubling bug)")}
        picked = []
        ti = fi = ui = 0
        for v in pat:
            src = {"T": claims["T"], "F": claims["F"], "U": claims["U"]}[v]
            idx = {"T": ti, "F": fi, "U": ui}
            picked.append(src[idx[v] % len(src)])
            if v == "T": ti += 1
            elif v == "F": fi += 1
            else: ui += 1
        ev += ". the recap claims: " + " | ".join(picked)
        ask = ("classify all five claims and count each verdict - "
               "opinions and spreadsheet recomputations from the "
               "dashboard are out of scope")
        yield framed(vi, rules, ev, ask), truth, (
            f"epistemic; pattern {''.join(pat)} -> {t}T/{f}F/{u}U")


# E17 paging war-room (hand-computed variants) -------------------------
def e_paging():
    variants = [
        ("""sev2 A opened 08:30 and is still open; sev2 B opened 09:15,
            resolved 10:40; sev1 C opened 11:00, resolved 12:20;
            deploys attempted 11:30 and 13:00""", 6, 2, 1, 1),
        ("""sev2 A opened 10:00, resolved 10:50; sev2 B opened 10:10,
            resolved 13:20; sev1 D opened 15:00, resolved 15:45;
            deploy attempted 15:20""", 4, 0, 0, 1),
        ("""sev1 A opened 09:00, resolved 09:40; sev2 B opened 09:50
            and is still open at end of day; deploys attempted 09:10
            and 16:00""", 5, 2, 1, 1),
        ("""sev2 A opened 11:00, resolved 11:20; sev2 B opened 11:05,
            resolved 14:30; sev1 C opened 16:00, resolved 16:30;
            deploy attempted 16:10""", 4, 0, 0, 1),
    ]
    for vi, (evtxt, pages, slots, esc, blocked) in enumerate(variants):
        truth = {"pages_issued": pages, "slots_in_use": slots,
                 "escalations": esc, "deploys_blocked": blocked}
        rules = ("sev1 pages primary+secondary immediately (2 pager "
                 "slots); sev2 in business hours 09:00-17:00 pages "
                 "primary only (1 slot); sev2 outside those hours "
                 "pages the on-call rotation (1 slot); sev3 never "
                 "pages; a sev2 open longer than 240 minutes is "
                 "re-treated as sev1 - its earlier single page is "
                 "released and a fresh pair is issued; slots come "
                 "from a pool of six and free on resolve; while any "
                 "sev1 is open every deploy is blocked and counted")
        ask = ("as of end of day: total pages ever issued, pager "
               "slots currently in use, escalations that fired, "
               "deploys blocked")
        yield framed(vi, rules, evtxt.replace("\n", " "), ask), truth, (
            f"paging sim; hand-derived: pages {pages}, slots {slots}, "
            f"esc {esc}, blocked {blocked}")


def e_freeze():
    varsets = [
        ("20:35", [("gateway", 10, True), ("worker", 12, True),
                   ("batch", 5, False)], "21:00"),
        ("19:00", [("api", 15, True), ("jobs", 20, True),
                   ("web", 8, False)], "19:45"),
        ("22:10", [("auth", 12, False), ("sync", 9, False)],
         "23:00"),
        ("08:50", [("search", 20, True), ("indexer", 15, True),
                   ("queue", 10, True), ("audit", 6, False)],
         "09:40"),
    ]
    for vi, (now, jobs, freeze) in enumerate(varsets):
        fh, fm = map(int, freeze.split(":"))
        cur = int(now[:2]) * 60 + int(now[3:])
        t = cur
        shipped = 0
        for name, dur, needed in jobs:
            if needed:
                end = t + dur
                if end <= fh * 60 + fm:
                    shipped += 1
                    t = end
        truth = {"shipped": shipped,
                 "pending": len(jobs) - shipped,
                 "finished_by": freeze}
        jtxt = "; ".join(f"{n} needs a {d}-minute deploy"
                         + ("" if req else " and doesn't need to ship "
                            "tonight") for n, d, req in jobs)
        rules = (f"deploys run sequentially in the listed order, and "
                 f"work not finished by the {freeze} freeze waits for "
                 f"tomorrow - a deploy that would cross the freeze "
                 f"never starts")
        ev = f"it's {now}. {jtxt}"
        ask = ("how many shipped tonight, how many went pending, and "
               "the freeze time they bump against")
        yield framed(vi, rules, ev, ask), truth, (
            f"freeze; sequential from {now} vs {freeze} -> shipped "
            f"{shipped}/{len(jobs)}")


# E19 checkpoint replay -------------------------------------------------
def e_checkpoint():
    for vi, (n, s, k, w) in enumerate([(5, 4, 3, 2), (6, 3, 2, 1),
                                       (4, 5, 3, 3), (8, 2, 5, 2)]):
        truth = {"wall_min": n * s + w, "stages_rerun": 1}
        rules = (f"{n} stages at {s} minutes each, checkpoints after "
                 f"every stage, replay from the last checkpoint only; "
                 "wasted partial minutes still burn wall clock")
        ev = (f"stages 1..{k} committed, then the run crashed {w} "
              f"minutes into stage {k+1}; the replay redid stage "
              f"{k+1} from scratch and ran the rest clean")
        ask = "total wall minutes across both runs and stages re-executed"
        yield framed(vi, rules, ev, ask), truth, (
            f"checkpoint; {n}x{s} + {w} wasted = {n*s+w}; rerun 1")


# E20 residency (simulated) --------------------------------------------
def e_residency():
    varsets = [
        ([("A", 5), ("B", 3), ("C", 5)], {"m1": 5, "m2": 3},
         {"m1", "m2"}),
        ([("A", 5), ("B", 3), ("C", 3)], {"m1": 5, "m2": 3},
         {"m1"}),
        ([("A", 3), ("B", 5), ("C", 5)], {"m1": 3, "m2": 5},
         {"m2"}),
        ([("A", 5), ("B", 3), ("C", 5), ("D", 3)], {"m1": 5, "m2": 3},
         {"m1", "m2"}),
    ]
    for vi, (jobs, sizes, initially) in enumerate(varsets):
        resident = set(initially)
        loads = unloads = 0
        model_of = {nm: sz for nm, sz in [("m1", sizes["m1"]),
                                          ("m2", sizes["m2"])]}
        need = {nm: sz for nm, sz in
                [(j, s) for j, s in jobs]}
        for j, sz in jobs:
            m = "m1" if sz == sizes["m1"] else "m2"
            if m not in resident:
                loads += 1
                resident.add(m)
            resident.discard(m)
            unloads += 1
        truth = {"loads": loads, "load_s": loads * 30,
                 "unload_s": unloads * 10}
        jtxt = ", ".join(f"{j} needs the {sz} GB model"
                         for j, sz in jobs)
        itxt = ("both models" if len(initially) == 2 else
                f"only the {sorted(initially)[0] if len(initially)==1 else 'one'} model")
        rules = ("8 GB residency cap, a running job's model must be "
                 "resident at start, after each job its model unloads "
                 "for maintenance, each load costs 30 seconds and "
                 "each unload 10")
        ev = (f"jobs run strictly in order: {jtxt}; {itxt} begins "
              f"loaded")
        ask = ("total loads, load seconds, and unload seconds across "
               "the whole schedule")
        yield framed(vi, rules, ev, ask), truth, (
            f"residency sim; loads {loads} ({loads*30}s), unloads "
            f"{unloads} ({unloads*10}s)")


# E21 weighted split ----------------------------------------------------
def e_weighted():
    for vi, (W, ws) in enumerate([(100, (2, 3, 5)), (90, (1, 1, 1)),
                                  (120, (1, 2, 3)), (80, (3, 1, 1))]):
        tot = sum(ws)
        shares = [W * w // tot for w in ws]
        rem = W - sum(shares)
        shares[0] += rem
        truth = {"alpha": shares[0], "beta": shares[1],
                 "gamma": shares[2], "remainder_added_to": "alpha"}
        rules = (f"split a {W}-unit budget across alpha, beta, gamma "
                 f"by weights {ws[0]}:{ws[1]}:{ws[2]}, integer shares "
                 f"only, any remainder lands on alpha")
        ask = "the three shares and where the remainder went"
        yield framed(vi, rules, "the weights are final and certified", ask), truth, (
            f"weighted; shares {shares} (remainder {rem} to alpha)")


# E22 cron overlap ------------------------------------------------------
def e_cron():
    for vi, (i1, i2, off) in enumerate([(3, 4, 1), (2, 5, 3),
                                        (4, 6, 2), (5, 3, 0)]):
        count = sum(1 for t in range(60)
                    if t % i1 == 0 and (t - off) % i2 == 0 and t >= off)
        truth = {"overlaps_per_hour": count,
                 "job1_fires": sum(1 for t in range(60) if t % i1 == 0),
                 "job2_fires": sum(1 for t in range(60)
                                   if (t - off) % i2 == 0 and t >= off)}
        rules = (f"job 1 fires every {i1} minutes from the top of the "
                 f"hour; job 2 fires every {i2} minutes starting at "
                 f"minute {off}; each run takes under a minute so "
                 "overlap means firing in the same minute")
        ask = ("count same-minute overlaps inside one hour plus each "
               "job's fire count")
        yield framed(vi, rules, "both schedules are active all hour", ask), truth, (
            f"cron; lcm-ish brute force -> {count} overlaps")


# E23 log counts --------------------------------------------------------
def e_logs():
    for vi, (info, warn, err, hb) in enumerate(
            [(120, 14, 3, 40), (200, 9, 0, 60), (80, 22, 5, 25),
             (150, 6, 2, 50)]):
        truth = {"infos": info, "warns": warn, "errors": err,
                 "total_nonheartbeat": info + warn + err}
        rules = ("count the log by level, then subtract heartbeat "
                 "noise entirely - heartbeats are not log events for "
                 "these numbers, and a mirrored error file doubles "
                 "nothing because mirroring is display")
        ev = (f"this hour's log: {info} info lines, {warn} warnings, "
              f"{err} errors, and {hb} heartbeat lines")
        ask = "per-level counts and the non-heartbeat total"
        yield framed(vi, rules, ev, ask), truth, (
            f"logs; {info}+{warn}+{err} = {info+warn+err}; hb "
            f"{hb} excluded")


# E24 invalidation ------------------------------------------------------
def e_invalidation():
    chains = [
        (["fetch", "lint", "compile", "test", "package"], "lint",
         None),
        (["pull", "build", "test", "publish"], "test", None),
        (["extract", "transform", "load"], "transform", None),
        (["checkout", "build", "test", "package", "deploy"],
         "build", ["sync-docs"]),
    ]
    for vi, (steps, stale, unrelated) in enumerate(chains):
        k = steps.index(stale)
        rerun = steps[k:]
        truth = {"rerun": rerun, "first_invalid": stale,
                 "untouched": unrelated or []}
        stxt = ", ".join(steps)
        utxt = (f" an unrelated chain ({', '.join(unrelated)}) shares "
                f"the runner but depends on nothing here") if unrelated \
            else ""
        rules = ("each step depends on the one before it; a stale "
                 "input at one step poisons that step and everything "
                 "downstream, while upstream steps stay valid and "
                 "out-of-scope work never enters the rerun list")
        ev = (f"the pipeline ran {stxt}{utxt}; afterwards you learn "
              f"the {stale} step used a stale input")
        ask = ("list the steps to rerun in execution order, the first "
               "invalid step, and any untouched work")
        yield framed(vi, rules, ev, ask), truth, (
            f"invalidation; stale at {stale} -> rerun {rerun}; "
            f"untouched {unrelated or []}")


# E25 queue drain -------------------------------------------------------
def e_queue():
    for vi, (h, m, l, c) in enumerate([(7, 4, 2, 9), (5, 6, 3, 8),
                                       (4, 2, 6, 6), (9, 3, 3, 12)]):
        rem = c
        ph = min(h, rem); rem -= ph
        pm = min(m, rem); rem -= pm
        pl = min(l, rem); rem -= pl
        truth = {"processed_high": ph, "processed_med": pm,
                 "processed_low": pl,
                 "left_total": (h - ph) + (m - pm) + (l - pl)}
        rules = (f"three priority lanes drain HIGH then MED then LOW; "
                 f"the worker processes exactly {c} messages per pass "
                 f"and nothing carries partial credit between lanes")
        ev = f"arrivals: {h} HIGH, {m} MED, {l} LOW"
        ask = ("per-lane processed counts and what's left after the "
               "pass")
        yield framed(vi, rules, ev, ask), truth, (
            f"queue; cap {c} -> H{ph} M{pm} L{pl}; left "
            f"{truth['left_total']}")


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for p in OUT.glob("case-*.yml"):
        p.unlink()
    cases = []
    fixtures = ["# agentic-100 truths. Computed by the same arithmetic "
                "the prompts state (gen-agentic100-cases.py rule "
                "engines). NEVER from model output.\n"]
    manifest = ["# AGENTIC-100 Manifest\n",
                "100 cases, 25 engines x 4 variants. Difficulty H "
                "engines: sched, quota, counterfactual, epistemic, "
                "paging, residency, cascade.\n",
                "| case | engine | difficulty | words | keys |",
                "|---|---|---|---|---|"]
    H_ENGINES = {"sched", "quota", "counterfactual", "epistemic",
                 "paging", "residency", "cascade"}
    HOPS = {"sched": 6, "quota": 6, "counterfactual": 6, "epistemic": 6,
            "paging": 8, "residency": 6, "cascade": 6}
    HARD_VARIANTS = {"a1-checkpoint-02", "a1-checkpoint-03",
                     "a1-idem-04", "a1-residency-04"}
    for name, _d, _h, fn in ENGINES:
        for vi, (body, truth, deriv) in enumerate(fn()):
            cid = f"a1-{name}-{vi+1:02d}"
            diff = ("H" if name in H_ENGINES else "L")
            if cid in HARD_VARIANTS:
                diff = "H"
            hops = HOPS.get(name, 4)
            c, t, dv = case(cid, name, diff, hops, body,
                            truth, deriv, vi=vi + 1)
            cases.append(cid)
            fixtures.append(f"{cid}: "
                f"{json.dumps(json.loads(t))!r}\n# truth: {dv}\n")
            manifest.append(
                f"| {cid} | {name} | {diff} | "
                f"{len(body.split()) + 40} | "
                f"{', '.join(truth.keys())} |")
    FIX.write_text("\n".join(fixtures) + "\n")
    MAN.write_text("\n".join(manifest) + "\n")
    print(f"wrote {len(cases)} cases, fixtures, manifest")


def _register():
    for nm, diff in [("backoff", "L"), ("ratelimit", "L"),
                     ("canary", "L"), ("cache", "L"), ("sched", "H"),
                     ("resume", "L"), ("flag", "L"), ("budget", "L"),
                     ("timeline", "L"), ("semver", "L"),
                     ("idem", "L"), ("migration", "L"),
                     ("cascade", "H"), ("quota", "H"),
                     ("counterfactual", "H"), ("epistemic", "H"),
                     ("paging", "H"), ("freeze", "L"),
                     ("checkpoint", "L"), ("residency", "H"),
                     ("weighted", "L"), ("cron", "L"), ("logs", "L"),
                     ("invalidation", "L"), ("queue", "L")]:
        fn = {"backoff": e_backoff, "ratelimit": e_ratelimit,
              "canary": e_canary, "cache": e_cache, "sched": e_sched,
              "resume": e_resume, "flag": e_flag, "budget": e_budget,
              "timeline": e_timeline, "semver": e_semver,
              "idem": e_idem, "migration": e_migration,
              "cascade": e_cascade, "quota": e_quota,
              "counterfactual": e_counterfactual,
              "epistemic": e_epistemic, "paging": e_paging,
              "freeze": e_freeze, "checkpoint": e_checkpoint,
              "residency": e_residency, "weighted": e_weighted,
              "cron": e_cron, "logs": e_logs,
              "invalidation": e_invalidation, "queue": e_queue}[nm]
        hops = {"sched": 6, "quota": 6, "counterfactual": 6,
                "epistemic": 6, "paging": 8, "residency": 6,
                "cascade": 6}.get(nm, 4)
        engine(nm, diff, hops, fn)


_register()
if __name__ == "__main__":
    main()
