# Tier-A Case 087 — Book Queue Recommender (SYNTH)

Let's do construct a self-contained reporting exercise, entirely inside /tmp/opencode/books-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

so the scenario: a reading queue of 40 books with pages, topics, priority, and mood tags, plus constraints: alternate fiction and nonfiction, max 600 pages consecutive, finish the started ones first, after that a 12-book next-up plan is the deliverable, constraint-checked, plus two books are half-finished and count as started, with progress fractions. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

here's the task list, in sequence:

after that constraint checker written first, every rule mechanical, then a plan that passes. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
the 12-book sequence with alternation, page-limit, and started-first all verified. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
3. per-book one-line reason for its slot, topic diversity invoked where it matters. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first the trade-off note, what got postponed and the stated logic. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. queue.md plus plan.json plus check-output.txt. edge cases belong in the output, not in your head, list what you hit and what you did with each. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the data has problems, deliberately, and handling them is part of the job not an error condition: one book has zero pages listed, mood tags overlap heavily making diversity fuzzy, and a series must be read in order which adds an internal-ordering constraint. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
