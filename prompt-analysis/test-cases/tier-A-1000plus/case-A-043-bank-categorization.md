# Tier-A Case 043 — Bank Categorization (HEAVY)

So build me a full working analysis pipeline, fully self contained in /tmp/opencode/bankcat-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

the setup is this: a year of bank statement rows, 1200 transactions, description, date, amount, after that no categories exist, the task is building them, then rule-based first, merchant names hide inside noisy descriptions, after that groceries vs dining boundary cases will exist, acknowledge them. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

what i need done, in this order:

derive a category set from the data itself, 8 to 14 categories, defend the choice. edge cases belong in the output, not in your head, list what you hit and what you did with each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. rule set in rules.md, regex or matching logic per category, applied deterministically. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. categorize everything, unknown bucket allowed but under 5 percent or the rules are bad. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. monthly spend per category table plus the top 5 merchants overall. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. categorized.csv plus categories.md plus monthly.json. and be precise about what counts as done for that one, because vague is where shortcuts hide. with the method visible, not just the result, the how is the deliverable here as much as the what.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

now the mess, because a pipeline that only works on clean data is worthless to me: descriptions contain store numbers, dates, and reference codes mashed together, one merchant appears under four spellings, and twelve rows have a zero amount. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
