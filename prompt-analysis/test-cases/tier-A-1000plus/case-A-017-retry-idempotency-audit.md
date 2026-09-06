# Tier-A Case 017 — Retry Idempotency Audit (SYNTH)

another one for the pile: retry idempotency audit, with the usual rules. i want you to construct a self-contained reporting exercise for me, the whole thing lives under /tmp/opencode/idem-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

background, and i'm giving you the details the way they actually occur: an order system where clients retry with idempotency keys and 10,000 charge attempts across two weeks of records; also a key collision means two different orders sharing one key, very bad, and most retries are healthy, same key, same order, duplicate suppressed. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

then group by key, healthy retries vs suspicious collisions, same key different order payloads. with the method visible, not just the result, the how is the deliverable here as much as the what.
then count suppressed duplicates, money not double-charged. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
stage by stage: flag every collision for human review with both payloads side by side. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. timeline of collision frequency, is it getting worse. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: idem.md plus idem.json plus a flagged-collisions.csv. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and bake in the dirt, i've said before i want things tested against reality not the happy path: three keys appear with five different order payloads and one order payload appears under two different keys. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. how you handled each one gets written down where i can find it later.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
