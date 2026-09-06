# Tier-A Case 009 — Trace Waterfall (HEAVY)

new job, trace waterfall, same standards as always. build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/trace-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

so the scenario: distributed traces, 300 of them, each a json of nested spans, and spans have name, start microsecond, duration, and status and the slow span is somewhere in the middle of the call tree, then every trace starts with a root span called serve.request. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

concretely, do these stages, numbered so you don't creative-order them on me:

1. compute per-span-name average and p99 self time excluding children. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. find the three deepest traces by total duration and lay out their span waterfalls as indented lists. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the slowest span name overall and what percentage of total time it burns. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
first status breakdown per span name, how often each fails. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. waterfall.md plus waterfall.json, verifier recomputes from the raw trace files. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the realistic part, the warts: a few spans overlap their parents illegally, negative durations exist in a handful, and one trace is a single orphan span. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. how you handled each one gets written down where i can find it later.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
