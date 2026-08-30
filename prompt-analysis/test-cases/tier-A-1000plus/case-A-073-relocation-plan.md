# Tier-A Case 073 — Relocation Plan (SYNTH)

Assemble a compact analysis-and-audit job, that's the job, workspace is /tmp/opencode/relocate-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

so the scenario: a tree with 30 large files over 100mb that must move to a cold folder, and 60 symlinks point into the tree from elsewhere, then the plan must keep every symlink working, via relocation or fixup; also two large files are actively written to per mtime evidence and must not move yet. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

concretely, do these stages, numbered so you don't creative-order them on me:

1. candidate list with sizes, mtimes, and the active-write flags with evidence. and be precise about what counts as done for that one, because vague is where shortcuts hide. and be precise about what counts as done for that one, because vague is where shortcuts hide.
next for each candidate: which symlinks point at it, relocate-and-fixup versus skip decision. with the method visible, not just the result, the how is the deliverable here as much as the what.
after that the ordered runbook, commands in sequence, with a rollback note per step. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. space accounting, freed now, freed after the active files age out. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. relocate.md plus plan.csv plus symlink-map.json. and be precise about what counts as done for that one, because vague is where shortcuts hide. with the method visible, not just the result, the how is the deliverable here as much as the what.

and bake in the dirt, i've said before i want things tested against reality not the happy path: one symlink is relative and breaks under a different interpretation, a symlink points at another symlink in a chain, and the cold folder already contains a file with a colliding name. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
