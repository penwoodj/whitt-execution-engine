# Tier-A Case 100 — Api Schema Diff (SYNTH)

the subject this time is api schema diff, and i want it done like i mean it. i want you to construct a self-contained reporting exercise for me, the whole thing lives under /tmp/opencode/schemadiff-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the shape of the data: an openapi-ish spec at version 4 and version 5. the diff must be consumer-facing: breaking versus additive, per endpoint, then a renamed field with a shim parameter is soft-breaking, classify honestly, then three deprecated endpoints in v4 are gone in v5. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

what i need done, in this order:

first endpoint-level diff, added, removed, changed, with the change detail. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. breaking classification with rules, field removed, type narrowed, required-added, and the shim case as soft. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the consumer impact ranked list, worst first, with migration hint per item. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
machine-readable diff as the deliverable twin. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then schemadiff.md plus diff.json plus breaking.csv. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the data has problems, deliberately, and handling them is part of the job not an error condition: one endpoint moved paths with a redirect declared in prose, type formats loosen in one spot and tighten in another, and the v5 file has a typo making one schema unresolvable. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
