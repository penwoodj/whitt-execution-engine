# Tier-A Case 040 — Contact Sheet Report (SYNTH)

this one is about contact sheet report, read it all before touching anything. here's the job: assemble a compact analysis-and-audit job, all under /tmp/opencode/photo-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

the shape of the data: a photo shoot dump metadata csv, 400 rows, no actual images needed, after that fields: filename, size, dimensions, camera, iso, timestamp, keep-or-toss flag, plus the shoot ran two days and the second day had a settings screwup, and a contact-sheet report summarizes for humans. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. per-day stats, counts, size totals, iso ranges, dimension outliers. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: the day-two screwup, what changed in settings and starting when, iso spike. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
burst detection, photos within 2 seconds of each other grouped. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. keep-or-toss audit, flagged-toss that look like bursts of the best stuff. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. sheet.md plus sheet.json, every stat recomputable from the csv. edge cases belong in the output, not in your head, list what you hit and what you did with each.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the realistic part, the warts: twelve rows have iso as a string with noise like 400+, timestamps span two formats, and the same filename exists with different sizes. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the rule you applied to each wart belongs in the rules file, next to the wart itself.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
