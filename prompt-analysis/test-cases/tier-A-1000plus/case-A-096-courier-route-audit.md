# Tier-A Case 096 — Courier Route Audit (HEAVY)

Build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/courier-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

what you're working with: a day of courier stops, 60 deliveries with addresses as grid coordinates and time windows, plus the route as driven versus a recomputed better route, plus distance metric is manhattan grid distance, state it, and three deliveries missed their windows, that is the failure to explain. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

what i need done, in this order:

1. as-driven total distance and window compliance, the three misses with their windows and arrival times. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. recomputed route, nearest-neighbor then 2-opt improve, both distances shown. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. compliance recheck under the new route, misses resolved or not. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: the trade-off analysis, distance saved versus window risk, with a recommendation. edge cases belong in the output, not in your head, list what you hit and what you did with each.
courier.md plus routes.json plus compliance.csv. with the method visible, not just the result, the how is the deliverable here as much as the what. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: coordinates include two identical points for different deliveries, one time window is wider than the working day, and the depot appears twice under different names. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
