# Tier-A Case 024 — Capacity Forecast (HEAVY)

new job, capacity forecast, same standards as always. so put together an end-to-end processing exercise, fully self contained in /tmp/opencode/forecast-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

context so you're not guessing: twelve months of monthly traffic per service, six services; also growth is roughly linear for four services, seasonal for two and forecast next six months with honest error bars, plus a hardware purchase decision hangs on the q3 projection. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. fit a linear trend per service, slope, intercept, fit quality. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. for the two seasonal ones, detect the season period and amplitude. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. six-month projections with error bars from fit residuals. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
first the q3 answer, when each service crosses its current capacity ceiling. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. forecast.md plus forecast.json, method documented, no black boxes. with the method visible, not just the result, the how is the deliverable here as much as the what.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

now the mess, because a pipeline that only works on clean data is worthless to me: one month is missing entirely for two services and december has an obvious outlier spike for everyone. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. no silent absorption, each case logged with the rule that resolved it.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
