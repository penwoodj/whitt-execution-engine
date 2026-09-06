# Tier-A Case 012 — Error Budget Burn (HEAVY)

new job, error budget burn, same standards as always. what i want is build me a full working analysis pipeline, set up inside /tmp/opencode/budget-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: three services with 30 days of uptime checks every minute and slo targets are 99.9, 99.5, and 99.0 percent respectively, after that burn rate is how fast you eat the monthly error budget, and one service already burned its whole budget by day nine. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

what i need done, in this order:

then availability per service per day and per month-to-date. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. error budget size in minutes per service, remaining budget timeline. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first burn rate per day, flag any day over 3x sustainable burn. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. the day-nine budget exhaustion, was it one bad hour or a slow grind. edge cases belong in the output, not in your head, list what you hit and what you did with each.
stage by stage: budget.md plus budget.json with every number recomputable from the check files. edge cases belong in the output, not in your head, list what you hit and what you did with each.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: some checks are missing rather than failed and must not count as downtime silently, decide and document the rule. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the decisions file gets one line per mess type saying how you dealt with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
