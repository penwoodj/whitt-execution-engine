# Tier-A Case 070 — Quarantine Workflow (SYNTH)

I need build a small but complete tool with its own verification done in /tmp/opencode/quarantine-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

background, and i'm giving you the details the way they actually occur: an uploads folder, 200 files of mixed types, some untrusted, and policy: executables and archives-with-executables quarantine, documents pass to scan, unknown extensions quarantine. archive inspection must recurse, an archive inside an archive is a real case here, plus the quarantine manifest needs a review queue ordered by risk. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

what i need done, in this order:

1. classify every file, type by extension and magic-prefix both, mismatches flagged. with the method visible, not just the result, the how is the deliverable here as much as the what. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. recursive archive inspection with depth guard, contents inventoried. with the method visible, not just the result, the how is the deliverable here as much as the what.
first apply the policy, move nothing, output the disposition plan, pass, quarantine, review. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. the review queue ordered by a stated risk score, executables-in-double-archive on top. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. quarantine.md plus disposition.csv plus queue.json. with the method visible, not just the result, the how is the deliverable here as much as the what. and be precise about what counts as done for that one, because vague is where shortcuts hide.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: one archive is self-referential via a symlink trick, three files have double extensions like pdf.exe, and a document type is unknown to the magic table requiring a documented fallback. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
