# Tier-A Case 074 — Stale Branch Sweep (SYNTH)

I need assemble a compact analysis-and-audit job done in /tmp/opencode/branches-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

so the scenario: a git bundle dump with 80 branches across a year, plus stale means no commits in 90 days and no open pr referencing it in the dump's pr notes, plus protected branches are listed by name pattern and untouchable, after that merge-status matters: unmerged work deserves a second look before deletion. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. per-branch: last commit date, author, ahead/behind main, merge status. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. classify: protected, active, stale-merged, stale-unmerged, and the weird ones. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the deletion list, stale-merged only, with a per-branch one-line eulogy. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. the review list, stale-unmerged, what unmerged work exists and who last touched it. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
stage by stage: branches.md plus delete-list.csv plus review-list.csv. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: branch names encode dates inconsistently, three branches point at commits that exist in no other branch, and one protected pattern matches two branches by accident of naming. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
