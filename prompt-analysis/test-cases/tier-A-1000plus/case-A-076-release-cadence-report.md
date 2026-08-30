# Tier-A Case 076 — Release Cadence Report (SYNTH)

What i want is assemble a compact analysis-and-audit job, set up inside /tmp/opencode/cadence-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

here's the situation: a tag dump, 40 releases over two years, annotated dates, after that cadence questions: gap statistics, the slowdown, and the burst, plus two tags are hotfixes out-of-band and should be marked, not blended into cadence, then scope proxy: commits-per-release as a rough size signal. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. release list with dates, gap-in-days to previous, hotfix flags. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
2. gap statistics, mean, median, max, min, and the trend over time windows. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
next the slowdown located, when it started, before and after gap medians. with the method visible, not just the result, the how is the deliverable here as much as the what.
first burst detection, gaps under a quarter of median clustered together. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. cadence.md plus cadence.json, plus a two-sentence exec summary at top. edge cases belong in the output, not in your head, list what you hit and what you did with each. edge cases belong in the output, not in your head, list what you hit and what you did with each.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the realistic part, the warts: one tag points at a commit missing from the dump, two release dates are ambiguous between tag time and announcement time, and a retagged release has the same name twice. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
