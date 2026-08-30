# Tier-A Case 032 — Markdown Link Rot (SYNTH)

I need construct a self-contained reporting exercise done in /tmp/opencode/linkrot-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

the setup is this: a docs folder, 60 markdown files, cross-referencing each other and assets and links are relative, some point at moved or deleted files, after that anchors matter too, links to headings that no longer exist, and external http links exist but are out of scope, offline check only. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

concretely, do these stages, numbered so you don't creative-order them on me:

1. extract every link with source file, target, and anchor. edge cases belong in the output, not in your head, list what you hit and what you did with each. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
2. resolve relative targets against the actual tree, mark live, dead, moved-guessable. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. anchor check, does the target heading exist in the target file. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. the moved-guessable ones, propose the correct path by filename search. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
next linkrot.md plus linkrot.json plus a fix-list.csv ready to apply. edge cases belong in the output, not in your head, list what you hit and what you did with each. with the method visible, not just the result, the how is the deliverable here as much as the what.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and bake in the dirt, i've said before i want things tested against reality not the happy path: three files have identical names in different folders, one link contains a space unencoded, and one file is a symlink loop. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
