# Tier-A Case 051 — Fuel Discrepancy (HEAVY)

new job, fuel discrepancy, same standards as always. what i want is run a proper multi-stage data job, set up inside /tmp/opencode/fuel-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: fleet fuel card transactions plus odometer readings for 20 vans over 3 months, and mpg per van per month computable from liters and kilometers and one van looks like it is siphoning fuel or the data is lying, plus fuel type mismatch, diesel van with petrol charges, is a data-quality axis too. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

here's the task list, in sequence:

1. per-van monthly liters, km, computed efficiency, flagged outliers beyond 2 sigma from fleet norm. with the method visible, not just the result, the how is the deliverable here as much as the what.
next the suspect van deep-dive, month by month, when did it drift. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. fuel-type mismatches listed, van, transaction dates, severity. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
after that odometer monotonicity check, any van where km goes backwards. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. fuel.md plus fuel.json plus suspect-drilldown.txt. with the method visible, not just the result, the how is the deliverable here as much as the what.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: odometer readings are taken irregularly, two vans swapped cards for one week and it shows, and some transactions lack liters but have a cost. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. every one of those needs a documented disposition, handled, quarantined, or rejected, with the rule cited.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
