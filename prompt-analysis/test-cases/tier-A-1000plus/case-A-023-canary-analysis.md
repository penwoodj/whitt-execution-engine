# Tier-A Case 023 — Canary Analysis (HEAVY)

What i want is put together an end-to-end processing exercise, set up inside /tmp/opencode/canary-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

here's the situation: a canary deploy, 5 percent of traffic on the new version for six hours, then per-request outcomes for both versions, baseline and canary, and success criteria agreed beforehand: error rate not worse, p95 not worse by more than 5 percent, plus the canary looks slightly better on errors and slightly worse on latency. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

what i need done, in this order:

1. error rate per version with real denominators, not percentages of percentages. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. p50 and p95 per version and the delta against the agreed threshold. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. verdict, promote or rollback, strictly against the pre-agreed criteria. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. the weird middle hour where canary traffic dipped to 2 percent, does excluding it change the verdict. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. canary.md plus canary.json, criteria quoted verbatim in the report. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and be precise about what counts as done for that one, because vague is where shortcuts hide.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and bake in the dirt, i've said before i want things tested against reality not the happy path: the version label is missing on some records and must be inferred from a header field, document the inference rule. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
