# Tier-A Case 098 — Product Recall Trace (HEAVY)

What i want is build me a full working analysis pipeline, set up inside /tmp/opencode/recall-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the world of this little exercise: a batch genealogy: 30 production batches, ingredient lots flowing into batches into cases. one ingredient lot is contaminated, the recall must trace everything downstream, and the graph is many-to-many, lots to batches, batches to cases, with quantities, then one case was reworked from two batches, complicating purity. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

after that build the genealogy graph with quantities on edges. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. edge cases belong in the output, not in your head, list what you hit and what you did with each.
next downstream closure from the bad lot, affected batches, cases, counts, with quantities propagated. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. the reworked case handled, both parents flagged. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
first the unaffected list too, what is provably clean, with proof trail. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that recall.md plus closure.json plus clean-list.csv. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the realistic part, the warts: one batch consumes the same lot twice on different days, case quantities are approximate pounds not counts, and a lot id appears with a hyphen variant. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
