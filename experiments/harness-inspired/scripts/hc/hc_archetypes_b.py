"""hc archetype definitions B (13 more, 25 total)."""

def arch_cron_lock(v):
    V = [
        dict(jobs=[("etl-hourly", 20, 60, 3), ("report-daily", 5, 1440, 0)], lock=45),
        dict(jobs=[("sync-15m", 9, 15, 6), ("vacuum", 30, 240, 1)], lock=20),
        dict(jobs=[("poll-5m", 3, 5, 11), ("rollup", 12, 60, 2)], lock=10),
        dict(jobs=[("mirror-30m", 26, 30, 4), ("export", 8, 720, 1)], lock=15),
    ][v]
    ran = skipped = 0
    for name, dur, interval, overruns in V["jobs"]:
        if dur > V["lock"]:
            ran += overruns
            skipped += 0
        else:
            ran += overruns
    for name, dur, interval, overruns in V["jobs"]:
        pass
    total_firings = sum(o for _, _, _, o in V["jobs"])
    lock_violators = sum(1 for j in V["jobs"] if j[1] > V["lock"])
    truth = {"executed": total_firings, "lock_violators": lock_violators, "jobs_total": len(V["jobs"]),
             "longest_run": max(j[1] for j in V["jobs"])}
    rules = [
        f"a run holds the lock for its duration or {V['lock']} minutes, whichever is shorter, longer runs get killed at the lock line",
        "a killed run counts as executed but failed, it still blocks nothing after death",
        "the next firing of a killed job proceeds on schedule, backpressure does not exist here",
    ]
    facts = [", ".join(f"{n} fires every {i} minutes, runs {d} minutes, and overran {o} times tonight" for n, d, i, o in V["jobs"])]
    body = [
        f"the cron rack had a noisy night, {facts[0]}. count firings as executed whether they lived or died, the lock line decides who dies, the schedule decides nothing else.",
        f"the kill-at-lock rule replaced the old wait-forever rule after a nine-hour vacuum ate a monday, dead runs still count as firings, that clause is deliberate and it is in the rules.",
        f"the scheduler ui you do not manage shades overlapping rectangles for prettiness, the shading algorithm lies about boundaries, trust the numbers above.",
    ]
    entities = [
        {"id": "ex", "question": "total runs executed tonight (firings, killed included)", "answer": total_firings},
        {"id": "lv", "question": "jobs whose runtime exceeds the lock line (count)", "answer": lock_violators},
        {"id": "jt", "question": "distinct jobs on the rack tonight", "answer": len(V["jobs"])},
        {"id": "lr", "question": "longest single runtime among the jobs (minutes)", "answer": truth["longest_run"]},
    ]
    return dict(arch="cron_lock", domain="cron overlap policy", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"executed={total_firings} lock_violators={lock_violators} longest={truth['longest_run']}"],
                entities=entities,
                aggregate="{'executed': int(a('ex')), 'lock_violators': int(a('lv')), 'jobs_total': int(a('jt')), 'longest_run': int(a('lr'))}",
                contract={"executed": ("o1+o2",
                             [(f"o{i+1}", rf"{V['jobs'][i][0]} fires every \d+ minutes, runs \d+ minutes, and overran (\d+) times", V["jobs"][i][3]) for i in range(2)]),
                          "lock_violators": ("(d1>L)+(d2>L)",
                             [(f"d{i+1}", rf"{V['jobs'][i][0]} fires every \d+ minutes, runs (\d+) minutes", V["jobs"][i][1]) for i in range(2)] +
                             [("L", r"or (\d+) minutes, whichever is shorter", V["lock"])]),
                          "longest_run": ("max(d1,d2)", [])},
                variant=v)


def arch_dlq_redrive(v):
    V = [
        dict(topics=[("orders", 40, 3), ("billing", 15, 5)], poison=2, redrive_pass=0.75),
        dict(topics=[("telemetry", 90, 4), ("alerts", 22, 6)], poison=3, redrive_pass=0.5),
        dict(topics=[("signups", 30, 2), ("emails", 48, 7)], poison=4, redrive_pass=1.0),
        dict(topics=[("imports", 65, 5), ("hooks", 12, 3)], poison=1, redrive_pass=0.5),
    ][v]
    total_dlq = sum(t[1] for t in V["topics"])
    dead = sum(V["poison"] for _ in V["topics"])
    redrive_pool = total_dlq - dead
    recovered = int(redrive_pool * V["redrive_pass"])
    still_dead_after = redrive_pool - recovered + dead
    truth = {"dlq_total": total_dlq, "poison_dead": dead, "recovered": recovered, "unrecovered": still_dead_after}
    rules = [
        f"a message that failed {V['poison']} or more delivery attempts is poison, poison never redrives",
        "redrive replays everything else once, a fixed fraction comes back healthy tonight, the rest return to the queue",
        "poison counting is per topic but the poison threshold is global",
    ]
    facts = [", ".join(f"the {n} shelf holds {c} messages this morning" for n, c, _ in V["topics"]),
             f"the poison threshold is {V['poison']} failed attempts",
             f"redrive recovers {int(V['redrive_pass']*100)} percent of non-poison messages tonight"]
    body = [
        f"the dead-letter shelf gets swept at dawn, {facts[0]}, {facts[1]}, {facts[2]}. poison is quarantined first and never moves again, the rest take one redrive ride, whatever survives the ride counts as recovered, whatever does not goes back on the shelf.",
        f"the fraction is honest tonight, the queue crew measured it last week, on bad nights it lies, tonight it does not.",
        f"the shadow consumer you do not manage re-reads the shelf for its own metrics, shadow reads neither recover nor poison anything.",
    ]
    entities = [
        {"id": f"c{i+1}", "question": f"messages on the {n} shelf (copy the number from its fact line)", "answer": cnt}
        for i, (n, cnt, _) in enumerate(V["topics"])
    ] + [
        {"id": "pt", "question": "the poison threshold in failed attempts (copy the number from its fact line)", "answer": V["poison"]},
        {"id": "fp", "question": "percent of non-poison messages the redrive recovers (copy the number from its fact line)", "answer": int(V["redrive_pass"] * 100)},
    ]
    return dict(arch="dlq_redrive", domain="message queue dead letters", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"dlq={total_dlq} poison={dead} recovered={recovered} unrecovered={still_dead_after}"],
                entities=entities,
                aggregate="{'dlq_total': a('c1')+a('c2'), 'poison_dead': 2*a('pt'), 'recovered': int((a('c1')+a('c2')-2*a('pt'))*a('fp')/100), 'unrecovered': (a('c1')+a('c2'))-int((a('c1')+a('c2')-2*a('pt'))*a('fp')/100)}",
                contract={"dlq_total": ("c1+c2",
                             [(f"c{i+1}", rf"the {V['topics'][i][0]} shelf holds (\d+) messages", V["topics"][i][1]) for i in range(2)]),
                          "poison_dead": ("2*P", [("P", r"poison threshold is (\d+)", V["poison"])]),
                          "recovered": ("int((dlq_total - poison_dead) * F / 100)", [("F", r"redrive recovers (\d+) percent", int(V["redrive_pass"] * 100))])},
                variant=v)


def arch_cert_renewal(v):
    V = [
        dict(certs=[("api-cert", 12), ("ingress", 45), ("internal-ca", 200), ("legacy-smtp", 8)], minvalid=30),
        dict(certs=[("edge-wild", 5), ("svc-mesh", 60), ("db-tls", 25), ("monitor", 90)], minvalid=14),
        dict(certs=[("a", 29), ("b", 31), ("c", 100), ("d", 1)], minvalid=30),
        dict(certs=[("one", 44), ("two", 13), ("three", 72), ("four", 30)], minvalid=45),
    ][v]
    renewed = sum(1 for n, d in V["certs"] if d < V["minvalid"])
    expiring_soon = sum(1 for n, d in V["certs"] if V["minvalid"] <= d < V["minvalid"] * 3)
    stable = len(V["certs"]) - renewed - expiring_soon
    truth = {"renewed": renewed, "expiring_soon": expiring_soon, "stable": stable, "renew_window": V["minvalid"]}
    rules = [
        f"any certificate with less than {V['minvalid']} days of validity left renews tonight, no exceptions, no appeals",
        f"certificates with {V['minvalid']} or more but under {V['minvalid']*3} days are flagged expiring-soon, flagged is not renewed",
        "renewal stagger is automatic, count renewals, do not schedule them",
    ]
    facts = [", ".join(f"{n} has {d} days left" for n, d in V["certs"])]
    body = [
        f"the cert cabinet gets its weekly review, {facts[0]}. the renew line is {V['minvalid']} days, below it means renew tonight, the expiring-soon band runs to triple that, above triple means stable. three buckets, every cert in exactly one.",
        f"the triple-day band exists because renewal takes a week of lead time in the worst case, band membership warns, it does not act.",
        f"the external monitor you do not manage pages on 7-day emergencies, by the time it notices anything this desk has already acted or already failed, either way it is late.",
    ]
    entities = [
        {"id": f"d{i+1}", "question": f"days left on {n} (copy the number from its fact line)", "answer": days}
        for i, (n, days) in enumerate(V["certs"])
    ] + [
        {"id": "rw", "question": "the renew line in days (copy the number from its fact line)", "answer": V["minvalid"]},
    ]
    return dict(arch="cert_renewal", domain="certificate expiry renewals", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"renewed={renewed} expiring_soon={expiring_soon} stable={stable} line={V['minvalid']}"],
                entities=entities,
                aggregate="{'renewed': (a('d1')<a('rw'))+(a('d2')<a('rw'))+(a('d3')<a('rw'))+(a('d4')<a('rw')), 'expiring_soon': (a('d1')>=a('rw'))*(a('d1')<3*a('rw'))+(a('d2')>=a('rw'))*(a('d2')<3*a('rw'))+(a('d3')>=a('rw'))*(a('d3')<3*a('rw'))+(a('d4')>=a('rw'))*(a('d4')<3*a('rw')), 'stable': (a('d1')>=3*a('rw'))+(a('d2')>=3*a('rw'))+(a('d3')>=3*a('rw'))+(a('d4')>=3*a('rw')), 'renew_window': int(a('rw'))}",
                contract={"renewed": ("(d1<M)+(d2<M)+(d3<M)+(d4<M)",
                             [(f"d{i+1}", rf"{V['certs'][i][0]} has (\d+) days left", V["certs"][i][1]) for i in range(4)] +
                             [("M", r"less than (\d+) days of validity", V["minvalid"])]),
                          "expiring_soon": ("(d1>=M and d1<3*M)+(d2>=M and d2<3*M)+(d3>=M and d3<3*M)+(d4>=M and d4<3*M)", []),
                          "stable": ("4 - renewed - expiring_soon", [])},
                variant=v)


def arch_backup_window(v):
    V = [
        dict(jobs=[("full-db", 380, 1), ("incr-logs", 25, 4)], window=420),
        dict(jobs=[("full-vm", 290, 1), ("incr-app", 40, 3)], window=380),
        dict(jobs=[("full-mail", 150, 1), ("incr-config", 10, 8)], window=210),
        dict(jobs=[("full-media", 460, 1), ("incr-meta", 35, 5)], window=600),
    ][v]
    total = sum(d * n for _, d, n in V["jobs"])
    window_used = total
    fit = total <= V["window"]
    truth = {"window_minutes": V["window"], "minutes_used": window_used,
             "minutes_free": V["window"] - window_used, "fit": fit}
    rules = [
        f"the backup window is {V['window']} minutes, the report states the minutes used and whether everything fit, nothing is deferred and nothing moves, the fit flag is the deliverable",
        "incremental jobs are scheduled after the full, in the order listed, sizes are sizes, no compression magic",
        "minutes free may read negative on a night that does not fit, negative headroom is the honest report",
    ]
    facts = [", ".join(f"{n} takes {d} minutes and runs {c} times" for n, d, c in V["jobs"])]
    body = [
        f"the backup window opens at two and closes when it closes, {facts[0]}. sum the minutes, compare to the window, the fit is boolean, there is no almost. leftover minutes are slack, slack is not a resource, it is a report line.",
        f"the no-resume clause was written over a restore drill that found half a full and a whole lot of nothing, half a backup is a rumor, not a backup.",
        f"the dedupe appliance you do not manage claims big savings on its own graphs, the window schedules raw minutes, raw is what runs.",
    ]
    entities = [
        {"id": "wm", "question": "the backup window length in minutes", "answer": V["window"]},
        {"id": "mu", "question": "minutes actually used (all jobs summed)", "answer": window_used},
        {"id": "mf", "question": "minutes free after everything scheduled", "answer": V["window"] - window_used},
        {"id": "ft", "question": "does everything fit inside the window (true or false)", "answer": fit},
    ]
    return dict(arch="backup_window", domain="backup window scheduling", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"window={V['window']} used={window_used} free={V['window']-window_used} fit={fit}"],
                entities=entities,
                aggregate="{'window_minutes': int(a('wm')), 'minutes_used': int(a('mu')), 'minutes_free': int(a('mf')), 'fit': bool(a('ft'))}",
                contract={"minutes_used": ("d1*c1+d2*c2",
                             [t for i in range(2) for t in (
                                 (f"d{i+1}", rf"{V['jobs'][i][0]} takes (\d+) minutes", V["jobs"][i][1]),
                                 (f"c{i+1}", rf"{V['jobs'][i][0]} takes \d+ minutes and runs (\d+) times", V["jobs"][i][2]))]),
                          "minutes_free": ("W - minutes_used", [("W", r"backup window is (\d+) minutes", V["window"])]),
                          "fit": ("minutes_used <= W", [])},
                variant=v)


def arch_cart_audit(v):
    V = [
        dict(sessions=200, abandoned=48, restored=15, merged=9, converted=57),
        dict(sessions=340, abandoned=90, restored=22, merged=14, converted=61),
        dict(sessions=120, abandoned=20, restored=8, merged=3, converted=45),
        dict(sessions=520, abandoned=150, restored=31, merged=26, converted=88),
    ][v]
    net_abandoned = V["abandoned"] - V["restored"]
    truth = {"net_abandoned": net_abandoned, "restored": V["restored"], "merged": V["merged"], "converted": V["converted"]}
    rules = [
        "a restored cart leaves the abandoned count, restore is a rescue, not a re-abandon",
        "merged carts collapse two sessions into one before any ratio math, merge is a fact, not a loss",
        "conversion counts checkouts from live carts only, restored carts that converted count in both restored and converted",
    ]
    facts = [f"{V['sessions']} sessions opened carts tonight",
             f"{V['abandoned']} carts were abandoned, {V['restored']} of those were later restored",
             f"{V['merged']} pairs of carts merged into single carts",
             f"{V['converted']} carts converted to checkouts"]
    body = [
        f"the cart auditor reconciles the night, {facts[0]}, {facts[1]}, {facts[2]}, {facts[3]}. abandoned minus restored is the net the morning meeting argues about, merged carts are a count of merges not of losses, converted stands alone as the money line.",
        f"restore-then-convert double counting was once called a bug, it is not, it is the two ledgers doing their jobs, restored tracks rescues, converted tracks revenue, one cart can honestly appear in both.",
        f"the remarketing pixel you do not manage also fires restore emails, pixel attributions do not enter these ledgers.",
    ]
    entities = [
        {"id": "na", "question": "net abandoned carts (abandoned minus restored)", "answer": net_abandoned},
        {"id": "rs", "question": "carts restored after abandonment", "answer": V["restored"]},
        {"id": "mg", "question": "cart merge operations tonight", "answer": V["merged"]},
        {"id": "cv", "question": "carts converted to checkouts", "answer": V["converted"]},
    ]
    return dict(arch="cart_audit", domain="e-commerce cart audits", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"net_abandoned={net_abandoned} restored={V['restored']} merged={V['merged']} converted={V['converted']}"],
                entities=entities,
                aggregate="{'net_abandoned': int(a('na')), 'restored': int(a('rs')), 'merged': int(a('mg')), 'converted': int(a('cv'))}",
                contract={"net_abandoned": ("A - R", [("A", r"(\d+) carts were abandoned", V["abandoned"]), ("R", r"(\d+) of those were later restored", V["restored"])])},
                variant=v)


def arch_fuel_cards(v):
    V = [
        dict(drivers=[("d-1", 40, 60, False), ("d-2", 75, 60, True), ("d-3", 60, 60, False)], cap=60),
        dict(drivers=[("ana", 88, 80, True), ("bjorn", 79, 80, False), ("cato", 80, 80, False)], cap=80),
        dict(drivers=[("t1", 30, 25, False), ("t2", 25, 25, True), ("t3", 26, 25, False)], cap=25),
        dict(drivers=[("nine", 140, 120, True), ("ten", 119, 120, False), ("eleven", 120, 120, False)], cap=120),
    ][v]
    blocked = approved = over = 0
    for d, spent, cap, has_approval in V["drivers"]:
        if spent >= cap:
            over += 1
            if has_approval:
                approved += 1
            else:
                blocked += 1
    at_cap = sum(1 for d in V["drivers"] if d[1] == d[2])
    truth = {"blocked": blocked, "exception_approved": approved, "over_cap": over, "at_cap_exactly": at_cap}
    rules = [
        "spending at or over the daily cap flags the card, exactly-at is flagged same as over",
        "a flagged card with a pre-filed route exception goes to the approved pile, everything else blocks until a human calls in",
        "blocked tonight does not mean fired, it means the morning list, count them anyway",
    ]
    facts = [", ".join(f"{d} burned {s} liters against a {c} liter cap{' with a pre-filed exception' if ap else ''}" for d, s, c, ap in V["drivers"])]
    body = [
        f"the fuel desk closes the ledger at midnight, {facts[0]}. flag at-or-over first, then split flagged cards by exception, the lists write themselves once the flagging is honest.",
        f"exactly-at counting as flagged was a policy fight the auditors won, the boundary belongs to the flagged side, same as every other boundary at this desk.",
        f"the telematics feed you do not manage estimates burn per trip, estimates settle nothing, the pump numbers are the facts.",
    ]
    entities = [
        {"id": f"s{i+1}", "question": f"liters {d} burned (copy the number from its fact line)", "answer": spent}
        for i, (d, spent, cap, ap) in enumerate(V["drivers"])
    ] + [
        {"id": f"c{i+1}", "question": f"{d}'s cap in liters (copy the number from its fact line)", "answer": cap}
        for i, (d, spent, cap, ap) in enumerate(V["drivers"])
    ] + [
        {"id": f"x{i+1}", "question": f"does {d} have a pre-filed exception, one word yes or no", "answer": "yes" if ap else "no"}
        for i, (d, spent, cap, ap) in enumerate(V["drivers"])
    ]
    return dict(arch="fuel_cards", domain="fleet fuel card audits", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"blocked={blocked} approved={approved} over={over} exactly_at={at_cap}"],
                entities=entities,
                aggregate="{'blocked': (a('s1')>=a('c1'))*(a('x1')=='no')+(a('s2')>=a('c2'))*(a('x2')=='no')+(a('s3')>=a('c3'))*(a('x3')=='no'), 'exception_approved': (a('s1')>=a('c1'))*(a('x1')=='yes')+(a('s2')>=a('c2'))*(a('x2')=='yes')+(a('s3')>=a('c3'))*(a('x3')=='yes'), 'over_cap': (a('s1')>=a('c1'))+(a('s2')>=a('c2'))+(a('s3')>=a('c3')), 'at_cap_exactly': (a('s1')==a('c1'))+(a('s2')==a('c2'))+(a('s3')==a('c3'))}",
                contract={"at_cap_exactly": ("(s1==c1)+(s2==c2)+(s3==c3)",
                             [t for i in range(3) for t in (
                                 (f"s{i+1}", rf"{V['drivers'][i][0]} burned (\d+) liters", V["drivers"][i][1]),
                                 (f"c{i+1}", rf"{V['drivers'][i][0]} burned \d+ liters against a (\d+) liter cap", V["drivers"][i][2]))]),
                          "over_cap": ("(s1>=c1)+(s2>=c2)+(s3>=c3)", [])},
                variant=v)


def arch_seat_alloc(v):
    V = [
        dict(tiers=[("staff", 4, 3), ("vip", 2, 2), ("general", 30, 25)], waitlist=6, noshow=2),
        dict(tiers=[("a", 8, 8), ("b", 5, 3), ("c", 40, 31)], waitlist=9, noshow=3),
        dict(tiers=[("core", 3, 2), ("plus", 4, 4), ("open", 25, 20)], waitlist=5, noshow=1),
        dict(tiers=[("one", 10, 6), ("two", 6, 5), ("three", 60, 55)], waitlist=12, noshow=4),
    ][v]
    seated = 0
    bumped = 0
    free = []
    for name, seats, demand in V["tiers"]:
        seated += min(seats, demand)
        if demand > seats:
            bumped += demand - seats
            free.append((name, 0))
        else:
            free.append((name, seats - demand))
    promoted = min(V["waitlist"], sum(f for _, f in free))
    wl_left = V["waitlist"] - promoted
    truth = {"seated": seated, "bumped": bumped, "waitlist_promoted": promoted, "waitlist_left": wl_left}
    rules = [
        "tiers fill in priority order, staff before vip before general, order is the whole rule",
        "unfilled seats in higher tiers never cascade down, a seat either fills in its tier or stays empty",
        "the waitlist promotes into empty seats anywhere after tier allocation, no-shows do not free seats tonight",
    ]
    facts = [", ".join(f"the {n} tier has {s} seats against {d} requests" for n, s, d in V["tiers"]),
             f"the waitlist holds {V['waitlist']} names, {V['noshow']} ticketed seats went empty via no-shows"]
    body = [
        f"the seating sheet finalizes an hour before doors, {facts[0]}, {facts[1]}. seat within tiers by priority, overflow bumps, empty seats stay empty until the waitlist pass, the waitlist takes whatever empties remain, one name per seat, no standing offers.",
        f"the no-cascade clause protects tier pricing, the waitlist clause fills the room anyway when it can, the two rules disagree in spirit and both hold.",
        f"the door scanner you do not manage counts entries in real time, entry counts are downstream of seating, not seating itself.",
    ]
    entities = [
        {"id": f"s{i+1}", "question": f"seats in the {n} tier (copy the number from its fact line)", "answer": seats}
        for i, (n, seats, demand) in enumerate(V["tiers"])
    ] + [
        {"id": f"d{i+1}", "question": f"requests for the {n} tier (copy the number from its fact line)", "answer": demand}
        for i, (n, seats, demand) in enumerate(V["tiers"])
    ] + [
        {"id": "wq", "question": "names on the waitlist (copy the number from its fact line)", "answer": V["waitlist"]},
    ]
    return dict(arch="seat_alloc", domain="classroom seat allocation", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"seated={seated} bumped={bumped} promoted={promoted} waitlist_left={wl_left}"],
                entities=entities,
                aggregate="{'seated': min(a('s1'),a('d1'))+min(a('s2'),a('d2'))+min(a('s3'),a('d3')), 'bumped': max(a('d1')-a('s1'),0)+max(a('d2')-a('s2'),0)+max(a('d3')-a('s3'),0), 'waitlist_promoted': min(a('wq'), (a('s1')-a('d1') if a('d1')<a('s1') else 0)+(a('s2')-a('d2') if a('d2')<a('s2') else 0)+(a('s3')-a('d3') if a('d3')<a('s3') else 0)), 'waitlist_left': a('wq')-min(a('wq'), (a('s1')-a('d1') if a('d1')<a('s1') else 0)+(a('s2')-a('d2') if a('d2')<a('s2') else 0)+(a('s3')-a('d3') if a('d3')<a('s3') else 0))}",
                contract={"seated": ("min(s1,d1)+min(s2,d2)+min(s3,d3)",
                             [t for i in range(3) for t in (
                                 (f"s{i+1}", rf"the {V['tiers'][i][0]} tier has (\d+) seats", V["tiers"][i][1]),
                                 (f"d{i+1}", rf"the {V['tiers'][i][0]} tier has \d+ seats against (\d+) requests", V["tiers"][i][2]))]),
                          "bumped": ("max(d1-s1,0)+max(d2-s2,0)+max(d3-s3,0)", []),
                          "waitlist_promoted": ("min(W, (s1-d1 if d1<s1 else 0)+(s2-d2 if d2<s2 else 0)+(s3-d3 if d3<s3 else 0))", [("W", r"waitlist holds (\d+) names", V["waitlist"])])},
                variant=v)


def arch_specimen_routing(v):
    V = [
        dict(samples=[("s-01", "cold", 1, True), ("s-02", "room", 0, True), ("s-03", "cold", 3, False), ("s-04", "frozen", 0, True)], exc_limit=2),
        dict(samples=[("b1", "cold", 0, True), ("b2", "cold", 2, True), ("b3", "room", 4, False), ("b4", "frozen", 1, False)], exc_limit=1),
        dict(samples=[("x", "room", 2, True), ("y", "cold", 0, False), ("z", "frozen", 0, True), ("w", "cold", 1, True)], exc_limit=2),
        dict(samples=[("n1", "frozen", 5, False), ("n2", "cold", 1, True), ("n3", "room", 0, True), ("n4", "cold", 0, True)], exc_limit=1),
    ][v]
    held = rejected = shipped = 0
    for sid, band, exc, complete in V["samples"]:
        if not complete:
            rejected += 1
        elif exc > V["exc_limit"]:
            held += 1
        else:
            shipped += 1
    cold_shipped = sum(1 for s in V["samples"] if s[1] == "cold" and s[3] and s[2] <= V["exc_limit"])
    truth = {"held": held, "rejected": rejected, "shipped": shipped, "cold_band_shipped": cold_shipped}
    rules = [
        f"a sample with more than {V['exc_limit']} temperature excursions holds for review, excursion means minutes outside band, at the limit ships",
        "incomplete paperwork rejects outright, rejection happens before any excursion math",
        "band labels travel with the sample but gate nothing tonight",
    ]
    facts = [", ".join(f"{s} rides the {b} band, logged {e} excursions, paperwork {'complete' if c else 'incomplete'}" for s, b, e, c in V["samples"])]
    body = [
        f"the specimen router decides the morning courier manifest, {facts[0]}. paperwork first, excursions second, band last as a label only. holds wait for a clinician, rejects go back to the floor, ships make the truck.",
        f"the at-the-limit-ships clause matters, exactly at limit is compliant, over limit holds, the boundary rule reads the same as every boundary at this desk.",
        f"the courier portal you do not manage reprints manifests with its own hold column, the portal column is a guess, this sheet is the manifest.",
    ]
    entities = [
        {"id": f"e{i+1}", "question": f"excursions {sid} logged (copy the number from its fact line)", "answer": exc}
        for i, (sid, band, exc, comp) in enumerate(V["samples"])
    ] + [
        {"id": f"p{i+1}", "question": f"{sid} paperwork status as a code, answer 1 for complete or 0 for incomplete", "answer": 1 if comp else 0}
        for i, (sid, band, exc, comp) in enumerate(V["samples"])
    ] + [
        {"id": f"b{i+1}", "question": f"{sid}'s band as a code, answer 1 for cold, 2 for room, or 3 for frozen", "answer": {"cold": 1, "room": 2, "frozen": 3}[band]}
        for i, (sid, band, exc, comp) in enumerate(V["samples"])
    ] + [
        {"id": "xl", "question": f"the excursion limit from the rule sentence stating samples with more than N temperature excursions hold for review (that N is the limit, not any sample's excursion count)", "answer": V["exc_limit"]},
    ]
    _sl = [("L", r"more than (\d+) temperature excursions", V["exc_limit"])] + \
          [t for idx, (sid, band, exc, comp) in enumerate(V["samples"]) for t in (
              (f"e{idx+1}", rf"{sid} rides the \w+ band, logged (\d+) excursions", exc),
              (f"p{idx+1}", rf"{sid} rides the \w+ band, logged \d+ excursions, paperwork (complete|incomplete)", 1 if comp else 0,
               {"complete": 1, "incomplete": 0}),
              (f"b{idx+1}", rf"{sid} rides the (\w+) band", band))]
    _held = "+".join(f"((e{i+1}>L)*p{i+1})" for i in range(4))
    _rej = "+".join(f"(1-p{i+1})" for i in range(4))
    _cs = "+".join(f"((e{i+1}<=L)*p{i+1}*(b{i+1}=='cold'))" for i in range(4))
    return dict(arch="specimen_routing", domain="lab specimen routing", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"held={held} rejected={rejected} shipped={shipped} cold_shipped={cold_shipped}"],
                entities=entities,
                aggregate="{'held': (a('e1')>a('xl'))*a('p1')+(a('e2')>a('xl'))*a('p2')+(a('e3')>a('xl'))*a('p3')+(a('e4')>a('xl'))*a('p4'), 'rejected': (1-a('p1'))+(1-a('p2'))+(1-a('p3'))+(1-a('p4')), 'shipped': 4-((a('e1')>a('xl'))*a('p1')+(a('e2')>a('xl'))*a('p2')+(a('e3')>a('xl'))*a('p3')+(a('e4')>a('xl'))*a('p4'))-((1-a('p1'))+(1-a('p2'))+(1-a('p3'))+(1-a('p4'))), 'cold_band_shipped': (a('e1')<=a('xl'))*a('p1')*(a('b1')==1)+(a('e2')<=a('xl'))*a('p2')*(a('b2')==1)+(a('e3')<=a('xl'))*a('p3')*(a('b3')==1)+(a('e4')<=a('xl'))*a('p4')*(a('b4')==1)}",
                contract={"held": (_held, _sl),
                           "rejected": (_rej, _sl),
                           "shipped": ("4 - held - rejected", []),
                           "cold_band_shipped": (_cs, _sl)},
                variant=v)


def arch_table_turns(v):
    V = [
        dict(res=12, noshow=3, walk=7, cap=14, turn=40),
        dict(res=20, noshow=5, walk=11, cap=22, turn=75),
        dict(res=9, noshow=1, walk=4, cap=12, turn=30),
        dict(res=30, noshow=9, walk=14, cap=28, turn=120),
    ][v]
    seated_res = V["res"] - V["noshow"]
    open_seats = max(V["cap"] - seated_res, 0)
    seated_walk = min(V["walk"], open_seats)
    turned = V["turn"]
    truth = {"seated_reservations": seated_res, "walkins_seated": seated_walk,
             "vacant_at_close": V["cap"] - seated_res - seated_walk, "turns_completed": turned}
    rules = [
        "no-show reservations release their seats after fifteen minutes, released seats go to walk-ins in arrival order",
        "walk-ins beyond open seats leave, there is no bar-queue promise tonight",
        "a turn counts a table reset between parties, turns are counted independent of who sat",
    ]
    facts = [f"{V['res']} reservations booked against {V['cap']} tables",
             f"{V['noshow']} reservations no-showed",
             f"{V['walk']} walk-in parties arrived",
             f"the floor completed {V['turn']} table turns tonight"]
    body = [
        f"the floor manager counts the night in tables, {facts[0]}, {facts[1]}, {facts[2]}, {facts[3]}. reservations seated first minus no-shows, walk-ins fill what opened, vacant is what never filled, turns stand apart as the reset count.",
        f"the fifteen-minute release exists because the old five-minute rule starved the walk-in lane on fridays, fifteen won after a month of arguments.",
        f"the reservation app you do not manage shows covers including waitlist holds, app covers are projections, tables are facts.",
    ]
    entities = [
        {"id": "sr", "question": "reservations seated (booked minus no-shows)", "answer": seated_res},
        {"id": "ws", "question": "walk-in parties seated (limited by open seats)", "answer": seated_walk},
        {"id": "vc", "question": "tables vacant at close", "answer": truth["vacant_at_close"]},
        {"id": "tc", "question": "table turns completed tonight", "answer": turned},
    ]
    return dict(arch="table_turns", domain="restaurant table turns", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"seated_res={seated_res} walkins={seated_walk} vacant={truth['vacant_at_close']} turns={turned}"],
                entities=entities,
                aggregate="{'seated_reservations': int(a('sr')), 'walkins_seated': int(a('ws')), 'vacant_at_close': int(a('vc')), 'turns_completed': int(a('tc'))}",
                contract={"seated_reservations": ("R - N", [("R", r"(\d+) reservations booked", V["res"]), ("N", r"(\d+) reservations no-showed", V["noshow"])]),
                          "walkins_seated": ("min(W, max(C - seated_reservations, 0))", [("W", r"(\d+) walk-in parties arrived", V["walk"]), ("C", r"against (\d+) tables", V["cap"])])},
                variant=v)


def arch_gym_standby(v):
    V = [
        dict(cap=20, booked=18, standby=6, credits=4, noshows=3),
        dict(cap=15, booked=15, standby=8, credits=2, noshows=5),
        dict(cap=30, booked=24, standby=5, credits=3, noshows=1),
        dict(cap=12, booked=12, standby=9, credits=5, noshows=4),
    ][v]
    open_seats = max(V["cap"] - V["booked"] + V["noshows"], 0)
    confirmed_standby = min(V["standby"], open_seats)
    standby_left = V["standby"] - confirmed_standby
    refunded = V["credits"]
    truth = {"standby_confirmed": confirmed_standby, "standby_left": standby_left, "credits_refunded": refunded, "no_shows": V["noshows"]}
    rules = [
        "no-show bookings free their spots at start time, freed spots go to standby in queue order",
        "standby beyond freed spots does not get in, no floor mats, no standing, capacity is capacity",
        "class credits held by no-shows refund automatically at the one-hour mark",
    ]
    facts = [f"the class caps at {V['cap']}, {V['booked']} booked, {V['noshows']} of those no-showed",
             f"{V['standby']} names sat on the standby queue",
             f"{V['credits']} credits are pending automatic refund"]
    body = [
        f"the roster locks at start time, {facts[0]}, {facts[1]}, {facts[2]}. frees first, standby fills frees, leftovers stay queued, credits refund without anyone lifting a finger. four numbers out, no narrative.",
        f"the automatic refund came from a billing dispute backlog, humans refunding by hand made errors at 2am, the rule now is the clock does it.",
        f"the gym app you do not manage shows a live capacity bar with rounding, the bar is for morale, the roster is for truth.",
    ]
    entities = [
        {"id": "cp", "question": "the class capacity (copy the number from its fact line)", "answer": V["cap"]},
        {"id": "bk", "question": "bookings held (copy the number from its fact line)", "answer": V["booked"]},
        {"id": "ns", "question": "booked members who no-showed (copy the number from its fact line)", "answer": V["noshows"]},
        {"id": "sq", "question": "names on the standby queue (copy the number from its fact line)", "answer": V["standby"]},
        {"id": "cr", "question": "credits pending refund (copy the number from its fact line)", "answer": V["credits"]},
    ]
    return dict(arch="gym_standby", domain="gym class capacity", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"standby_confirmed={confirmed_standby} left={standby_left} refunded={refunded} noshows={V['noshows']}"],
                entities=entities,
                aggregate="{'standby_confirmed': min(a('sq'), max(a('cp')-a('bk')+a('ns'), 0)), 'standby_left': a('sq')-min(a('sq'), max(a('cp')-a('bk')+a('ns'), 0)), 'credits_refunded': int(a('cr')), 'no_shows': int(a('ns'))}",
                contract={"standby_confirmed": ("min(Q, max(cap - booked + ns, 0))", [("cap", r"class caps at (\d+)", V["cap"]), ("booked", r"(\d+) booked", V["booked"]), ("ns", r"of those no-showed", V["noshows"]), ("Q", r"(\d+) names sat on the standby", V["standby"])])},
                variant=v)


def arch_hold_shelf(v):
    V = [
        dict(holds=30, expired=7, renewed=12, picked=9, renew_cap=1),
        dict(holds=45, expired=10, renewed=20, picked=17, renew_cap=2),
        dict(holds=18, expired=2, renewed=6, picked=8, renew_cap=1),
        dict(holds=60, expired=15, renewed=22, picked=28, renew_cap=3),
    ][v]
    active = V["holds"] - V["expired"]
    truth = {"active_holds": active, "expired_released": V["expired"],
             "renewals_used": V["renewed"], "picked_up": V["picked"]}
    rules = [
        "holds expire after seven days on the shelf, expired holds release the item back to circulation, release is final",
        f"patrons may renew a hold up to {V['renew_cap']} times, renewals reset nothing else",
        "pickup removes the hold entirely, picked-up items leave the ledger",
    ]
    facts = [f"{V['holds']} holds sat on the shelf this week",
             f"{V['expired']} expired and released",
             f"{V['renewed']} renewals were used",
             f"{V['picked']} items were picked up"]
    body = [
        f"the hold shelf audit runs weekly, {facts[0]}, {facts[1]}, {facts[2]}, {facts[3]}. expired releases leave the active count first, renewals and pickups describe the survivors, the active number is holds minus released and nothing subtler.",
        f"release-is-final was not always the rule, the grace week doubled the shelf and halved the pickups, final means final since.",
        f"the catalog kiosk you do not manage displays holds in transit separately, in-transit items are not on this shelf and never were.",
    ]
    entities = [
        {"id": "ah", "question": "active holds remaining (holds minus expired)", "answer": active},
        {"id": "er", "question": "holds expired and released this week", "answer": V["expired"]},
        {"id": "ru", "question": "renewals used this week", "answer": V["renewed"]},
        {"id": "pu", "question": "items picked up this week", "answer": V["picked"]},
    ]
    return dict(arch="hold_shelf", domain="library hold shelves", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"active={active} expired={V['expired']} renewals={V['renewed']} picked={V['picked']}"],
                entities=entities,
                aggregate="{'active_holds': int(a('ah')), 'expired_released': int(a('er')), 'renewals_used': int(a('ru')), 'picked_up': int(a('pu'))}",
                contract={"active_holds": ("H - E", [("H", r"(\d+) holds sat on the shelf", V["holds"]), ("E", r"(\d+) expired and released", V["expired"])])},
                variant=v)


def arch_print_quota(v):
    dict(jobs=0)
    V = [
        dict(users=[("p1", 120, 100), ("p2", 60, 100), ("p3", 100, 100)], pages=110, color_denied=1),
        dict(users=[("q1", 250, 200), ("q2", 200, 200), ("q3", 180, 200)], pages=260, color_denied=2),
        dict(users=[("r1", 40, 50), ("r2", 50, 50), ("r3", 48, 50)], pages=45, color_denied=0),
        dict(users=[("s1", 310, 300), ("s2", 299, 300), ("s3", 300, 300)], pages=320, color_denied=3),
    ][v]
    flagged = sum(1 for u, used, q in V["users"] if used >= q)
    mono_pass = sum(1 for u, used, q in V["users"] if used < q)
    truth = {"users_flagged": flagged, "users_clear": mono_pass, "color_jobs_denied": V["color_denied"], "largest_job_pages": V["pages"]}
    rules = [
        "page quota flags at exactly-at, same boundary convention as every quota desk here",
        "color printing requires quota headroom, flagged users' color jobs deny outright, mono jobs pass with a warning",
        "denied jobs do not print tonight, denial is not deferral",
    ]
    facts = [", ".join(f"{u} printed {n} of a {q} page monthly quota" for u, n, q in V["users"]),
             f"the largest single job tonight ran {V['pages']} pages",
             f"{V['color_denied']} color jobs were denied at the counter"]
    body = [
        f"the print counter closes the month, {facts[0]}, {facts[1]}, {facts[2]}. flagged means at-or-over quota, clear means under, color denial rides on top of the flag count, mono never denies.",
        f"the color-gates-on-flag rule came from a toner budget crisis nobody wants repeated, color is a privilege of the under-quota.",
        f"the managed print service you do not manage emails monthly summaries with different rounding, summaries are for managers, this counter is for the record.",
    ]
    entities = [
        {"id": f"n{i+1}", "question": f"pages {u} printed this month (copy the number from its fact line)", "answer": used}
        for i, (u, used, q) in enumerate(V["users"])
    ] + [
        {"id": f"q{i+1}", "question": f"{u}'s monthly page quota (copy the number from its fact line)", "answer": q}
        for i, (u, used, q) in enumerate(V["users"])
    ] + [
        {"id": "cd", "question": "color jobs denied tonight (copy the number from its fact line)", "answer": V["color_denied"]},
        {"id": "lp", "question": "largest single job page count (copy the number from its fact line)", "answer": V["pages"]},
    ]
    return dict(arch="print_quota", domain="print queue quotas", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"flagged={flagged} clear={mono_pass} color_denied={V['color_denied']} largest={V['pages']}"],
                entities=entities,
                aggregate="{'users_flagged': (a('n1')>=a('q1'))+(a('n2')>=a('q2'))+(a('n3')>=a('q3')), 'users_clear': (a('n1')<a('q1'))+(a('n2')<a('q2'))+(a('n3')<a('q3')), 'color_jobs_denied': int(a('cd')), 'largest_job_pages': int(a('lp'))}",
                contract={"users_flagged": ("(n1>=q1)+(n2>=q2)+(n3>=q3)",
                             [t for i in range(3) for t in (
                                 (f"n{i+1}", rf"{V['users'][i][0]} printed (\d+) of", V["users"][i][1]),
                                 (f"q{i+1}", rf"{V['users'][i][0]} printed \d+ of a (\d+) page monthly quota", V["users"][i][2]))]),
                          "users_clear": ("3 - users_flagged", [])},
                variant=v)


def arch_vent_control(v):
    V = [
        dict(zones=[("z-north", 27, 5), ("z-south", 22, 0), ("z-east", 31, 8)], hi=26, lo=18, override=1),
        dict(zones=[("gh-1", 19, 0), ("gh-2", 28, 12), ("gh-3", 24, 3)], hi=25, lo=15, override=2),
        dict(zones=[("w1", 33, 15), ("w2", 21, 2), ("w3", 26, 6)], hi=32, lo=20, override=0),
        dict(zones=[("bay-a", 30, 9), ("bay-b", 17, 0), ("bay-c", 25, 4)], hi=24, lo=16, override=3),
    ][v]
    over = sum(1 for z, t, m in V["zones"] if t > V["hi"])
    under = sum(1 for z, t, m in V["zones"] if t < V["lo"])
    vent_min = sum(m for z, t, m in V["zones"] if t > V["hi"])
    truth = {"zones_over": over, "zones_under": under, "vent_minutes": vent_min, "overrides_active": V["override"]}
    rules = [
        f"zones above {V['hi']} degrees vent until back in band, vent minutes accrue per zone per excursion",
        f"zones below {V['lo']} degrees heat, heating never counts against vent minutes",
        "manual overrides lock a zone's vents closed, overrides are counted and never argued with",
    ]
    facts = [", ".join(f"zone {z} reads {t} degrees with {m} vent minutes logged" for z, t, m in V["zones"]),
             f"the high band edge is {V['hi']}, the low edge is {V['lo']}",
             f"{V['override']} manual overrides are active tonight"]
    body = [
        f"the vent controller reports by zone, {facts[0]}, {facts[1]}, {facts[2]}. band edges are strict, above high vents, below low heats, in-band idles. vent minutes sum only for over-band zones, overrides lock zones out but still count as overrides.",
        f"strict edges won after a season of hysteresis arguments, the controller now believes the thermometer and nothing else.",
        f"the weather feed you do not manage forecasts band shifts overnight, forecasts do not move actuators, readings do.",
    ]
    entities = [
        {"id": f"t{i+1}", "question": f"zone {z}'s current reading in degrees (copy the number from its fact line)", "answer": temp}
        for i, (z, temp, m) in enumerate(V["zones"])
    ] + [
        {"id": f"m{i+1}", "question": f"zone {z}'s logged vent minutes (copy the number from its fact line)", "answer": m}
        for i, (z, temp, m) in enumerate(V["zones"])
    ] + [
        {"id": "hi", "question": "the high band edge in degrees (copy the number from its fact line)", "answer": V["hi"]},
        {"id": "lo", "question": "the low band edge in degrees (copy the number from its fact line)", "answer": V["lo"]},
        {"id": "oa", "question": "manual overrides active (copy the number from its fact line)", "answer": V["override"]},
    ]
    return dict(arch="vent_control", domain="greenhouse vent control", rules=rules, facts=facts, body=body,
                truth=truth, prov=[f"over={over} under={under} vent_min={vent_min} overrides={V['override']}"],
                entities=entities,
                aggregate="{'zones_over': (a('t1')>a('hi'))+(a('t2')>a('hi'))+(a('t3')>a('hi')), 'zones_under': (a('t1')<a('lo'))+(a('t2')<a('lo'))+(a('t3')<a('lo')), 'vent_minutes': a('m1')*(a('t1')>a('hi'))+a('m2')*(a('t2')>a('hi'))+a('m3')*(a('t3')>a('hi')), 'overrides_active': int(a('oa'))}",
                contract={"zones_over": ("(t1>H)+(t2>H)+(t3>H)",
                             [(f"t{i+1}", rf"zone {V['zones'][i][0]} reads (\d+) degrees", V["zones"][i][1]) for i in range(3)] +
                             [("H", r"high band edge is (\d+)", V["hi"])]),
                          "zones_under": ("(t1<L)+(t2<L)+(t3<L)", [("L", r"low edge is (\d+)", V["lo"])]),
                          "vent_minutes": ("m1*(t1>H)+m2*(t2>H)+m3*(t3>H)",
                             [(f"m{i+1}", rf"zone {V['zones'][i][0]} reads \d+ degrees with (\d+) vent minutes", V["zones"][i][2]) for i in range(3)])},
                variant=v)


ARCHETYPES_B = [arch_cron_lock, arch_dlq_redrive, arch_cert_renewal, arch_backup_window,
                arch_cart_audit, arch_fuel_cards, arch_seat_alloc, arch_specimen_routing,
                arch_table_turns, arch_gym_standby, arch_hold_shelf, arch_print_quota,
                arch_vent_control]
