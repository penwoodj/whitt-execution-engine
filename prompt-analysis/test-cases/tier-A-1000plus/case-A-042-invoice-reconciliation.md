# Tier-A Case 042 — Invoice Reconciliation (HEAVY)

I want you to put together an end-to-end processing exercise for me, the whole thing lives under /tmp/opencode/invoice-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

so the scenario: two ledgers for the same quarter, our accounts payable and their statement. 300 invoices per side, ids, amounts, dates, statuses, then discrepancies: paid-here-unpaid-there, amount mismatches of cents, duplicate ids; also cents matter, a 0.02 drift multiplied by 300 invoices is real money. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. join both sides by invoice id, full outer, nothing unmatched silent. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. with the method visible, not just the result, the how is the deliverable here as much as the what.
next discrepancy taxonomy, amount-mismatch, status-conflict, one-sided, duplicate. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. aging of unresolved items, how old is the oldest open conflict. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first total exposure number, absolute sum of all mismatches. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. recon.md plus recon.csv of every discrepancy plus totals.json. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. with the method visible, not just the result, the how is the deliverable here as much as the what.

and bake in the dirt, i've said before i want things tested against reality not the happy path: ids differ by leading zeros on one side, one ledger uses comma decimals, and three invoices appear twice on their side with different amounts. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
