# Tier-A Case 089 — Garden Calendar (SYNTH)

I want you to assemble a compact analysis-and-audit job for me, the whole thing lives under /tmp/opencode/garden-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

so the scenario: frost dates, a 20-crop list with germination days, transplant tolerance, and maturity days and the deliverable: a planting calendar keyed to last-frost day, and succession planting for fast crops, three sowings, is wanted. one crop is tropical and must never go out before soil warmth, a different trigger than frost. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. compute sow and transplant windows per crop from last-frost plus offsets, all arithmetic shown. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then the calendar as a week-by-week table, indoor sow, outdoor sow, transplant, harvest-estimate columns. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. succession schedule for the fast crops, three waves spaced correctly. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. the tropical crop handled on soil-temperature logic, flagged distinctly. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. garden.md plus calendar.csv plus crops.json. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and bake in the dirt, i've said before i want things tested against reality not the happy path: maturity days are ranges not points, two crops share a name variant requiring canonicalization, and one frost date is given as a probability band not a date requiring a stated policy. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
