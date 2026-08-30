"""HD archetypes A (1-13). Each function takes variant v in 0..3 and returns
the standard archetype dict. Boundary conventions FLIP across variants:
v0/v2 inclusive ("at-or-over"), v1/v3 strict ("strictly past") — the operator
GE is chosen accordingly and the SAME expression is used for truth, contract
formula, and aggregate, so all three agree by construction.

Derived cells (answer not a verbatim fact literal) carry unit wording in
their question (unit-drift guard). All arithmetic integer-safe."""
import math


def E(eid, q, ans):
    return {"id": eid, "question": q, "answer": ans}


def _ge(v):
    # inclusive boundary on even variants, strict on odd
    return ">=" if v % 2 == 0 else ">"


def _conv_word(v):
    return "at-or-over" if v % 2 == 0 else "strictly past"


# 1. quota_proration — prorated license charge, integer ceil, full-month edge
def arch_quota_proration(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(seats=[("alpha-desk", 14, 9000), ("bravo-desk", 30, 12000), ("charlie-desk", 47, 6000)]),
        dict(seats=[("north-vane", 10, 7500), ("south-vane", 33, 11000), ("east-vane", 28, 9500)]),
        dict(seats=[("kite-line", 21, 8400), ("mill-line", 30, 10500), ("loom-line", 52, 7000)]),
        dict(seats=[("quay-7", 9, 6000), ("quay-8", 44, 12000), ("quay-9", 30, 9000)]),
    ][v]
    charge = {n: (d * r + 29) // 30 for n, d, r in V["seats"]}
    full = sum(1 for n, d, r in V["seats"] if (d >= 30 if GE == ">=" else d > 30))
    total = sum(charge.values())
    truth = {"seats_full_month": full, "charge_charlie_cents": charge[V["seats"][2][0]] if False else charge[V["seats"][-1][0]],
             "total_proration_cents": total}
    # normalize third key name (last seat charge)
    last = V["seats"][-1][0]
    truth = {"seats_full_month": full, "largest_seat_charge_cents": max(charge.values()),
             "total_proration_cents": total}
    rules = [
        f"a desk {'counts as a full month at-or-over 30 active days' if GE == '>=' else 'counts as a full month only when strictly past 30 active days'}, partial months prorate as whole cents rounded up, ceil, never round down",
        "proration for a desk is its active days times its monthly rate, divided by 30, rounded up to a whole cent per desk",
        "the report needs the full-month desk count, the largest single desk charge, and the total proration in whole cents",
    ]
    facts = [", ".join(f"{n} ran {d} active days at {r} whole cents per 30 days" for n, d, r in V["seats"]),
             "every desk above is billed on the same 30-day convention tonight"]
    charge_pairs = " ".join(f"{n} charges {charge[n]} whole cents" for n, d, r in V["seats"])
    facts.append("the per-desk prorated charges work out to " + charge_pairs + " when the rule is applied faithfully")
    body = [
        f"the proration bench sees three desks tonight and nothing about them is symmetric, the days differ, the rates differ, and the only thing they share is the 30-day convention, which is exactly why the {'at-or-over' if GE == '>=' else 'strictly-past'} edge gets read out loud before any pencil moves",
        "someone will eventually ask why ceil and not round-to-nearest, the answer is buried in a billing dispute from years back where the floor convention cost real money, the ceil convention costs only patience",
        "the desk recomputes each desk charge from days and rate independently, then totals, totals assembled from assumptions rather than charges are the ones that end up in the incident binder",
    ]
    entities = [E(f"d{i+1}", f"{n}'s active day count (copy the number from its fact line)", d) for i, (n, d, r) in enumerate(V["seats"])] + \
               [E(f"r{i+1}", f"{n}'s monthly rate in whole cents (copy the number from its fact line)", r) for i, (n, d, r) in enumerate(V["seats"])] + \
               [E(f"c{i+1}", f"{n}'s prorated charge for the month in whole cents (apply the proration rule to its days and rate)", charge[n]) for i, (n, d, r) in enumerate(V["seats"])] + \
               [E("fm", "how many desks count as a full month under tonight's boundary word (count them)", full),
                E("tot", "the total proration across all desks in whole cents (sum of the per-desk charges)", total)]
    agg = ("{'seats_full_month': " +
           " + ".join(f"(a('d{i+1}') {GE} 30)" for i in range(3)) +
           ", 'largest_seat_charge_cents': max(a('c1'), a('c2'), a('c3'))"
           ", 'total_proration_cents': a('c1') + a('c2') + a('c3')}")
    cf = [f"((d{i+1} * r{i+1} + 29) // 30)" for i in range(3)]
    contract = {
        "seats_full_month": (f"{' + '.join(f'(d{i+1} {GE} 30)' for i in range(3))}",
                             [(f"d{i+1}", rf"{V['seats'][i][0]} ran (\d+) active days", V["seats"][i][1]) for i in range(3)]),
        "largest_seat_charge_cents": ("max(" + ", ".join(cf) + ")",
                                      [(f"d{i+1}", rf"{V['seats'][i][0]} ran (\d+) active days", V["seats"][i][1]) for i in range(3)] +
                                      [(f"r{i+1}", rf"{V['seats'][i][0]} ran \d+ active days at (\d+) whole cents", V["seats"][i][2]) for i in range(3)]),
        "total_proration_cents": (" + ".join(cf),
                                  [(f"d{i+1}", rf"{V['seats'][i][0]} ran (\d+) active days", V["seats"][i][1]) for i in range(3)] +
                                  [(f"r{i+1}", rf"{V['seats'][i][0]} ran \d+ active days at (\d+) whole cents", V["seats"][i][2]) for i in range(3)]),
    }
    prov = [f"charges={ {n: charge[n] for n, d, r in V['seats']} }", f"full={full} (op '{GE}')",
            f"largest={max(charge.values())} total={total}",
            f"per-desk ceil formula: (days*rate+29)//30"]
    return dict(arch="quota_proration", domain="software license seat proration", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 2. tiered_postage — flat band by weight, edge convention flips
def arch_tiered_postage(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(w=480, edges=(100, 250), rates=(120, 180, 260)),
        dict(w=100, edges=(100, 250), rates=(120, 180, 260)),
        dict(w=250, edges=(150, 400), rates=(90, 150, 210)),
        dict(w=150, edges=(150, 400), rates=(90, 150, 210)),
    ][v]
    w, e1, e2, p1, p2, p3 = V["w"], V["edges"][0], V["edges"][1], *V["rates"]
    band = 1 if (w < e1 if GE == ">=" else w <= e1) else (2 if (w < e2 if GE == ">=" else w <= e2) else 3)
    charge = {1: p1, 2: p2, 3: p3}[band]
    urgent = charge * 2
    truth = {"band": band, "charge_cents": charge, "urgent_cents": urgent}
    rules = [
        f"parcel weight {'moves up a band at-or-over the band edge' if GE == '>=' else 'moves up a band only when strictly past the band edge'}, edges measured to the whole gram, no rounding of weight",
        f"band 1 runs under {e1} grams at {p1} whole cents, band 2 runs to under {e2} grams at {p2} whole cents, band 3 is everything at or beyond that at {p3} whole cents",
        "urgent parcels pay double the band charge, doubling happens after banding, never before",
    ]
    facts = [f"tonight's parcel weighs {w} whole grams",
             f"the band edges are {e1} grams and {e2} grams",
             f"the band rates run {p1}, {p2}, then {p3} whole cents from the lowest band up"]
    body = [
        f"the tiering station handles one parcel tonight and the whole exercise hangs on a single boundary read, {'an at-or-over read pushes weight up at the edge itself' if GE == '>=' else 'a strictly-past read leaves edge weight in the lower band'}, and the two conventions have never once agreed at the edge",
        "the rate card is laminated, the edges are not negotiable, and the urgent multiplier is a flat two, applied last, applied always",
        "the desk works the band first, the charge second, the doubling third, in that order, because every other order has been tried and every other order has failed",
    ]
    entities = [E("w", "the parcel weight in whole grams (copy the number from its fact line)", w),
                E("e1", "the first band edge in whole grams (copy the number from its fact line)", e1),
                E("e2", "the second band edge in whole grams (copy the number from its fact line)", e2),
                E("p1", "band 1 rate in whole cents (copy the number from its fact line)", p1),
                E("p2", "band 2 rate in whole cents (copy the number from its fact line)", p2),
                E("p3", "band 3 rate in whole cents (copy the number from its fact line)", p3),
                E("band", "the parcel's band number, 1, 2, or 3 (apply the boundary rule to the weight)", band),
                E("chg", "the band charge in whole cents (the rate for the band you derived)", charge),
                E("urg", "the urgent charge in whole cents (double the band charge)", urgent)]
    inner1 = "(a('w') < a('e1'))" if GE == ">=" else "(a('w') <= a('e1'))"
    inner2 = "(a('w') < a('e2'))" if GE == ">=" else "(a('w') <= a('e2'))"
    agg = ("{'band': 1 if " + inner1 + " else (2 if " + inner2 + " else 3), "
           "'charge_cents': (a('p1') if " + inner1 + " else (a('p2') if " + inner2 + " else a('p3'))), "
           "'urgent_cents': 2 * (a('p1') if " + inner1 + " else (a('p2') if " + inner2 + " else a('p3')))}")
    f1 = f"(1 if (w < e1) else (2 if (w < e2) else 3))" if GE == ">=" else f"(1 if (w <= e1) else (2 if (w <= e2) else 3))"
    contract = {
        "band": (f"{f1}", [("w", r"parcel weighs (\d+) whole grams", w), ("e1", r"edges are (\d+) grams", e1), ("e2", r"and (\d+) grams", e2)]),
        "charge_cents": ("p1 if band == 1 else (p2 if band == 2 else p3)",
                         [("p1", r"rates run (\d+),", p1), ("p2", r", (\d+), then", p2), ("p3", r"then (\d+) whole cents from", p3)]),
        "urgent_cents": ("2 * charge_cents", []),
    }
    prov = [f"w={w} edges={e1},{e2} op='{GE}' band={band} charge={charge} urgent={urgent}"]
    return dict(arch="tiered_postage", domain="parcel postage tiers", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 3. ballot_quorum — quorum edge + majority of cast
def arch_ballot_quorum(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(M=12, P=11, A=2, R=1, Y=4),
        dict(M=9, P=5, A=1, R=0, Y=2),
        dict(M=15, P=8, A=0, R=1, Y=4),
        dict(M=7, P=7, A=3, R=1, Y=2),
    ][v]
    M, P, A, R, Y = V["M"], V["P"], V["A"], V["R"], V["Y"]
    half = (M + 1) // 2  # smallest integer at-or-over half of M
    quorum_met = (P >= half) if GE == ">=" else (P > half - 1)  # strict variant: strictly past half-minus... keep simple parity
    quorum_met = (P >= half) if GE == ">=" else (P > M // 2)
    cast = P - A - R
    needed = cast // 2 + 1
    margin = Y - needed
    truth = {"quorum_met": bool(quorum_met), "votes_cast": cast, "yes_needed": needed, "margin": margin}
    rules = [
        f"quorum needs present members {'at-or-over half the seated board' if GE == '>=' else 'strictly past half the seated board'}, present means seated and in the room, abstain and recuse still count as present for quorum only",
        "abstentions and recusals reduce the votes cast, votes cast is present minus abstain minus recuse, never negative",
        "a motion carries with a majority of votes cast, majority of cast is cast divided by two plus one, the margin is yes votes minus that number",
    ]
    facts = [f"the board seats {M} members",
             f"{P} members are present in the room tonight",
             f"{A} members abstain and {R} members recuse",
             f"{Y} members vote yes"]
    body = [
        f"the quorum rail is the oldest rule in this binder and the one most often misread, the {'at-or-over' if GE == '>=' else 'strictly-past'} reading of half the seated board is tonight's reading, and the abstentions do not help anyone the way newcomers hope they might",
        "the desk computes quorum first, cast second, majority third, margin last, and refuses to skip ahead even when the room is impatient, especially when the room is impatient",
        "recusals are not abstentions, abstentions are not absences, the vocabulary is load-bearing and the binder says so in underlined ink",
    ]
    entities = [E("M", "seated members on the board (copy the number from its fact line)", M),
                E("P", "members present tonight (copy the number from its fact line)", P),
                E("A", "members abstaining (copy the number from its fact line)", A),
                E("R", "members recusing (copy the number from its fact line)", R),
                E("Y", "yes votes (copy the number from its fact line)", Y),
                E("cast", "votes cast (present minus abstain minus recuse, count them)", cast),
                E("need", "yes votes needed to carry as a count (majority of votes cast)", needed),
                E("qm", "quorum met tonight, true or false (apply the boundary word to present vs half the board)", bool(quorum_met)),
                E("marg", "the margin in whole votes (yes votes minus the needed count, negative allowed)", margin)]
    qe = f"(P >= ((M + 1) // 2))" if GE == ">=" else "(P > (M // 2))"
    agg = ("{'quorum_met': bool(" + qe.replace("P", "a('P')").replace("M", "a('M')") + "), "
           "'votes_cast': a('P') - a('A') - a('R'), "
           "'yes_needed': (a('P') - a('A') - a('R')) // 2 + 1, "
           "'margin': a('Y') - ((a('P') - a('A') - a('R')) // 2 + 1)}")
    contract = {
        "quorum_met": (f"bool({qe})", [("M", r"board seats (\d+) members", M), ("P", r"(\d+) members are present", P)]),
        "votes_cast": ("P - A - R", [("P", r"(\d+) members are present", P), ("A", r"(\d+) members abstain", A), ("R", r"and (\d+) members recuse", R)]),
        "yes_needed": ("votes_cast // 2 + 1", []),
        "margin": ("Y - yes_needed", [("Y", r"(\d+) members vote yes", Y)]),
    }
    prov = [f"M={M} P={P} A={A} R={R} Y={Y}", f"quorum_op='{GE}' quorum_met={quorum_met}",
            f"cast={cast} needed={needed} margin={margin}"]
    return dict(arch="ballot_quorum", domain="board ballot quorums", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 4. disk_quota_conv — GiB→MiB per volume vs MiB quota (REA+ dead class)
def arch_disk_quota_conv(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(gibs=[("vault-a", 4, 6144), ("vault-b", 2, 2048), ("vault-c", 8, 8192)]),
        dict(gibs=[("annex-a", 8, 6144), ("annex-b", 2, 3072), ("annex-c", 4, 5120)]),
        dict(gibs=[("crate-x", 5, 5120), ("crate-y", 2, 3072), ("crate-z", 9, 8192)]),
        dict(gibs=[("drum-4", 7, 7168), ("drum-5", 2, 2048), ("drum-6", 4, 4096)]),
    ][v]
    conv = {n: g * 1024 for n, g, q in V["gibs"]}
    over = {n: (conv[n] >= q if GE == ">=" else conv[n] > q) for n, g, q in V["gibs"]}
    n_over = sum(over.values())
    total = sum(conv.values())
    overage = sum(max(0, conv[n] - q) for n, g, q in V["gibs"])
    truth = {"volumes_over": n_over, "total_mib": total, "overage_mib": overage}
    rules = [
        f"a volume is over quota when its usage in whole MiB is {'at-or-over its quota' if GE == '>=' else 'strictly past its quota'}, usage is recorded in GiB and converted at 1024 MiB per GiB exactly, no decimal shortcuts",
        "overage for a volume is its usage in MiB minus its quota in MiB, floored at zero, overage never goes negative",
        "the report wants the over-quota volume count, total usage in whole MiB, and total overage in whole MiB",
    ]
    facts = [", ".join(f"{n} holds {g} GiB against a {q} MiB quota" for n, g, q in V["gibs"]),
             "conversion is fixed tonight at 1024 MiB per GiB, whole MiB only"]
    body = [
        "the vault ledger quotes volumes in GiB and quotas in MiB on purpose, the conversion is where lazy nights go to die, 1024 and never 1000, the desk has a scar about this",
        f"quota breaches read {'at-or-over' if GE == '>=' else 'strictly-past'} the quota line tonight, and the distinction only matters for volumes parked exactly on the line, which happens more than anyone expects",
        "the desk converts first, compares second, sums last, converting and comparing in the same breath is how volumes end up over- or under-counted by a whole gigabyte",
    ]
    entities = [E(f"g{i+1}", f"{n}'s usage in whole GiB (copy the number from its fact line)", g) for i, (n, g, q) in enumerate(V["gibs"])] + \
               [E(f"q{i+1}", f"{n}'s quota in whole MiB (copy the number from its fact line)", q) for i, (n, g, q) in enumerate(V["gibs"])] + \
               [E(f"m{i+1}", f"{n}'s usage converted to whole MiB (convert its GiB at 1024)", conv[n]) for i, (n, g, q) in enumerate(V["gibs"])] + \
               [E("over", "how many volumes are over quota under tonight's boundary word (count them)", n_over),
                E("ovg", "total overage across all volumes in whole MiB, zero-floored per volume", overage)]
    agg = ("{'volumes_over': " + " + ".join(f"(a('m{i+1}') {GE} a('q{i+1}'))" for i in range(3)) +
           ", 'total_mib': a('m1') + a('m2') + a('m3')" +
           ", 'overage_mib': max(0, a('m1') - a('q1')) + max(0, a('m2') - a('q2')) + max(0, a('m3') - a('q3'))}")
    mexpr = [f"(g{i+1} * 1024)" for i in range(3)]
    contract = {
        "volumes_over": (" + ".join(f"({mexpr[i]} {GE} q{i+1})" for i in range(3)),
                         [(f"g{i+1}", rf"{V['gibs'][i][0]} holds (\d+) GiB", V["gibs"][i][1]) for i in range(3)] +
                         [(f"q{i+1}", rf"{V['gibs'][i][0]} holds \d+ GiB against a (\d+) MiB quota", V["gibs"][i][2]) for i in range(3)]),
        "total_mib": (" + ".join(mexpr),
                      [(f"g{i+1}", rf"{V['gibs'][i][0]} holds (\d+) GiB", V["gibs"][i][1]) for i in range(3)]),
        "overage_mib": (" + ".join(f"max(0, {mexpr[i]} - q{i+1})" for i in range(3)),
                        [(f"q{i+1}", rf"{V['gibs'][i][0]} holds \d+ GiB against a (\d+) MiB quota", V["gibs"][i][2]) for i in range(3)]),
    }
    facts.append("the desk converts before comparing and sums only after every volume is converted")
    prov = [f"conv={ {n: conv[n] for n, g, q in V['gibs']} }", f"over_op='{GE}' n_over={n_over}",
            f"total={total} overage={overage}", "formula: MiB = GiB*1024, overage floored at 0"]
    return dict(arch="disk_quota_conv", domain="archive volume unit math", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 5. escrow_ladder — 4 tranches gated by inspection counts
def arch_escrow_ladder(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(total=480000, tranches=[(25, 2), (25, 3), (30, 4), (20, 5)], passed=[3, 3, 4, 4]),
        dict(total=360000, tranches=[(20, 2), (30, 3), (30, 4), (20, 6)], passed=[2, 2, 5, 7]),
        dict(total=520000, tranches=[(30, 2), (25, 4), (25, 5), (20, 5)], passed=[1, 4, 3, 6]),
        dict(total=600000, tranches=[(25, 3), (25, 3), (25, 4), (25, 6)], passed=[4, 3, 2, 9]),
    ][v]
    T = V["total"]
    released_c = []
    for (pct, thr), got in zip(V["tranches"], V["passed"]):
        ok = (got >= thr) if GE == ">=" else (got > thr)
        released_c.append(T * pct // 100 if ok else 0)
    n_rel = sum(1 for c in released_c if c > 0)
    cents_rel = sum(released_c)
    truth = {"tranches_released": n_rel, "cents_released": cents_rel, "cents_held": T - cents_rel}
    rules = [
        f"each tranche releases its percentage of the total only when its inspection count is {'at-or-over its threshold' if GE == '>=' else 'strictly past its threshold'}, percentages apply to the full escrow total, never to a remainder",
        "integer cents throughout, percentage of total computed as total times percent floor-divided by 100",
        "held is total minus released, held is never negative and never silent, unreleased tranches stay held entirely",
    ]
    facts = [f"the escrow total is {T} whole cents"] if False else [f"the escrow total is {T}"]
    facts = [f"the escrow total is {T} whole units",
             ", ".join(f"tranche {i+1} is {pct} percent with threshold {thr} inspections passed {got}" for i, ((pct, thr), got) in enumerate(zip(V["tranches"], V["passed"]))),
             "no tranche amount is taken on faith tonight, every release is recomputed from the gate"]
    body = [
        f"the escrow rail pays out in rungs and tonight the {'at-or-over' if GE == '>=' else 'strictly-past'} reading of each threshold decides which rungs hold weight, an inspection sitting exactly on a threshold is the whole game when the convention is strict",
        "the desk computes each tranche against the full total, the remainder-based reading was retired after the year it silently underpaid two tranches",
        "nothing here rolls over, nothing here compounds, a tranche releases or it does not, binary as a rivet",
    ]
    entities = [E("T", "the escrow total in whole units (copy the number from its fact line)", T)] + \
               [E(f"thr{i+1}", f"tranche {i+1}'s inspection threshold (copy the number from its fact line)", thr) for i, (pct, thr) in enumerate(V["tranches"])] + \
               [E(f"got{i+1}", f"tranche {i+1}'s inspections passed (copy the number from its fact line)", got) for i, (pct, thr), got in [(i, pt, g) for i, (pt, g) in enumerate(zip(V["tranches"], V["passed"]))]] + \
               [E(f"rel{i+1}", f"tranche {i+1}'s released amount in whole units (apply the gate to its threshold, zero if held)", released_c[i]) for i in range(4)] + \
               [E("nrel", "how many tranches released (count them)", n_rel),
                E("held", "the held amount in whole units (total minus everything released)", T - cents_rel)]
    agg = ("{'tranches_released': " + " + ".join(f"((a('got{i+1}') {GE} a('thr{i+1}')) and 1 or 0)" for i in range(4)) +
           ", 'cents_released': " + " + ".join(f"((a('got{i+1}') {GE} a('thr{i+1}')) and (a('T') * {V['tranches'][i][0]} // 100) or 0)" for i in range(4)) +
           ", 'cents_held': a('T') - (" + " + ".join(f"((a('got{i+1}') {GE} a('thr{i+1}')) and (a('T') * {V['tranches'][i][0]} // 100) or 0)" for i in range(4)) + ")}")
    contract = {
        "tranches_released": (" + ".join(f"((g{i+1} {GE} t{i+1}) and 1 or 0)" for i in range(4)),
                              [(f"g{i+1}", rf"tranche {i+1} is \d+ percent with threshold \d+ inspections passed (\d+)", V["passed"][i]) for i in range(4)] +
                              [(f"t{i+1}", rf"tranche {i+1} is \d+ percent with threshold (\d+) inspections", V["tranches"][i][1]) for i in range(4)]),
        "cents_released": (" + ".join(f"((g{i+1} {GE} t{i+1}) and (T * {V['tranches'][i][0]} // 100) or 0)" for i in range(4)),
                           [("T", r"escrow total is (\d+) whole units", T)] +
                           [(f"g{i+1}", rf"tranche {i+1} is \d+ percent with threshold \d+ inspections passed (\d+)", V["passed"][i]) for i in range(4)] +
                           [(f"t{i+1}", rf"tranche {i+1} is \d+ percent with threshold (\d+) inspections", V['tranches'][i][1]) for i in range(4)]),
        "cents_held": ("T - cents_released", []),
    }
    prov = [f"released={released_c}", f"op='{GE}' n_rel={n_rel} cents_rel={cents_rel} held={T - cents_rel}",
            f"per-tranche: {' '.join(f'{V['tranches'][i][0]}%x{T}//100 gated {V['passed'][i]}vs{V['tranches'][i][1]}' for i in range(4))}"]
    return dict(arch="escrow_ladder", domain="construction escrow tranches", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 6. per_diem_holds — reimbursement min(s, cap), flag edge
def arch_per_diem_holds(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(rate=16500, rows=[("june-a", 3, 45000), ("june-b", 4, 70200), ("june-c", 2, 33000)]),
        dict(rate=15000, rows=[("trip-9", 5, 78000), ("trip-8", 2, 30000), ("trip-7", 4, 57000)]),
        dict(rate=18000, rows=[("leg-11", 1, 18000), ("leg-12", 6, 111000), ("leg-13", 3, 52000)]),
        dict(rate=14000, rows=[("run-2", 4, 56000), ("run-3", 7, 91000), ("run-4", 2, 31000)]),
    ][v]
    rate = V["rate"]
    caps = {n: rate * k for n, k, s in V["rows"]}
    reimb = {n: min(s, caps[n]) for n, k, s in V["rows"]}
    flagged = {n: (s >= caps[n] if GE == ">=" else s > caps[n]) for n, k, s in V["rows"]}
    n_flag = sum(flagged.values())
    tot_reimb = sum(reimb.values())
    claw = sum(s - caps[n] for n, k, s in V["rows"] if flagged[n])
    truth = {"travelers_flagged": n_flag, "total_reimbursed_cents": tot_reimb, "clawed_back_cents": claw}
    rules = [
        f"per-diem cap is the nightly rate times nights, a claim {'flags at-or-over its cap' if GE == '>=' else 'flags only when strictly past its cap'}, flagging is about the claim, not the payout",
        "reimbursement pays the smaller of submitted and cap, whole cents, never negative, never rounded",
        "clawed back is the part of flagged claims above their caps, summed across flagged travelers only",
    ]
    facts = [f"the nightly rate is {rate} whole cents",
             ", ".join(f"{n} spent {k} nights and submitted {s} whole cents" for n, k, s in V["rows"]),
             "the booth recomputes every cap and every payout from the rate and the nights"]
    body = [
        f"the per-diem booth processes three claims tonight and the only debate is the flag line, {'at-or-over' if GE == '>=' else 'strictly-past'} the cap, a claim that lands exactly on its cap is the classic desk argument, settled by tonight's convention and re-argued every night after",
        "the payout itself is boring, the smaller of two numbers, the interesting part is what the flag implies for next quarter's advance, which is why the flag word gets read carefully",
        "clawback accounting only ever touches flagged claims, unflagged claims contribute nothing to the claw total, not even their rounding dust",
    ]
    entities = [E("rate", "the nightly rate in whole cents (copy the number from its fact line)", rate)] + \
               [E(f"k{i+1}", f"{n}'s nights (copy the number from its fact line)", k) for i, (n, k, s) in enumerate(V["rows"])] + \
               [E(f"s{i+1}", f"{n}'s submitted amount in whole cents (copy the number from its fact line)", s) for i, (n, k, s) in enumerate(V["rows"])] + \
               [E(f"cp{i+1}", f"{n}'s cap in whole cents (rate times nights)", caps[n]) for i, (n, k, s) in enumerate(V["rows"])] + \
               [E(f"rb{i+1}", f"{n}'s reimbursement in whole cents (smaller of submitted and cap)", reimb[n]) for i, (n, k, s) in enumerate(V["rows"])] + \
               [E("flag", "how many travelers flag under tonight's boundary word (count them)", n_flag),
                E("claw", "total clawed back in whole cents (flagged claims' overage only)", claw)]
    agg = ("{'travelers_flagged': " + " + ".join(f"(a('s{i+1}') {GE} a('cp{i+1}'))" for i in range(3)) +
           ", 'total_reimbursed_cents': " + " + ".join(f"min(a('s{i+1}'), a('cp{i+1}'))" for i in range(3)) +
           ", 'clawed_back_cents': " + " + ".join(f"((a('s{i+1}') {GE} a('cp{i+1}')) and (a('s{i+1}') - a('cp{i+1}')) or 0)" for i in range(3)) + "}")
    cpx = [f"(rate * k{i+1})" for i in range(3)]
    contract = {
        "travelers_flagged": (" + ".join(f"(s{i+1} {GE} {cpx[i]})" for i in range(3)),
                              [(f"s{i+1}", rf"{V['rows'][i][0]} spent \d+ nights and submitted (\d+) whole cents", V["rows"][i][2]) for i in range(3)] +
                              [(f"k{i+1}", rf"{V['rows'][i][0]} spent (\d+) nights", V["rows"][i][1]) for i in range(3)] +
                              [("rate", r"nightly rate is (\d+) whole cents", rate)]),
        "total_reimbursed_cents": (" + ".join(f"min(s{i+1}, {cpx[i]})" for i in range(3)),
                                   [(f"k{i+1}", rf"{V['rows'][i][0]} spent (\d+) nights", V["rows"][i][1]) for i in range(3)] +
                                   [("rate", r"nightly rate is (\d+) whole cents", rate)] +
                                   [(f"s{i+1}", rf"{V['rows'][i][0]} spent \d+ nights and submitted (\d+) whole cents", V["rows"][i][2]) for i in range(3)]),
        "clawed_back_cents": (" + ".join(f"((s{i+1} {GE} {cpx[i]}) and (s{i+1} - {cpx[i]}) or 0)" for i in range(3)), []),
    }
    prov = [f"caps={caps}", f"reimb={reimb}", f"op='{GE}' flag={n_flag} reimb={tot_reimb} claw={claw}",
            "payout=min(submitted, cap); claw=overage of flagged only"]
    return dict(arch="per_diem_holds", domain="travel per-diem audits", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 7. multi_wallet — ordered deduction with floor-at-zero + bundle discount
def arch_multi_wallet(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(bal=[1500, 800, 2000], cost=3800, thr=3500, disc=5),
        dict(bal=[900, 2400, 600], cost=3100, thr=3000, disc=7),
        dict(bal=[2200, 1100, 400], cost=4100, thr=4000, disc=5),
        dict(bal=[1700, 1700, 300], cost=2900, thr=3000, disc=9),
    ][v]
    b1, b2, b3 = V["bal"]
    C0, thr, disc = V["cost"], V["thr"], V["disc"]
    discounted = (C0 >= thr) if GE == ">=" else (C0 > thr)
    C = C0 * (100 - disc) // 100 if discounted else C0
    u1 = min(b1, C)
    u2 = min(b2, C - u1) if C - u1 > 0 else 0
    u3 = min(b3, C - u1 - u2) if C - u1 - u2 > 0 else 0
    shortfall = max(0, C - u1 - u2 - u3)
    truth = {"wallet1_used_cents": u1, "wallet3_used_cents": u3, "shortfall_cents": shortfall}
    rules = [
        f"a basket {'earns the bundle discount at-or-over' if GE == '>=' else 'earns the bundle discount only strictly past'} the bundle threshold, the discount percentage applies to the sticker price once, before any wallet is touched",
        "payment drains wallets in fixed order, wallet one, then two, then three, each wallet floors at zero, no wallet ever goes negative",
        "shortfall is whatever the purchase still owes after all three wallets, floored at zero, the desk reports it even when it is zero, especially when it is zero",
    ]
    facts = [f"the sticker price is {C0} whole cents and the bundle threshold is {thr} whole cents with a {disc} percent bundle discount",
             f"wallet one holds {b1}, wallet two holds {b2}, wallet three holds {b3}, all whole cents",
             "the cage quotes no payable tonight, the bundle rule decides it and the desk applies it"]
    body = [
        "the wallet cage pays in strict order, the order predates everyone here, and the floor-at-zero rule predates the order, together they make the payout boring, which is the highest compliment this desk pays",
        f"the bundle threshold is the only fork in tonight's road, {'at-or-over earns it' if GE == '>=' else 'strictly past earns it'}, and the discount lands on the sticker price, not on some running remainder, the remainder reading is how the cage once paid nine cents on a bicycle",
        "the desk recomputes each wallet's drain from the payable left after the previous wallet, never from the sticker, never from memory",
    ]
    entities = [E("C0", "the sticker price in whole cents (copy the number from its fact line)", C0),
                E("thr", "the bundle threshold in whole cents (copy the number from its fact line)", thr),
                E("disc", "the bundle discount percent (copy the number from its fact line)", disc),
                E("C", "the payable in whole cents (apply the bundle rule to the sticker price)", C),
                E("b1", "wallet one balance in whole cents (copy the number from its fact line)", b1),
                E("b2", "wallet two balance in whole cents (copy the number from its fact line)", b2),
                E("b3", "wallet three balance in whole cents (copy the number from its fact line)", b3),
                E("u1", "wallet one's drain in whole cents (smallest of its balance and the payable)", u1),
                E("u3", "wallet three's drain in whole cents (what remains after wallets one and two, floored at zero)", u3),
                E("sh", "the shortfall in whole cents (what the purchase still owes after all wallets)", shortfall)]
    agg = ("{'wallet1_used_cents': min(a('b1'), a('C')), "
           "'wallet3_used_cents': max(0, min(a('b3'), a('C') - min(a('b1'), a('C')) - max(0, min(a('b2'), a('C') - min(a('b1'), a('C')))))), "
           "'shortfall_cents': max(0, a('C') - a('b1') - a('b2') - a('b3'))}")
    Cx = f"((C0 * (100 - disc) // 100) if (C0 {GE} thr) else C0)"
    contract = {
        "wallet1_used_cents": (f"min(b1, {Cx})",
                               [("b1", r"wallet one holds (\d+)", b1), ("C0", r"sticker price is (\d+) whole cents", C0),
                                ("thr", r"threshold is (\d+) whole cents", thr), ("disc", r"(\d+) percent bundle discount", disc)]),
        "wallet3_used_cents": (f"max(0, min(b3, {Cx} - min(b1, {Cx}) - max(0, min(b2, {Cx} - min(b1, {Cx})))))",
                               [("b2", r"wallet two holds (\d+)", b2), ("b3", r"wallet three holds (\d+)", b3)]),
        "shortfall_cents": (f"max(0, {Cx} - b1 - b2 - b3)", []),
    }
    prov = [f"discounted={discounted} (op '{GE}' on {C0} vs {thr})", f"payable={C}",
            f"u1={u1} u2={u2} u3={u3} shortfall={shortfall}",
            f"drain order fixed, floors at 0"]
    return dict(arch="multi_wallet", domain="arcade wallet ledgers", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 8. rebate_tiers — retroactive tier on full volume
def arch_rebate_tiers(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(units=520, price=2500, t2=100, t3=500, p1=5, p2=12, p3=18),
        dict(units=100, price=3000, t2=100, t3=500, p1=5, p2=12, p3=18),
        dict(units=500, price=1800, t2=80, t3=400, p1=4, p2=10, p3=15),
        dict(units=400, price=2200, t2=80, t3=400, p1=4, p2=10, p3=15),
    ][v]
    U, price = V["units"], V["price"]
    t2, t3, p1, p2, p3 = V["t2"], V["t3"], V["p1"], V["p2"], V["p3"]
    tier = 1 if (U < t2 if GE == ">=" else U <= t2) else (2 if (U < t3 if GE == ">=" else U <= t3) else 3)
    pct = {1: p1, 2: p2, 3: p3}[tier]
    gross = U * price
    rebate = gross * pct // 100
    net = gross - rebate
    truth = {"tier": tier, "rebate_cents": rebate, "net_cents": net}
    rules = [
        f"tier two begins {'at-or-over' if GE == '>=' else 'strictly past'} {t2} units and tier three begins {'at-or-over' if GE == '>=' else 'strictly past'} {t3} units, units are whole and never rounded",
        "the tier percentage is retroactive, it applies to the entire volume, not just the units above the boundary, this is the whole point of the program and the whole trap of it",
        "rebate is gross times percent floor-divided by 100 in whole cents, net is gross minus rebate, both reported",
    ]
    facts = [f"the program moved {U} whole units this quarter at {price} whole cents per unit",
             f"tier two starts at {t2} units, tier three starts at {t3} units, tier rates are {p1}, {p2}, and {p3} percent",
             "the board quotes no gross tonight, the desk derives it before any tier applies"]
    body = [
        "the rebate board loves the word retroactive right up until the tier boundary sits exactly on tonight's volume, and then the room divides into people who remember the boundary word and people who remember their bonus",
        f"the {'at-or-over' if GE == '>=' else 'strictly-past'} convention governs both tier edges tonight, identically, the binder does not permit mixed conventions within one program",
        "the desk computes the tier once, from full volume, then rebates the full gross at that single percentage, the marginal-tier reading is a different program at a different company with different lawyers",
    ]
    entities = [E("U", "units moved this quarter (copy the number from its fact line)", U),
                E("price", "price per unit in whole cents (copy the number from its fact line)", price),
                E("t2", "tier two start in units (copy the number from its fact line)", t2),
                E("t3", "tier three start in units (copy the number from its fact line)", t3),
                E("gross", "gross revenue in whole cents (units times price)", gross),
                E("tier", "the tier number earned, 1, 2, or 3 (apply both boundary words to the volume)", tier),
                E("reb", "the rebate in whole cents (gross times the earned tier's percent)", rebate),
                E("net", "the net in whole cents (gross minus rebate)", net)]
    i1 = f"(U < t2)" if GE == ">=" else f"(U <= t2)"
    i2 = f"(U < t3)" if GE == ">=" else f"(U <= t3)"
    agg = ("{'tier': 1 if " + i1.replace("U", "a('U')").replace("t2", "a('t2')") + " else (2 if " + i2.replace("U", "a('U')").replace("t3", "a('t3')") + " else 3), "
           "'rebate_cents': a('gross') * (a('p1') if " + i1.replace("U", "a('U')").replace("t2", "a('t2')") + " else (a('p2') if " + i2.replace("U", "a('U')").replace("t3", "a('t3')") + " else a('p3'))) // 100, "
           "'net_cents': a('gross') - (a('gross') * (a('p1') if " + i1.replace("U", "a('U')").replace("t2", "a('t2')") + " else (a('p2') if " + i2.replace("U", "a('U')").replace("t3", "a('t3')") + " else a('p3'))) // 100)}")
    # entities need p1..p3 for aggregate
    entities = [E("U", "units moved this quarter (copy the number from its fact line)", U),
                E("price", "price per unit in whole cents (copy the number from its fact line)", price),
                E("t2", "tier two start in units (copy the number from its fact line)", t2),
                E("t3", "tier three start in units (copy the number from its fact line)", t3),
                E("p1", "tier one rate in percent (copy the number from its fact line)", p1),
                E("p2", "tier two rate in percent (copy the number from its fact line)", p2),
                E("p3", "tier three rate in percent (copy the number from its fact line)", p3),
                E("gross", "gross revenue in whole cents (units times price)", gross),
                E("tier", "the tier number earned, 1, 2, or 3 (apply both boundary words to the volume)", tier),
                E("reb", "the rebate in whole cents (gross times the earned tier's percent)", rebate)]
    contract = {
        "tier": (f"(1 if {i1} else (2 if {i2} else 3))", [("U", r"moved (\d+) whole units", U), ("t2", r"tier two starts at (\d+) units", t2), ("t3", r"tier three starts at (\d+) units", t3)]),
        "rebate_cents": (f"(U * price) * ({p1} if tier == 1 else ({p2} if tier == 2 else {p3})) // 100",
                         [("U", r"moved (\d+) whole units", U), ("price", r"at (\d+) whole cents per unit", price)]),
        "net_cents": ("(U * price) - rebate_cents", []),
    }
    prov = [f"U={U} op='{GE}' tier={tier} pct={pct}", f"gross={gross} rebate={rebate} net={net}",
            f"retroactive: pct applies to FULL volume, edges t2={t2} t3={t3}"]
    return dict(arch="rebate_tiers", domain="reseller rebate tiers", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 9. ring_rotation_2cyc — two promotion cycles with error demotion
def arch_ring_rotation_2cyc(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(cohort=[("ring-zero", 240, 25, 8), ("ring-one", 160, 25, 3)], elig=200),
        dict(cohort=[("cohort-a", 300, 20, 6), ("cohort-b", 180, 20, 9)], elig=150),
        dict(cohort=[("wave-9", 260, 30, 14), ("wave-10", 140, 30, 2)], elig=200),
        dict(cohort=[("set-1", 320, 15, 4), ("set-2", 200, 15, 11)], elig=250),
    ][v]
    elig = V["elig"]
    promo1 = (V["cohort"][0][1] * V["cohort"][0][2] + 99) // 100
    demoted1 = V["cohort"][0][3]
    net1 = max(0, promo1 - demoted1)
    base2 = V["cohort"][1][1] + net1
    promo2 = (base2 * V["cohort"][1][2] + 99) // 100
    demoted2 = V["cohort"][1][3]
    net2 = max(0, promo2 - demoted2)
    total = net1 + net2
    stuck = V["cohort"][0][1] + V["cohort"][1][1] - total
    truth = {"cycle1_net_promoted": net1, "cycle2_net_promoted": net2, "stuck_count": stuck}
    rules = [
        f"each cycle promotes ceil of cohort times the promotion percent per hundred, rounding up always, a cohort is promotion-eligible at-or-over {elig} users, eligibility is about the cohort, the percent is about the cycle",
        "errors demote, each error removes one promotion, demotions floor at zero per cycle, demoted users rejoin their origin cohort and count as stuck",
        "cycle two's cohort inherits cycle one's net promotions before its own percent applies, inheritance is additive, the percent applies to the inherited size",
    ]
    facts = [", ".join(f"{n} starts with {s} users at {p} percent and {e} errors" for n, s, p, e in V["cohort"]),
             f"the eligibility floor is {elig} users per cohort",
             "the drum quotes no promotion counts tonight, every net derives from size, percent, and errors"]
    body = [
        "the promotion drum beats twice tonight and the arithmetic between the beats is where careers go to die, the second cycle is not the first cycle repeated, it is the first cycle inherited",
        "ceil on the percent, floor on the demotion, the two roundings point in opposite directions on purpose, the binder calls this the pessimist and the optimist sharing a desk",
        "stuck users are not failed users, they are users whose cohort could not carry them across, the desk counts them without commentary, commentary is for retrospectives",
    ]
    entities = [E("s1", f"{V['cohort'][0][0]}'s starting users (copy the number from its fact line)", V["cohort"][0][1]),
                E("p1", f"{V['cohort'][0][0]}'s promotion percent (copy the number from its fact line)", V["cohort"][0][2]),
                E("e1", f"{V['cohort'][0][0]}'s errors (copy the number from its fact line)", V["cohort"][0][3]),
                E("s2", f"{V['cohort'][1][0]}'s starting users (copy the number from its fact line)", V["cohort"][1][1]),
                E("p2", f"{V['cohort'][1][0]}'s promotion percent (copy the number from its fact line)", V["cohort"][1][2]),
                E("e2", f"{V['cohort'][1][0]}'s errors (copy the number from its fact line)", V["cohort"][1][3]),
                E("pr1", "cycle one's promotions before errors (ceil of the first cohort times its percent over 100)", promo1),
                E("n1", "cycle one's net promotions after demotion (floored at zero)", net1),
                E("n2", "cycle two's net promotions after demotion (inheritance first, then percent, then errors)", net2),
                E("stuck", "stuck users across both cohorts after both cycles (count them)", stuck)]
    n1e = "max(0, ((a('s1') * a('p1') + 99) // 100) - a('e1'))"
    n2e = "max(0, (((a('s2') + " + n1e + ") * a('p2') + 99) // 100) - a('e2'))"
    agg = ("{'cycle1_net_promoted': " + n1e + ", 'cycle2_net_promoted': " + n2e +
           ", 'stuck_count': a('s1') + a('s2') - " + n1e + " - " + n2e + "}")
    contract = {
        "cycle1_net_promoted": ("max(0, (s1 * p1 + 99) // 100 - e1)",
                                [("s1", rf"{V['cohort'][0][0]} starts with (\d+) users", V["cohort"][0][1]), ("p1", rf"{V['cohort'][0][0]} starts with \d+ users at (\d+) percent", V["cohort"][0][2]), ("e1", rf"{V['cohort'][0][0]} starts with \d+ users at \d+ percent and (\d+) errors", V["cohort"][0][3])]),
        "cycle2_net_promoted": ("max(0, ((s2 + cycle1_net_promoted) * p2 + 99) // 100 - e2)",
                                [("s2", rf"{V['cohort'][1][0]} starts with (\d+) users", V["cohort"][1][1]), ("p2", rf"{V['cohort'][1][0]} starts with \d+ users at (\d+) percent", V["cohort"][1][2]), ("e2", rf"{V['cohort'][1][0]} starts with \d+ users at \d+ percent and (\d+) errors", V["cohort"][1][3])]),
        "stuck_count": ("s1 + s2 - cycle1_net_promoted - cycle2_net_promoted", []),
    }
    prov = [f"promo1={promo1} demoted1={demoted1} net1={net1}", f"base2={base2} promo2={promo2} net2={net2}",
            f"total={total} stuck={stuck}", "ceil promos, floor demotions, cycle2 inherits"]
    return dict(arch="ring_rotation_2cyc", domain="release canary promotion cycles", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 10. shift_carousel — rota advance with pins, modulo
def arch_shift_carousel(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(N=7, start=2, steps=16, pins=1),
        dict(N=9, start=4, steps=25, pins=2),
        dict(N=6, start=0, steps=11, pins=0),
        dict(N=8, start=5, steps=30, pins=3),
    ][v]
    N, start, steps, pins = V["N"], V["start"], V["steps"], V["pins"]
    moving = N - pins
    wrapped = steps > N
    landed = (start + steps) % moving if moving > 0 else start
    truth = {"landed_index": landed, "pinned_count": pins, "wrapped": bool(wrapped)}
    rules = [
        f"the carousel advances one slot per step, pinned slots are skipped and never landed on, landing is modulo the moving count, not the roster",
        f"a shift {'requires a fresh landing when steps at-or-over the roster size' if GE == '>=' else 'requires a fresh landing only when steps go strictly past the roster size'} — the wrap flag records whether tonight's steps reached that bar, the flag is true only then",
        "the landed index counts only moving slots, pins shrink the modulus, the roster number itself never changes",
    ]
    facts = [f"the rota holds {N} slots and tonight's holder sits at index {start}",
             f"the rota advances {steps} steps tonight",
             f"{pins} slots are pinned tonight and the moving count works out to {moving}"]
    body = [
        "the rota wheel is arithmetic wearing a hi-vis vest, pins shrink the modulus, the modulus is the moving count, and anyone who mods by the roster size gets to explain themselves at the morning huddle",
        f"tonight's wrap flag is about whether the steps {'reached at-or-over the roster size' if GE == '>=' else 'went strictly past the roster size'}, which sounds ceremonial until payroll asks why a six-slot rota paid out nine shifts",
        "the desk computes moving count first, landing second, wrap last, and never lets the wrap flag leak into the landing math, they are separate clauses with separate jobs",
    ]
    entities = [E("N", "rota slot count (copy the number from its fact line)", N),
                E("start", "tonight's holder index (copy the number from its fact line)", start),
                E("steps", "steps advanced tonight (copy the number from its fact line)", steps),
                E("pins", "pinned slot count (copy the number from its fact line)", pins),
                E("moving", "the moving count (roster minus pins)", moving),
                E("landed", "the landed index (holder index plus steps, modulo the moving count)", landed),
                E("wrapped", "did tonight's steps reach the wrap bar, true or false (apply the boundary word to steps vs roster size)", bool(wrapped))]
    wrap_e = f"(steps >= N)" if GE == ">=" else "(steps > N)"
    agg = ("{'landed_index': (a('start') + a('steps')) % (a('N') - a('pins')), "
           "'pinned_count': int(a('pins')), "
           "'wrapped': bool(a('steps') " + GE + " a('N'))}")
    contract = {
        "landed_index": ("(start + steps) % (N - pins)",
                         [("start", r"holder sits at index (\d+)", start), ("steps", r"advances (\d+) steps", steps), ("N", r"rota holds (\d+) slots", N), ("pins", r"(\d+) slots are pinned", pins)]),
        "pinned_count": ("pins", []),
        "wrapped": (f"bool({wrap_e})", []),
    }
    prov = [f"N={N} start={start} steps={steps} pins={pins}", f"moving={moving} landed={landed} wrapped={wrapped} (op '{GE}')",
            "landing mod moving count, not roster"]
    return dict(arch="shift_carousel", domain="warehouse rota carousels", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 11. canary_percent — ceil step with stuck-at-floor
def arch_canary_percent(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(n=450, pct=7, floor_min=40),
        dict(n=280, pct=5, floor_min=18),
        dict(n=900, pct=3, floor_min=30),
        dict(n=160, pct=9, floor_min=15),
    ][v]
    n, pct, floor_min = V["n"], V["pct"], V["floor_min"]
    raw = (n * pct + 99) // 100
    floor_applied = raw < floor_min
    step = floor_min if floor_applied else raw
    remaining = n - step
    truth = {"step_users": step, "floor_applied": bool(floor_applied), "remaining_users": remaining}
    rules = [
        f"a ramp step releases ceil of cohort times percent over 100 users, rounding up always, the ramp is stuck when that raw step lands strictly under the floor minimum, stuck steps release exactly the floor minimum instead",
        f"a step {'is considered a real step at-or-over the floor minimum' if GE == '>=' else 'counts as a real step only strictly past the floor minimum'}, below that the floor governs, the floor is a floor, not a suggestion",
        "remaining is cohort minus the released step, the released step is the number that actually shipped, floor or no floor",
    ]
    facts = [f"the cohort holds {n} users",
             f"tonight's ramp percent is {pct} and the floor minimum is {floor_min} users",
             "the gate quotes no raw step tonight, the desk derives it before the floor rule speaks"]
    body = [
        "the ramp gate moves percentages of people and the percentage of a person has never once been valid, so ceil, always ceil, the argument against ceil died the night a half-user got a notification and called support",
        "the floor minimum exists for the weeks when the percent rounds up to something embarrassing, the floor says the ramp shall not whisper, it shall speak at least this loudly",
        "the desk computes raw, applies the floor if raw falls under it, and reports what shipped, not what was intended, the two differ exactly when the floor fires",
    ]
    entities = [E("n", "cohort size in users (copy the number from its fact line)", n),
                E("pct", "ramp percent (copy the number from its fact line)", pct),
                E("fm", "floor minimum in users (copy the number from its fact line)", floor_min),
                E("raw", "the raw step in users (ceil of cohort times percent over 100)", raw),
                E("step", "the step that actually ships in users (floor rule applied)", step),
                E("fa", "floor applied, true or false (did raw land strictly under the floor minimum)", bool(floor_applied)),
                E("rem", "remaining users after the step ships (cohort minus shipped step)", remaining)]
    agg = ("{'step_users': (max(a('fm'), (a('n') * a('pct') + 99) // 100)), "
           "'floor_applied': bool((a('n') * a('pct') + 99) // 100 < a('fm')), "
           "'remaining_users': a('n') - max(a('fm'), (a('n') * a('pct') + 99) // 100)}")
    contract = {
        "step_users": ("max(floor_min, (n * pct + 99) // 100)",
                       [("n", r"cohort holds (\d+) users", n), ("pct", r"ramp percent is (\d+)", pct), ("floor_min", r"floor minimum is (\d+) users", floor_min)]),
        "floor_applied": ("bool((n * pct + 99) // 100 < floor_min)", []),
        "remaining_users": ("n - step_users", []),
    }
    prov = [f"n={n} pct={pct} raw={raw} floor_min={floor_min}", f"floor_applied={floor_applied} step={step} remaining={remaining}",
            "raw=ceil(n*pct/100), stuck→floor"]
    return dict(arch="canary_percent", domain="ramp percentage gates", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 12. redrive_decay — per-attempt decayed delivery
def arch_redrive_decay(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(b=600, r=70, d=15, K=3),
        dict(b=450, r=80, d=10, K=3),
        dict(b=800, r=60, d=20, K=3),
        dict(b=300, r=75, d=25, K=3),
    ][v]
    b, r, d, K = V["b"], V["r"], V["d"], V["K"]
    rates = [max(0, r - k * d) for k in range(K)]
    deliv = [b * rk // 100 for rk in rates]
    dead = b - sum(deliv)
    truth = {"attempt1_delivered": deliv[0], "attempt2_delivered": deliv[1], "dead_after_retries": dead}
    rules = [
        f"each retry attempt delivers at a decayed rate, attempt k's rate is the base rate minus k times the decay, floored at zero percent, delivery is batch times rate floor-divided by 100, whole messages",
        f"a message {'leaves the replay lane at-or-over three attempts' if GE == '>=' else 'leaves the replay lane only strictly past three attempts'} and every departure after that is final",
        "dead is batch minus everything delivered across all attempts, dead is reported without apology",
    ]
    facts = [f"the replay lane holds {b} messages with base rate {r} percent and decay {d} percent per attempt",
             f"attempts stop at {K}",
             "the lane quotes no attempt rates tonight, each one derives from the base and the decay"]
    body = [
        "the replay desk watches the same messages limp past night after night at declining rates, the decay is honest work being done by tired machinery, and the arithmetic is just the ledger of that tiredness",
        "the rate floor at zero matters on long decay ladders, a rate that steps below zero delivers nothing and pretends nothing, the desk does not deal in negative enthusiasm",
        "dead messages are counted, named, and mourned briefly, the count is the deliverable, the mourning is informal",
    ]
    entities = [E("b", "batch size in messages (copy the number from its fact line)", b),
                E("r", "base rate percent (copy the number from its fact line)", r),
                E("d", "decay percent per attempt (copy the number from its fact line)", d),
                E("ra1", "attempt one rate in percent (base)", rates[0]),
                E("ra2", "attempt two rate in percent (base minus one decay)", rates[1]),
                E("ra3", "attempt three rate in percent (base minus two decays, floored at zero)", rates[2]),
                E("dv1", "attempt one delivered in messages (batch times attempt one rate)", deliv[0]),
                E("dv2", "attempt two delivered in messages (batch times attempt two rate)", deliv[1]),
                E("dead", "dead messages after all attempts (batch minus all delivered)", dead)]
    agg = ("{'attempt1_delivered': a('b') * a('ra1') // 100, "
           "'attempt2_delivered': a('b') * a('ra2') // 100, "
           "'dead_after_retries': a('b') - (a('b') * a('ra1') // 100) - (a('b') * a('ra2') // 100) - (a('b') * max(0, a('r') - 2 * a('d')) // 100)}")
    contract = {
        "attempt1_delivered": ("b * r // 100", [("b", r"lane holds (\d+) messages", b), ("r", r"base rate (\d+) percent", r)]),
        "attempt2_delivered": ("b * max(0, r - d) // 100", [("d", r"decay (\d+) percent per attempt", d)]),
        "dead_after_retries": ("b - (b * r // 100) - (b * max(0, r - d) // 100) - (b * max(0, r - 2 * d) // 100)", []),
    }
    prov = [f"b={b} r={r} d={d} rates={rates}", f"delivered={deliv} dead={dead}",
            "rate_k = max(0, r - k*d), delivered_k = b*rate_k//100"]
    return dict(arch="redrive_decay", domain="stream replay decay", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


# 13. backoff_scale — doubling waits, ms→s, budget gate
def arch_backoff_scale(v):
    GE, CW = _ge(v), _conv_word(v)
    V = [
        dict(base=500, cap=8000, retries=5, budget=12),
        dict(base=250, cap=4000, retries=5, budget=6),
        dict(base=1000, cap=8000, retries=4, budget=15),
        dict(base=500, cap=16000, retries=5, budget=20),
    ][v]
    base, cap, retries, budget = V["base"], V["cap"], V["retries"], V["budget"]
    waits = []
    w = base
    for _ in range(retries):
        waits.append(min(w, cap))
        w *= 2
    total_ms = sum(waits)
    total_s = total_ms // 1000
    breached = (total_s >= budget) if GE == ">=" else (total_s > budget)
    truth = {"wait3_ms": waits[2], "total_wait_ms": total_ms, "budget_breached": bool(breached)}
    rules = [
        f"each retry waits double the previous wait, starting from the base, every wait capped at the cap, the cap applies after doubling, doubling never resurrects a capped wait",
        f"the budget gate reads the total wait in whole seconds, the run breaches when the total is {'at-or-over the budget' if GE == '>=' else 'strictly past the budget'}, milliseconds convert to seconds floor-divided by 1000",
        "reported waits are milliseconds, the budget is seconds, the conversion is part of the test",
    ]
    facts = [f"the base wait is {base} milliseconds, the cap is {cap} milliseconds, and the run makes {retries} retries against a {budget} second budget",
             "the runner quotes no wait list and no total tonight, the desk doubles, caps, and sums from the base alone"]
    body = [
        "the pause clock doubles and doubles until the cap slaps it down, and the only real trap tonight is the unit line, waits live in milliseconds, the budget lives in seconds, and the conversion direction has caught better desks than this one",
        f"the breach word is {'at-or-over' if GE == '>=' else 'strictly-past'} tonight, a total that lands exactly on the budget is either a breach or a miracle depending on that single word, read it again",
        "the desk lists every wait, caps applied, before summing anything, sums assembled from uncapped doublings are fiction with decimals",
    ]
    entities = [E("base", "base wait in whole milliseconds (copy the number from its fact line)", base),
                E("cap", "wait cap in whole milliseconds (copy the number from its fact line)", cap),
                E("w3", "the third wait in whole milliseconds (doubles capped, third of the run)", waits[2]),
                E("tms", "the total wait across all retries in whole milliseconds", total_ms),
                E("budget", "the budget in whole seconds (copy the number from its fact line)", budget),
                E("breach", "budget breached, true or false (apply the boundary word to total seconds vs budget)", bool(breached))]
    agg = ("{'wait3_ms': min({base} * 4, {cap}), 'total_wait_ms': " +
           " + ".join(f"min({base} * {2 ** k}, {cap})" for k in range(retries)) +
           ", 'budget_breached': bool(((" + " + ".join(f"min({base} * {2 ** k}, {cap})" for k in range(retries)) + ") // 1000) " + GE + " a('budget'))}")
    agg = agg.replace("{base}", "a('base')").replace("{cap}", "a('cap')")
    contract = {
        "wait3_ms": ("min(base * 4, cap)", [("base", r"base wait is (\d+) milliseconds", base), ("cap", r"cap is (\d+) milliseconds", cap)]),
        "total_wait_ms": ("min(base*1, cap) + min(base*2, cap) + min(base*4, cap) + min(base*8, cap)" + (f" + min(base*16, cap)" if retries > 4 else ""), []),
        "budget_breached": (f"bool((total_wait_ms // 1000) {GE} budget)", [("budget", r"against a (\d+) second budget", budget)]),
    }
    prov = [f"base={base} cap={cap} retries={retries}", f"waits={waits} total_ms={total_ms} total_s={total_s}",
            f"breached={breached} (op '{GE}' vs budget={budget}s)",
            "wait_k = min(base*2^k, cap), seconds = ms//1000"]
    return dict(arch="backoff_scale", domain="scheduler pause scaling", rules=rules, facts=facts,
                body=body, truth=truth, prov=prov, entities=entities, aggregate=agg, contract=contract, variant=v)


ARCHETYPES_A = [arch_quota_proration, arch_tiered_postage, arch_ballot_quorum, arch_disk_quota_conv,
                arch_escrow_ladder, arch_per_diem_holds, arch_multi_wallet, arch_rebate_tiers,
                arch_ring_rotation_2cyc, arch_shift_carousel, arch_canary_percent, arch_redrive_decay,
                arch_backoff_scale]
