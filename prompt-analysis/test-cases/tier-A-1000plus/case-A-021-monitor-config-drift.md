# Tier-A Case 021 — Monitor Config Drift (SYNTH)

I've got assemble a compact analysis-and-audit job to do and i want it done properly, which means /tmp/opencode/mondrift-run for everything, real scripts i can rerun, real files on disk, not a description of what would happen.

the setup is this: monitoring config as it exists now vs the golden baseline from three months ago; also yaml files for 40 monitors, thresholds, alert rules, owners. drift includes added monitors, removed ones, and threshold tweaks. some drift is legitimate tuning, some is fat-fingering. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

what i need done, in this order:

next three-way diff summary, added, removed, changed, with counts and specifics. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.
first threshold changes table, old value, new value, monitor, and a sanity verdict. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
first monitors with no owner in the current set. with the method visible, not just the result, the how is the deliverable here as much as the what.
a drift-score per category so this can be tracked month over month. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. drift.md plus drift.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the data has problems, deliberately, and handling them is part of the job not an error condition: one file is valid yaml but wrong schema, keys exist that the baseline format does not define, handle it explicitly. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
