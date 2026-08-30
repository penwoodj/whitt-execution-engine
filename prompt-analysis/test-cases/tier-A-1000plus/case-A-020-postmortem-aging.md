# Tier-A Case 020 — Postmortem Aging (SYNTH)

Let's do build a small but complete tool with its own verification, entirely inside /tmp/opencode/aging-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

the world of this little exercise: 20 past postmortems, each with action items and owners, plus some action items are done, some in progress, some never started and anything over 60 days old and not started is dead in practice, after that recurring root causes across postmortems are the real signal. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

concretely, do these stages, numbered so you don't creative-order them on me:

1. per-postmortem action item status table with age in days. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: the dead list, over 60 days untouched, with owners. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. recurring root cause clusters, same theme appearing in multiple postmortems. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. completion rate over time, are we getting better or just busier. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that aging.md plus aging.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: owners are spelled inconsistently, jon, jonathan, and j.d. are the same human, and three action items have no owner at all. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
