# Tier-A Case 059 — Email Thread Decisions (SYNTH)

Build a small but complete tool with its own verification, that's the job, workspace is /tmp/opencode/thread-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

context so you're not guessing: one long email thread exported as text, 60 messages, three weeks. a decision got made somewhere in the middle and unmade near the end, plus participants drift in and out, quoting styles vary wildly; also the extraction target: who decided what when, with message numbers as evidence. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

here's the task list, in sequence:

next message index, number, from, date, one-line gist. and that one gets checked by the verifier too, it's not just a produce-and-hope step. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first decision timeline, proposals, counter-proposals, the landing point, with message refs. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. the unmaking, which message reversed it and who objected. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. participants roster with message counts and influence note, who drove the outcome. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. thread.md plus timeline.json plus roster.csv. edge cases belong in the output, not in your head, list what you hit and what you did with each. with the method visible, not just the result, the how is the deliverable here as much as the what.

the realistic part, the warts: quoting makes attribution genuinely hard, three messages are forwards containing nested older threads, and one message has no author header at all. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
