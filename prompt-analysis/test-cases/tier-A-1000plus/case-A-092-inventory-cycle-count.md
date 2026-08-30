# Tier-A Case 092 — Inventory Cycle Count (HEAVY)

I want you to run a proper multi-stage data job for me, the whole thing lives under /tmp/opencode/cyclecount-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the shape of the data: a warehouse snapshot plus cycle-count sheets from four physical counts, plus each sheet covers a zone, lists sku and counted quantity, with counter initials, after that discrepancies against system stock, plus counter-error patterns by counter, plus one counter consistently miscounts by one carton, that is the lore, verify it. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

after that join counts to system stock per sku per zone, variance absolute and signed. edge cases belong in the output, not in your head, list what you hit and what you did with each. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. discrepancy rate per zone, per counter, the repeat-offender analysis on the suspect counter. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: shrinkage estimate in units and value using the price file. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next the recount list, high-variance skus worth a second look, ranked. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
stage by stage: cycle.md plus variances.csv plus counters.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: sku formats differ by leading zeros, one zone was counted twice on different days with drift between, and 30 skus in the sheets do not exist in the system snapshot at all. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
