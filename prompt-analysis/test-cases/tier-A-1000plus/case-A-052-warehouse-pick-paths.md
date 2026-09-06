# Tier-A Case 052 — Warehouse Pick Paths (SYNTH)

next up, warehouse pick paths, whole thing specced below. i want you to build a small but complete tool with its own verification for me, the whole thing lives under /tmp/opencode/picking-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the setup is this: an order set of 40 picks in a small warehouse grid, 12 by 8 cells, then pick locations, quantities, and a depot at cell 1,1; also route efficiency question: naive versus a better heuristic, measured in cells walked, plus the grid has two blocked cells that force detours. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

concretely, do these stages, numbered so you don't creative-order them on me:

next model the grid, blocked cells enforced in all pathing, no cutting through. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. naive route, picks in given order, total distance walked. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. an improved heuristic, nearest-neighbor or sector sweep, implemented and compared. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
the distance table per order under both, plus total and percent saved. with the method visible, not just the result, the how is the deliverable here as much as the what.
first picking.md plus routes.json plus the grid map rendered in ascii. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

now the mess, because a pipeline that only works on clean data is worthless to me: two pick locations are the same cell for different items, one pick quantity is zero, and the blocked cells sit exactly on the naive path of order 17. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. write down what you did with each defect and why, in the file, not the chat.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
