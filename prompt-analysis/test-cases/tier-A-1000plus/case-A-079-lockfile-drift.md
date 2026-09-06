# Tier-A Case 079 — Lockfile Drift (SYNTH)

next up, lockfile drift, whole thing specced below. construct a self-contained reporting exercise, that's the job, workspace is /tmp/opencode/lockdrift-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

so the scenario: a lockfile and three manifest files it theoretically pins and drift: manifest asks for x, lock has y, versions, missing entries, extras. the lock was hand-edited once, a version exists in lock that no manifest ever requested; also resolution must respect that lock wins for runtime but manifest wins for intent. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

then parse both formats, lock entries and manifest requirements, into comparable units. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. three-way status per package, in-sync, version-drift, lock-only, manifest-only. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
3. the hand-edit evidence, lock entries matching no manifest history. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first the reconciliation proposal per drift item, and what a regenerate would change. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that lockdrift.md plus drift.csv plus proposal.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the realistic part, the warts: version strings mix exact pins, ranges, and carets, one package renamed upstream mid-stream with a deprecation note, and the lock references a registry mirror that moved. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. no silent absorption, each case logged with the rule that resolved it.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
