# Tier-A Case 031 — Schema Migration Order (SYNTH)

What i want is assemble a compact analysis-and-audit job, set up inside /tmp/opencode/migration-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: a pile of 15 numbered sql migration files, after that dependencies exist but numbering is unreliable, renames and later alters, after that one migration depends on a column created in a later-numbered file; also the goal is a correct linear order that would apply cleanly. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

here's the task list, in sequence:

1. parse each file for tables and columns touched, creates, alters, drops. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
next build the dependency graph and produce a valid topological order. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
next the numbering-vs-actual-order diff, where the numbers lie. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. simulate the reordered run against a fresh empty sqlite, all 15 must apply clean. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. order.md with the new sequence plus sim-log.txt plus graph.json. and be precise about what counts as done for that one, because vague is where shortcuts hide. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the realistic part, the warts: two migrations have the same number, one has a syntax error on a rarely-checked constraint, and one drops a column another later re-adds. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
