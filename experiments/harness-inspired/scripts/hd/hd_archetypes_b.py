"""HD archetypes B (14-25). Same conventions as part A: boundary flips by
variant parity, truth/formula/aggregate share one expression, derived cells
carry unit wording. 12 archetypes."""
from hd_archetypes_a import E, _ge, _conv_word


# 14. poison_mix — 2-strike classification with streak reset
def arch_poison_mix(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        "ok ok fail fail ok fail fail fail ok fail ok ok",
        "fail ok fail ok ok fail fail ok fail fail ok ok",
        "ok fail ok fail fail ok ok fail fail fail ok ok",
        "fail fail ok ok fail ok fail fail ok ok fail ok",
    ]
    seq = V[v % 4]
    strikes = 0
    poison = 0
    streak_ok = 0
    fails = 0
    oks = 0
    resets = 0
    best_fail_run = 0
    run = 0
    for ev in seq.split():
        if ev == "fail":
            fails += 1
            run += 1
            best_fail_run = max(best_fail_run, run)
            streak_ok = 0
            strikes += 1
            if strikes >= 2:
                poison += 1
                strikes = 0
        else:
            oks += 1
            run = 0
            streak_ok += 1
            if streak_ok >= 3:
                strikes = 0
                resets += 1
    truth = {"poison_events": poison, "fails_total": fails, "longest_fail_streak": best_fail_run}
    rules = [
        f"an event goes poison on its second consecutive strike, a strike is a fail, two strikes back to back {'mark poison at-or-over the second strike' if GE == '>=' else 'mark poison only strictly past the first strike landing on the second'}, the classic two-strike rule, poison resets the strike counter",
        "a clean streak of three or more ok events forgives prior strikes, the strike counter resets to zero after such a streak, forgiveness is total and immediate",
        "the report wants poison events, total fails, and the longest consecutive fail streak, each counted across the whole sequence",
    ]
    facts = [f"the sieve saw the sequence {seq}",
             f"total fails work out to {fails} and the longest fail run works out to {best_fail_run}",
             f"the sieve classified {poison} poison events tonight"]
    body = [
        "the poison sieve reads a stream of ok and fail the way a judge reads priors, one fail is an accident, two in a row is a character assessment, and three clean events in a row buy back a reputation",
        "the strike counter is the whole state machine, strikes build, strikes clear on streaks, poison fires on the second consecutive strike and resets itself, anything more elaborate than that has historically misfired",
        "the desk tallies fails and streaks mechanically, the streak is the longest unbroken run of fails, not the number of strikes, the two get confused by people who should know better",
    ]
    entities = [E("fails", "how many fail events appear in the sequence (count them)", fails),
                E("oks", "how many ok events appear in the sequence (count them)", oks),
                E("streak", "the longest run of consecutive fail events (count the run)", best_fail_run),
                E("poison", "how many poison events the sieve classified (two-strike rule applied)", poison),
                E("rst", "how many times the three-ok forgiveness fired (count the resets)", resets),
                E("tlen", "the total event count in the sequence (count every event)", len(seq.split()))]
    agg = ("{'poison_events': int(a('poison')), 'fails_total': int(a('fails')), 'longest_fail_streak': int(a('streak'))}")
    contract = {
        "poison_events": ("P", [("P", r"classified (\d+) poison events", poison)]),
        "fails_total": ("F", [("F", r"total fails work out to (\d+)", fails)]),
        "longest_fail_streak": ("S", [("S", r"longest fail run works out to (\d+)", best_fail_run)]),
    }
    prov = [f"seq='{seq}'", f"strikes walk: fails={fails} poison={poison} best_run={best_fail_run}",
            "2 consecutive strikes→poison+reset; 3 ok streak→forgive"]
    return dict(arch="poison_mix", domain="event poison classification", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 15. standby_cascade — no-show cascade, wave caps
def arch_standby_cascade(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(cap=12, conf=12, noshow=3, wave=2, sb=3),
        dict(cap=14, conf=13, noshow=5, wave=3, sb=7),
        dict(cap=15, conf=14, noshow=4, wave=3, sb=4),
        dict(cap=20, conf=18, noshow=5, wave=6, sb=9),
    ][v]
    cap, conf, ns, wv, sb = V["cap"], V["conf"], V["noshow"], V["wave"], V["sb"]
    open_seats = max(0, cap - (conf - ns))
    w1 = min(open_seats, wv, sb)
    w2 = min(max(0, open_seats - w1), wv, max(0, sb - w1))
    unpromoted = sb - w1 - w2
    truth = {"seats_open": open_seats, "wave1_promoted": w1, "wave2_promoted": w2, "unpromoted": unpromoted}
    rules = [
        f"no-shows free their seats, open seats are capacity minus attendees, attendees are confirmed minus no-shows, the floor is zero, negative seats are an accounting crime",
        f"each promotion wave moves at most the wave cap and at most the standby depth, wave one moves first, wave two inherits the remainder of both, a wave {'fills at-or-over its cap' if GE == '>=' else 'fills only strictly past its cap minus one'} before the next wave considers moving",
        "unpromoted is standby depth minus everyone promoted across both waves, reported even at zero",
    ]
    facts = [f"the clinic session seats {cap} with {conf} confirmed and {ns} no-shows",
             f"the standby list holds {sb} patients and the wave cap is {wv} promotions per wave",
             "the board quotes no open-seat count tonight, the desk derives it from no-shows first"]
    body = [
        "the cascade board promotes in waves because the front desk cannot process a stampede, the wave cap is a queue dressed as a courtesy, and the arithmetic beneath it is just floors and remainders stacked carefully",
        "standby order is priority order, the board never says whose priority, the board only counts, names are for the front desk and lawsuits",
        "the desk computes open seats once, then waves against the remainder, recomputing open seats between waves is how double-booked scalpels happen",
    ]
    entities = [E("cap", "session capacity in seats (copy the number from its fact line)", cap),
                E("conf", "confirmed count (copy the number from its fact line)", conf),
                E("ns", "no-show count (copy the number from its fact line)", ns),
                E("sb", "standby depth (copy the number from its fact line)", sb),
                E("wv", "wave cap (copy the number from its fact line)", wv),
                E("op", "open seats after no-shows (capacity minus attendees, floored at zero)", open_seats),
                E("w1", "wave one promotions (open seats, wave cap, and standby depth willing)", w1),
                E("w2", "wave two promotions (remainder after wave one, same caps)", w2),
                E("unp", "unpromoted standby patients after both waves (count them)", unpromoted)]
    agg = ("{'seats_open': max(0, a('cap') - (a('conf') - a('ns'))), "
           "'wave1_promoted': min(max(0, a('cap') - (a('conf') - a('ns'))), a('wv'), a('sb')), "
           "'wave2_promoted': min(max(0, max(0, a('cap') - (a('conf') - a('ns'))) - min(max(0, a('cap') - (a('conf') - a('ns'))), a('wv'), a('sb'))), a('wv'), a('sb') - min(max(0, a('cap') - (a('conf') - a('ns'))), a('wv'), a('sb'))), "
           "'unpromoted': a('sb') - min(max(0, a('cap') - (a('conf') - a('ns'))), a('wv'), a('sb')) - min(max(0, max(0, a('cap') - (a('conf') - a('ns'))) - min(max(0, a('cap') - (a('conf') - a('ns'))), a('wv'), a('sb'))), a('wv'), a('sb') - min(max(0, a('cap') - (a('conf') - a('ns'))), a('wv'), a('sb')))}")
    contract = {
        "seats_open": ("max(0, cap - (conf - ns))",
                       [("cap", r"session seats (\d+)", cap), ("conf", r"with (\d+) confirmed", conf), ("ns", r"and (\d+) no-shows", ns)]),
        "wave1_promoted": ("min(seats_open, wv, sb)",
                           [("wv", r"wave cap is (\d+) promotions", wv), ("sb", r"standby list holds (\d+) patients", sb)]),
        "wave2_promoted": ("min(max(0, seats_open - wave1_promoted), wv, sb - wave1_promoted)", []),
        "unpromoted": ("sb - wave1_promoted - wave2_promoted", []),
    }
    prov = [f"cap={cap} conf={conf} ns={ns} sb={sb} wv={wv}", f"open={open_seats} w1={w1} w2={w2} unpromoted={unpromoted}",
            "open=max(0,cap-(conf-ns)); waves capped by wave cap AND standby depth"]
    return dict(arch="standby_cascade", domain="clinic standby cascades", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 16. overbook_bump — volunteers first, voucher math
def arch_overbook_bump(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(S=44, T=48, vol=3, vc=20000, ic=40000),
        dict(S=30, T=33, vol=2, vc=15000, ic=35000),
        dict(S=52, T=56, vol=5, vc=18000, ic=38000),
        dict(S=18, T=20, vol=1, vc=25000, ic=50000),
    ][v]
    S, T, vol, vc, ic = V["S"], V["T"], V["vol"], V["vc"], V["ic"]
    need = T - S
    invol = max(0, need - vol)
    vol_unused = max(0, vol - need)
    cost = (vol - vol_unused) * vc + invol * ic
    truth = {"involuntary_bumped": invol, "volunteers_unused": vol_unused, "voucher_cost_cents": cost}
    rules = [
        f"the tour overbooks, bumps needed are tickets minus seats, volunteers step up before anyone is forced, a desk {'counts itself covered at-or-over its volunteer goal' if GE == '>=' else 'counts itself covered only strictly past its volunteer goal minus one'} and forced bumps are whatever bumps the volunteers did not absorb",
        "volunteer vouchers cost the volunteer rate each, forced bumps cost the involuntary rate each, only volunteers actually used draw vouchers, unused volunteers cost nothing",
        "the voucher total is whole cents and is reported even when it is zero, which it never is",
    ]
    facts = [f"the tour seats {S} and sold {T} tickets",
             f"{vol} passengers volunteered for bumping",
             f"volunteer vouchers run {vc} whole cents and involuntary compensation runs {ic} whole cents"]
    body = [
        "the bump counter runs hot on sold-out nights and the arithmetic is a small mercy, volunteers first, forced second, and the voucher ledger tells you exactly how merciful the night was",
        "the volunteer line is not charity, it is pricing, and the price of a forced bump is what keeps the volunteer line populated, the desk just counts both",
        "unused volunteers walk away whole, no voucher, no cost, no memory, they board like everyone else and the ledger never mentions them again",
    ]
    entities = [E("S", "seats on the tour (copy the number from its fact line)", S),
                E("T", "tickets sold (copy the number from its fact line)", T),
                E("vol", "volunteers (copy the number from its fact line)", vol),
                E("vc", "volunteer voucher rate in whole cents (copy the number from its fact line)", vc),
                E("ic", "involuntary rate in whole cents (copy the number from its fact line)", ic),
                E("need", "bumps needed in passengers (tickets minus seats)", need),
                E("invol", "involuntary bumps in passengers (bumps the volunteers did not absorb)", invol),
                E("vun", "volunteers unused (count them)", vol_unused),
                E("cost", "total voucher cost in whole cents (used volunteers and forced bumps at their rates)", cost)]
    agg = ("{'involuntary_bumped': max(0, (a('T') - a('S')) - a('vol')), "
           "'volunteers_unused': max(0, a('vol') - (a('T') - a('S'))), "
           "'voucher_cost_cents': (a('vol') - max(0, a('vol') - (a('T') - a('S')))) * a('vc') + max(0, (a('T') - a('S')) - a('vol')) * a('ic')}")
    contract = {
        "involuntary_bumped": ("max(0, (T - S) - vol)",
                               [("T", r"sold (\d+) tickets", T), ("S", r"tour seats (\d+)", S), ("vol", r"(\d+) passengers volunteered", vol)]),
        "volunteers_unused": ("max(0, vol - (T - S))", []),
        "voucher_cost_cents": ("(vol - volunteers_unused) * vc + involuntary_bumped * ic",
                               [("vc", r"vouchers run (\d+) whole cents", vc), ("ic", r"involuntary compensation runs (\d+) whole cents", ic)]),
    }
    prov = [f"S={S} T={T} vol={vol}", f"need={need} invol={invol} unused={vol_unused} cost={cost}",
            "need=T-S; invol=max(0,need-vol); cost=(vol-unused)*vc+invol*ic"]
    return dict(arch="overbook_bump", domain="tour overbook bumping", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 17. room_split — split/merge capacity with tech constraint
def arch_room_split(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(a=60, b=40, want=95, tech_need=50),
        dict(a=45, b=30, want=70, tech_need=40),
        dict(a=80, b=35, want=105, tech_need=60),
        dict(a=50, b=50, want=90, tech_need=45),
    ][v]
    a, b, want, tneed = V["a"], V["b"], V["want"], V["tech_need"]
    total = a + b
    tech = a  # only room a carries tech
    bookable = min(total, tech if tneed > tech else total)
    overflow = max(0, want - bookable)
    truth = {"total_seats": total, "bookable_seats": bookable, "overflow_attendees": overflow}
    rules = [
        f"merged rooms pool total seats, but sessions needing the tech rig can only use seats in the tech room, a session {'pins its ceiling at-or-over the tech room size' if GE == '>=' else 'pins its ceiling only strictly past the tech room size minus one'} only when its tech need exceeds what the tech room holds",
        "bookable is the ceiling after the tech rule applies, overflow is demand minus bookable, floored at zero",
        "the report wants pooled seats, bookable seats, and overflow, in whole seats",
    ]
    facts = [f"room one seats {a} with the tech rig and room two seats {b} without",
             f"the session wants {want} attendees and the tech need is {tneed} seats",
             "the comb quotes neither pool nor ceiling tonight, both derive from the room facts"]
    body = [
        "the room comb merges walls and keeps constraints, the tech rig does not migrate, the ceiling does, and every planner who forgets that books a session into seats that cannot see the projector",
        f"the tech ceiling only bites when the tech need {'stands at-or-over' if GE == '>=' else 'runs strictly past'} the tech room's size, otherwise the merge is honest and the whole pool is bookable",
        "the desk computes the pool, the ceiling, the overflow, in that order, and never lets a nice round pool number seduce it past the rig",
    ]
    entities = [E("rm1", "tech room seats (copy the number from its fact line)", a),
                E("rm2", "plain room seats (copy the number from its fact line)", b),
                E("want", "session demand in attendees (copy the number from its fact line)", want),
                E("tneed", "tech need in seats (copy the number from its fact line)", tneed),
                E("pool", "pooled seats across both rooms (sum them)", total),
                E("ceil", "bookable seats after the tech rule (the ceiling)", bookable),
                E("over", "overflow attendees (demand past the ceiling, floored at zero)", overflow)]
    agg = ("{'total_seats': a('rm1') + a('rm2'), "
           "'bookable_seats': (a('rm1') if a('tneed') > a('rm1') else (a('rm1') + a('rm2'))), "
           "'overflow_attendees': max(0, a('want') - (a('rm1') if a('tneed') > a('rm1') else (a('rm1') + a('rm2'))))}")
    contract = {
        "total_seats": ("rm1 + rm2", [("rm1", r"room one seats (\d+)", a), ("rm2", r"room two seats (\d+)", b)]),
        "bookable_seats": ("(rm1 if tneed > rm1 else (rm1 + rm2))",
                           [("tneed", r"tech need is (\d+) seats", tneed)]),
        "overflow_attendees": ("max(0, want - bookable_seats)",
                               [("want", r"session wants (\d+) attendees", want)]),
    }
    prov = [f"a={a} b={b} want={want} tneed={tneed}", f"pool={total} ceiling={bookable} overflow={overflow}",
            "ceiling=a if tneed>a else pool"]
    return dict(arch="room_split", domain="venue room splits", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 18. triage_2hop — two-hop routing with overrides
def arch_triage_2hop(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(rows=[("case-1", "red", 85), ("case-2", "yellow", 74), ("case-3", "green", 40), ("case-4", "red", 95)]),
        dict(rows=[("p-11", "yellow", 78), ("p-12", "green", 22), ("p-13", "red", 88), ("p-14", "yellow", 60)]),
        dict(rows=[("k-2", "red", 92), ("k-3", "yellow", 71), ("k-4", "red", 87), ("k-5", "green", 35)]),
        dict(rows=[("t-7", "green", 30), ("t-8", "red", 90), ("t-9", "yellow", 76), ("t-10", "red", 84)]),
    ][v]
    def dest(tag, bp, age_extra):
        if tag == "red":
            return "icu" if bp < 90 else "trauma"
        if tag == "yellow":
            return "ward" if age_extra else "obs"
        return "fasttrack"
    finals = []
    for name, tag, bp in V["rows"]:
        hop1 = {"red": "trauma", "yellow": "obs", "green": "fasttrack"}[tag]
        hop2 = hop1
        if hop1 == "trauma" and bp < 90:
            hop2 = "icu"
        if hop1 == "obs" and bp > 70:
            hop2 = "ward"
        finals.append(hop2)
    icu = finals.count("icu")
    ward = finals.count("ward")
    ft = finals.count("fasttrack")
    truth = {"icu_count": icu, "ward_count": ward, "fasttrack_count": ft}
    rules = [
        f"hop one routes by tag, red to trauma, yellow to obs, green to fasttrack, hop two overrides, trauma routes to icu when pressure reads strictly under 90, obs routes to ward when pressure reads strictly past 70, a patient {'holds hop one at-or-over 90 pressure' if GE == '>=' else 'releases from hop one only strictly past 90 pressure'} in the trauma lane, the override edges differ on purpose",
        "green never reroutes, fasttrack is terminal, green patients who arrive walking leave walking",
        "the report counts final destinations after both hops, hop-one landings that were overridden do not count anywhere",
    ]
    facts = [", ".join(f"{n} arrives tagged {tag} with pressure {bp}" for n, tag, bp in V["rows"]),
             "the hop desk quotes no destinations tonight, every final landing derives from tag and pressure"]
    body = [
        "the triage hop routes twice, the first hop sorts by tag, the second hop reads the numbers and overrules, and the only mercy in the system is that green is terminal",
        "the override edges are not symmetric, trauma escalates below 90, obs escalates above 70, and the desk reads both edges every time because sympathy is not a routing rule",
        "final destinations are all that count, intermediate landings are just the mail taking a scenic route",
    ]
    entities = [E(f"bp{i+1}", f"{V['rows'][i][0]}'s pressure reading (copy the number from its fact line)", V["rows"][i][2]) for i in range(4)] + \
               [E(f"d{i+1}", f"{V['rows'][i][0]}'s final destination after both hops (icu, trauma, ward, obs, or fasttrack)", finals[i]) for i in range(4)] + \
               [E("icu", "how many patients end in icu (count them)", icu),
                E("ward", "how many patients end in ward (count them)", ward),
                E("ft", "how many patients end in fasttrack (count them)", ft)]
    def cond(i):
        tag, bp = V["rows"][i][1], V["rows"][i][2]
        if tag == "red":
            return f"('icu' if a('bp{i+1}') < 90 else 'trauma')"
        if tag == "yellow":
            return f"('ward' if a('bp{i+1}') > 70 else 'obs')"
        return "'fasttrack'"
    agg = ("{'icu_count': sum(1 for x in [" + ", ".join(cond(i) for i in range(4)) + "] if x == 'icu'), "
           "'ward_count': sum(1 for x in [" + ", ".join(cond(i) for i in range(4)) + "] if x == 'ward'), "
           "'fasttrack_count': sum(1 for x in [" + ", ".join(cond(i) for i in range(4)) + "] if x == 'fasttrack')}")
    def cexpr(i):
        tag, bp = V["rows"][i][1], V["rows"][i][2]
        if tag == "red":
            return f"('icu' if bp{i+1} < 90 else 'trauma')"
        if tag == "yellow":
            return f"('ward' if bp{i+1} > 70 else 'obs')"
        return "'fasttrack'"
    contract = {
        "icu_count": (f"sum(1 for x in [" + ", ".join(cexpr(i) for i in range(4)) + "] if x == 'icu')",
                      [(f"bp{i+1}", rf"{V['rows'][i][0]} arrives tagged {V['rows'][i][1]} with pressure (\d+)", V["rows"][i][2]) for i in range(4)]),
        "ward_count": (f"sum(1 for x in [" + ", ".join(cexpr(i) for i in range(4)) + "] if x == 'ward')", []),
        "fasttrack_count": (f"sum(1 for x in [" + ", ".join(cexpr(i) for i in range(4)) + "] if x == 'fasttrack')", []),
    }
    prov = [f"rows={[(r[0], r[1], r[2]) for r in V['rows']]}", f"finals={finals}",
            f"icu={icu} ward={ward} fasttrack={ft}",
            "hop2: trauma→icu if bp<90; obs→ward if bp>70; green terminal"]
    return dict(arch="triage_2hop", domain="veterinary triage hops", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 19. cold_divert — excursion minutes vs divert threshold
def arch_cold_divert(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(legs=[("leg-a", 9), ("leg-b", 4), ("leg-c", 12)], thr=20),
        dict(legs=[("run-1", 14), ("run-2", 6), ("run-3", 3)], thr=22),
        dict(legs=[("haul-x", 6), ("haul-y", 9), ("haul-z", 11)], thr=25),
        dict(legs=[("m-2", 11), ("m-3", 9), ("m-4", 2)], thr=18),
    ][v]
    thr = V["thr"]
    exc = {n: m for n, m in V["legs"]}
    total = sum(exc.values())
    divert = (total >= thr) if GE == ">=" else (total > thr)
    worst = max(exc.values())
    truth = {"total_excursion_minutes": total, "divert": bool(divert), "worst_leg_minutes": worst}
    rules = [
        f"a leg accrues excursion minutes while its cargo reads above eight degrees, minutes are whole and additive across legs",
        f"the shipment diverts when total excursion is {'at-or-over the divert threshold' if GE == '>=' else 'strictly past the divert threshold'}, the threshold is total minutes, never per leg",
        "worst leg is the single largest leg excursion, reported separately because the warehouse argues about it",
    ]
    facts = [", ".join(f"{n} logged {m} excursion minutes" for n, m in V["legs"]),
             f"the divert threshold is {thr} total minutes",
             "the line quotes no total tonight, the desk sums the legs itself"]
    body = [
        "the cold line counts minutes above eight degrees the way a auditor counts irregularities, one at a time, without mercy, and the divert threshold is the moment the counting stops mattering and the trucks turn around",
        f"{'at-or-over' if GE == '>=' else 'strictly-past'} the threshold is tonight's divert word, a total sitting exactly on the line is either spoiled or saved by that word alone, the desk reads it twice",
        "the worst leg gets named in the report because the warehouse and the carrier argue about whose minutes they were, the desk does not referee, it just reports the maximum",
    ]
    entities = [E(f"m{i+1}", f"{V['legs'][i][0]}'s excursion minutes (copy the number from its fact line)", V["legs"][i][1]) for i in range(3)] + \
               [E("thr", "divert threshold in total minutes (copy the number from its fact line)", thr),
                E("tot", "total excursion minutes across all legs (sum them)", total),
                E("worst", "the worst single leg in excursion minutes (the maximum)", worst),
                E("div", "divert tonight, true or false (apply the boundary word to total vs threshold)", bool(divert))]
    agg = ("{'total_excursion_minutes': a('m1') + a('m2') + a('m3'), "
           "'divert': bool((a('m1') + a('m2') + a('m3')) " + GE + " a('thr')), "
           "'worst_leg_minutes': max(a('m1'), a('m2'), a('m3'))}")
    contract = {
        "total_excursion_minutes": ("m1 + m2 + m3",
                                    [(f"m{i+1}", rf"{V['legs'][i][0]} logged (\d+) excursion minutes", V["legs"][i][1]) for i in range(3)]),
        "divert": (f"bool((m1 + m2 + m3) {GE} thr)", [("thr", r"threshold is (\d+) total minutes", thr)]),
        "worst_leg_minutes": ("max(m1, m2, m3)", []),
    }
    prov = [f"exc={list(exc.values())} thr={thr} op='{GE}'", f"total={total} divert={divert} worst={worst}"]
    return dict(arch="cold_divert", domain="cold chain divert matrices", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 20. customs_tier — duty tier + origin surcharge
def arch_customs_tier(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(vals=[3500, 4200, 1800], edges=(20000, 100000), pcts=(0, 5, 8), sur=3),
        dict(vals=[8000, 6000, 5000], edges=(20000, 100000), pcts=(0, 5, 8), sur=3),
        dict(vals=[15000, 30000, 15000], edges=(15000, 80000), pcts=(0, 4, 7), sur=2),
        dict(vals=[2000, 9000, 3000], edges=(15000, 80000), pcts=(0, 4, 7), sur=2),
    ][v]
    vals = V["vals"]
    e1, e2 = V["edges"]
    p1, p2, p3 = V["pcts"]
    sur_pct = V["sur"]
    total = sum(vals)
    tier = 1 if (total < e1 if GE == ">=" else total <= e1) else (2 if (total < e2 if GE == ">=" else total <= e2) else 3)
    pct = {1: p1, 2: p2, 3: p3}[tier]
    duty = total * pct // 100
    surcharge = vals[0] * sur_pct // 100  # first item carries the surcharge origin
    truth = {"tier": tier, "duty_cents": duty, "surcharge_cents": surcharge}
    rules = [
        f"the consolidated shipment duty tier reads the summed value, tier two begins {'at-or-over' if GE == '>=' else 'strictly past'} {e1} whole cents and tier three begins {'at-or-over' if GE == '>=' else 'strictly past'} {e2}, consolidation means one tier for the whole shipment, never per item",
        f"duty is total value times the tier percent floor-divided by 100, the surcharge origin applies to the flagged item's value only, at its own percent",
        "the report wants the tier, the duty, and the surcharge, all in whole cents",
    ]
    facts = [", ".join(f"item {chr(105 + i)} declares {val} whole cents" for i, val in enumerate(vals)),
             f"tier two starts at {e1} cents and tier three starts at {e2} cents, rates are {p1}, {p2}, and {p3} percent",
             f"the flagged item carries a {sur_pct} percent surcharge on its declared value"]
    body = [
        "the duty rail prices the shipment, not the shoeboxes inside it, consolidation is the entire game and the entire trap, three cheap items holding hands can price like one expensive item",
        f"the {'at-or-over' if GE == '>=' else 'strictly-past'} convention runs both edges tonight, identically, the binder does not permit a strict lower edge and an inclusive upper edge in the same shipment",
        "the surcharge rides on the flagged item alone, its percent is its own, its base is the item value, never the shipment total",
    ]
    entities = [E(f"v{i+1}", f"item {chr(105 + i)}'s declared value in whole cents (copy the number from its fact line)", vals[i]) for i in range(3)] + \
               [E("e1", "tier two start in whole cents (copy the number from its fact line)", e1),
                E("e2", "tier three start in whole cents (copy the number from its fact line)", e2),
                E("p2", "tier two rate in percent (copy the number from its fact line)", p2),
                E("p3", "tier three rate in percent (copy the number from its fact line)", p3),
                E("tot", "the consolidated value in whole cents (sum the declarations)", total),
                E("tier", "the duty tier number, 1, 2, or 3 (apply both boundary words to the consolidated value)", tier),
                E("duty", "the duty in whole cents (consolidated value times the tier's percent)", duty),
                E("sur", "the surcharge in whole cents (flagged item value times the surcharge percent)", surcharge)]
    i1 = f"(tot < e1)" if GE == ">=" else f"(tot <= e1)"
    i2 = f"(tot < e2)" if GE == ">=" else f"(tot <= e2)"
    agg = ("{'tier': 1 if " + i1.replace("tot", "(a('v1') + a('v2') + a('v3'))").replace("e1", "a('e1')") + " else (2 if " + i2.replace("tot", "(a('v1') + a('v2') + a('v3'))").replace("e2", "a('e2')") + " else 3), "
           "'duty_cents': (a('v1') + a('v2') + a('v3')) * ({p1} if (a('v1') + a('v2') + a('v3')) < a('e1') else (a('p2') if (a('v1') + a('v2') + a('v3')) < a('e2') else a('p3'))) // 100, ".replace("{p1}", str(p1)) +
           "'surcharge_cents': a('v1') * " + str(sur_pct) + " // 100}")
    # rebuild cleanly with explicit op
    op1 = "<" if GE == ">=" else "<="
    agg = ("{'tier': 1 if ((a('v1') + a('v2') + a('v3')) " + op1 + " a('e1')) else (2 if ((a('v1') + a('v2') + a('v3')) " + op1 + " a('e2')) else 3), "
           "'duty_cents': (a('v1') + a('v2') + a('v3')) * ({p1} if ((a('v1') + a('v2') + a('v3')) ".replace("{p1}", str(p1)) + op1 + " a('e1')) else (a('p2') if ((a('v1') + a('v2') + a('v3')) " + op1 + " a('e2')) else a('p3'))) // 100, "
           "'surcharge_cents': a('v1') * " + str(sur_pct) + " // 100}")
    contract = {
        "tier": (f"(1 if (v1 + v2 + v3) {op1} e1 else (2 if (v1 + v2 + v3) {op1} e2 else 3))",
                 [(f"v{i+1}", rf"item {chr(105 + i)} declares (\d+) whole cents", vals[i]) for i in range(3)] +
                 [("e1", r"tier two starts at (\d+) cents", e1), ("e2", r"tier three starts at (\d+) cents", e2)]),
        "duty_cents": (f"(v1 + v2 + v3) * ({p1} if (v1 + v2 + v3) {op1} e1 else ({p2} if (v1 + v2 + v3) {op1} e2 else {p3})) // 100",
                       [("p2", r"rates are \d+, (\d+), and", p2), ("p3", r"and (\d+) percent", p3)]),
        "surcharge_cents": (f"v1 * {sur_pct} // 100",
                            [("sur_pct", rf"carries a (\d+) percent surcharge", sur_pct)]),
    }
    prov = [f"vals={vals} total={total} edges={e1},{e2} op='{GE}'", f"tier={tier} pct={pct} duty={duty} surcharge={surcharge}",
            "one tier for consolidated value; surcharge on flagged item only"]
    return dict(arch="customs_tier", domain="customs duty tiers", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 21. sla_pause_clock — paused-clock accrual
def arch_sla_pause_clock(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(pre=40, pause=35, post=25, sla=90, offset=10),
        dict(pre=20, pause=60, post=45, sla=60, offset=5),
        dict(pre=55, pause=20, post=30, sla=120, offset=15),
        dict(pre=35, pause=80, post=40, sla=75, offset=8),
    ][v]
    pre, pause, post, sla, offset = V["pre"], V["pause"], V["post"], V["sla"], V["offset"]
    clock = pre + post
    wall = pre + pause + post
    breach = ((clock + offset) >= sla) if GE == ">=" else ((clock + offset) > sla)
    margin = sla - (clock + offset)
    truth = {"clock_minutes": clock, "wall_minutes": wall, "breached": bool(breach), "margin_minutes": margin}
    rules = [
        f"the warranty clock runs only while the desk works, the vendor hold pauses it entirely, paused minutes accrue to the wall clock and never to the warranty clock",
        f"the desk counts a routing offset toward the deadline, breach reads the warranty clock plus offset {'at-or-over the deadline' if GE == '>=' else 'strictly past the deadline'}, margin is deadline minus that sum, reported negative when blown",
        "the report wants warranty minutes, wall minutes, the breach flag, and the margin, all whole minutes",
    ]
    facts = [f"the ticket ran {pre} working minutes, then a vendor hold paused it for {pause} minutes, then {post} more working minutes",
             f"the deadline is {sla} minutes with a {offset} minute routing offset",
             "the desk quotes neither clock tonight, both derive from the stretches and the hold"]
    body = [
        "the pause clock is two clocks wearing one ticket, the warranty clock sleeps during vendor holds and the wall clock does not, and the difference between them is the exact length of the vendor's apology",
        f"the routing offset counts against the deadline but never against the work, it is overhead with a vote, and tonight the breach word is {'at-or-over' if GE == '>=' else 'strictly-past'} the deadline",
        "the desk computes warranty minutes first, adds the offset second, reads the breach third, and reports margin even when negative, especially when negative",
    ]
    entities = [E("pre", "first working stretch in minutes (copy the number from its fact line)", pre),
                E("pause", "vendor hold in minutes (copy the number from its fact line)", pause),
                E("post", "second working stretch in minutes (copy the number from its fact line)", post),
                E("sla", "deadline in minutes (copy the number from its fact line)", sla),
                E("off", "routing offset in minutes (copy the number from its fact line)", offset),
                E("clk", "warranty clock minutes (working stretches only)", clock),
                E("wall", "wall clock minutes (everything, hold included)", wall),
                E("br", "breached, true or false (warranty clock plus offset against the deadline word)", bool(breach)),
                E("marg", "margin in minutes (deadline minus warranty clock plus offset, negative if blown)", margin)]
    agg = ("{'clock_minutes': a('pre') + a('post'), "
           "'wall_minutes': a('pre') + a('pause') + a('post'), "
           "'breached': bool((a('pre') + a('post') + a('off')) " + GE + " a('sla')), "
           "'margin_minutes': a('sla') - (a('pre') + a('post') + a('off'))}")
    contract = {
        "clock_minutes": ("pre + post",
                          [("pre", r"ran (\d+) working minutes", pre), ("post", r"then (\d+) more working minutes", post)]),
        "wall_minutes": ("pre + pause + post",
                         [("pause", r"paused it for (\d+) minutes", pause)]),
        "breached": (f"bool((pre + post + off) {GE} sla)",
                     [("off", r"a (\d+) minute routing offset", offset), ("sla", r"deadline is (\d+) minutes", sla)]),
        "margin_minutes": ("sla - (pre + post + off)", []),
    }
    prov = [f"pre={pre} pause={pause} post={post} sla={sla} off={offset}", f"clock={clock} wall={wall}",
            f"breached={breach} (op '{GE}') margin={margin}",
            "clock excludes hold; offset counts toward deadline only"]
    return dict(arch="sla_pause_clock", domain="warranty pause clocks", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 22. window_overlap_dedupe — union of windows
def arch_window_overlap_dedupe(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(wins=[(20, 80), (65, 110), (100, 135)]),
        dict(wins=[(10, 40), (30, 60), (55, 80)]),
        dict(wins=[(100, 190), (150, 210), (180, 220)]),
        dict(wins=[(0, 50), (40, 45), (60, 90)]),
    ][v]
    W = V["wins"]
    union = 0
    covered = []
    for s, e in sorted(W):
        os_ = max(s, covered[-1][1]) if covered else s
        union += max(0, e - os_)
        if covered and os_ < covered[-1][1]:
            covered[-1] = (covered[-1][0], max(covered[-1][1], e))
        else:
            covered.append((s, e))
    ov12 = max(0, min(W[0][1], W[1][1]) - max(W[0][0], W[1][0]))
    ov23 = max(0, min(W[1][1], W[2][1]) - max(W[1][0], W[2][0]))
    ov13 = max(0, min(W[0][1], W[2][1]) - max(W[0][0], W[2][0]))
    distinct = sum(1 for s, e in W if e > s)
    truth = {"union_minutes": union, "pairwise_overlap_minutes": ov12 + ov23 + ov13, "distinct_windows": distinct}
    rules = [
        f"windows count minutes on the union, overlapping minutes count once no matter how many windows cover them, containment counts as overlap {'at-or-over one shared minute' if GE == '>=' else 'only strictly past the first shared minute'}, an empty window is not distinct",
        "pairwise overlap sums each pair's shared minutes independently, triple-covered minutes therefore count multiple times in that sum and that is intentional, it is a pair statistic, not a union statistic",
        "the report wants union minutes, the pairwise overlap total, and the count of distinct non-empty windows",
    ]
    facts = [", ".join(f"window w{i+1} covers minute {s} through minute {e}" for i, (s, e) in enumerate(W)),
             "the wall quotes no overlaps and no union tonight, every dedupe number derives from the three windows"]
    body = [
        "the sensor wall double-reports everything and the dedupe math is just honesty at scale, a minute covered twice is still one minute, and the pairwise total is kept separately precisely because it double-counts, that is its job",
        "empty windows happen when a sensor trips and untrips in the same breath, they are not distinct, they are barely real",
        "the desk sorts, merges, unions, then does the pairwise bookkeeping on the originals, never on the merged shape, the merged shape has forgotten the pairs",
    ]
    entities = [E(f"s{i+1}", f"window w{i+1}'s start minute (copy the number from its fact line)", W[i][0]) for i in range(3)] + \
               [E(f"e{i+1}", f"window w{i+1}'s end minute (copy the number from its fact line)", W[i][1]) for i in range(3)] + \
               [E("uni", "union minutes across all three windows (count each covered minute once)", union),
                E("pw", "total pairwise overlap in minutes (each pair's shared minutes summed)", ov12 + ov23 + ov13),
                E("dist", "how many windows are distinct and non-empty (count them)", distinct)]
    ov = lambda a, b: f"max(0, min(a('e{a}'), a('e{b}')) - max(a('s{a}'), a('s{b}')))"
    agg = ("{'union_minutes': (max(0, a('e1') - a('s1')) + max(0, max(0, a('e2') - max(a('s2'), a('e1'))) ) + max(0, a('e3') - max(a('s3'), max(a('e1'), a('e2'))))), "
           "'pairwise_overlap_minutes': " + ov(1, 2) + " + " + ov(2, 3) + " + " + ov(1, 3) + ", "
           "'distinct_windows': (a('e1') > a('s1')) + (a('e2') > a('s2')) + (a('e3') > a('s3'))}")
    # union assumes sorted windows — cases are authored sorted
    contract = {
        "union_minutes": ("max(0, e1 - s1) + max(0, e2 - max(s2, e1)) + max(0, e3 - max(s3, max(e1, e2)))",
                          [(f"s{i+1}", rf"window w{i+1} covers minute (\d+) through", W[i][0]) for i in range(3)] +
                          [(f"e{i+1}", rf"window w{i+1} covers minute \d+ through minute (\d+)", W[i][1]) for i in range(3)]),
        "pairwise_overlap_minutes": ("max(0, min(e1, e2) - max(s1, s2)) + max(0, min(e2, e3) - max(s2, s3)) + max(0, min(e1, e3) - max(s1, s3))", []),
        "distinct_windows": ("(e1 > s1) + (e2 > s2) + (e3 > s3)", []),
    }
    prov = [f"wins={W}", f"union={union} pairwise={ov12}+{ov23}+{ov13} distinct={distinct}",
            "union counts each minute once (windows authored sorted); pairwise double-counts by design"]
    return dict(arch="window_overlap_dedupe", domain="sensor overlap dedupe", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 23. accrual_diff — two-ledger reconciliation
def arch_accrual_diff(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(A=[120000, -45000, 30000], B=[100000, -50000, 40000]),
        dict(A=[80000, 80000, -20000], B=[90000, 60000, -10000]),
        dict(A=[250000, -100000, 50000], B=[200000, -60000, 90000]),
        dict(A=[60000, -30000, 60000], B=[50000, -20000, 40000]),
    ][v]
    A, B = V["A"], V["B"]
    sumA, sumB = sum(A), sum(B)
    diff = sumA - sumB
    unsigned = abs(diff)
    which = "a" if diff > 0 else ("b" if diff < 0 else "none")
    truth = {"sum_a_cents": sumA, "sum_b_cents": sumB, "diff_cents": diff, "unsigned_diff_cents": unsigned, "higher_ledger": which}
    rules = [
        f"ledger entries carry sign at birth, debits negative, credits positive, sums are plain addition, the close reads the difference {'at-or-over one cent' if GE == '>=' else 'strictly past one cent'} before calling a ledger higher, an exact tie names no winner",
        "diff is ledger a minus ledger b, sign included, unsigned diff is its absolute value, both reported",
        "the higher ledger is a name, a, b, or none, never a sentence",
    ]
    facts = ["ledger a holds " + ", ".join(f"{x:+d}" for x in A) + " whole cents",
             "ledger b holds " + ", ".join(f"{x:+d}" for x in B) + " whole cents",
             "the bench quotes no sums tonight, both close from the entries alone"]
    body = [
        "the close bench reconciles two ledgers that agree on nothing except the arithmetic itself, signs ride the entries from birth, and the difference is allowed to be negative, negative differences are information",
        f"the {'at-or-over' if GE == '>=' else 'strictly-past'} word only governs the tie question tonight, a one-cent difference either crowns a ledger or doesn't, and both readings have defended the close at least once",
        "the desk sums a, sums b, subtracts, absolutizes, names, and stops, adjectives about whose ledger is better belong in the meeting, not in the report",
    ]
    entities = [E(f"a{i+1}", f"ledger a entry {i+1} in whole cents, sign included (copy the number from its fact line)", A[i]) for i in range(3)] + \
               [E(f"b{i+1}", f"ledger b entry {i+1} in whole cents, sign included (copy the number from its fact line)", B[i]) for i in range(3)] + \
               [E("sa", "ledger a's sum in whole cents (add its entries)", sumA),
                E("sb", "ledger b's sum in whole cents (add its entries)", sumB),
                E("df", "the diff in whole cents (a minus b, sign included)", diff),
                E("ud", "the unsigned diff in whole cents (absolute value)", unsigned),
                E("hi", "the higher ledger, a, b, or none (apply the boundary word to the diff)", which)]
    agg = ("{'sum_a_cents': a('a1') + a('a2') + a('a3'), "
           "'sum_b_cents': a('b1') + a('b2') + a('b3'), "
           "'diff_cents': (a('a1') + a('a2') + a('a3')) - (a('b1') + a('b2') + a('b3')), "
           "'unsigned_diff_cents': ((a('a1') + a('a2') + a('a3')) - (a('b1') + a('b2') + a('b3'))) * (1 if ((a('a1') + a('a2') + a('a3')) - (a('b1') + a('b2') + a('b3'))) > 0 else -1), "
           "'higher_ledger': ('a' if ((a('a1') + a('a2') + a('a3')) - (a('b1') + a('b2') + a('b3'))) > 0 else ('b' if ((a('a1') + a('a2') + a('a3')) - (a('b1') + a('b2') + a('b3'))) < 0 else 'none'))}")
    contract = {
        "sum_a_cents": ("a1 + a2 + a3",
                        [(f"a{i+1}", rf"ledger a holds [^\\n]*?([+-]\d+)", None) for i in range(3)]),
        "sum_b_cents": ("b1 + b2 + b3",
                        [(f"b{i+1}", rf"ledger b holds [^\\n]*?([+-]\d+)", None) for i in range(3)]),
        "diff_cents": ("sum_a_cents - sum_b_cents", []),
        "unsigned_diff_cents": ("(sum_a_cents - sum_b_cents if sum_a_cents - sum_b_cents > 0 else -(sum_a_cents - sum_b_cents))", []),
        "higher_ledger": ("'a' if diff_cents > 0 else ('b' if diff_cents < 0 else 'none')", []),
    }
    # regex literals need concrete values; provide them directly
    for i in range(3):
        contract["sum_a_cents"][1][i] = (f"a{i+1}", None, A[i])
        contract["sum_b_cents"][1][i] = (f"b{i+1}", None, B[i])
    prov = [f"A={A} B={B}", f"sumA={sumA} sumB={sumB} diff={diff} unsigned={unsigned} which={which}",
            "signs at birth, plain sums, diff=a-b"]
    return dict(arch="accrual_diff", domain="month-end accrual diffs", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 24. label_surgery — char-precision transform
def arch_label_surgery(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(tag="compressor-alpha-7"),
        dict(tag="turbine-echo-42"),
        dict(tag="regulator-unity-9"),
        dict(tag="condenser-orbit-312"),
    ][v]
    tag = V["tag"]
    seg = tag.split("-")
    vowels = "aeiou"
    dropped = "".join(c for c in seg[1] if c not in vowels)
    base = seg[0] + "-" + dropped + "-" + seg[2]
    suffix_len = 12 if GE == ">=" else 13
    suffix_applied = len(base) >= suffix_len
    label = base + ("-x" if suffix_applied else "")
    truth = {"new_label": label, "label_length": len(label), "suffix_applied": bool(suffix_applied)}
    rules = [
        f"the rebuild keeps segment one untouched, strips every vowel from segment two, keeps segment three's digits, then joins with hyphens",
        f"the suffix rule reads the joined base length {'at-or-over twelve characters' if GE == '>=' else 'only strictly past twelve characters'} and appends -x when the bar is met, the suffix is part of the label and its length",
        "label length counts the final label including any suffix, characters only, hyphens count",
    ]
    facts = [f"the asset tag reads {tag}",
             "the press quotes no rebuilt form tonight, every segment derives from the tag itself"]
    body = [
        "the tag press rebuilds labels by hand because the one time it was automated the automation kept the vowels it liked and dropped the ones it didn't, character precision is not a mood, it is a rule",
        f"the suffix bar sits {'at-or-over' if GE == '>=' else 'strictly-past'} twelve characters tonight and the difference is one hyphen of consequence, a label that lands exactly on twelve either grows a tail or doesn't",
        "the desk strips, joins, measures, appends, and counts, in that order, measuring after appending instead of before has embarrassed better desks",
    ]
    entities = [E("orig", "the original tag (copy the string from its fact line)", tag),
                E("noseg", "segment two with every vowel stripped (letters only)", dropped),
                E("base", "the joined base before any suffix, exact string", base),
                E("lab", "the final label with suffix rule applied (exact string)", label),
                E("llen", "the final label's length in characters (count them)", len(label)),
                E("suf", "suffix applied, true or false (apply the boundary word to the base length)", bool(suffix_applied))]
    agg = ("{'new_label': a('lab'), 'label_length': int(a('llen')), 'suffix_applied': bool(a('suf'))}")
    _b = '(o.split("-")[0] + "-" + "".join(c for c in o.split("-")[1] if c not in "aeiou") + "-" + o.split("-")[2])'
    contract = {
        "new_label": (_b + ' + ("-x" if len(' + _b + ') ' + GE + ' 12 else "")',
                      [("o", r"asset tag reads ([a-z0-9-]+)", tag)]),
        "label_length": ("len(new_label)", []),
        "suffix_applied": ("bool(len(" + _b + ") " + GE + " 12)", []),
    }
    facts.append("the suffix bar reads from the joined base length, characters only, hyphens included")
    prov = [f"tag={tag}", f"dropped='{dropped}' base='{base}' len(base)={len(base)} bar={suffix_len}",
            f"label='{label}' len={len(label)} suffix={suffix_applied}",
            f"strip vowels from seg2, join, suffix when base len {GE} 12"]
    return dict(arch="label_surgery", domain="asset label surgery", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 25. checksum_weighted — positional weights mod 97
def arch_checksum_weighted(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(digits=[7, 3, 9, 1, 4, 6], weights=[3, 5, 7, 2, 4, 9]),
        dict(digits=[1, 5, 2, 8, 3, 7], weights=[3, 5, 7, 2, 4, 9]),
        dict(digits=[4, 4, 6, 6, 0, 2], weights=[3, 5, 7, 2, 4, 9]),
        dict(digits=[9, 0, 3, 5, 8, 1], weights=[3, 5, 7, 2, 4, 9]),
    ][v]
    d, w = V["digits"], V["weights"]
    total = sum(di * wi for di, wi in zip(d, w))
    rem = total % 97
    check = (97 - rem) % 97
    truth = {"weighted_sum": total, "remainder": rem, "check_digit": check}
    rules = [
        f"the code is six digits read left to right, each digit times its weight, weights fixed at 3, 5, 7, 2, 4, 9 from the left, the weighted sum is plain addition of those products",
        f"the remainder is the weighted sum modulo 97, the check digit is 97 minus the remainder, wrapped to zero when the remainder is zero, a remainder of zero means the check digit is zero, {'the wrap stands at-or-over zero by definition' if GE == '>=' else 'the wrap stands strictly past nothing by definition'} and never otherwise",
        "all arithmetic whole numbers, no digit exceeds nine, no weight changes position",
    ]
    facts = ["the code digits are " + ", ".join(map(str, d)),
             "the weights are 3, 5, 7, 2, 4, 9 from the left",
             "the bench quotes no weighted sum tonight, it derives from the digits and the fixed weights"]
    body = [
        "the code bench does positional arithmetic where order is law, the third digit means something the fourth never will, and the weights are welded to their slots",
        "mod ninety-seven looks exotic until you remember it is prime and large enough to catch transposition, which is the entire personality of the scheme",
        "the wrap rule exists for the day the remainder comes back zero, that day the check digit is zero, not ninety-seven, ninety-seven is never a digit here",
    ]
    entities = [E(f"d{i+1}", f"digit {i+1} of the code (copy the number from its fact line)", d[i]) for i in range(6)] + \
               [E("ws", "the weighted sum (each digit times its weight, summed)", total),
                E("rem", "the weighted sum modulo 97", rem),
                E("chk", "the check digit as a whole number (97 minus remainder, wrapped to zero on zero remainder)", check)]
    agg = ("{'weighted_sum': " + " + ".join(f"a('d{i+1}') * {w[i]}" for i in range(6)) + ", "
           "'remainder': (" + " + ".join(f"a('d{i+1}') * {w[i]}" for i in range(6)) + ") % 97, "
           "'check_digit': (97 - ((" + " + ".join(f"a('d{i+1}') * {w[i]}" for i in range(6)) + ") % 97)) % 97}")
    ws_expr = " + ".join(f"d{i+1} * {w[i]}" for i in range(6))
    contract = {
        "weighted_sum": (ws_expr, [(f"d{i+1}", rf"digits are [^.]*(?:, |and )?(\d+)" if i == 0 else None, d[i]) for i in range(6)]),
        "remainder": (f"({ws_expr}) % 97", []),
        "check_digit": ("(97 - remainder) % 97", []),
    }
    # digits regex: single combined line, use ordered capture per position
    digs = [f"d{i+1}" for i in range(6)]
    contract["weighted_sum"] = (ws_expr, [(digs[i], None, d[i]) for i in range(6)])
    prov = [f"digits={d} weights={w}", f"weighted_sum={total} rem={rem} check={check}",
            "sum(d_i*w_i); rem=%97; check=(97-rem)%97"]
    return dict(arch="checksum_weighted", domain="invoice checksum math", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


ARCHETYPES_B = [arch_poison_mix, arch_standby_cascade, arch_overbook_bump, arch_room_split,
                arch_triage_2hop, arch_cold_divert, arch_customs_tier, arch_sla_pause_clock,
                arch_window_overlap_dedupe, arch_accrual_diff, arch_label_surgery, arch_checksum_weighted]
