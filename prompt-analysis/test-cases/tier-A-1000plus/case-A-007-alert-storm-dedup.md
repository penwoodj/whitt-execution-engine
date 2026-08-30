# Tier-A Case 007 — Alert Storm Dedup (HEAVY)

Build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/alert-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

here's the situation: a monitoring system that fired 2000 alerts in one bad hour, then alerts have service, severity, message template, and timestamp, after that most alerts are the same underlying incident fanned out across services, and severities are critical, warning, info, and one fat-fingered CRITICAL. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

after that group alerts into incidents by template and time proximity, define the rule and write it down. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. count incidents, the biggest one, its span, and its service blast radius. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. find alerts that fired more than ten times for the same thing, the noise makers. edge cases belong in the output, not in your head, list what you hit and what you did with each.
first severity normalization pass including the all-caps typo. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: storm-report.md plus storm.json, and the grouping rule goes in a rules.md. and be precise about what counts as done for that one, because vague is where shortcuts hide. with the method visible, not just the result, the how is the deliverable here as much as the what.

the data has problems, deliberately, and handling them is part of the job not an error condition: timestamps are in two formats mixed together and eight alerts have timestamps out of order. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
