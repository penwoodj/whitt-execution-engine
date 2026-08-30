# Tier-A Case 079 — Lockfile Drift (SYNTH)

Construct a self-contained reporting exercise, that's the job, workspace is /tmp/opencode/lockdrift-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

the setup is this: a lockfile and three manifest files it theoretically pins, after that drift: manifest asks for x, lock has y, versions, missing entries, extras and the lock was hand-edited once, a version exists in lock that no manifest ever requested. resolution must respect that lock wins for runtime but manifest wins for intent. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

concretely, do these stages, numbered so you don't creative-order them on me:

parse both formats, lock entries and manifest requirements, into comparable units. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. three-way status per package, in-sync, version-drift, lock-only, manifest-only. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. the hand-edit evidence, lock entries matching no manifest history. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
first the reconciliation proposal per drift item, and what a regenerate would change. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that lockdrift.md plus drift.csv plus proposal.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the realistic part, the warts: version strings mix exact pins, ranges, and carets, one package renamed upstream mid-stream with a deprecation note, and the lock references a registry mirror that moved. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
