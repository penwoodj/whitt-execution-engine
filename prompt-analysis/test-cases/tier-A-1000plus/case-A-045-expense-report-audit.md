# Tier-A Case 045 — Expense Report Audit (SYNTH)

expense report audit. been meaning to get this done properly for a while. here's the job: construct a self-contained reporting exercise, all under /tmp/opencode/expaudit-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

the world of this little exercise: an expense export, 400 claims across a quarter, employee, amount, category, date, receipt flag. policy: meals under 75 no receipt, anything over 300 needs a flag, weekends are suspicious for office supplies. one employee claims exactly 74.90 for meals suspiciously often. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

concretely, do these stages, numbered so you don't creative-order them on me:

1. policy checks, no-receipt-over-threshold, over-300 unflagged, weekend anomalies. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. the 74.90 pattern, exact-below-threshold gaming detection, generalizable beyond this one employee. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: per-employee claim profile, mean, count, category mix. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
then ranked anomaly list with a reason string each. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
audit.md plus audit.json plus anomalies.csv. with the method visible, not just the result, the how is the deliverable here as much as the what.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

now the mess, because a pipeline that only works on clean data is worthless to me: categories have six spellings of travel, two claims are negative refunds, and one date is february 30th. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. every one of those needs a documented disposition, handled, quarantined, or rejected, with the rule cited.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
