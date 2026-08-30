# Tier-A Case 051 — Fuel Discrepancy (HEAVY)

What i want is run a proper multi-stage data job, set up inside /tmp/opencode/fuel-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: fleet fuel card transactions plus odometer readings for 20 vans over 3 months, plus mpg per van per month computable from liters and kilometers, and one van looks like it is siphoning fuel or the data is lying and fuel type mismatch, diesel van with petrol charges, is a data-quality axis too. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

concretely, do these stages, numbered so you don't creative-order them on me:

1. per-van monthly liters, km, computed efficiency, flagged outliers beyond 2 sigma from fleet norm. edge cases belong in the output, not in your head, list what you hit and what you did with each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. the suspect van deep-dive, month by month, when did it drift. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
3. fuel-type mismatches listed, van, transaction dates, severity. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. odometer monotonicity check, any van where km goes backwards. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
after that fuel.md plus fuel.json plus suspect-drilldown.txt. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: odometer readings are taken irregularly, two vans swapped cards for one week and it shows, and some transactions lack liters but have a cost. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
