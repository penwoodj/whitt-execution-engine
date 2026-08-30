# Tier-A Case 018 — Perf Regression Tracker (HEAVY)

I need run a proper multi-stage data job done in /tmp/opencode/perf-reg-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

the world of this little exercise: benchmark results for 20 commits, each with timing of 6 scenarios, then each scenario timed three repeats per commit. a regression is any scenario slowing more than 10 percent commit-over-commit, and one commit claims to be a pure refactor but timing says otherwise. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

what i need done, in this order:

stage by stage: normalize the repeats into a best-and-median per scenario per commit. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: commit-over-commit delta table with percent change. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
flag regressions, and specifically audit the pure refactor commit. with the method visible, not just the result, the how is the deliverable here as much as the what.
after that scenario-level verdicts plus one overall per-commit verdict. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. perf.md plus perf.json, verifier recomputes from raw timing files. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. with the method visible, not just the result, the how is the deliverable here as much as the what.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: two commits share a timestamp making order ambiguous, one scenario has only one repeat, and timings have outlier spikes from machine noise. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
