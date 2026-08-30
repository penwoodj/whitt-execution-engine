# Tier-A Case 060 — Experiment Aggregation (HEAVY)

Build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/expagg-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

what you're working with: five runs of the same experiment under nominally identical conditions; also each run has per-trial measurements, 200 trials, but run 3 used a slightly older config, after that the question: aggregate and report with honest uncertainty, or flag run 3 as excludable, plus a known instrument warmup effect poisons the first 20 trials of every run. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

here's the task list, in sequence:

after that per-run summary stats before any pooling, mean, stddev, distribution shape note. and be precise about what counts as done for that one, because vague is where shortcuts hide. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that warmup trim, first 20 trials out, justified by the known effect. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. run 3 homogeneity test against the others, include or exclude with evidence. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. pooled result with uncertainty from between-run variance, not just within. and be precise about what counts as done for that one, because vague is where shortcuts hide.
next agg.md plus agg.json plus the exclusion memo as its own file. with the method visible, not just the result, the how is the deliverable here as much as the what. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

now the mess, because a pipeline that only works on clean data is worthless to me: one run file has 198 trials not 200 with no explanation, timestamps suggest run 4 overlapped run 3 physically, and two trials in run 5 are exact duplicates of each other. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
