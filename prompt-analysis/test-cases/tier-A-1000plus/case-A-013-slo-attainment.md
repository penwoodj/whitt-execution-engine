# Tier-A Case 013 — Slo Attainment (SYNTH)

What i want is construct a self-contained reporting exercise, set up inside /tmp/opencode/slo-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

so the scenario: six services, each with its own slo and a month of per-request latency and error data and attainment means percent of good minutes or good requests, pick and justify; also two services are on the boundary at 99.0 and 99.1 and data lives in per-service folders with one jsonl per day. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

what i need done, in this order:

1. one attainment number per service with the definition you chose written down first. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. with the method visible, not just the result, the how is the deliverable here as much as the what.
next per-week breakdown showing trends, improving or degrading. with the method visible, not just the result, the how is the deliverable here as much as the what.
first the boundary services need extra scrutiny, hour-level resolution, because rounding could flip them. with the method visible, not just the result, the how is the deliverable here as much as the what.
stage by stage: a league table sorted worst to best with a traffic-weighted overall row. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. attainment.md plus attainment.json, verifier recomputes independently. with the method visible, not just the result, the how is the deliverable here as much as the what. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

now the mess, because a pipeline that only works on clean data is worthless to me: two services have a duplicated day file with slightly different contents, you must detect and handle the conflict explicitly. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
