# Tier-A Case 041 — Directory Snapshot Delta (HEAVY)

directory snapshot delta is the job, and i want the work to show for it. let's build me a full working analysis pipeline, entirely inside /tmp/opencode/dirsnap-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

here's the situation: two directory snapshots taken a month apart, as manifest files with path, size, mtime, hash and the delta report is what changed: new, gone, grown, shrunk, rewritten, after that mtime alone lies, some files were touched without content change; also hash is truth where present, four files lack hashes. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

what i need done, in this order:

1. five-way delta, added, removed, grown, shrunk, same-size-rewritten, using hash over mtime where available. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. mtime-liar list, mtime changed but hash identical. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
3. the four hashless files handled by size-and-mtime heuristic, flagged low-confidence. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. totals that add up, every file in either snapshot lands in exactly one bucket. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. delta.md plus delta.json, conservation check mandatory. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

now the mess, because a pipeline that only works on clean data is worthless to me: paths moved rather than removed-and-added, detect three renames by hash matching across snapshots, and one path exists in both with same hash but different case. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
