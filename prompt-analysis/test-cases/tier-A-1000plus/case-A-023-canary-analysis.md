# Tier-A Case 023 — Canary Analysis (HEAVY)

new job, canary analysis, same standards as always. what i want is put together an end-to-end processing exercise, set up inside /tmp/opencode/canary-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

context so you're not guessing: a canary deploy, 5 percent of traffic on the new version for six hours, and per-request outcomes for both versions, baseline and canary, plus success criteria agreed beforehand: error rate not worse, p95 not worse by more than 5 percent, then the canary looks slightly better on errors and slightly worse on latency. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

what i need done, in this order:

1. error rate per version with real denominators, not percentages of percentages. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. p50 and p95 per version and the delta against the agreed threshold. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then verdict, promote or rollback, strictly against the pre-agreed criteria. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then the weird middle hour where canary traffic dipped to 2 percent, does excluding it change the verdict. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. canary.md plus canary.json, criteria quoted verbatim in the report. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: the version label is missing on some records and must be inferred from a header field, document the inference rule. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. no silent absorption, each case logged with the rule that resolved it.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
