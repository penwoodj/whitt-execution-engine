# Tier-A Case 073 — Relocation Plan (SYNTH)

relocation plan, that's the theme, here's the full spec: assemble a compact analysis-and-audit job, that's the job, workspace is /tmp/opencode/relocate-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

so the scenario: a tree with 30 large files over 100mb that must move to a cold folder, and 60 symlinks point into the tree from elsewhere, then the plan must keep every symlink working, via relocation or fixup; also two large files are actively written to per mtime evidence and must not move yet. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. candidate list with sizes, mtimes, and the active-write flags with evidence. and be precise about what counts as done for that one, because vague is where shortcuts hide.
next for each candidate: which symlinks point at it, relocate-and-fixup versus skip decision. with the method visible, not just the result, the how is the deliverable here as much as the what.
after that the ordered runbook, commands in sequence, with a rollback note per step. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. space accounting, freed now, freed after the active files age out. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. relocate.md plus plan.csv plus symlink-map.json. and be precise about what counts as done for that one, because vague is where shortcuts hide.

the data has problems, deliberately, and handling them is part of the job not an error condition: one symlink is relative and breaks under a different interpretation, a symlink points at another symlink in a chain, and the cold folder already contains a file with a colliding name. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. every one of those needs a documented disposition, handled, quarantined, or rejected, with the rule cited.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
