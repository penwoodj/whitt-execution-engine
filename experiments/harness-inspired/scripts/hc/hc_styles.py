"""Style pools for hc-* case factory. User prompting style:
lowercase, comma-spliced imperatives, stacked rules, quantified budgets,
hard cutoffs, out-of-scope distractors, recheck tails."""

OPENERS = [
    "you own the {shift} shift tonight, {sysname} is yours until handoff, nobody else will touch it",
    "you are the {role} for {sysname}, the standing orders live in the runbook you helped write last spring",
    "{sysname} fell to you in the rotation, the last person who held it left notes and a warning",
    "you inherited {sysname} three weeks ago, the runbook has been rewritten twice since then, this is the current version",
    "the {role} badge is yours tonight, {sysname} answers to you and to nobody else on the floor",
    "cover the {shift} for {sysname}, the regular is out, the runbook is the only authority that matters",
    "{sysname} is your assignment, the rules below came from a postmortem nobody wants repeated",
    "you hold the pager for {sysname} until midnight, after that it goes to someone who trusts your counts",
    "run {sysname} for the {shift}, the previous operator logged everything, verify nothing from memory",
    "tonight you are the {role}, {sysname} has rules with teeth, they were written after the march incident",
    "the {sysname} desk is yours, the binder is old but the rules inside it are current and enforced",
    "you take {sysname} for the {shift}, read the rules twice, they interact in ways that surprise people",
    "{sysname}, {shift}, your name on the log, every number you commit is audited against these rules",
    "you are standing in as {role}, {sysname} does not care who you are, only that the rules hold",
    "the day crew handed you {sysname} with a shrug, the runbook is thinner than they admit, these rules bind",
    "you run {sysname} during the {shift} window, the escalation path is you, then still you",
    "assignment: {sysname}, {shift} rotation, the rules came down from compliance last quarter",
    "your watch over {sysname} starts now, the rules stack on top of each other, work them in order",
]

TAILS = [
    "before committing recheck each number against its rule, the log you sign becomes the record",
    "before committing recheck each number against its rule, half the audits that fail fail on a skipped clause",
    "before committing recheck each number against its rule, the morning crew will not re-derive your work, they will trust it",
    "before committing recheck each number against its rule, wrong counts here page people who are asleep",
    "before committing recheck each number against its rule, the spreadsheet you leave behind is the contract",
    "before committing recheck each number against its rule, one missed interaction cascades into tomorrow",
    "before committing recheck each number against its rule, the archive copy is immutable once signed",
    "before committing recheck each number against its rule, last quarter someone confident was wrong four times",
    "before committing recheck each number against its rule, the compliance pull quotes your numbers verbatim",
    "before committing recheck each number against its rule, nothing downstream checks your math, everything downstream depends on it",
    "before committing recheck each number against its rule, the handoff note is read aloud at standup",
    "before committing recheck each number against its rule, cross-check the boundary cases first, boundaries are where errors live",
    "before committing recheck each number against its rule, if two rules seem to conflict the stricter one wins",
    "before committing recheck each number against its rule, the numbers you write are the numbers that get acted on",
]

DISTRRACTORS = [
    "the {dname} you do not manage also writes to the same dashboard, its rows are not your counts, ignore them",
    "a {dname} you do not manage polls {sysname} every few minutes, its scrapes are not part of any tally here",
    "the {dname} you do not manage mirrors old records into the archive, mirrored rows never count twice",
    "a background {dname} you do not manage keeps its own ledger of similar events, that ledger is out of scope",
    "the {dname} you do not manage sends summary emails to the list, those summaries are estimates, not counts",
    "an intern's {dname} you do not manage tracks the same names in a spreadsheet, that sheet drifted months ago",
    "the {dname} you do not manage fires a webhook on every change, webhook deliveries are not events",
    "a {dname} you do not manage replays traffic for load testing, replayed traffic is invisible to every rule here",
    "the {dname} you do not manage compiles a nightly digest from partial data, the digest never matches the source of truth",
    "a legacy {dname} you do not manage still logs in deprecated units, conversion is somebody else's chore",
    "the {dname} you do not manage photographs the whiteboard hourly, photographs are not records",
    "a vendor's {dname} you do not manage reports monthly with rounding, rounded figures are useless for these rules",
]

DNAMES = [
    "metrics relay", "log compactor", "health poller", "billing meter", "tracing sampler",
    "audit mirror", "newsletter bot", "capacity dashboard", "replay harness", "archive drone",
    "fleet reporter", "compliance scraper", "metrics sidecar", "snapshot service", "usage estimator",
    "observability agent", "nightly summarizer", "change-feed watcher",
]

NAMES = [
    "mira", "tobias", "lena", "oskar", "priya", "dmitri", "sofia", "hugo", "yuki", "amara",
    "cyrus", "elke", "mateo", "nadia", "peter", "sana", "ravi", "greta", "omar", "jules",
    "ingrid", "kofi", "petra", "sven", "aisha", "lars", "mei", "noor", "arthur", "tessa",
    "bekim", "clara", "dane", "edda", "farid", "gwen", "hakan", "iris", "jonas", "karin",
]

SHIFTS = ["night", "evening", "weekend", "graveyard", "swing", "early", "holiday", "storm"]

ROLES = [
    "floor coordinator", "queue owner", "duty officer", "release manager", "intake lead",
    "dispatch supervisor", "ops deputy", "shift lieutenant", "runbook keeper", "night auditor",
    "control desk operator", "gatekeeper", "traffic warden", "watch officer",
]

SYSNAMES = [
    "the intake desk", "the retry engine", "the pager tree", "the deploy lane", "the token shed",
    "the restock floor", "the triage board", "the retention sweeper", "the rate gate",
    "the rollout rings", "the quota desk", "the cron rack", "the dead-letter shelf",
    "the cert cabinet", "the backup window", "the cart auditor", "the fuel desk",
    "the seating sheet", "the specimen router", "the floor map", "the class roster",
    "the hold shelf", "the print counter", "the vent controller", "the sweep ledger",
    "the mixing board", "the gate ledger", "the rota desk",
]

CONNECT = [
    "count what landed, not what tried",
    "the rules stack, later rules do not cancel earlier ones",
    "a thing only counts once, no matter how many lists it appears on",
    "work the rules in order, the order is the logic",
    "partial work counts as nothing at the cutoff",
    "exactly-once means exactly-once, no charity",
    "the cap is the cap, overages defer, they never drop silently",
    "state at the cutoff is what you report, history is context only",
    "when a rule sends work back, the second pass follows the same rules",
    "boundaries are inclusive unless a rule says otherwise, and these rules say otherwise where it matters",
]
