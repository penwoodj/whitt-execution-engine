# Tier-A Case 077 — Freeze Violations (SYNTH)

new job, freeze violations, same standards as always. i want you to assemble a compact analysis-and-audit job for me, the whole thing lives under /tmp/opencode/freeze-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the shape of the data: a commit dump spanning a code freeze window, dec 15 to jan 5. violations: non-approved commits merged during the window, after that an approvals list exists but uses ticket ids while commits use message refs, matching is fuzzy, then emergency commits are legal with an incident tag in the message. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

here's the task list, in sequence:

first freeze window applied, every commit inside classified: incident-tagged, approved, or violation. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. the fuzzy matching from commit refs to approval tickets documented, with match confidence. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: violation list, author, what, when, and severity by lines touched. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. approval anomalies, approvals granted after the commit, or by the committer themselves. and be precise about what counts as done for that one, because vague is where shortcuts hide.
next freeze.md plus violations.csv plus matching.md. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

the realistic part, the warts: three commits straddle the boundary by timezone ambiguity, one incident tag is misspelled, and a merge commit parents hide four actual commits inside the window. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. write down what you did with each defect and why, in the file, not the chat.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
