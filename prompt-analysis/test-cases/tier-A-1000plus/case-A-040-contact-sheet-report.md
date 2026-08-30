# Tier-A Case 040 — Contact Sheet Report (SYNTH)

Here's the job: assemble a compact analysis-and-audit job, all under /tmp/opencode/photo-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

the world of this little exercise: a photo shoot dump metadata csv, 400 rows, no actual images needed, then fields: filename, size, dimensions, camera, iso, timestamp, keep-or-toss flag, after that the shoot ran two days and the second day had a settings screwup, plus a contact-sheet report summarizes for humans. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

per-day stats, counts, size totals, iso ranges, dimension outliers. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: the day-two screwup, what changed in settings and starting when, iso spike. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
burst detection, photos within 2 seconds of each other grouped. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. keep-or-toss audit, flagged-toss that look like bursts of the best stuff. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. sheet.md plus sheet.json, every stat recomputable from the csv. edge cases belong in the output, not in your head, list what you hit and what you did with each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: twelve rows have iso as a string with noise like 400+, timestamps span two formats, and the same filename exists with different sizes. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
