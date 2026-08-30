"""Entity-question enrichment hints (v10).

Each entry: arch -> {entity_id: (rule restatement, worked micro-example)}.
Appended to entity questions at case-gen time. Micro-examples use numbers
that never appear in real cases. No truth values leak: examples are complete
mini-derivations on their own numbers.
"""

HINTS = {
    "cron_lock": {
        "ex": ("every overrun is one executed run, killed runs still count as firings, sum the overrun counts per job",
               "a job that overran 3 times and another that overran 2 times -> 3+2 = 5 executed"),
        "lv": ("a job violates the lock when its runtime exceeds the lock line, compare each job's run minutes against the line, count jobs not runs",
               "runtimes of 20 and 9 minutes against a 10 minute line -> only the 20 minute job violates, count is 1"),
        "jt": ("count the distinct job names listed on the rack tonight, one per name",
               "names keeper and watcher listed -> 2 jobs"),
        "lr": ("the single largest runtime number among the jobs, no arithmetic",
               "runtimes 20 and 9 -> longest is 20"),
    },
    "dlq_redrive": {
        "dt": ("sum the dead-letter shelf counts across every topic listed",
               "topics holding 12 and 8 dead messages -> 20 total"),
        "pd": ("poison quarantine is threshold dead messages per topic that has any, add threshold once per topic with a nonzero shelf",
               "threshold 4, two topics with dead letters -> 4+4 = 8 quarantined"),
        "rc": ("recovered is what remains per topic after poison quarantine is removed, sum across topics",
               "topic with 12 dead, threshold 4 -> 12-4 = 8 recovered from it"),
        "ur": ("unresolved is poison plus anything recovery could not save, with threshold quarantine and full recovery of the rest it is just the quarantined count",
               "8 quarantined, everything else recovered -> 8 unresolved"),
    },
    "fuel_cards": {
        "oc": ("a card is flagged when its spend is at or over its cap, at-the-cap counts as flagged, count cards",
               "spends 250 of 250 and 180 of 200 -> only the 250 card is flagged, count 1"),
        "ax": ("exactly-at means spend equals cap with no room left, count those cards alone",
               "spends 250 of 250 and 249 of 250 -> one card exactly at"),
        "bl": ("blocked is flagged cards with no exception on file, remove exception-cleared cards from the flagged set",
               "2 flagged, 1 with exception -> 1 blocked"),
        "ap": ("cleared cards are the flagged ones carrying a pre-filed exception, count them",
               "2 flagged, 1 with exception -> 1 cleared"),
    },
    "print_quota": {
        "uf": ("over quota includes sitting exactly at the quota, at-or-over is flagged, count users",
               "printed 100 of 100 and 60 of 100 -> one user flagged"),
        "uc": ("under quota means strictly below the quota number, count users",
               "printed 99 of 100 and 100 of 100 -> one user under"),
        "cd": ("color jobs are denied when the user is flagged, count denied color jobs across flagged users",
               "flagged user queued 3 color jobs -> 3 denied"),
        "lp": ("the single largest page count among the jobs listed, no arithmetic",
               "jobs of 40, 12, 8 pages -> largest is 40"),
    },
    "vent_control": {
        "zo": ("a zone is over the high edge when its reading is strictly above the high band number, count zones",
               "readings 26 and 19 against a high edge of 25 -> one zone over"),
        "zu": ("a zone is under the low edge when its reading is strictly below the low band number, count zones",
               "readings 14 and 17 against a low edge of 15 -> one zone under"),
        "vm": ("vent minutes only accrue for zones above the high edge, sum those zones' logged minutes, ignore in-band and under-band zones entirely",
               "over zones logged 30 and 10 minutes -> 40 vent minutes"),
        "oa": ("count the overrides listed as active, no arithmetic",
               "two overrides listed -> 2"),
    },
    "cache_sweep": {
        "t": ("touched is the raw sum of per-class entry counts before anything is removed",
               "classes with 80 and 40 entries -> 120 touched"),
        "e": ("evicted is the entries beyond each class ceiling, subtract the ceiling from the count when positive, sum across classes",
               "class with 80 entries against a 50 ceiling -> 30 evicted from it"),
        "d": ("duplicate entries are folded away before any counting, sum the dupes listed per class",
               "dupes of 6 and 4 -> 10 folded"),
        "r": ("retained is touched minus evicted minus folded duplicates, one subtraction chain",
               "120 touched, 30 evicted, 10 dupes -> 80 retained"),
    },
    "payroll_holds": {
        "rc": ("a row over the hold threshold is released when it carries an approved exception, under-threshold rows are always released, released count includes both kinds",
               "rows of 900 and 400 against a 500 threshold, the 900 row has an exception -> both released, count 2"),
        "hc": ("a row is held only when it is over the threshold and has no exception, count those rows",
               "rows of 900 with exception and 700 without, threshold 500 -> only the 700 row held, count 1"),
        "net": ("net paid sums the amounts of released rows only, released means under threshold or over-with-exception",
               "released rows of 400 and 900 -> net 1300"),
        "hv": ("held value sums the amounts of held rows only, the over-threshold no-exception ones",
               "one held row of 700 -> held value 700"),
    },
    "specimen_routing": {
        "hd": ("a sample is held when it has more excursions than the limit and its paperwork is complete, both conditions together",
               "limit 2, sample with 3 excursions and complete paperwork -> held"),
        "rj": ("a sample is rejected outright when its paperwork is incomplete, regardless of excursions",
               "sample with 0 excursions but incomplete paperwork -> rejected"),
        "sp": ("a sample ships when its paperwork is complete and its excursions are at or under the limit",
               "limit 2, sample with 1 excursion and complete paperwork -> ships"),
        "cs": ("cold-band shipped counts only samples that both ship and ride the cold band",
               "two shipping samples, one cold one ambient -> 1 cold-band shipped"),
    },
    "freeze_windows": {
        "shipped_n": ("a service ships when its deploy finishes at or before the freeze minute and a slot was consumed for it, convert clock times to minutes past midnight and add the deploy duration, compare against the freeze time",
               "clock 20:35 is 1235 minutes, deploy 10 minutes -> finishes 1245, freeze at 21:00 is 1260, 1245 <= 1260 so it ships"),
        "rolled_n": ("a service needs rollback when its error rate is at or above the threshold, count services",
               "rates 0.7 and 0.4 against a 0.5 threshold -> one service rolls back"),
        "slots_l": ("each deploy that ran tonight consumed one slot forever, slots left is the starting slots minus consumed deploys, rollbacks never return slots",
               "4 slots, 2 deploys consumed -> 2 left"),
        "pending_n": ("a service is pending when it neither shipped nor rolled back tonight, subtract shipped and rolled from the total services",
               "3 services, 2 shipped, 1 rolled -> 0 pending"),
    },
    "backoff_budget": {
        "burned": ("each failed attempt burns its capped wait plus its run duration, the wait doubles from the base each retry and is capped, the final attempt burns only its run time with no wait after it",
               "base 5, cap 20, run 4, 3 fails -> waits 5+10+20 = 35 plus runs 4+4+4 = 12, total 47"),
        "left": ("budget left is the starting budget minus everything burned, it may go negative, report the ledger line as is",
               "budget 60, burned 47 -> 13 left"),
        "njobs": ("count the jobs listed for the retry engine tonight, no arithmetic",
               "two jobs listed -> 2"),
        "maxwait": ("the longest single wait any job sat through, waits are capped so the cap is the ceiling, take the max across jobs",
               "waits of 10 and 20 with cap 20 -> maxwait 20"),
    },
    "rollout_rings": {
        "rp": ("rings pass in order, counting stops at the first ring whose fire reports reach the abort threshold, passed rings are the ones before that stop",
               "threshold 4, rings with 2, 1, 5 fires -> first two pass, third aborts, passed count 2"),
        "ab": ("the rollout is halted when any ring's fire reports reach the abort threshold, true or false",
               "a ring with 5 fires against threshold 4 -> halted, true"),
        "ux": ("users exposed is the sum of users on every ring that entered rollout, including the aborting ring, rings after the halt never enter",
               "rings of 100, 50, 30 users where the third aborts -> 100+50+30 = 180 exposed"),
        "hr": ("the halt ring is the id of the first ring whose fires reached the threshold, empty string when nothing halted",
               "ring ids r1 r2 r3, r2 fires hit the threshold -> halt ring r2"),
    },
    "gym_standby": {
        "sc": ("standby confirms fill the no-show freed spots in standby-list order, confirmed is the smaller of freed spots and standby names",
               "5 freed spots, 8 standby names -> 5 confirmed"),
        "sl": ("standby left is the names still unconfirmed after filling freed spots, standby total minus confirmed",
               "8 standby, 5 confirmed -> 3 left"),
        "cr": ("credits refund automatically for standby names that never got a spot, count the refunded ones",
               "3 left unconfirmed and all had credits -> 3 refunded"),
        "ns": ("no-shows are booked members who did not arrive, count them",
               "list shows 5 members absent -> 5"),
    },
    "hold_shelf": {
        "ah": ("active holds are total holds minus the ones that expired, one subtraction",
               "12 holds, 3 expired -> 9 active"),
        "er": ("expired holds released this week, the count as listed",
               "3 expired -> 3"),
        "ru": ("renewals used this week, the count as listed",
               "2 renewals -> 2"),
        "pu": ("items picked up this week, the count as listed",
               "4 pickups -> 4"),
    },
    "restock_waves": {
        "ro": ("a line reorders when its effective stock is at or below its reorder point, effective stock is on-hand minus damaged, compare per line",
               "on-hand 18, damaged 6, reorder point 12 -> effective 12, 12 <= 12 so it reorders"),
        "ql": ("count lines with any damage above zero",
               "damage of 6 and 0 -> one line with a quarantine event"),
        "du": ("sum the damaged units across every line",
               "damage 6 and 4 -> 10 damaged units"),
        "hl": ("count lines with zero damage",
               "damage 6 and 0 -> one healthy line"),
    },
    "cert_renewal": {
        "rn": ("a cert renews when its remaining days are strictly under the renew line",
               "days 20 against a 30 day line -> renews"),
        "es": ("expiring-soon is at the line or above it but under triple it",
               "days 30 of a 30 line -> expiring-soon"),
        "st": ("stable is triple the line or beyond",
               "days 95 against line 30 -> stable"),
        "rw": ("the renew line in days, the number as stated in the rules",
               "line of 30 -> 30"),
    },
    "seat_alloc": {
        "sd": ("patrons seated via tiers is the sum of seats taken per tier, capped by each tier's size",
               "tiers of 6 and 4 seats all filled -> 10 seated"),
        "bp": ("bumped requests happen when a tier is full and the request cannot seat, count them",
               "2 requests hit a full tier -> 2 bumped"),
        "wp": ("waitlist promotions fill seats left empty by no-shows, one promotion per empty seat",
               "3 empty seats, 5 waitlisted -> 3 promoted"),
        "wl": ("waitlist left is waitlist total minus promotions",
               "5 waitlisted, 3 promoted -> 2 left"),
    },
}



EXTRACTION_HINT = "copy the number exactly as its fact line states it, no arithmetic, no rounding, no derivation"
WORD_HINT = "answer with exactly one word copied from the fact line"

REWRITE = {
    "print_quota": {"*": EXTRACTION_HINT},
    "vent_control": {"*": EXTRACTION_HINT, "hi": EXTRACTION_HINT, "lo": EXTRACTION_HINT},
    "cert_renewal": {"*": EXTRACTION_HINT},
    "dlq_redrive": {"*": EXTRACTION_HINT, "fp": EXTRACTION_HINT},
    "fuel_cards": {"*": EXTRACTION_HINT, "x*": WORD_HINT},
    "specimen_routing": {"e*": EXTRACTION_HINT, "xl": EXTRACTION_HINT},
    "cache_sweep": {"*": EXTRACTION_HINT},
    "gym_standby": {"*": EXTRACTION_HINT},
    "payroll_holds": {"h*": "a row is held only when its amount is over the hold threshold AND it has no approved exception, under-threshold rows are always released, over-threshold rows with an approved exception are also released, answer exactly one word held or released", "m*": EXTRACTION_HINT, "th": EXTRACTION_HINT},
    "freeze_windows": {"*": EXTRACTION_HINT},
    "backoff_budget": {"*": EXTRACTION_HINT},
    "restock_waves": {"*": EXTRACTION_HINT},
    "quota_grace": {"*": EXTRACTION_HINT},
    "seat_alloc": {"*": EXTRACTION_HINT},
}


def enrich_question(arch, eid, question):
    hints = REWRITE.get(arch, {})
    star = hints.get("*")
    matched = None
    for pat, h in hints.items():
        if pat == "*":
            continue
        import fnmatch
        if fnmatch.fnmatch(eid, pat):
            matched = h
            break
    if matched:
        return f"{question}. hint: {matched}."
    if star:
        return f"{question}. hint: {star}."
    h = HINTS.get(arch, {}).get(eid)
    if not h:
        return question
    rule, example = h
    return f"{question}. rule: {rule}. worked example: {example}."
    h = HINTS.get(arch, {}).get(eid)
    if not h:
        return question
    rule, example = h
    return f"{question}. rule: {rule}. worked example: {example}."
