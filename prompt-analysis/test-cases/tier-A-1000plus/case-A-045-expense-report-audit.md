# Tier-A Case 045 — Expense Report Audit (SYNTH)

Here's the job: construct a self-contained reporting exercise, all under /tmp/opencode/expaudit-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

so the scenario: an expense export, 400 claims across a quarter, employee, amount, category, date, receipt flag, plus policy: meals under 75 no receipt, anything over 300 needs a flag, weekends are suspicious for office supplies. one employee claims exactly 74.90 for meals suspiciously often. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

here's the task list, in sequence:

after that policy checks, no-receipt-over-threshold, over-300 unflagged, weekend anomalies. edge cases belong in the output, not in your head, list what you hit and what you did with each. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. the 74.90 pattern, exact-below-threshold gaming detection, generalizable beyond this one employee. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: per-employee claim profile, mean, count, category mix. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
then ranked anomaly list with a reason string each. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
audit.md plus audit.json plus anomalies.csv. with the method visible, not just the result, the how is the deliverable here as much as the what. and be precise about what counts as done for that one, because vague is where shortcuts hide.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and bake in the dirt, i've said before i want things tested against reality not the happy path: categories have six spellings of travel, two claims are negative refunds, and one date is february 30th. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
