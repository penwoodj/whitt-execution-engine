# Tier-A Case 068 — Secrets Pattern Scan (HEAVY)

Build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/secrets-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

the shape of the data: a fake repo dump, 3000 files, deliberately planted secrets of known shapes and planted: aws-style keys, private key blocks, connection strings with passwords, and generic high-entropy strings and severity tiers by exploitability, not by pattern match; also the same secret committed then deleted still lives in history-simulation files included in the dump. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

here's the task list, in sequence:

1. scan with a documented pattern set, every hit with file, line, match excerpt, tier. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: entropy analysis for generic candidates, score and threshold stated. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the deleted-but-present class, secrets whose only live copy is in history-simulation files, flagged distinctly. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
dedupe identical secrets across files, one finding, many locations. and be precise about what counts as done for that one, because vague is where shortcuts hide.
after that secrets.md findings report plus findings.json, counts by tier and by type. and be precise about what counts as done for that one, because vague is where shortcuts hide. and be precise about what counts as done for that one, because vague is where shortcuts hide.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the data has problems, deliberately, and handling them is part of the job not an error condition: three planted secrets are split across two lines to defeat naive matching, one file is a test fixture containing obviously fake keys that should be tiered lowest, and a readme documents one secret intentionally. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
