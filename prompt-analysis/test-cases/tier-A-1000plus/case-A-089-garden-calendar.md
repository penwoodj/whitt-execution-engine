# Tier-A Case 089 — Garden Calendar (SYNTH)

new job, garden calendar, same standards as always. i want you to assemble a compact analysis-and-audit job for me, the whole thing lives under /tmp/opencode/garden-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

here's the situation: frost dates, a 20-crop list with germination days, transplant tolerance, and maturity days, and the deliverable: a planting calendar keyed to last-frost day. succession planting for fast crops, three sowings, is wanted, after that one crop is tropical and must never go out before soil warmth, a different trigger than frost. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

what i need done, in this order:

1. compute sow and transplant windows per crop from last-frost plus offsets, all arithmetic shown. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. the calendar as a week-by-week table, indoor sow, outdoor sow, transplant, harvest-estimate columns. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. succession schedule for the fast crops, three waves spaced correctly. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. the tropical crop handled on soil-temperature logic, flagged distinctly. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. garden.md plus calendar.csv plus crops.json. with the method visible, not just the result, the how is the deliverable here as much as the what.

the data has problems, deliberately, and handling them is part of the job not an error condition: maturity days are ranges not points, two crops share a name variant requiring canonicalization, and one frost date is given as a probability band not a date requiring a stated policy. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. write down what you did with each defect and why, in the file, not the chat.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
