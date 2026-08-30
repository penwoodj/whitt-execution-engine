# Tier-A Case 009 — Trace Waterfall (HEAVY)

Build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/trace-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

the setup is this: distributed traces, 300 of them, each a json of nested spans, after that spans have name, start microsecond, duration, and status, and the slow span is somewhere in the middle of the call tree and every trace starts with a root span called serve.request. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. compute per-span-name average and p99 self time excluding children. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. find the three deepest traces by total duration and lay out their span waterfalls as indented lists. and be precise about what counts as done for that one, because vague is where shortcuts hide.
after that the slowest span name overall and what percentage of total time it burns. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
first status breakdown per span name, how often each fails. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. waterfall.md plus waterfall.json, verifier recomputes from the raw trace files. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

now the mess, because a pipeline that only works on clean data is worthless to me: a few spans overlap their parents illegally, negative durations exist in a handful, and one trace is a single orphan span. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
