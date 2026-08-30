"""Fresh style pools for hd-* cases. Deliberately distinct openers from
hc (no first-5-word overlap), same lowercase comma-spliced voice."""

OPENERS = [
    "the {sysname} closes its ledger at 03:00 and everyone here treats that closing as gospel",
    "night two of the {shift} and the {role} desk is down to one working pen and one working mind",
    "the {sysname} has opinions, the {role} desk has the binder, the binder wins every argument",
    "every shift on the {sysname} starts the same way, coffee, then the counts, then the arguments about the counts",
    "the {role} desk keeps the {sysname} honest, which is a full-time job on a {shift}",
    "nobody remembers when the {sysname} was installed, everybody remembers the night it was wrong",
    "the {shift} crew calls the {sysname} the oracle, mostly affectionately, occasionally in vain",
    "on the {sysname} the {role} desk is judge, jury, and arithmetic, in that order",
    "the binder for the {sysname} lives under the {role} desk, chained there by tradition and one actual chain",
    "three generations of {role} desks have run the {sysname} the same way, slowly, out loud, and twice",
    "the {sysname} prints a receipt for everything, the {role} desk reads approximately half of them, the rules cover the rest",
    "when the audit team visits the {sysname} they leave within the hour, the {role} desk's margins scare them off",
    "the {shift} begins with a handoff note that always ends the same way, trust the rules, not the totals",
    "the {sysname} is older than everyone who touches it, the arithmetic it demands is older still",
]

CONNECT = [
    "and tonight is one of those nights where every clause earns its keep",
    "which is how the desk finds itself doing this by hand again",
    "the counts below are tonight's, the rules below are forever",
    "and the nightly ritual starts with reading every rule before touching any number",
    "so the desk works the problem the slow way, on purpose",
    "and the tally lamp is on, which means somebody's numbers are about to be wrong",
]

TAILS = [
    "before committing, recheck each number against its rule, then recheck the boundaries, the boundaries are where careers go to die",
    "before committing, recheck each number against its rule, twice is the house minimum here",
    "before committing, recheck each number against its rule, the desk does not apologize for redundancy",
    "before committing, recheck each number against its rule, then read the boundary clauses one final time out loud",
    "before committing, recheck each number against its rule, slow hands survive longer than fast ones",
    "before committing, recheck each number against its rule, the recheck is part of the count, not after it",
]

DISTRRACTORS = [
    "the {dname} team you do not manage also runs a {sysname} counter with friendlier thresholds and looser edges, their numbers arrive by email and are filed for entertainment only",
    "the {dname} dashboard you do not manage rounds differently and colors outside the lines, it is decoration, the binder is the law",
    "upstairs the {dname} folks keep a parallel tally of roughly the same things, you do not manage their tally and their tally does not manage this desk",
    "the {dname} report you do not manage rebaselines monthly and disagrees constantly, it has never once been cited in an incident review, the binder has",
    "the {dname} crew you do not manage loves a good estimate, this desk does not deal in estimates, deal in the rules or deal elsewhere",
    "somewhere the {dname} system you do not manage recomputes all this with smoother conventions, its output is for slide decks, not for the record",
]

SYSNAMES = [
    "proration bench", "tiering station", "quorum rail", "vault ledger",
    "escrow rail", "per-diem booth", "wallet cage", "rebate board",
    "promotion drum", "rota wheel", "ramp gate", "replay desk",
    "pause clock", "poison sieve", "cascade board", "bump counter",
    "room comb", "triage hop", "cold line", "duty rail",
    "warranty clock", "sensor wall", "close bench", "tag press", "code bench",
]

ROLES = [
    "closing", " reconciliation", "audit", "counts", "verifications",
    "boundary", "night", "settlement", "custody", "intake",
]

SHIFTS = ["night", "evening", "weekend", "graveyard", "swing", "storm", "holiday", "cold"]

DNAMES = [
    "upstairs", "regional", "vendor", "central", "satellite",
    "downtown", "hq", "sister-site", "contractor", "offsite",
]

PAD = [
    "the desk keeps one drawer of pencils graded by trust level, the unsharpened ones are for drafts, the drafts never survive contact with the rules anyway",
    "old hands say the arithmetic is the easy ten percent, the other ninety is remembering which convention tonight's boundary clause uses, they say it while checking the clause again",
    "every rule here was paid for, some in rework, some in apologies, a few in actual money, which is why nobody proposes removing one anymore",
    "the margin notes in the binder argue with each other in three different hands, the rules themselves have never once argued back",
    "there is a brass plaque on this desk that reads a count is right or it is tomorrow, nobody remembers installing the plaque, everybody obeys it",
    "the night the numbers went unmoved for an hour, the desk learned that urgency is a poor substitute for a boundary clause read twice",
    "trainees arrive full of shortcuts, the desk is patient, the desk simply reruns every shortcut through the rules until the shortcut retires itself",
    "the clock on the wall runs two minutes fast on purpose, the counts close on the ledger time, the wall clock is a social creature, the ledger is not",
    "somewhere there is a manual for all this, it is out of date in the specific way that matters, the binder is current because the binder is amended by hand the same night the rule changes",
    "when the counts disagree with a summary, the counts win, when the counts disagree with each other, someone skipped a clause, that someone gets named in the closing note",
    "the desk has survived four reorgs by being the only thing in the building that can reproduce last tuesday, reproducibility is a lifestyle here",
    "there are no participation trophies on this wall, just a running tally of nights the boundaries held, the tally is the trophy",
]
