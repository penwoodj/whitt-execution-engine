# Tier-A Case 058 — Meeting Notes Consolidation (SYNTH)

I want you to construct a self-contained reporting exercise for me, the whole thing lives under /tmp/opencode/notes-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the world of this little exercise: a month of meeting notes, 18 meetings, as loose markdown files, then each has attendees, scattered action items, and decisions half-made, after that the deliverable: one status doc of decisions, action items with owners and dates, and open questions, and the same decision was revisited three times and walked back once. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

concretely, do these stages, numbered so you don't creative-order them on me:

next extract all decisions with meeting, date, and final status, standing, revisited, or walked-back. and be precise about what counts as done for that one, because vague is where shortcuts hide. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. action item register, owner, due, status inferred from later meetings, done or outstanding. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. the walk-back chain for the flip-flopped decision, quoted chronologically. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
open questions list, raised, never resolved, age in days. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. status.md plus actions.csv plus decisions.json. with the method visible, not just the result, the how is the deliverable here as much as the what. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

the data has problems, deliberately, and handling them is part of the job not an error condition: two meetings have identical titles a week apart, one file is raw paste with no headers at all, and an action item owner is referred to only by first initial. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
