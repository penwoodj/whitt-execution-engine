# Tier-A Case 072 — Filetype Census (LIGHT)

the task is filetype census, and the details are below, all of them. what i want is run one straightforward job and hand me the summary, set up inside /tmp/opencode/census-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

here's the situation: a shared drive tree manifest, 5000 paths with sizes. the census: file types by count and bytes, extension anomalies, and the space hogs, then extensions lie, executables named .txt are the classic case, magic bytes unavailable, manifest only. a cleanup proposal must pay for itself in reclaimed space. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. extension census, count and bytes per extension, top 20 by bytes. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
after that anomaly flags, executable-ish sizes with text extensions, zero-byte files, dotfiles with no extension. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that the space-hog list, top 50 files overall with full paths. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. cleanup proposal with estimated reclaim per action and a total, versus cost of churn. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. census.md plus census.json plus cleanup.csv. edge cases belong in the output, not in your head, list what you hit and what you did with each.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

now the mess, because a pipeline that only works on clean data is worthless to me: paths have spaces and unicode, 40 files have no extension at all, and one folder is recursively duplicated inside itself four levels deep. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
