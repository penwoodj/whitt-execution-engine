# Tier-A Case 056 — Ticket Routing Rules (HEAVY)

I want you to run a proper multi-stage data job for me, the whole thing lives under /tmp/opencode/routing-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the shape of the data: a historical dump of 1000 support tickets, subject, body, product, severity, final queue, then the routing rules were in someone's head and that person left, after that induce the rules from history: keyword to queue mapping with confidence; also a holdout of 200 tickets exists to score any rule set honestly. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. derive keyword rules from 800 tickets, rule, queue, support count, precision. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. apply to the 200-ticket holdout, accuracy overall and per queue, confusion pairs. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
after that the ambiguity report, queues confusable and the phrases that drive it. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first the final rule set as an ordered decision list in rules.md, first match wins. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
routing.md plus rules.md plus holdout-scores.json. and be precise about what counts as done for that one, because vague is where shortcuts hide. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the data has problems, deliberately, and handling them is part of the job not an error condition: queue names were renamed mid-history with an old-to-new mapping buried in a comment field, and 40 tickets were double-routed with two queue labels. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
