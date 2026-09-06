# Tier-A Case 048 — Volunteer Shift Solver (HEAVY)

new job, volunteer shift solver, same standards as always. here's the job: run a proper multi-stage data job, all under /tmp/opencode/shifts-run, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.

the world of this little exercise: a festival needing 120 shift slots covered across 3 days, plus 25 volunteers with availability windows, max hours, and a few paired-buddy constraints. understaffing is possible, the output must say where and by how much and fairness matters, spread hours, nobody does all the closing shifts. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

stage by stage: coverage plan maximizing filled slots under availability and max-hour caps. edge cases belong in the output, not in your head, list what you hit and what you did with each.
first fairness metric, hours stddev across volunteers, report it. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. buddy pairs honored or explicitly broken with an apology line each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
then the gap list, unfilled slots, day, timeslot, severity by expected traffic. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. shifts.md plus shifts.csv plus gaps.json. with the method visible, not just the result, the how is the deliverable here as much as the what.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and bake in the dirt, i've said before i want things tested against reality not the happy path: availability is expressed in four different formats, two volunteers withdrew mid-planning marked only by a comment, and one timeslot was added twice under different names. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. how you handled each one gets written down where i can find it later.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
