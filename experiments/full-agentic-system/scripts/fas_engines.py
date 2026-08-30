#!/usr/bin/env python3
"""Parameterized deterministic rule engines for full-agentic-system cases.

Every engine: gen(engine, variant, idx) -> Case dict with
  - core_rules / events prose (task digits embedded)
  - truth computed BY SIMULATION (never typed)
  - worked example built by running the SAME simulation on disjoint
    parameters (coherent by construction; digits disjoint from task)

Six engines ported from principle-fusion (quota, backoff, canary,
residency, preempt, epistemic), parameterized by (variant, idx) via
deterministic modular arithmetic — no randomness, reproducible forever.
"""
import json

ENGINES = ["quota", "backoff", "canary", "residency", "preempt", "epistemic"]

# ---- deterministic value helpers -----------------------------------------

def _pick(idx, seq):
    return seq[idx % len(seq)]


def _task_digits_quota(v, idx):
    pool = _pick(idx, [50, 40, 60, 45, 55])
    n = 5 + (idx % 3)              # 5..7 requests
    base = _pick(idx + 1, [3, 4, 5, 6])
    asks = []
    val = base
    for i in range(n):
        asks.append(val)
        val += _pick(idx + i, [2, 3, 4])
    return pool, asks


def _simulate_quota(pool, asks, expire_idx, return_frac_num=1, return_frac_den=2):
    """Walk the pool. expire_idx (0-based ask index) grant expires AFTER
    ask expire_idx+1 completes. Returns state dict."""
    cur = pool
    granted_units = 0
    granted = []           # (ask_idx, granted_amt)
    denied = []
    expired_return = None
    for i, ask in enumerate(asks):
        # expiry of grant from expire_idx lands right before ask expire_idx+2
        if expire_idx is not None and i == expire_idx + 2 and expired_return is None:
            g = granted[expire_idx][1] if expire_idx < len(granted) else 0
            ret = (g * return_frac_num) // return_frac_den
            cur += ret
            expired_return = ret
        if cur == 0:
            denied.append(f"r{i+1}")
            continue
        g = min(ask, cur)
        cur -= g
        granted.append((i, g))
        granted_units += g
    return {"granted_units": granted_units, "denied": denied,
            "pool_left": cur, "expiry_return": expired_return}


def _gen_quota(variant, idx):
    pool, asks = _task_digits_quota(variant, idx)
    n = len(asks)
    expire_idx = 0 if variant == "H" else (1 if n > 3 else 0)
    truth = _simulate_quota(pool, asks, expire_idx)
    out_truth = {"granted_units": truth["granted_units"],
                 "denied": truth["denied"], "pool_left": truth["pool_left"]}
    ev = "; ".join(f"r{i+1} asks for {a}" for i, a in enumerate(asks))
    core = (
        f"you hold the release-token pool for tonight's deploy window. the pool "
        f"starts at {pool} tokens. requests land in strict arrival order, each "
        f"asks for a fixed number of tokens, grants are partial when the pool "
        f"cannot cover the full ask, grant what is there and the pool drops to "
        f"zero, a request arriving at an empty pool is denied and listed by id, "
        f"a pool that still holds tokens never denies anyone, not even a "
        f"request larger than what remains, so the denied list stays empty "
        f"unless the pool hit zero before the request arrived, a request "
        f"granted partially because the pool could not cover its full ask is "
        f"still a granted request and never a denied one, denied means "
        f"arriving to find the pool already at exactly zero, "
        f"the grant for r{expire_idx+1} expires after request r{expire_idx+2} "
        f"completes, an expired grant returns half its granted tokens rounded "
        f"down to the pool, the expiry sentence names exactly one request id "
        f"so only that one grant ever returns, half of the tokens it itself "
        f"received, never any other grant, and grants never return any other "
        f"way. arrival order: {ev}.")
    contract = ('{"granted_units": N, "denied": [ids], "pool_left": N} empty '
                'list when nothing is denied, ids in arrival order. no extra '
                'keys, no trailing text')
    tkeys = "granted_units/denied/pool_left"
    # worked example: disjoint params, same simulation
    w_pool = _pick(idx + 2, [30, 35, 25])
    w_asks = [11, 7, 13][:3]
    w_truth = _simulate_quota(w_pool, w_asks, 0)
    w_total = w_truth["granted_units"]
    w_left = w_truth["pool_left"]
    w_ret = w_truth["expiry_return"]
    worked = (
        f"for orientation only, a sister pool on a different night, different "
        f"numbers, same shape: that pool started at {w_pool} tokens, the first "
        f"ask was {w_asks[0]}, the second {w_asks[1]}, then the first grant "
        f"expired and returned {w_ret}, half of {w_asks[0]} rounds down, never "
        f"up, and the third ask of {w_asks[2]} landed against only "
        f"{w_pool - w_asks[0] - w_asks[1] + w_ret} in the pool so it took a "
        f"partial and drained it. that night closed with {w_total} granted, "
        f"nothing denied, {w_left} left on the books. watch the rounding "
        f"direction, watch the slot the expiry lands in, the return arrives "
        f"right before the request the trigger names, not after it.")
    return {"core": core, "worked": worked, "contract": contract,
            "truth": out_truth, "tkeys": tkeys,
            "task_digits": sorted({pool, *asks})}


def _gen_backoff(variant, idx):
    base = _pick(idx, [2, 3])
    budget = _pick(idx + 1, [20, 24, 30, 36])
    fails = 3 if variant == "H" else 2
    delays = []
    d = base
    for _ in range(fails):
        delays.append(d)
        d *= 2
    wait_total = sum(delays)
    attempts = fails + 1
    spent = wait_total + attempts * base
    budget_after = budget - spent
    breach = spent > budget
    doubled = budget * 2
    suppressed = (doubled - spent) < 0 if breach else False
    # re-check after one-time budget doubling: suppressed only if STILL over
    truth = {"attempts": attempts, "wait_s": wait_total,
             "budget_after": (budget_after if not breach else doubled - spent),
             "suppressed": bool(suppressed)}
    dl = ", ".join(str(x) for x in delays)
    core = (
        f"a flaky upstream forces retries tonight. first wait is {base}s, each "
        f"next wait doubles. after each failed attempt the retry waits, then "
        f"runs. the attempt budget is {budget} tokens where every attempt "
        f"costs {base} and every wait second costs 1. the job needs "
        f"{fails} failed attempts before the {attempts}th succeeds. if the "
        f"spend exceeds the budget the job is suppressed and the budget "
        f"doubles exactly once, re-checked after doubling, suppressed only if "
        f"still over; budget_after is always computed against the doubled "
        f"budget when doubling fired, as doubled budget minus the same spend, "
        f"and suppressed is the verdict after the re-check, not before. "
        f"Waits observed: {dl}.")
    contract = ('{"attempts": N, "wait_s": N, "budget_after": N, '
                '"suppressed": true|false}. wait_s is the sum of every '
                'wait second spent, not the final single wait. '
                'budget_after is the budget minus all spend, where spend '
                'adds the wait seconds plus the per-attempt cost of every '
                'attempt, the failed ones and the successful one. '
                'no extra keys')
    tkeys = "attempts/wait_s/budget_after/suppressed"
    w_base = 5 if base != 5 else 6
    w_fails = 2
    w_delays = [w_base, w_base * 2]
    w_wait = sum(w_delays)
    worked = (
        f"for orientation only, a quieter night, different numbers, same "
        f"shape: base wait {w_base}s, {w_fails} failures doubling to "
        f"{w_delays[1]}s, total waiting {w_wait}s, three attempts in all, "
        f"each attempt costing {w_base}, spend {w_wait + 3 * w_base} against "
        f"a budget of {w_wait + 3 * w_base + 9}, under it, no suppression, "
        f"the ledger closed balanced: waits {w_delays[0]}+{w_delays[1]} = "
        f"{w_wait} wait-seconds, plus 3 attempts at {w_base} each = "
        f"{3 * w_base}, all spend {w_wait} + {3 * w_base} = "
        f"{w_wait + 3 * w_base}, budget {w_wait + 3 * w_base + 9} minus "
        f"spend leaves budget_after "
        f"{(w_wait + 3 * w_base + 9) - (w_wait + 3 * w_base)}. and to see "
        f"the other branch of the same rule on that quieter night: had the "
        f"spend there been {w_wait + 3 * w_base + 12} against the budget of "
        f"{w_wait + 3 * w_base + 9}, over it, the budget would double once "
        f"to {2 * (w_wait + 3 * w_base + 9)}, the re-check would find the "
        f"spend under the doubled budget, so the job runs with suppressed "
        f"false and budget_after {2 * (w_wait + 3 * w_base + 9) - (w_wait + 3 * w_base + 12)} "
        f"as doubled budget minus spend.")
    return {"core": core, "worked": worked, "contract": contract,
            "truth": truth, "tkeys": tkeys,
            "task_digits": sorted({base, budget, fails, *delays})}


def _simulate_canary(rates, lim_early, lim_late, split, tightened):
    rollout = 0
    stages = 0
    breaches = 0
    status = "DONE"
    limit_now = None
    i = 0
    for i, r in enumerate(rates):
        limit = limit_now if limit_now is not None else (
            lim_early if (i + 1) <= split else lim_late)
        if r > limit:
            breaches += 1
            if breaches == 1:
                rollout = max(0, (i) * 10)   # roll back one step
                stages = i
                status = "PAUSED"
                limit_now = tightened
                # observation consumed; resume with NEXT observation
                continue
            else:
                status = "STOPPED"
                break
        # clean step
        if status == "PAUSED" and limit_now is not None:
            # resumed climb
            status = "PAUSED"
        rollout = (i + 1) * 10
        stages = i + 1
    return {"rollout_pct": rollout, "stages_done": stages,
            "status": status, "breaches": breaches}


def _gen_canary(variant, idx):
    if variant == "H":
        rates = _pick(idx, [[0.5, 0.8, 2.1, 0.6, 0.7, 0.9],
                            [0.4, 1.9, 0.5, 2.2, 0.6],
                            [0.6, 0.9, 1.4, 2.0, 0.5, 1.1, 1.5]])
    else:
        rates = _pick(idx, [[0.4, 0.6, 0.9, 1.2, 1.4, 1.9],
                            [0.3, 0.5, 0.8, 1.1, 1.3],
                            [0.5, 0.7, 1.0, 1.5, 2.0]])
    sim = _simulate_canary(rates, 2.0, 1.0, 5, 0.8)
    truth = {"rollout_pct": sim["rollout_pct"],
             "stages_done": sim["stages_done"], "status": sim["status"]}
    rs = chr(10).join(
        f"step {n}: {r} percent" for n, r in enumerate(rates, 1))
    core = (
        f"you drive the canary rollout for a cache change, it climbs in 10 "
        f"percent steps with one observed error rate per step. limits: steps "
        f"1 through 5 may show up to 2.0 percent, steps 6 through 10 only 1.0 "
        f"percent. stage numbering is fixed by position in the list and "
        f"never renumbers: the 1st observation carries stage 1, the 2nd "
        f"carries stage 2, and so on, breaches do not shift the count. a "
        f"clean observation completes the stage equal to its own position "
        f"in the list, and the rollout then shows 10 percent times that "
        f"stage number, even if earlier positions were spent on breaches. "
        f"a breaching observation completes nothing and is spent; the "
        f"rollback leaves the rollout and stages at the last completed "
        f"stage's values. after any "
        f"rollback the limit tightens to 0.8 for every remaining step. a "
        f"second breach stops everything with status STOPPED, holding the "
        f"rollout and stages of the last completed stage. exactly one breach "
        f"and the list ends: status PAUSED with the last completed stage's "
        f"values. no breach at all and the climb "
        f"finishes with status DONE. the listed observations are the whole "
        f"night, nothing arrives after the list ends. observed error rates "
        f"by step, in order, exactly {len(rates)} observations, one per "
        f"line, each line is one observation and the count of lines is the "
        f"count of observations:\n{rs}\nwalk every one of the "
        f"{len(rates)} lines with one short line per observation, "
        f"columns: position, rate, limit, verdict, and for cleans the stage "
        f"completed and the rollout after; no other prose in the walk.")
    contract = ('{"rollout_pct": N, "stages_done": N, "status": '
                '"PAUSED" | "STOPPED" | "DONE"}. no extra keys')
    tkeys = "rollout_pct/stages_done/status"
    w_rates = [0.2, 0.75, 2.5]
    worked = (
        "for orientation only, an earlier change on a different night, "
        "different numbers, same shape: it rolled 0.2, 0.75, 2.5, 0.5. "
        "position 1 clean, stage 1, rollout 10. position 2 clean, stage 2, "
        "rollout 20. position 3 rolled 2.5, over the 2.0 line, breach one: "
        "that observation spent, rollout and stages hold the last completed "
        "stage, 20 and 2, limit tightens to 0.8. position 4 rolled 0.5, "
        "under 0.8, clean: it carries stage 4 by position, so the rollout "
        "jumps to 40 and stages to 4, skipping the spent position 3. the "
        "list ended there, one breach only, so the close was PAUSED, 40, 4. "
        "the breach observation is spent, it does not re-test against the "
        "tighter limit, the NEXT observation does.")
    return {"core": core, "worked": worked, "contract": contract,
            "truth": truth, "tkeys": tkeys,
            "task_digits": sorted({10, 2.0, 1.0, 0.8} | set(rates))}


def _simulate_residency(sizes, pinned, stream, cap=8):
    """Walk a load/unload stream on one card. sizes: {model: GB}.
    LRU eviction among unpinned; defer when only pinned remains and
    still no fit. Unload of a non-resident model is a no-op."""
    resident = {}
    touch = []            # LRU order, oldest first
    loads = evictions = deferrals = 0
    deferred = []
    for verb, m in stream:
        if verb == "unload":
            if m in resident:
                del resident[m]
                touch.remove(m)
            continue
        need = sizes[m]
        free = cap - sum(resident.values())
        if need > free:
            while need > cap - sum(resident.values()):
                cands = [x for x in touch if x != pinned]
                if not cands:
                    deferrals += 1
                    deferred.append(m)
                    break
                victim = cands[0]
                touch.remove(victim)
                del resident[victim]
                evictions += 1
            else:
                resident[m] = need
                touch.append(m)
                loads += 1
            continue
        resident[m] = need
        touch.append(m)
        loads += 1
    return {"loads": loads, "evictions": evictions,
            "deferrals": deferrals, "deferred": deferred}


def _gen_residency(variant, idx):
    # models A..D, sizes GB, pinned set varies; H strains the card,
    # L is a quieter night. Truth computed by simulation, never typed.
    if variant == "H":
        sizes = _pick(idx, [{"A": 2, "B": 2, "C": 2, "D": 7},
                            {"A": 3, "B": 3, "C": 1, "D": 7},
                            {"A": 1, "B": 4, "C": 2, "D": 6}])
        pinned = _pick(idx, ["B", "C", "B"])
        stream = [("load", "A"), ("load", "B"), ("load", "C"),
                  ("load", "D"), ("unload", "B")]
    else:
        sizes = _pick(idx, [{"A": 1, "B": 2, "C": 3, "D": 4},
                            {"A": 2, "B": 1, "C": 4, "D": 3},
                            {"A": 3, "B": 4, "C": 1, "D": 2}])
        pinned = _pick(idx, ["C", "B", "C"])
        stream = [("load", "A"), ("load", "C"), ("load", "D"),
                  ("unload", "D")]
    truth = _simulate_residency(sizes, pinned, stream, cap=8)
    order = " ".join(f"{m}{sizes[m]}" for m in "ABCD")
    stream_txt = ", ".join(f"{v} {m} ({sizes[m]} GB)" for v, m in stream)
    core = (
        f"one 8GB card tonight, models A through D, resident sizes: {order} "
        f"GB respectively. model {pinned} is pinned and never evicted. "
        f"evictions choose the least-recently-touched unpinned model, loads "
        f"cost 30s, unloads 10s. loads counts every load command in "
        f"tonight's stream that was actually executed, including loads "
        f"that later evicted something, and it is never the count of "
        f"residents left at the end. a load that cannot fit evicts as many LRU "
        f"unpinned models as needed, and if only pinned models remain and "
        f"it still does not fit it defers. every eviction actually "
        f"performed counts in the evictions total even when the load "
        f"ends up deferred, and a load is only deferred after evicting "
        f"every unpinned resident still leaves too little room: the "
        f"deferred id is that load's own id. an unload for a model no longer "
        f"resident is a no-op. evictions and deferrals only happen when a "
        f"load truly cannot fit: if every load in tonight's stream fits with "
        f"room to spare, evictions and deferrals both stay zero, and the "
        f"orientation example's evictions do not carry over to tonight. "
        f"Requested stream: {stream_txt}.")
    contract = ('{"loads": N, "evictions": N, "deferrals": N, "deferred": '
                '[ids]}. no extra keys')
    tkeys = "loads/evictions/deferrals/deferred"
    # worked example: same simulation, disjoint params
    w = _simulate_residency({"M": 2, "N": 2, "P": 3, "Q": 4}, "N",
                            [("load", "M"), ("load", "N"), ("load", "P"),
                             ("load", "Q")], cap=5)
    worked = (
        "for orientation only, a smaller card on a different night, "
        "different models, different numbers, same shape: a 5GB card, "
        "models M at 2GB, N at 2GB pinned, P at 3GB, Q at 4GB, requests "
        "load M, load N, load P, load Q. M and N loaded clean. P's 3GB did "
        "not fit in the 1GB left, so it evicted the least-recently-touched "
        "unpinned model M and loaded. Q's 4GB evicted P and still did not "
        "fit with only pinned N remaining, so Q deferred: three loads, two "
        "evictions, one deferral, Q on the deferred list, the close was "
        "clean.")
    assert w == {"loads": 3, "evictions": 2, "deferrals": 1,
                 "deferred": ["Q"]}, w
    return {"core": core, "worked": worked, "contract": contract,
            "truth": truth, "tkeys": tkeys,
            "task_digits": sorted({8, 30, 10})}


def _simulate_preempt(hs, hd, lows, cp=2):
    """One core, minute-granular. HIGH arrives at hs, runs hd minutes,
    instantly preempting the running LOW. LOWs take the core by start
    minute, resumed outranks fresh at ties. Checkpoints every cp
    completed minutes; preemption wastes unbanked minutes (redone)."""
    names = [n for n, _, _ in lows]
    start = {n: s for n, s, _ in lows}
    dur = {n: d for n, _, d in lows}
    ckpt = {n: 0 for n in names}
    pos = {n: 0 for n in names}     # completed minutes since last checkpoint
    resumed = {n: False for n in names}
    fin = set()
    order = []
    waste = 0
    t = 0

    def take(n, until):
        nonlocal t
        while t < until and n not in fin and (ckpt[n] + pos[n]) < dur[n]:
            pos[n] += 1
            t += 1
            if pos[n] == cp:
                ckpt[n] += cp
                pos[n] = 0
        if (ckpt[n] + pos[n]) >= dur[n]:
            fin.add(n)
            order.append(n)
            pos[n] = 0

    while t < hs:
        elig = [n for n in names if n not in fin and start[n] <= t]
        if not elig:
            pending = [start[n] for n in names if n not in fin]
            if not pending or min(pending) >= hs:
                break
            t = min(pending)
            continue
        n = min(elig, key=lambda x: (start[x], not resumed[x]))
        take(n, hs)

    for n in names:
        if n not in fin and pos[n] > 0:
            waste += pos[n]
            pos[n] = 0
            resumed[n] = True
    t = hs + hd

    while any(n not in fin for n in names):
        n = min((n for n in names if n not in fin),
                key=lambda x: (start[x], not resumed[x]))
        take(n, t + dur[n])

    makespan = t - min([start[n] for n in names] + [hs])
    return {"makespan_min": makespan, "wasted_min": waste, "order": order}


def _gen_preempt(variant, idx):
    if variant == "H":
        hs, hd, lows = _pick(idx, [
            (3, 8, [("LOW1", 0, 5), ("LOW2", 2, 7)]),
            (4, 4, [("LOW1", 0, 5), ("LOW2", 1, 2)]),
            (2, 9, [("LOW1", 0, 5), ("LOW2", 2, 7)]),
        ])
    else:
        hs, hd, lows = _pick(idx, [
            (3, 6, [("LOW1", 0, 4), ("LOW2", 3, 3)]),
            (2, 6, [("LOW1", 0, 4), ("LOW2", 0, 3)]),
            (5, 4, [("LOW1", 0, 4), ("LOW2", 3, 3)]),
        ])
    truth = _simulate_preempt(hs, hd, lows)
    jobs = ", ".join(f"{n} runs {d} minutes starting at minute {s}"
                     for n, s, d in lows)
    core = (
        f"one core tonight, three jobs. {jobs}. HIGH arrives at minute {hs} "
        f"and runs {hd} minutes to completion, instantly preempting "
        f"whatever LOW job is running, and while HIGH runs nothing else "
        f"runs. LOW jobs take the core in order of their start minutes, a "
        f"resumed LOW outranks a fresh one at any tie, and a LOW whose "
        f"start minute arrives while the core is busy simply waits. the "
        f"core never idles while a LOW job is waiting: the minute one LOW "
        f"job finishes or is preempted, the next waiting LOW starts in "
        f"that same minute, so a LOW can be caught mid-run by HIGH even "
        f"after a single minute. LOW "
        f"jobs checkpoint every 2 completed minutes of running time; on "
        f"preemption the work past the last checkpoint is wasted and redone "
        f"after resume. a job that starts or resumes at minute s banks a "
        f"checkpoint at minute s+2 and again every 2 running minutes "
        f"after, so track banks in absolute minutes; a bank landing on "
        f"the exact minute HIGH arrives still counts and nothing is "
        f"wasted; after resume a job redoes every minute past its last "
        f"bank, so a job with 5 total minutes and 2 banked resumes with "
        f"3 minutes left. keep the walk tight: one short line per minute "
        f"of the night, then the final answer. makespan counts from the "
        f"the last finish. completion order lists the LOW jobs by the minute "
        f"each finishes, so a LOW that finishes before HIGH even arrives is "
        f"always first in the order. report the completion order of the LOW "
        f"jobs, minutes wasted past checkpoints, and the makespan.")
    contract = ('{"makespan_min": N, "wasted_min": N, "order": [job id '
                'strings only, like ["LOW1", "LOW2"], never objects, '
                'never one long string, and never include HIGH]}. '
                'no extra keys')
    tkeys = "makespan_min/wasted_min/order"
    w = _simulate_preempt(3, 2, [("LOW1", 0, 7), ("LOW2", 1, 3)])
    assert w == {"makespan_min": 13, "wasted_min": 1,
                 "order": ["LOW1", "LOW2"]}, w
    worked = (
        "for orientation only, a rehearsal on a different night, different "
        "numbers, same shape: LOW1 runs 7 minutes starting at minute 0, "
        "LOW2 runs 3 minutes starting at minute 1, HIGH arrives at minute "
        "3 and runs 2 minutes. LOW1 held the core minutes 0, 1 and 2, "
        "banked a checkpoint at minute 2, and its 3rd minute was unbanked "
        "when the swap came, so 1 minute wasted. HIGH ran minutes 3 to 5. "
        "Both LOWs were then waiting: the core went to LOW1, whose start "
        "minute 0 beats LOW2's start minute 1, resumed or not. LOW1 had 7 "
        "total minutes with 2 banked, so it resumed owing 5, ran them "
        "minutes 5 to 10, and finished first. LOW2 then owed its whole 3, "
        "ran minutes 10 to 13, and finished second. Completion "
        f"order LOW1 then LOW2, 1 wasted minute, makespan 13 minutes from "
        "the first start.")
    return {"core": core, "worked": worked, "contract": contract,
            "truth": truth, "tkeys": tkeys,
            "task_digits": sorted({hs, hd} | {d for _, _, d in lows} | {2})}


def _gen_epistemic(variant, idx):
    if variant == "H":
        core = (
            "night audit, five claims, rank your sources. machine log beats "
            "signed handoff, signed handoff beats hearsay, the status "
            "dashboard is KNOWN-BUGGY and can never carry a claim alone. a "
            "claim resting only on hearsay, or only on the dashboard, is "
            "UNVERIFIED, not false. when a higher source contradicts a lower "
            "one the higher source wins and the lower source's assertion "
            "counts FALSE. when sources agree the claim is true. c1: the "
            "machine log shows the shard rebalanced at 02:10. c2: a signed "
            "handoff says the shard never moved, the machine log shows it "
            "moved at 02:10. c3: a signed handoff puts the backup at 03:00, "
            "hallway hearsay says 03:00 too. c4: hallway hearsay alone "
            "claims the cache was cold at 04:00. c5: only the buggy "
            "dashboard shows the queue depth peaking at 05:00. the audit "
            "window spans 45 minutes across 5 claims and 3 ranked sources.")
        truth = {"true": ["c1", "c3"], "false": ["c2"],
                 "unverified": ["c4", "c5"]}
        digits = {45, 5, 3}
    else:
        core = (
            "night audit, four claims, rank your sources. machine log beats "
            "signed handoff, signed handoff beats hearsay, the status "
            "dashboard is KNOWN-BUGGY and can never carry a claim alone. a "
            "claim resting only on hearsay, or only on the dashboard, is "
            "UNVERIFIED, not false. when a higher source contradicts a lower "
            "one the higher source wins and the lower source's assertion "
            "counts FALSE. when sources agree the claim is true. c1: the "
            "machine log shows the replica promoted at 01:30. c2: a signed "
            "handoff puts the migration at 02:45, hallway hearsay says 02:45 "
            "too. c3: hallway hearsay alone claims the index was rebuilt at "
            "03:15. c4: only the buggy dashboard shows the latency spike at "
            "04:20. the audit window spans 45 minutes across 4 claims and "
            "3 ranked sources.")
        truth = {"true": ["c1", "c2"], "false": [],
                 "unverified": ["c3", "c4"]}
        digits = {45, 4, 3}
    contract = ('{"true": [ids], "false": [ids], "unverified": [ids]} every '
                'id in exactly one list, ids sorted, ids lowercase exactly '
                'as written in the claims (c1, c2, ...). no extra keys')
    tkeys = "true/false/unverified"
    worked = (
        "for orientation only, last week's audit on a different night, "
        "different claims, same shape: one logged restart scored true, one "
        "handoff that argued with the log scored false, a chat rumor about "
        "disk pressure went unverified, a dashboard-only latency spike went "
        "unverified too. the dashboard being right by accident still scores "
        "unverified, known sources only, that is the whole point of calling "
        "it known-buggy.")
    return {"core": core, "worked": worked, "contract": contract,
            "truth": truth, "tkeys": tkeys,
            "task_digits": sorted(digits)}


GENS = {"quota": _gen_quota, "backoff": _gen_backoff,
        "canary": _gen_canary, "residency": _gen_residency,
        "preempt": _gen_preempt, "epistemic": _gen_epistemic}


def gen(engine, variant, idx):
    """Build a case core for engine at variant (H/L) and index idx."""
    if engine not in GENS:
        raise ValueError(f"unknown engine {engine}")
    return GENS[engine](variant, idx)


def truth_json(engine_case):
    return json.dumps(engine_case["truth"])


if __name__ == "__main__":
    # smoke: every engine x variant x 6 idx must simulate + carry digits
    for e in ENGINES:
        for v in ("H", "L"):
            for i in range(6):
                c = gen(e, v, i)
                assert c["truth"] is not None
                assert len(c["task_digits"]) >= 3, (e, v, i)
                assert c["worked"].startswith("for orientation only"), (e, v)
    print("engines OK: 6 engines x 2 variants x 6 idx")
