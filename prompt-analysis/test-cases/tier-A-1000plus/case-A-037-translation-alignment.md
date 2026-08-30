# Tier-A Case 037 — Translation Alignment (HEAVY)

Let's do build me a full working analysis pipeline, entirely inside /tmp/opencode/align-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

the setup is this: a translated document pair, english source and german target, as paragraph-per-line txt and paragraph counts differ because translators split and merge, then alignment means matching each source paragraph to its translation and a few paragraphs were skipped by the translator entirely. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. align by similarity of position plus numbers and proper nouns that survive translation. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first output aligned pairs with confidence per pair. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the skipped paragraphs, present in source, absent in target, quoted. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. merge-split cases where two sources map to one target, handled explicitly. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. align.md method doc plus aligned.json plus skipped.txt. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the data has problems, deliberately, and handling them is part of the job not an error condition: numbers are localized, 1,000 becomes 1.000, one paragraph appears twice in the target, and the files have different encodings declared. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
