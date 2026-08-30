# Tier-A Case 022 — Golden Signal Dashboard Data (SYNTH)

I want you to build a small but complete tool with its own verification for me, the whole thing lives under /tmp/opencode/golden-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

so the scenario: four golden signals, latency, traffic, errors, saturation, for five services. one week of per-minute metrics in csv files per signal and dashboard data must be pre-aggregated so the dashboard loads fast and the raw data is 700k rows across the files. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. aggregate to five-minute windows, mean latency, max latency, traffic sum, error rate, saturation peak. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
first keep a raw-count column so aggregations are auditable. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. sanity checks, no window with more errors than traffic, no negative anything. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. compression report, rows before and after, and why the dashboard needs this. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. golden.md plus the aggregated csv files plus checks.json. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

the realistic part, the warts: one csv has a duplicated five-minute block, and saturation for one service is recorded as a fraction while others are percentages. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
