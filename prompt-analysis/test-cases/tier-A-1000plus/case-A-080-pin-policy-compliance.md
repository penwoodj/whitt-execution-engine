# Tier-A Case 080 — Pin Policy Compliance (SYNTH)

What i want is build a small but complete tool with its own verification, set up inside /tmp/opencode/pins-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

background, and i'm giving you the details the way they actually occur: a manifest dump of 200 dependencies across 5 projects, after that policy: runtime deps pinned exact, dev deps caret-ok, git deps commit-pinned, then violations by class, and the worst offender projects, then one dependency is pinned to a version that does not exist, a typo-pin. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

what i need done, in this order:

1. classify every dependency entry, runtime, dev, git, other, by manifest section. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. policy check per class, compliant or violation type, exact counts by project. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. the typo-pin found via existence checking against the versions index file provided. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. per-project compliance score and the overall league table. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. pins.md plus violations.csv plus scores.json. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. with the method visible, not just the result, the how is the deliverable here as much as the what.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

now the mess, because a pipeline that only works on clean data is worthless to me: sections are inconsistently named across projects, one project nests its real manifest in a subfolder, and the versions index is itself slightly stale missing two legitimate versions. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
