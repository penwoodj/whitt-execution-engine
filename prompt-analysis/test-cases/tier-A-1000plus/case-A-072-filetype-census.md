# Tier-A Case 072 — Filetype Census (LIGHT)

What i want is run one straightforward job and hand me the summary, set up inside /tmp/opencode/census-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the setup is this: a shared drive tree manifest, 5000 paths with sizes and the census: file types by count and bytes, extension anomalies, and the space hogs. extensions lie, executables named .txt are the classic case, magic bytes unavailable, manifest only, then a cleanup proposal must pay for itself in reclaimed space. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

extension census, count and bytes per extension, top 20 by bytes. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that anomaly flags, executable-ish sizes with text extensions, zero-byte files, dotfiles with no extension. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that the space-hog list, top 50 files overall with full paths. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. cleanup proposal with estimated reclaim per action and a total, versus cost of churn. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. census.md plus census.json plus cleanup.csv. edge cases belong in the output, not in your head, list what you hit and what you did with each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the data has problems, deliberately, and handling them is part of the job not an error condition: paths have spaces and unicode, 40 files have no extension at all, and one folder is recursively duplicated inside itself four levels deep. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
