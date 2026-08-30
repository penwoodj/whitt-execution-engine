# Tier-A Case 029 — Csv Merge Conflicts (HEAVY)

Here's the job: run a proper multi-stage data job, all under /tmp/opencode/csvmerge-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

what you're working with: three csv exports of the same customer table from three systems, and columns mostly overlap but headers differ, email vs email_address and row conflicts where the same customer has different values per source, after that a trust order exists: billing beats crm beats spreadsheet. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

here's the task list, in sequence:

first normalize headers to one canonical set, mapping written in a mapping.md. with the method visible, not just the result, the how is the deliverable here as much as the what. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
merge into one table, trust order applied to conflicting values. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: conflict ledger, every cell where sources disagreed, old values and winner. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
then row counts in vs out, every dropped or merged row accounted for. with the method visible, not just the result, the how is the deliverable here as much as the what.
then merged.csv plus conflicts.csv plus ledger-summary.json. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and be precise about what counts as done for that one, because vague is where shortcuts hide.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: one file is tab-separated pretending to be comma, another has a duplicated header row mid-file, and emails have inconsistent casing. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
