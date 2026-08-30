# Tier-A Case 065 — Dotfile Diff Report (LIGHT)

Let's do do a focused single-purpose task with a clean report at the end, entirely inside /tmp/opencode/dotfiles-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

the setup is this: my dotfile folder snapshot from this machine and from the laptop, as two trees, then the report: what differs, what is machine-specific and should stay that way, what drifted and should be reconciled, then known machine-specific: anything with a hostname in it; also i will hand-merge, the deliverable is the report, not the merge. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

what i need done, in this order:

after that full recursive diff of the two trees, file-level, present-here-absent-there and differing. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. classify each difference, machine-specific by content marker, config drift, or junk like caches and locks. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
3. drift items with both versions excerpted side by side, trimmed intelligently. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that the reconcile shortlist, ordered by how much it probably matters. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. dotdiff.md plus drift.csv, no modifications to either tree. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the realistic part, the warts: cache and state files pollute both trees heavily and must be auto-classified as junk by pattern, one file is binary but named like config, and two configs differ only by trailing newline. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
