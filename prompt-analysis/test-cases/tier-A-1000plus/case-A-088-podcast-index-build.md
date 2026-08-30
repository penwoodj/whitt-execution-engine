# Tier-A Case 088 — Podcast Index Build (LIGHT)

Let's do do a focused single-purpose task with a clean report at the end, entirely inside /tmp/opencode/podcast-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

background, and i'm giving you the details the way they actually occur: an rss archive dump, 300 episodes across 6 years, titles, durations, dates, shownotes blobs, plus the index: chronological listing, duration totals, guest detection from notes, and topic tags; also guest means a name flagged in the notes with a pattern like with so-and-so, and one year had a renumbering that makes episode ids collide. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

parse and normalize, the renumbering collision resolved with a documented scheme. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. chronological index with per-year stats, counts, total hours, mean duration. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. guest roster with appearance counts, top 10, from note patterns. with the method visible, not just the result, the how is the deliverable here as much as the what.
stage by stage: topic tags from title keywords, tag cloud as a frequency table. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. index.md plus episodes.csv plus stats.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

now the mess, because a pipeline that only works on clean data is worthless to me: durations mix mm:ss and raw seconds, one episode is listed twice under different ids, and shownotes sometimes contain the string with guest inside a url which must not count. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
