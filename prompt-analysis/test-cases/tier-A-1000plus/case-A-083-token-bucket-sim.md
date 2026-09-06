# Tier-A Case 083 — Token Bucket Sim (HEAVY)

the task is token bucket sim, and the details are below, all of them. what i want is run a proper multi-stage data job, set up inside /tmp/opencode/bucket-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

background, and i'm giving you the details the way they actually occur: a token bucket rate limiter, capacity 100, refill 10 per second, plus a request trace of 5000 requests with timestamps and weights, after that the walk: accept, reject, or partially serve, with bucket state every step, and two bursts in the trace should stress the bucket differently. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

what i need done, in this order:

1. deterministic step-by-step simulation, bucket state after each event, integer arithmetic only. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
2. acceptance stats overall and per minute, rejection spikes located. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the two bursts analyzed, bucket drain and refill dynamics around each. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then what-if: capacity 150 rerun, same trace, delta in acceptance rate. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: bucket.md plus trace-out.jsonl plus stats.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the realistic part, the warts: three requests have weight zero, one timestamp is out of order by a second and policy for it must be chosen and documented, and weights include fractional millis that must be rounded consistently. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the decisions file gets one line per mess type saying how you dealt with it.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
