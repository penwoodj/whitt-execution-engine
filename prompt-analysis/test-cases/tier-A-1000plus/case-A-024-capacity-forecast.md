# Tier-A Case 024 — Capacity Forecast (HEAVY)

So put together an end-to-end processing exercise, fully self contained in /tmp/opencode/forecast-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

the world of this little exercise: twelve months of monthly traffic per service, six services, then growth is roughly linear for four services, seasonal for two; also forecast next six months with honest error bars and a hardware purchase decision hangs on the q3 projection. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. fit a linear trend per service, slope, intercept, fit quality. and be precise about what counts as done for that one, because vague is where shortcuts hide. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
then for the two seasonal ones, detect the season period and amplitude. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. six-month projections with error bars from fit residuals. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
first the q3 answer, when each service crosses its current capacity ceiling. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. forecast.md plus forecast.json, method documented, no black boxes. with the method visible, not just the result, the how is the deliverable here as much as the what. edge cases belong in the output, not in your head, list what you hit and what you did with each.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the realistic part, the warts: one month is missing entirely for two services and december has an obvious outlier spike for everyone. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
