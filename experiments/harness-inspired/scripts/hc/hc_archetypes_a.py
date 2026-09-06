"""hc archetype definitions A (13 of 25). Each build(v) returns:
arch, domain, rules[], facts[], body[para], truth{}, prov[], entities[],
aggregate expr, contract{key: (formula, [(name, regex, val)])}.
Truth computed from params by the same arithmetic the prose states."""


def arch_queue_cutoff(v):
    V = [
        dict(queues=[("alpha", 34, 30), ("bravo", 22, 30), ("charlie", 41, 30)], rate=20, cutoff=80),
        dict(queues=[("north", 47, 40), ("south", 33, 40), ("east", 52, 40)], rate=25, cutoff=100),
        dict(queues=[("r1", 18, 15), ("r2", 29, 15), ("r3", 11, 15)], rate=12, cutoff=45),
        dict(queues=[("moto", 63, 50), ("kilo", 44, 50), ("lima", 71, 50)], rate=30, cutoff=150),
    ][v]
    per_q, rate, cutoff = [], 0, []
    for name, arrive, cap in V["queues"]:
        done = min(arrive, cap)
        deferred = arrive - done
        per_q.append((name, arrive, cap, done, deferred))
    total_done = sum(p[3] for p in per_q)
    for name, arrive, cap, done, deferred in per_q:
        if total_done >= V["cutoff"]:
            break
    over_cap = [p for p in per_q if p[4] > 0]
    truth = {
        "processed": total_done,
        "deferred": sum(p[4] for p in per_q),
        "queues_over_cap": len(over_cap),
        "cutoff_hit": total_done >= V["cutoff"],
    }
    qlines = ", ".join(f"{n} holds {a} items against a cap of {c}" for n, a, c, d, f in per_q)
    prov = [f"processed = sum of min(arrival, cap) = " + " + ".join(str(p[3]) for p in per_q) + f" = {total_done}",
            f"deferred = sum of (arrival - cap where positive) = " + " + ".join(str(p[4]) for p in per_q) + f" = {truth['deferred']}",
            f"queues_over_cap = queues with arrival > cap = {truth['queues_over_cap']}",
            f"cutoff_hit = processed {total_done} >= nightly cutoff {V['cutoff']} is {truth['cutoff_hit']}"]
    rules = [
        f"each queue processes at most its own cap per night, overages defer to tomorrow, they are never dropped",
        f"the nightly cutoff is {V['cutoff']} processed items across all queues combined, if the summed processed count reaches or passes it the cutoff flag flips true, the flag is the report, nothing truncates, queues run to their caps regardless",
        f"deferred means arrived but not processed, arrived items sitting in a capped-out queue are deferred",
    ]
    facts = [f"{qlines}"] + [f"queue {p[0]}: {p[1]} arrived, cap {p[2]}, so {p[3]} processed and {p[4]} deferred" for p in per_q]
    body = [
        f"three queues feed one processor, each queue has its own nightly cap, {qlines}. the processor drains them in the order listed, it never reorders, it never revisits a queue once capped out. the arithmetic that matters is small, min of arrival and cap per queue, summed, nothing fancier.",
        f"the cutoff rule came later than the caps, it was bolted on after a night when the processor ran hot past dawn. {V['cutoff']} is the combined ceiling, if the summed processed count reaches it mid-queue the remaining arrivals defer with everything else. tonight the numbers sit where they sit, work the rules in the order they were given.",
        f"the log convention: an item is processed when the processor finishes it, arrival alone is not processing. the overnight batcher replays yesterday's overflow before yours, that replay is not in these counts, it belongs to the prior ledger.",
    ]
    entities = [
        {"id": "processed_total", "question": "total items processed tonight across all queues (sum of min(arrival, cap) per queue)", "answer": truth["processed"]},
        {"id": "deferred_total", "question": "total items deferred tonight (arrival minus cap where positive, summed)", "answer": truth["deferred"]},
        {"id": "over_cap_count", "question": "how many queues exceeded their own cap (arrival strictly greater than cap)", "answer": truth["queues_over_cap"]},
        {"id": "cutoff_flag", "question": "did the combined processed count reach the nightly cutoff (true or false)", "answer": truth["cutoff_hit"]},
    ]
    contract = {
        "processed": ("min(a1,c1)+min(a2,c2)+min(a3,c3)",
                      [("a1", rf"{per_q[0][0]} holds (\d+) items", per_q[0][1]), ("c1", rf"{per_q[0][0]} holds \d+ items against a cap of (\d+)", per_q[0][2]),
                       ("a2", rf"{per_q[1][0]} holds (\d+) items", per_q[1][1]), ("c2", rf"{per_q[1][0]} holds \d+ items against a cap of (\d+)", per_q[1][2]),
                       ("a3", rf"{per_q[2][0]} holds (\d+) items", per_q[2][1]), ("c3", rf"{per_q[2][0]} holds \d+ items against a cap of (\d+)", per_q[2][2])]),
        "deferred": ("max(a1-c1,0)+max(a2-c2,0)+max(a3-c3,0)", []),
        "queues_over_cap": ("(a1>c1)+(a2>c2)+(a3>c3)", []),
        "cutoff_hit": ("processed >= K", [("K", r"nightly cutoff is (\d+) processed items", V["cutoff"])]),
    }
    return dict(arch="queue_cutoff", domain="night queue processing", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities,
                aggregate="{'processed': int(a('processed_total')), 'deferred': int(a('deferred_total')), 'queues_over_cap': int(a('over_cap_count')), 'cutoff_hit': bool(a('cutoff_flag'))}",
                contract=contract, variant=v)


def arch_ack_windows(v):
    V = [
        dict(pages=[("p1", "sev1", 5, 10), ("p2", "sev1", 12, 10), ("p3", "sev2", 25, 30), ("p4", "sev3", 60, 120)], repages=1),
        dict(pages=[("k1", "sev1", 4, 15), ("k2", "sev2", 20, 25), ("k3", "sev2", 41, 25), ("k4", "sev3", 100, 90)], repages=2),
        dict(pages=[("m1", "sev1", 9, 5), ("m2", "sev2", 30, 15), ("m3", "sev2", 14, 30), ("m4", "sev3", 44, 60)], repages=2),
        dict(pages=[("z1", "sev1", 16, 20), ("z2", "sev2", 8, 30), ("z3", "sev2", 35, 30), ("z4", "sev3", 119, 120)], repages=1),
    ][v]
    rows = []
    acked = escalated = 0
    for pid, sev, ack, win in V["pages"]:
        ok = ack <= win
        rows.append((pid, sev, ack, win, ok))
        acked += ok
        escalated += (not ok)
    sev1_total = sum(1 for r in rows if r[1] == "sev1")
    sev1_escalated = sum(1 for r in rows if r[1] == "sev1" and not r[4])
    truth = {"acked_on_time": acked, "escalated": escalated, "sev1_pages": sev1_total, "sev1_escalated": sev1_escalated}
    plines = ", ".join(f"{p} came in as {s}, was acked in {a} minutes against a {w} minute window" for p, s, a, w, ok in rows)
    prov = [f"acked_on_time = pages with ack <= window = " + " + ".join(str(int(r[4])) for r in rows) + f" = {acked}",
            f"escalated = pages with ack > window = {escalated}",
            f"sev1_pages = {sev1_total}", f"sev1_escalated = {sev1_escalated}"]
    rules = [
        "an ack counts as on-time only if it lands within the window, an ack exactly at the window edge is on-time, past it is a miss",
        "a missed window escalates once to the secondary, the secondary ack does not rewind the original miss",
        "severity is a label for reporting, it changes no windows, windows are per-page",
    ]
    facts = [plines]
    body = [
        f"the pager tree woke you {len(rows)} times tonight, {plines}. the ack clock starts when the page fires, not when you read it, that distinction has cost people their morning. count the misses straight, no rounding, no benefit of the doubt.",
        f"the escalation ladder was rewritten after a sev1 sat unowned for forty minutes while two people each assumed the other had it. one escalation per miss, the secondary either catches it or it goes to the bridge, tonight it never reached the bridge.",
        f"the paging gateway you do not manage also posts reminder thumps every ten minutes for un-acked pages, reminders are not acks and never were.",
    ]
    entities = [
        {"id": "acked", "question": "pages acked within their window (ack minutes <= window minutes)", "answer": acked},
        {"id": "esc", "question": "pages that missed their window and escalated (ack minutes > window minutes)", "answer": escalated},
        {"id": "s1_total", "question": "total sev1 pages tonight", "answer": sev1_total},
        {"id": "s1_esc", "question": "sev1 pages that escalated", "answer": sev1_escalated},
    ]
    def _lit(i):
        pid, sev, ack, win, ok = rows[i]
        return [("a%d" % (i + 1), rf"{pid} came in as \w+, was acked in (\d+) minutes", ack),
                ("w%d" % (i + 1), rf"{pid} came in as \w+, was acked in \d+ minutes against a (\d+) minute", win)]

    _lits = [l for i in range(4) for l in _lit(i)]
    _slits = [(f"g{i+1}", rf"{rows[i][0]} came in as (\w+),", rows[i][1]) for i in range(4)]
    return dict(arch="ack_windows", domain="on-call paging tiers", rules=rules, facts=facts, body=body,
                truth=truth, prov=prov, entities=entities,
                aggregate="{'acked_on_time': int(a('acked')), 'escalated': int(a('esc')), 'sev1_pages': int(a('s1_total')), 'sev1_escalated': int(a('s1_esc'))}",
                contract={"acked_on_time": ("(a1<=w1)+(a2<=w2)+(a3<=w3)+(a4<=w4)", _lits),
                           "escalated": ("4 - acked_on_time", []),
                           "sev1_pages": ("(g1=='sev1')+(g2=='sev1')+(g3=='sev1')+(g4=='sev1')", _slits),
                           "sev1_escalated": ("((g1=='sev1')*(a1>w1))+((g2=='sev1')*(a2>w2))+((g3=='sev1')*(a3>w3))+((g4=='sev1')*(a4>w4))", _lits + _slits)},
                variant=v)


def arch_freeze_windows(v):
    V = [
        dict(clock="20:35", freeze="21:00",
             svc=[("gateway", 0.4, 10, False), ("worker", 0.7, 12, True), ("batch", 0.1, 5, False)], slots=4),
        dict(clock="20:10", freeze="21:00",
             svc=[("edge", 0.3, 8, False), ("api", 0.6, 15, True), ("cron", 0.2, 6, False)], slots=3),
        dict(clock="20:50", freeze="21:15",
             svc=[("front", 0.5, 12, False), ("auth", 0.9, 10, True), ("sink", 0.3, 7, False)], slots=2),
        dict(clock="19:40", freeze="21:00",
             svc=[("web", 0.2, 25, False), ("jobs", 0.8, 9, True), ("mail", 0.1, 14, False)], slots=5),
    ][v]
    h, m = map(int, V["clock"].split(":"))
    fh, fm = map(int, V["freeze"].split(":"))
    now = h * 60 + m
    fz = fh * 60 + fm
    t = now
    shipped = rolled = 0
    used = 0
    detail = []
    for name, err, dur, needs_rollback in V["svc"]:
        if err >= 0.5:
            rolled += 1
        start = t
        end = t + dur
        if end <= fz and used < V["slots"]:
            shipped += 1
            used += 1
            t = end
            detail.append(f"{name} deployed")
        else:
            detail.append(f"{name} pending")
    truth = {"shipped": shipped, "rolled_back": rolled, "slots_left": V["slots"] - used, "pending": len(V["svc"]) - shipped}
    rules = [
        f"a deploy may start only before {V['freeze']}, work unfinished at {V['freeze']} waits for tomorrow, partial work does not carry",
        "a service showing an error rate at or above 0.5 percent must roll back before any new deploy of that service, rollbacks consume no slots",
        f"each new deploy consumes one of the {V['slots']} build slots held for the night, slots are never returned",
    ]
    facts = [f"the clock reads {V['clock']}", f"the freeze lands at {V['freeze']}",
             ", ".join(f"{n} shows {e} percent and its deploy takes {d} minutes" for n, e, d, rb in V["svc"])]
    body = [
        f"the deploy lane is yours until the freeze. {', '.join(facts[2])}. the clock reads {V['clock']}, the freeze lands at {V['freeze']}, the arithmetic is timeline arithmetic, walk the services in the order given and let the clock do the deciding.",
        f"the slot ledger exists because a night in april burned nine builds on a flaky test rig and nobody could say where the capacity went. one slot per deploy, rollbacks ride free, slots do not come back even when the deploy rolls back later.",
        f"error rate gates came out of a different postmortem, the one where a 0.7 percent release shipped on top of itself twice. at or above the line means rollback first, then and only then a fresh deploy if the clock allows it.",
        f"the canary analyst you do not manage also publishes a deploy confidence score, that score has never gated anything and does not gate tonight either.",
    ]
    entities = [
        {"id": "ch", "question": "the clock hour (copy the number from its fact line)", "answer": h},
        {"id": "cm", "question": "the clock minute (copy the number from its fact line)", "answer": m},
        {"id": "fh", "question": "the freeze hour (copy the number from its fact line)", "answer": fh},
        {"id": "fm", "question": "the freeze minute (copy the number from its fact line)", "answer": fm},
        {"id": "sk", "question": "build slots held for the night (copy the number from its fact line)", "answer": V["slots"]},
    ] + [
        {"id": f"e{i+1}", "question": f"{n}'s error rate percent (copy the number from its fact line)", "answer": err}
        for i, (n, err, dur, rb) in enumerate(V["svc"])
    ] + [
        {"id": f"d{i+1}", "question": f"{n}'s deploy duration in minutes (copy the number from its fact line)", "answer": dur}
        for i, (n, err, dur, rb) in enumerate(V["svc"])
    ]
    _fl = [("ch", r"clock reads (\d+):", h), ("cm", r"clock reads \d+:(\d+)", m),
           ("fh", r"freeze lands at (\d+):", fh), ("fm", r"freeze lands at \d+:(\d+)", fm),
           ("T", r"at or above (\d+\.?\d*) percent", 0.5)] + \
          [t for i, (n, e, d, rb) in enumerate(V["svc"]) for t in (
              (f"e{i+1}", rf"{n} shows (\d+\.?\d*) percent", e),
              (f"d{i+1}", rf"{n} shows \d+\.?\d* percent and its deploy takes (\d+) minutes", d))]
    _c = "(ch*60+cm)"
    _f = "(fh*60+fm)"
    _s1 = f"({_c}+d1<={_f})"
    _s2 = f"({_s1}*({_c}+d1+d2<={_f})*(1<K))"
    _s3 = f"({_s2}*({_c}+d1+d2+d3<={_f})*(2<K))"
    _shipped = "((a('ch')*60+a('cm'))+a('d1')<=(a('fh')*60+a('fm')))+(((a('ch')*60+a('cm'))+a('d1')<=(a('fh')*60+a('fm')))*((a('ch')*60+a('cm'))+a('d1')+a('d2')<=(a('fh')*60+a('fm')))*(1<a('sk')))+((((a('ch')*60+a('cm'))+a('d1')<=(a('fh')*60+a('fm')))*((a('ch')*60+a('cm'))+a('d1')+a('d2')<=(a('fh')*60+a('fm')))*(1<a('sk')))*((a('ch')*60+a('cm'))+a('d1')+a('d2')+a('d3')<=(a('fh')*60+a('fm')))*(2<a('sk')))"
    return dict(arch="freeze_windows", domain="deploy freeze windows", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"shipped={shipped} rolled={rolled} slots_left={V['slots']-used} pending={len(V['svc'])-shipped}"],
                entities=entities,
                aggregate="{'shipped': " + _shipped + ", 'rolled_back': (a('e1')>=0.5)+(a('e2')>=0.5)+(a('e3')>=0.5), 'slots_left': a('sk')-(" + _shipped + "), 'pending': 3-(" + _shipped + ")}",
                contract={"shipped": (f"{_s1}+{_s2}+{_s3}", _fl + [("K", r"(\d+) build slots held for the night", V["slots"])]),
                          "rolled_back": ("(e1>=T)+(e2>=T)+(e3>=T)", _fl),
                          "slots_left": ("K - shipped", []),
                          "pending": ("3 - shipped", [])},
                variant=v)


def arch_backoff_budget(v):
    V = [
        dict(jobs=[("j1", 4, 2), ("j2", 3, 5)], base=2, cap=8, budget=30),
        dict(jobs=[("ingest", 5, 3), ("render", 2, 10)], base=3, cap=12, budget=40),
        dict(jobs=[("scan-a", 3, 2), ("scan-b", 6, 4)], base=1, cap=6, budget=25),
        dict(jobs=[("sync", 4, 4), ("index", 4, 7)], base=2, cap=16, budget=50),
    ][v]
    def series(fails, base, cap):
        wait, total, n = base, 0, 0
        seq = []
        for i in range(fails):
            seq.append(min(wait, cap))
            total += min(wait, cap)
            wait *= 2
        return seq, total
    used_total = 0
    details = []
    for name, fails, dur in V["jobs"]:
        seq, wtotal = series(fails, V["base"], V["cap"])
        used = wtotal + fails * dur
        used_total += used
        details.append((name, fails, dur, seq, used))
    truth = {"budget_burned": used_total, "budget_left": V["budget"] - used_total,
             "jobs_count": len(V["jobs"]), "longest_wait": max(min(s[-1], V["cap"]) for _, _, _, s, _ in details)}
    rules = [
        f"each retry waits double the previous wait, starting at {V['base']} minutes, no single wait exceeds the {V['cap']} minute cap",
        f"the shared retry budget is {V['budget']} minutes across all jobs, it is a ledger line, work already started always finishes, overruns show up as negative headroom in the report",
        "a job that exhausts its declared failures stops, it does not borrow budget from tomorrow",
    ]
    facts = [", ".join(f"{n} failed {f} times with each run taking {d} minutes" for n, f, d, s, u in details)]
    body = [
        f"the retry engine burned through the evening, {facts[0]}. waits double from {V['base']} minutes and cap at {V['cap']}, run time counts the same as wait time, the budget ledger does not care which kind of minute it was.",
        f"the doubling rule predates the cap, the cap predates the shared budget, all three exist because of three different bad nights. stack them in order, double first, cap second, sum third.",
        f"the failure inbox you do not manage also queues complaint emails per failure, the inbox is loud, the inbox is not the budget.",
    ]
    entities = [
        {"id": "bs", "question": "the starting wait in minutes (copy the number from its fact line)", "answer": V["base"]},
        {"id": "cp", "question": "the wait cap in minutes (copy the number from its fact line)", "answer": V["cap"]},
        {"id": "bt", "question": "the shared retry budget in minutes (copy the number from its fact line)", "answer": V["budget"]},
    ] + [
        {"id": f"f{i+1}", "question": f"times {n} failed (copy the number from its fact line)", "answer": fails}
        for i, (n, fails, dur) in enumerate(V["jobs"])
    ] + [
        {"id": f"du{i+1}", "question": f"minutes each {n} run takes (copy the number from its fact line)", "answer": dur}
        for i, (n, fails, dur) in enumerate(V["jobs"])
    ]
    _bl = [("BASE", r"starting at (\d+) minutes", V["base"]), ("CAP", r"exceeds the (\d+) minute cap", V["cap"])] + \
          [t for idx, (name, fails, dur) in enumerate(V["jobs"]) for t in (
              (f"F{idx+1}", rf"{name} failed (\d+) times", fails),
              (f"D{idx+1}", rf"{name} failed \d+ times with each run taking (\d+) minutes", dur))]
    _j1, _j2 = V["jobs"]
    _burn1 = "+".join(f"(min({2**k}*BASE,CAP)*({k}<F1))" for k in range(6)) + "+F1*D1"
    _burn2 = "+".join(f"(min({2**k}*BASE,CAP)*({k}<F2))" for k in range(6)) + "+F2*D2"
    _ab1 = "+".join(f"(min({2**k}*a('bs'),a('cp'))*({k}<a('f1')))" for k in range(6)) + "+a('f1')*a('du1')"
    _ab2 = "+".join(f"(min({2**k}*a('bs'),a('cp'))*({k}<a('f2')))" for k in range(6)) + "+a('f2')*a('du2')"
    return dict(arch="backoff_budget", domain="retry backoff budgets", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"budget_burned={used_total} budget_left={V['budget']-used_total} longest_wait={truth['longest_wait']}"],
                entities=entities,
                aggregate="{'budget_burned': " + _ab1 + "+" + _ab2 + ", 'budget_left': a('bt')-(" + _ab1 + "+" + _ab2 + "), 'jobs_count': 2, 'longest_wait': max(min(2**(a('f1')-1)*a('bs'),a('cp')),min(2**(a('f2')-1)*a('bs'),a('cp')))}",
                contract={"budget_burned": (f"{_burn1}+{_burn2}", _bl), "budget_left": ("T - budget_burned", [("T", r"shared retry budget is (\d+) minutes", V["budget"])]),
                           "jobs_count": ("2", []),
                           "longest_wait": ("max(min(2**(F1-1)*BASE,CAP),min(2**(F2-1)*BASE,CAP))", _bl)},
                variant=v)


def arch_cache_sweep(v):
    V = [
        dict(classes=[("hot", 120, 100), ("warm", 260, 200), ("cold", 90, 50)], pinned=15, dupes=12),
        dict(classes=[("edges", 340, 300), ("mid", 150, 100), ("deep", 60, 40)], pinned=22, dupes=9),
        dict(classes=[("l1", 80, 60), ("l2", 130, 120), ("l3", 200, 90)], pinned=8, dupes=20),
        dict(classes=[("fast", 410, 380), ("slow", 190, 150), ("archive", 75, 30)], pinned=31, dupes=17),
    ][v]
    touched = sum(c[1] for c in V["classes"])
    evictable = sum(max(c[1] - c[2], 0) for c in V["classes"])
    evicted = evictable
    retained = touched - evicted - V["dupes"]
    truth = {"touched": touched, "evicted": evicted, "retained": retained, "dupes_folded": V["dupes"]}
    rules = [
        "each class carries its own retention ceiling, entries beyond the ceiling are eviction candidates, nothing else is",
        "duplicated entries fold into one before any counting, a dupe never counts twice anywhere",
        "pinned entries survive every sweep, they are counted in retained, never in evicted",
    ]
    facts = [", ".join(f"the {n} class holds {h} entries against a ceiling of {c}" for n, h, c in V["classes"]),
             f"{V['dupes']} entries turned out to be duplicates folded away before counting",
             f"{V['pinned']} entries are pinned across all classes"]
    body = [
        f"the sweep ledger closes tonight, {facts[0]}, {facts[1]}, {facts[2]}. fold dupes first, then apply ceilings class by class, pinned rides through untouched. the retained number people quote in the morning is what is left, not what started.",
        f"the dupe rule was a hard lesson, two crawlers wrote the same keys for a month and every report doubled. fold first, always fold first, then the ceilings mean something.",
        f"the pin list belongs to the platform team, entries land on it for reasons the sweep does not audit, pins are facts here, not decisions.",
    ]
    entities = [
        {"id": f"h{i+1}", "question": f"entries the {n} class holds (copy the number from its fact line)", "answer": held}
        for i, (n, held, ceil) in enumerate(V["classes"])
    ] + [
        {"id": f"c{i+1}", "question": f"the {n} class ceiling (copy the number from its fact line)", "answer": ceil}
        for i, (n, held, ceil) in enumerate(V["classes"])
    ] + [
        {"id": "d", "question": "duplicate entries folded away (copy the number from its fact line)", "answer": V["dupes"]},
    ]
    return dict(arch="cache_sweep", domain="cache invalidation sweeps", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"touched={touched} evicted={evicted} retained={retained} dupes={V['dupes']}"],
                entities=entities,
                aggregate="{'touched': a('h1')+a('h2')+a('h3'), 'evicted': max(a('h1')-a('c1'),0)+max(a('h2')-a('c2'),0)+max(a('h3')-a('c3'),0), 'retained': a('h1')+a('h2')+a('h3')-(max(a('h1')-a('c1'),0)+max(a('h2')-a('c2'),0)+max(a('h3')-a('c3'),0))-a('d'), 'dupes_folded': a('d')}",
                contract={"touched": ("h1+h2+h3",
                             [("h1", rf"the {V['classes'][0][0]} class holds (\d+) entries", V["classes"][0][1]),
                              ("h2", rf"the {V['classes'][1][0]} class holds (\d+) entries", V["classes"][1][1]),
                              ("h3", rf"the {V['classes'][2][0]} class holds (\d+) entries", V["classes"][2][1]),
                              ("c1", rf"the {V['classes'][0][0]} class holds \d+ entries against a ceiling of (\d+)", V["classes"][0][2]),
                              ("c2", rf"the {V['classes'][1][0]} class holds \d+ entries against a ceiling of (\d+)", V["classes"][1][2]),
                              ("c3", rf"the {V['classes'][2][0]} class holds \d+ entries against a ceiling of (\d+)", V["classes"][2][2])]),
                          "evicted": ("max(h1-c1,0)+max(h2-c2,0)+max(h3-c3,0)", []),
                          "retained": ("touched - evicted - D", [("D", r"(\d+) entries turned out to be duplicates", V["dupes"])]),
                          "dupes_folded": ("D", [])},
                variant=v)


def arch_payroll_holds(v):
    V = [
        dict(rows=[("r-a", 1200, True), ("r-b", 3400, False), ("r-c", 800, True), ("r-d", 2100, False)], thresh=1000, approv=1),
        dict(rows=[("pr-1", 950, False), ("pr-2", 5000, True), ("pr-3", 1250, False), ("pr-4", 640, False)], thresh=2000, approv=2),
        dict(rows=[("k", 250, False), ("m", 3100, True), ("n", 1500, False), ("o", 4200, True)], thresh=1200, approv=1),
        dict(rows=[("w1", 7700, True), ("w2", 900, False), ("w3", 1300, False), ("w4", 2600, True)], thresh=3000, approv=2),
    ][v]
    held = []
    released = []
    for rid, amt, approvable in V["rows"]:
        if amt > V["thresh"] and not approvable:
            held.append((rid, amt))
        else:
            released.append((rid, amt))
    net = sum(a for _, a in released)
    truth = {"released_count": len(released), "held_count": len(held), "net_paid": net, "held_value": sum(a for _, a in held)}
    rules = [
        f"rows over {V['thresh']} currency units hold for manual review unless the row carries an approved exception",
        "held rows pay nothing tonight, they are not partial, they are not deferred pro-rated, they simply wait",
        "the exception column is the only override, verbal approvals count for nothing at this desk",
    ]
    facts = [", ".join(f"row {r} pays {a} units{' with an approved exception' if ap else ''}" for r, a, ap in V["rows"])]
    body = [
        f"the payroll batch runs tonight, {facts[0]}. the threshold is {V['thresh']} units, exceptions are pre-approved in writing or they do not exist. count rows, then sum money, in that order.",
        f"the hold queue exists because of a december when a mistyped zero paid out eleven times the intended amount, since then everything over the line waits for a human. tonight that human already stamped what the exception column shows.",
        f"the finance preview tool you do not manage shows a different net because it annualizes, annualized figures have no seat at this table.",
    ]
    entities = [{"id": f"h{i+1}", "question": f"is row {V['rows'][i][0]} HELD tonight, answer 1 if held or 0 if released. rule: held means the amount is OVER the hold threshold AND has no approved exception, amounts under the threshold are always released (0), over-threshold rows with an approved exception are also released (0). worked example: a row paying 400 against a 500 threshold is under the line, answer 0", "answer": (1 if (r[1] > V["thresh"] and not r[2]) else 0)} for i, r in enumerate(V["rows"])] + \
        [{"id": f"m{i+1}", "question": f"amount row {V['rows'][i][0]} pays, copy the number from its fact line", "answer": r[1]} for i, r in enumerate(V["rows"])] + \
        [{"id": "th", "question": "the hold threshold in units, copy the number from the rules", "answer": V["thresh"]}]
    _pl = [("T", r"over (\d+) currency units hold", V["thresh"])] + \
          [t for idx, (rid, amt, ap) in enumerate(V["rows"]) for t in (
              (f"a{idx+1}", rf"row {rid} pays (\d+) units", amt),
              (f"x{idx+1}", rf"row {rid} pays \d+ units( with an approved exception)?", 1 if ap else 0,
               {" with an approved exception": 1}))]
    _hc = "+".join(f"((a{i+1}>T)*(1-x{i+1}))" for i in range(4))
    _hv = "+".join(f"((a{i+1}>T)*(1-x{i+1})*a{i+1})" for i in range(4))
    return dict(arch="payroll_holds", domain="batch payroll runs", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"released={len(released)} held={len(held)} net={net} held_value={truth['held_value']}"],
                entities=entities,
                aggregate="{'released_count': (1-a('h1'))+(1-a('h2'))+(1-a('h3'))+(1-a('h4')), 'held_count': a('h1')+a('h2')+a('h3')+a('h4'), 'net_paid': a('m1')*(1-a('h1'))+a('m2')*(1-a('h2'))+a('m3')*(1-a('h3'))+a('m4')*(1-a('h4')), 'held_value': a('m1')*a('h1')+a('m2')*a('h2')+a('m3')*a('h3')+a('m4')*a('h4')}",
                contract={"held_count": (_hc, _pl),
                           "held_value": (_hv, _pl),
                           "released_count": ("4 - held_count", []),
                           "net_paid": ("(a1+a2+a3+a4) - held_value",
                                        [(f"a{i+1}", rf"row {V['rows'][i][0]} pays (\d+) units", V['rows'][i][1]) for i in range(4)])},
                variant=v)


def arch_restock_waves(v):
    V = [
        dict(items=[("bkt-1", 40, 25, 5), ("bkt-2", 12, 30, 0), ("bkt-3", 55, 25, 12)], qrate=2),
        dict(items=[("s-1", 90, 60, 10), ("s-2", 45, 50, 3), ("s-3", 20, 55, 0)], qrate=3),
        dict(items=[("hx-9", 33, 30, 8), ("hx-2", 70, 40, 15), ("hx-4", 10, 35, 0)], qrate=1),
        dict(items=[("t3", 150, 100, 20), ("t7", 80, 90, 6), ("t9", 60, 70, 9)], qrate=4),
    ][v]
    reordered = quarantined = backordered = 0
    for iid, stock, reorder, dmg in V["items"]:
        effective = stock - dmg
        if effective <= reorder * 0.5 and effective < reorder:
            reordered += 1
        if effective < reorder and effective > reorder * 0.5:
            reordered += 1
        quarantined += 1 if dmg > 0 else 0
    for iid, stock, reorder, dmg in V["items"]:
        if (stock - dmg) < reorder and (stock - dmg) > reorder * 0.5:
            pass
    below = sum(1 for i in V["items"] if (i[1] - i[3]) < i[2])
    truth = {"reordered": below, "quarantined_lines": quarantined,
             "damaged_units": sum(i[3] for i in V["items"]), "healthy_lines": len(V["items"]) - quarantined}
    rules = [
        "damaged units leave stock before any reorder math runs, damage first, thresholds second",
        "a line at or below its reorder point orders more, at-or-below means exactly equal counts too",
        "any line with damage above zero quarantines that line's damaged units, the rest of the line stays sellable",
    ]
    facts = [", ".join(f"{i} shows {s} on hand, reorder point {r}, damage {d}" for i, s, r, d in V["items"])]
    body = [
        f"the restock floor counts damage at the door, {facts[0]}. subtract damage, compare what is left to the reorder point, order or do not, then tally the quarantined units. the damage column gets read once and acted on everywhere.",
        f"the reorder rules were tightened after a wave where shelves looked full and were full of things nobody could sell. healthy stock is what counts toward reorder decisions, damaged stock counts toward quarantine and nothing else.",
        f"the damage inspector you do not manage logs suspected damage separately, suspicion is not damage until the clipboard says so, the clipboard here is the facts above.",
    ]
    entities = [
        {"id": f"s{i+1}", "question": f"units of {i} on hand (copy the number from its fact line)", "answer": stock}
        for i, (iid, stock, reorder, dmg) in enumerate(V["items"])
    ] + [
        {"id": f"r{i+1}", "question": f"{i}'s reorder point (copy the number from its fact line)", "answer": reorder}
        for i, (iid, stock, reorder, dmg) in enumerate(V["items"])
    ] + [
        {"id": f"d{i+1}", "question": f"{i}'s damaged units (copy the number from its fact line)", "answer": dmg}
        for i, (iid, stock, reorder, dmg) in enumerate(V["items"])
    ]
    return dict(arch="restock_waves", domain="inventory restock waves", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"reordered={below} quarantined_lines={quarantined} damaged={truth['damaged_units']} healthy={truth['healthy_lines']}"],
                entities=entities,
                aggregate="{'reordered': (a('s1')-a('d1')<=a('r1'))+(a('s2')-a('d2')<=a('r2'))+(a('s3')-a('d3')<=a('r3')), 'quarantined_lines': (a('d1')>0)+(a('d2')>0)+(a('d3')>0), 'damaged_units': a('d1')+a('d2')+a('d3'), 'healthy_lines': (a('d1')==0)+(a('d2')==0)+(a('d3')==0)}",
                contract={"damaged_units": ("d1+d2+d3",
                             [(f"d{i+1}", rf"{V['items'][i][0]} shows \d+ on hand, reorder point \d+, damage (\d+)", V["items"][i][3]) for i in range(3)]),
                          "quarantined_lines": ("(d1>0)+(d2>0)+(d3>0)", []),
                          "reordered": ("(s1-d1<=r1)+(s2-d2<=r2)+(s3-d3<=r3)",
                             [t for i in range(3) for t in (
                                 (f"s{i+1}", rf"{V['items'][i][0]} shows (\d+) on hand", V["items"][i][1]),
                                 (f"r{i+1}", rf"{V['items'][i][0]} shows \d+ on hand, reorder point (\d+)", V["items"][i][2]),
                                 (f"d{i+1}", rf"{V['items'][i][0]} shows \d+ on hand, reorder point \d+, damage (\d+)", V["items"][i][3]))])},
                variant=v)


def arch_triage_merge(v):
    V = [
        dict(srcs=[("email", 14), ("portal", 9), ("phone", 6)], dupes=7, sev1=3, sev2=8),
        dict(srcs=[("chat", 22), ("email", 11), ("walkup", 4)], dupes=12, sev1=5, sev2=13),
        dict(srcs=[("form", 8), ("form-b", 5), ("irc", 7)], dupes=4, sev1=2, sev2=9),
        dict(srcs=[("hotline", 17), ("portal", 13), ("fax", 3)], dupes=9, sev1=4, sev2=11),
    ][v]
    raw = sum(c for _, c in V["srcs"])
    unique = raw - V["dupes"]
    sev3 = unique - V["sev1"] - V["sev2"]
    truth = {"raw_inbound": raw, "unique_tickets": unique, "merged_away": V["dupes"], "sev3": sev3}
    rules = [
        "duplicate reports merge into the original before severity counting, a merged dupe carries no severity of its own",
        "every inbound report counts toward raw inbound exactly once, regardless of later merging",
        "sev3 is the remainder after sev1 and sev2 are counted on unique tickets",
    ]
    facts = [", ".join(f"the {s} channel delivered {c} reports" for s, c in V["srcs"]),
             f"{V['dupes']} reports matched existing tickets and merged away",
             f"on the unique tickets, {V['sev1']} came in sev1 and {V['sev2']} came in sev2"]
    body = [
        f"the triage board took fire tonight, {facts[0]}, {facts[1]}. raw inbound first, dedupe second, severity last, in that order, always in that order. the unique count is what survives to the severity table.",
        f"merging used to happen after severity assignment and the sev1 numbers were pure fiction, the fix was ordering, the ordering is the whole rule.",
        f"the channel analytics pane you do not manage also displays a unique figure computed with a different dedupe window, that pane is decoration.",
    ]
    entities = [
        {"id": "raw", "question": "raw inbound reports tonight (all channels summed)", "answer": raw},
        {"id": "uniq", "question": "unique tickets after merging (raw minus merged dupes)", "answer": unique},
        {"id": "mg", "question": "reports merged away as duplicates", "answer": V["dupes"]},
        {"id": "s3", "question": "unique tickets classified sev3 (remainder after sev1 and sev2)", "answer": sev3},
    ]
    return dict(arch="triage_merge", domain="support ticket triage", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"raw={raw} unique={unique} merged={V['dupes']} sev3={sev3}"],
                entities=entities,
                aggregate="{'raw_inbound': int(a('raw')), 'unique_tickets': int(a('uniq')), 'merged_away': int(a('mg')), 'sev3': int(a('s3'))}",
                contract={"raw_inbound": ("c1+c2+c3",
                             [(f"c{i+1}", rf"the {V['srcs'][i][0]} channel delivered (\d+) reports", V["srcs"][i][1]) for i in range(3)]),
                          "unique_tickets": ("raw_inbound - D", [("D", r"(\d+) reports matched existing tickets", V["dupes"])]),
                          "sev3": ("unique_tickets - S1 - S2", [("S1", r"(\d+) came in sev1", V["sev1"]), ("S2", r"(\d+) came in sev2", V["sev2"])])},
                variant=v)


def arch_retention_purge(v):
    V = [
        dict(classes=[("access", 400, 30, 45), ("debug", 900, 14, 120), ("audit", 250, 365, 0)], held=["debug"]),
        dict(classes=[("req", 620, 60, 88), ("resp", 810, 30, 210), ("sec", 140, 730, 0)], held=["resp"]),
        dict(classes=[("trace", 1500, 7, 640), ("metric", 300, 90, 25), ("event", 480, 180, 60)], held=["event"]),
        dict(classes=[("slow", 700, 21, 154), ("fast", 1200, 3, 400), ("arch", 90, 1095, 0)], held=["fast"]),
    ][v]
    rules = [
        "each class purges records older than its own retention days, no exceptions inside a class",
        "classes with an open legal hold skip the purge entirely tonight, hold means all-or-nothing per class",
        "the purge counts records deleted, not records scanned, scanning is free and counts for nothing",
    ]
    held_names = V["held"]
    deletable = sum(c[3] for c in V["classes"] if c[0] not in held_names)
    classes_held = len(held_names)
    classes_purged = len(V["classes"]) - classes_held
    truth = {"classes_purged": classes_purged, "classes_held": classes_held,
             "records_deleted": deletable, "classes_total": len(V["classes"])}
    facts = [", ".join(f"the {n} class scanned {s} records with a retention of {r} days, {b} beyond retention" for n, s, r, b in V["classes"]),
             "hold status by class: " + ", ".join(f"the {n} class is {'on hold' if n in held_names else 'not on hold'}" for n, s, r, b in V["classes"])
             + f", that is {classes_held} class{'es' if classes_held > 1 else ''} on hold"]
    body = [
        f"the retention sweeper runs while the floor is quiet, {facts[0]}, {facts[1]}. hold skips a class whole, no partial purges, no mercy purges. records older than retention go, the rest stay, and the morning report wants three numbers, not opinions.",
        f"legal holds arrived after an audit where a class got purged mid-investigation, the fix was class-level granularity, crude but airtight.",
        f"the archive tier you do not manage keeps its own copies with its own clock, archive copies do not resurrect purged records and do not count.",
    ]
    entities = [
        {"id": "cp", "question": "classes purged tonight (total minus held)", "answer": classes_purged},
        {"id": "ch", "question": "classes under legal hold (skipped entirely)", "answer": classes_held},
        {"id": "rd", "question": "records deleted tonight (sum of beyond-retention records in purged classes)", "answer": deletable},
        {"id": "ct", "question": "total classes in the sweep", "answer": len(V["classes"])},
    ]
    _rtl = [t for idx, (n, s, r, b) in enumerate(V["classes"]) for t in (
        (f"b{idx+1}", rf"the {n} class scanned \d+ records with a retention of \d+ days, (\d+) beyond retention", b),
        (f"h{idx+1}", rf"the {n} class is (on hold|not on hold)", 1 if n in held_names else 0,
         {"on hold": 1, "not on hold": 0}))]
    _rd = "+".join(f"((1-h{i+1})*b{i+1})" for i in range(3))
    return dict(arch="retention_purge", domain="log retention purges", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"classes_purged={classes_purged} held={classes_held} records_deleted={deletable}"],
                entities=entities,
                aggregate="{'classes_purged': int(a('cp')), 'classes_held': int(a('ch')), 'records_deleted': int(a('rd')), 'classes_total': int(a('ct'))}",
                contract={"classes_purged": ("3 - H", [("H", r"that is (\d+) class(?:es)? on hold", classes_held)]),
                           "records_deleted": (_rd, _rtl)},
                variant=v)


def arch_token_bucket(v):
    V = [
        dict(slots=[("s-a", 10, 15), ("s-b", 0, 25), ("s-c", 4, 20)], refill=3, requests=49),
        dict(slots=[("x1", 20, 20), ("x2", 6, 30), ("x3", 0, 15)], refill=4, requests=60),
        dict(slots=[("g", 5, 10), ("h", 12, 25), ("i", 3, 12)], refill=2, requests=38),
        dict(slots=[("up", 14, 40), ("down", 2, 35), ("side", 0, 20)], refill=5, requests=71),
    ][v]
    capacity = sum(c for _, _, c in V["slots"])
    start = sum(s for _, s, _ in V["slots"])
    available = start + V["refill"]
    served = min(V["requests"], available)
    throttled = V["requests"] - served
    truth = {"served": served, "throttled": throttled, "capacity": capacity, "refill_added": V["refill"]}
    rules = [
        "all slots pool into one serving pool, there is no per-slot fairness, the pool serves in arrival order",
        f"the refill adds {V['refill']} tokens once at the top of the window, mid-window refills do not exist",
        "throttled requests are rejected, not queued, rejected means gone",
    ]
    facts = [", ".join(f"slot {n} opened with {s} tokens of its {c} capacity" for n, s, c in V["slots"]),
             f"the window refill added {V['refill']} tokens",
             f"{V['requests']} requests arrived this window"]
    body = [
        f"the rate gate runs one pool, {facts[0]}, {facts[1]}, {facts[2]}. pool everything, add the refill once, serve until dry, everything left over is throttled and gone. capacity is bookkeeping, it bounds nothing tonight except the morning report.",
        f"pooling replaced per-slot fairness after a night where one hot slot starved two quiet ones, the pool does not care who brought the tokens.",
        f"the edge cache you do not manage also serves some requests before they reach the gate, edge-served requests never appear in the arrival count and never will.",
    ]
    entities = [
        {"id": "sv", "question": "requests served this window", "answer": served},
        {"id": "th", "question": "requests throttled (rejected, not queued)", "answer": throttled},
        {"id": "cap", "question": "total pool capacity (slot capacities summed)", "answer": capacity},
        {"id": "rf", "question": "tokens the window refill added", "answer": V["refill"]},
    ]
    return dict(arch="token_bucket", domain="rate limiter token pools", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"served={served} throttled={throttled} capacity={capacity} refill={V['refill']}"],
                entities=entities,
                aggregate="{'served': int(a('sv')), 'throttled': int(a('th')), 'capacity': int(a('cap')), 'refill_added': int(a('rf'))}",
                contract={"served": ("min(REQ, s1+s2+s3+REF)",
                          [("REQ", r"(\d+) requests arrived this window", V["requests"]), ("REF", r"refill added (\d+) tokens", V["refill"])] +
                          [(f"s{i+1}", rf"slot {V['slots'][i][0]} opened with (\d+) tokens", V["slots"][i][1]) for i in range(3)] +
                          [(f"c{i+1}", rf"slot {V['slots'][i][0]} opened with \d+ tokens of its (\d+) capacity", V["slots"][i][2]) for i in range(3)]),
                          "throttled": ("REQ - served", []),
                          "capacity": ("c1+c2+c3", [])},
                variant=v)


def arch_rollout_rings(v):
    V = [
        dict(rings=[("ring-1", 100, 5), ("ring-2", 500, 4), ("ring-3", 2000, 3)], abort=4, fire=2),
        dict(rings=[("canary", 50, 2), ("early", 400, 6), ("broad", 5000, 5)], abort=5, fire=3),
        dict(rings=[("r0", 20, 1), ("r1", 300, 5), ("r2", 8000, 8)], abort=6, fire=4),
        dict(rings=[("coal", 150, 7), ("bronze", 600, 2), ("silver", 3000, 9)], abort=8, fire=1),
    ][v]
    passed = 0
    halted = False
    for name, users, fires in V["rings"]:
        if halted:
            break
        if fires >= V["abort"]:
            halted = True
            break
        passed += 1
    exposed = sum(V["rings"][i][1] for i in range(passed)) + (0 if not halted else V["rings"][passed][1])
    truth = {"rings_passed": passed, "aborted": halted, "users_exposed": exposed, "halt_ring": V["rings"][passed][0] if halted else ""}
    rules = [
        f"rings promote in order, a ring only enters after the previous ring passes clean",
        f"a ring with {V['abort']} or more fire reports halts the rollout on the spot, halted means halted, no finish and see",
        "users exposed counts every user in rings that entered, including the ring that triggered the halt",
    ]
    facts = [", ".join(f"{n} covers {u} users and logged {f} fire reports" for n, u, f in V["rings"]),
             f"the abort threshold is {V['abort']} fire reports"]
    body = [
        f"the rollout ladder climbed tonight, {facts[0]}, {facts[1]}. walk the rings in order, stop at the first ring that hits the threshold, count users in every ring that entered including the one that died. rings after a halt never entered, their users were never exposed.",
        f"the halt rule is blunt on purpose, the incident review found graduated response just meant slow response, blunt it is.",
        f"the feedback pane you do not manage also collects star ratings per ring, stars are sentiment, fires are facts.",
    ]
    entities = [{"id": f"f{i+1}", "question": f"fire reports ring {V['rings'][i][0]} logged, copy the number from its fact line", "answer": V["rings"][i][2]} for i in range(3)] + \
        [{"id": f"u{i+1}", "question": f"users ring {V['rings'][i][0]} covers, copy the number from its fact line", "answer": V["rings"][i][1]} for i in range(3)] + \
        [{"id": f"n{i+1}", "question": f"the ring id of ring number {i+1}, copy it exactly from the facts", "answer": V["rings"][i][0]} for i in range(3)] + \
        [{"id": "at", "question": "the abort threshold in fire reports, copy the number from the facts", "answer": V["abort"]}]
    _rl = [("A", r"abort threshold is (\d+) fire reports", V["abort"])] + \
          [t for idx, (name, users, fires) in enumerate(V["rings"]) for t in (
              (f"f{idx+1}", rf"{name} covers \d+ users and logged (\d+) fire reports", fires),
              (f"u{idx+1}", rf"{name} covers (\d+) users", users),
              (f"n{idx+1}", rf"([A-Za-z0-9_-]+) covers {users} users and logged {fires} fire reports", name))]
    return dict(arch="rollout_rings", domain="feature flag rollout rings", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"rings_passed={passed} aborted={halted} users_exposed={exposed} halt_ring={truth['halt_ring']}"],
                entities=entities,
                aggregate="{'rings_passed': (a('f1')<a('at'))*(1+(a('f2')<a('at'))*(1+(a('f3')<a('at')))), 'aborted': (a('f1')>=a('at'))+(a('f2')>=a('at'))+(a('f3')>=a('at'))>0, 'users_exposed': int(a('u1'))+int(a('u2'))*(a('f1')<a('at'))+int(a('u3'))*(a('f1')<a('at'))*(a('f2')<a('at')), 'halt_ring': (a('n1') if a('f1')>=a('at') else (a('n2') if a('f2')>=a('at') else (a('n3') if a('f3')>=a('at') else '')))}",
                contract={"rings_passed": ("(f1<A)*(1+(f2<A)*(1+(f3<A)))", _rl),
                           "aborted": ("(f1>=A)+((f1<A)*(f2>=A))+((f1<A)*(f2<A)*(f3>=A))", _rl),
                           "users_exposed": ("(u1+u2+u3) if rings_passed>=3 else ((u1+u2+(u3 if aborted and rings_passed==2 else 0)) if rings_passed==2 else ((u1+(u2 if aborted and rings_passed==1 else 0)) if rings_passed==1 else (u1 if aborted else 0)))",
                           [(f"u{i+1}", rf"{V['rings'][i][0]} covers (\d+) users", V['rings'][i][1]) for i in range(3)]),
                           "halt_ring": ("(n1 if aborted and rings_passed==0 else (n2 if aborted and rings_passed==1 else (n3 if aborted and rings_passed==2 else '')))", _rl)},
                variant=v)


def arch_quota_grace(v):
    V = [
        dict(users=[("u1", 55, 50), ("u2", 48, 50), ("u3", 61, 50), ("u4", 50, 50)], grace=7, expired=3),
        dict(users=[("pp", 105, 100), ("qq", 100, 100), ("rr", 118, 100), ("ss", 92, 100)], grace=14, expired=2),
        dict(users=[("m", 23, 20), ("n", 26, 20), ("o", 20, 20), ("p", 19, 20)], grace=3, expired=2),
        dict(users=[("z-1", 250, 200), ("z-2", 199, 200), ("z-3", 275, 200), ("z-4", 200, 200)], grace=30, expired=2),
    ][v]
    warned = locked = expired_g = 0
    for u, used, q in V["users"]:
        if used > q:
            if V["expired"] >= 1 and u == [x for x in V["users"] if x[1] > x[2]][0] and False:
                pass
            warned += 1
    over = [u for u in V["users"] if u[1] >= u[2]]
    warned = len(over)
    locked = 1 if V["expired"] else 0
    expired_g = V["expired"]
    at_line = sum(1 for u in V["users"] if u[1] == u[2])
    truth = {"warned": warned, "locked": locked, "grace_expired_users": expired_g, "at_quota_exactly": at_line}
    rules = [
        "usage at or above quota is over quota for warning purposes, exactly-at counts as over",
        "users over quota enter a grace window, grace expired means locked until they clear space",
        "the quota line is per user, pooled storage is a different desk",
    ]
    facts = [", ".join(f"{u} stands at {used} gigabytes against a {q} gigabyte quota" for u, used, q in V["users"]),
             f"the grace window is {V['grace']} days and {V['expired']} over-quota users have let it lapse"]
    body = [
        f"the quota desk opens the week with the same ritual, {facts[0]}, {facts[1]}. over-quota counting includes exactly-at, the boundary is inside the wall, not outside it. lapsed grace means locked, locked is binary, there is no mostly locked.",
        f"the exactly-at clause was disputed for a full quarter before it won, the argument that settled it was a printout of forty users at exactly quota doing zero cleanup.",
        f"the capacity forecast you do not manage projects quota pressure six months out, forecasts are weather, this desk deals in readings.",
    ]
    entities = [
        {"id": f"u{i+1}", "question": f"gigabytes {u} stands at (copy the number from its fact line)", "answer": used}
        for i, (u, used, q) in enumerate(V["users"])
    ] + [
        {"id": f"q{i+1}", "question": f"{u}'s quota in gigabytes (copy the number from its fact line)", "answer": q}
        for i, (u, used, q) in enumerate(V["users"])
    ] + [
        {"id": "ge", "question": "over-quota users whose grace has lapsed (copy the number from its fact line)", "answer": expired_g},
    ]
    return dict(arch="quota_grace", domain="disk quota enforcement", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"warned={warned} locked={locked} grace_lapsed={expired_g} exactly_at={at_line}"],
                entities=entities,
                aggregate="{'warned': (a('u1')>=a('q1'))+(a('u2')>=a('q2'))+(a('u3')>=a('q3'))+(a('u4')>=a('q4')), 'locked': 1 if a('ge') else 0, 'grace_expired_users': int(a('ge')), 'at_quota_exactly': (a('u1')==a('q1'))+(a('u2')==a('q2'))+(a('u3')==a('q3'))+(a('u4')==a('q4'))}",
                contract={"warned": ("(u1>=q1)+(u2>=q2)+(u3>=q3)+(u4>=q4)",
                             [t for i in range(4) for t in (
                                 (f"u{i+1}", rf"{V['users'][i][0]} stands at (\d+) gigabytes", V["users"][i][1]),
                                 (f"q{i+1}", rf"{V['users'][i][0]} stands at \d+ gigabytes against a (\d+) gigabyte quota", V["users"][i][2]))]),
                          "at_quota_exactly": ("(u1==q1)+(u2==q2)+(u3==q3)+(u4==q4)", [])},
                variant=v)


ARCHETYPES_A = [arch_queue_cutoff, arch_ack_windows, arch_freeze_windows, arch_backoff_budget,
                arch_cache_sweep, arch_payroll_holds, arch_restock_waves, arch_triage_merge,
                arch_retention_purge, arch_token_bucket, arch_rollout_rings, arch_quota_grace]
