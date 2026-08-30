# Tier-A Case 084 — Circuit Breaker Walk (HEAVY)

I want you to put together an end-to-end processing exercise for me, the whole thing lives under /tmp/opencode/breaker-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the setup is this: a circuit breaker: closed, open, half-open, thresholds 5 failures to trip, 30 second cool-down, plus an outcome trace of 2000 calls, success, failure, with timestamps, then the walk must handle half-open probe semantics precisely, one probe at a time, after that a flapping period mid-trace will stress the state machine. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

first exact state machine walk, transition log with timestamps and causes. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that state-time summary, time in each state, trip count, flap count. edge cases belong in the output, not in your head, list what you hit and what you did with each.
the flapping period dissected, what the breaker did and whether policy helped or hurt. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. what-if thresholds: 3 and 8 failure reruns, trip counts compared. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. breaker.md plus transitions.csv plus summary.json. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

now the mess, because a pipeline that only works on clean data is worthless to me: one failure outcome arrives during cool-down ambiguity, the trace has one duplicate call id, and half-open probes must serialize meaning the walk pauses other calls, define and document. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
