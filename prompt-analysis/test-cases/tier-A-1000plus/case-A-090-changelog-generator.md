# Tier-A Case 090 — Changelog Generator (HEAVY)

Let's do build me a full working analysis pipeline, entirely inside /tmp/opencode/changelog-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

the world of this little exercise: a raw commit dump, 500 commits, since the last release, then the deliverable: a keepachangelog-style changelog, grouped added, changed, fixed, removed, then mapping commit to category needs message parsing plus a fallback for junk messages, and merge commits and chore noise must be filtered, with the filter auditable. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. message classification, conventional-commit prefix first, keyword fallback second, junk bucket third with contents listed. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then grouped changelog, bullet per change, human-readable, commit short-hash appended. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
the noise report, what got filtered and why, counts per filter reason. with the method visible, not just the result, the how is the deliverable here as much as the what.
after that breaking-change callouts pulled and featured at top. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that changelog.md plus classification.json plus noise-report.md. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. edge cases belong in the output, not in your head, list what you hit and what you did with each.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and bake in the dirt, i've said before i want things tested against reality not the happy path: commit messages are inconsistent, one prefix lies, a fix commit that adds a feature, the dump has two authors writing in different languages, and 40 messages are empty beyond the hash. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
