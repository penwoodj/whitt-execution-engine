# Tier-A Case 062 — Ab Test Sanity (HEAVY)

Let's do run a proper multi-stage data job, entirely inside /tmp/opencode/abtest-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

here's the situation: an a/b test log, 40,000 users split into control and treatment; also assignment was supposed to be random 50/50 but looks 47/53 and primary metric: conversion within 7 days, secondary: revenue per user; also a peeking violation occurred, someone checked results at day 2 and day 5. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

concretely, do these stages, numbered so you don't creative-order them on me:

1. assignment balance test with a stated threshold, is 47/53 explainable by chance. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
stage by stage: primary metric effect with interval, computed properly from user-level data. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the peeking violation quantified, what the day-2 and day-5 snapshots would have concluded, and why the final differs or does not. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. novelty check, effect by day-since-exposure, decaying or stable. and be precise about what counts as done for that one, because vague is where shortcuts hide.
first ab.md plus ab.json plus a peeking.txt appendix. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: fifty users appear in both arms with different ids but identical fingerprints, revenue has a 99th-percentile whale distorting means, and 300 users have exposure but zero subsequent activity meaning assignment leaked. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
