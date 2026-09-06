# Tier-A Case 082 — Install Order Plan (HEAVY)

next up, install order plan, whole thing specced below. so run a proper multi-stage data job, fully self contained in /tmp/opencode/install-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

so the scenario: a requirements dump, 150 packages with declared dependencies, no versions needed, then produce a valid install order, and a parallel install plan where independence allows, and post-install scripts exist for some packages and have ordering needs beyond dependencies, after that one dependency is satisfied by either of two alternatives, choice must be stated. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

what i need done, in this order:

first topological order over the dependency graph, determinism via name-sort among ready nodes. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first parallel batches, level-by-level, everything installable simultaneously grouped. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
3. script-ordering constraints layered on top, which scripts force serialization and why. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
after that the alternative-choice decision, which of the two picked and the stated reason. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that install.md plus order.txt plus batches.json. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and bake in the dirt, i've said before i want things tested against reality not the happy path: the dump has one package depending on itself, two packages declare mutually exclusive conflicts, and three dependency names reference packages absent from the dump requiring an assumed-external flag. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the rule you applied to each wart belongs in the rules file, next to the wart itself.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
