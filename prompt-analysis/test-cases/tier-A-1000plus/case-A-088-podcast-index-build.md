# Tier-A Case 088 — Podcast Index Build (LIGHT)

next up, podcast index build, whole thing specced below. let's do a focused single-purpose task with a clean report at the end, entirely inside /tmp/opencode/podcast-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

what you're working with: an rss archive dump, 300 episodes across 6 years, titles, durations, dates, shownotes blobs; also the index: chronological listing, duration totals, guest detection from notes, and topic tags, and guest means a name flagged in the notes with a pattern like with so-and-so; also one year had a renumbering that makes episode ids collide. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

here's the task list, in sequence:

1. parse and normalize, the renumbering collision resolved with a documented scheme. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. chronological index with per-year stats, counts, total hours, mean duration. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. guest roster with appearance counts, top 10, from note patterns. with the method visible, not just the result, the how is the deliverable here as much as the what.
stage by stage: topic tags from title keywords, tag cloud as a frequency table. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. index.md plus episodes.csv plus stats.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the realistic part, the warts: durations mix mm:ss and raw seconds, one episode is listed twice under different ids, and shownotes sometimes contain the string with guest inside a url which must not count. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the rule you applied to each wart belongs in the rules file, next to the wart itself.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
