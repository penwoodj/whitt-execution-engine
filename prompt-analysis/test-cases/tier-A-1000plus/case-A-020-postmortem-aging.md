# Tier-A Case 020 — Postmortem Aging (SYNTH)

today it's postmortem aging, and no, you don't get to improvise the scope. let's build a small but complete tool with its own verification, entirely inside /tmp/opencode/aging-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

what you're working with: 20 past postmortems, each with action items and owners and some action items are done, some in progress, some never started, after that anything over 60 days old and not started is dead in practice and recurring root causes across postmortems are the real signal. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

concretely, do these stages, numbered so you don't creative-order them on me:

stage by stage: per-postmortem action item status table with age in days. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. the dead list, over 60 days untouched, with owners. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. recurring root cause clusters, same theme appearing in multiple postmortems. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that completion rate over time, are we getting better or just busier. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. aging.md plus aging.json. edge cases belong in the output, not in your head, list what you hit and what you did with each.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: owners are spelled inconsistently, jon, jonathan, and j.d. are the same human, and three action items have no owner at all. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. write down what you did with each defect and why, in the file, not the chat.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
