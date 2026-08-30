# Tier-A Case 078 — Circular Dependency Detector (HEAVY)

Let's do build me a full working analysis pipeline, entirely inside /tmp/opencode/depcycles-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

background, and i'm giving you the details the way they actually occur: a package dependency dump, 500 packages with declared depends-on edges; also cycles must be found and reported, all of them, not just one; also self-dependencies and duplicate edges exist and need pre-cleaning. the biggest strongly-connected component is the headline number. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

what i need done, in this order:

after that clean the graph, dedupe, drop self-edges, log what was dropped. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and be precise about what counts as done for that one, because vague is where shortcuts hide.
all cycles via proper cycle detection, minimal cycle basis preferred over redundant enumerations. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: the biggest scc, members listed, with edges internal to it. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. break suggestions, one edge per suggested break, ranked by how much it shatters. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then cycles.md plus cycles.json plus graph-clean.csv. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

the realistic part, the warts: the dump lists dependencies as free text with version constraints attached, one package appears under two names by typo, and three edges are marked optional which arguably excuses two of the cycles. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
