# Tier-A Case 083 — Token Bucket Sim (HEAVY)

What i want is run a proper multi-stage data job, set up inside /tmp/opencode/bucket-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

background, and i'm giving you the details the way they actually occur: a token bucket rate limiter, capacity 100, refill 10 per second, after that a request trace of 5000 requests with timestamps and weights, plus the walk: accept, reject, or partially serve, with bucket state every step, after that two bursts in the trace should stress the bucket differently. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

next deterministic step-by-step simulation, bucket state after each event, integer arithmetic only. with the method visible, not just the result, the how is the deliverable here as much as the what. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. acceptance stats overall and per minute, rejection spikes located. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
the two bursts analyzed, bucket drain and refill dynamics around each. edge cases belong in the output, not in your head, list what you hit and what you did with each.
first what-if: capacity 150 rerun, same trace, delta in acceptance rate. with the method visible, not just the result, the how is the deliverable here as much as the what.
then bucket.md plus trace-out.jsonl plus stats.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: three requests have weight zero, one timestamp is out of order by a second and policy for it must be chosen and documented, and weights include fractional millis that must be rounded consistently. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
