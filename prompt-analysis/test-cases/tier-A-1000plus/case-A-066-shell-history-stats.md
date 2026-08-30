# Tier-A Case 066 — Shell History Stats (LIGHT)

Run one straightforward job and hand me the summary, that's the job, workspace is /tmp/opencode/histstat-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

the setup is this: a shell history file, 20,000 lines, from years of use; also the stats i want: top commands, real ones not wrappers, command pairs, and the hour-of-day histogram. aliases and compound commands muddy everything, define what counts as a command and the all-time number-one command will be embarrassing, include it anyway. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

parse, define command as the first token of each line after env-var prefixes, definition written down. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. edge cases belong in the output, not in your head, list what you hit and what you did with each.
top 25 commands by count, and separately top 15 two-token commands for realism. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
after that pair analysis, command b followed within 3 lines by command c, top pairs. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. hour-of-day and day-of-week histograms as ascii bars. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
first hist.md plus hist.json, plus a one-liner on what the history says about my habits. and that one gets checked by the verifier too, it's not just a produce-and-hope step. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: multiline commands wrap with continuation markers, sudo prefixes hide the real command, and the file mixes two shells with different event formats. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
