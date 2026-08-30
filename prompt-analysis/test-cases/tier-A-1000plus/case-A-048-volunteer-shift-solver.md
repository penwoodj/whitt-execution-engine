# Tier-A Case 048 — Volunteer Shift Solver (HEAVY)

Here's the job: run a proper multi-stage data job, all under /tmp/opencode/shifts-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

the world of this little exercise: a festival needing 120 shift slots covered across 3 days. 25 volunteers with availability windows, max hours, and a few paired-buddy constraints, plus understaffing is possible, the output must say where and by how much, plus fairness matters, spread hours, nobody does all the closing shifts. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

coverage plan maximizing filled slots under availability and max-hour caps. with the method visible, not just the result, the how is the deliverable here as much as the what. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
fairness metric, hours stddev across volunteers, report it. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. buddy pairs honored or explicitly broken with an apology line each. with the method visible, not just the result, the how is the deliverable here as much as the what.
first the gap list, unfilled slots, day, timeslot, severity by expected traffic. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
next shifts.md plus shifts.csv plus gaps.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

now the mess, because a pipeline that only works on clean data is worthless to me: availability is expressed in four different formats, two volunteers withdrew mid-planning marked only by a comment, and one timeslot was added twice under different names. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
