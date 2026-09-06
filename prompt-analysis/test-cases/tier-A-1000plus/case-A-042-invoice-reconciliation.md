# Tier-A Case 042 — Invoice Reconciliation (HEAVY)

this one is about invoice reconciliation, read it all before touching anything. i want you to put together an end-to-end processing exercise for me, the whole thing lives under /tmp/opencode/invoice-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the shape of the data: two ledgers for the same quarter, our accounts payable and their statement; also 300 invoices per side, ids, amounts, dates, statuses, then discrepancies: paid-here-unpaid-there, amount mismatches of cents, duplicate ids. cents matter, a 0.02 drift multiplied by 300 invoices is real money. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

next join both sides by invoice id, full outer, nothing unmatched silent. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. discrepancy taxonomy, amount-mismatch, status-conflict, one-sided, duplicate. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first aging of unresolved items, how old is the oldest open conflict. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. total exposure number, absolute sum of all mismatches. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. recon.md plus recon.csv of every discrepancy plus totals.json. edge cases belong in the output, not in your head, list what you hit and what you did with each.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and bake in the dirt, i've said before i want things tested against reality not the happy path: ids differ by leading zeros on one side, one ledger uses comma decimals, and three invoices appear twice on their side with different amounts. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. write down what you did with each defect and why, in the file, not the chat.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
