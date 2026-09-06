# Tier-A Case 071 — Checksum Tamper (SYNTH)

the subject this time is checksum tamper, and i want it done like i mean it. construct a self-contained reporting exercise, that's the job, workspace is /tmp/opencode/tamper-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

context so you're not guessing: a manifest of 500 files with sha256 checksums, taken at snapshot time. the current tree differs from the manifest in specific planted ways, and tamper means hash mismatch, but also: new files, missing files, and case-only renames; also one file legitimately changed and is documented as such in a changelog. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

what i need done, in this order:

next verify every manifest entry, match, mismatch, missing, plus unlisted new files. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. the documented legit change reconciled against the changelog, not flagged as tamper. and be precise about what counts as done for that one, because vague is where shortcuts hide.
then case-only renames detected, same hash different path case. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first tamper report ranked by plausibility of malice versus accident, with reasoning. and be precise about what counts as done for that one, because vague is where shortcuts hide.
after that tamper.md plus tamper.json plus the reconciled changelog cross-ref. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the data has problems, deliberately, and handling them is part of the job not an error condition: two files swapped contents but kept names, one manifest line has a truncated hash from a wrapping bug, and the changelog entry is vague about which file it means. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. no silent absorption, each case logged with the rule that resolved it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
