# Tier-A Case 005 — Api Latency Percentiles (HEAVY)

I've got put together an end-to-end processing exercise to do and i want it done properly, which means /tmp/opencode/latency-run for everything, real scripts i can rerun, real files on disk, not a description of what would happen.

what you're working with: four api endpoints called /login, /search, /checkout, /profile and latency samples in microseconds, millions of them, one csv per endpoint; also a p99 regression is suspected on /checkout after wednesday; also slo is 300ms p99 on everything except /login at 800ms. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. load all four csvs without melting, chunked or streamed, and say which you did. edge cases belong in the output, not in your head, list what you hit and what you did with each. edge cases belong in the output, not in your head, list what you hit and what you did with each.
first p50, p90, p99, p999 per endpoint per day, integers in milliseconds. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first the /checkout regression, when it started, how big in percent, and the worst single hour. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then a histogram of /checkout before and after wednesday in ten buckets. with the method visible, not just the result, the how is the deliverable here as much as the what.
then report.md plus latency.json that a verifier can recompute from the raw csvs. and be precise about what counts as done for that one, because vague is where shortcuts hide. edge cases belong in the output, not in your head, list what you hit and what you did with each.

and bake in the dirt, i've said before i want things tested against reality not the happy path: some latency values are zero, some are negative from a broken probe, and the file has three different header spellings. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
