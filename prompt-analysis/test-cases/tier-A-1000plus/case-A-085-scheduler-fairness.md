# Tier-A Case 085 — Scheduler Fairness (HEAVY)

new job, scheduler fairness, same standards as always. i want you to run a proper multi-stage data job for me, the whole thing lives under /tmp/opencode/sched-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the world of this little exercise: a job queue walk: 60 jobs, priorities high medium low, with arrival times and durations; also aging rule: waiting 10 minutes bumps priority one level, plus preemption: higher priority preempts, preempted jobs resume not restart, and the fairness question: do low jobs ever starve, and does aging save them. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

next deterministic walk, per-minute state, who runs, who waits, who resumes. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: wait-time stats per priority class, the starvation evidence or its absence. and be precise about what counts as done for that one, because vague is where shortcuts hide.
first aging events logged, when bumps happened and what they changed. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. no-aging rerun comparison, starve count with and without. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: sched.md plus walk.csv plus fairness.json. and be precise about what counts as done for that one, because vague is where shortcuts hide.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the data has problems, deliberately, and handling them is part of the job not an error condition: two jobs have identical arrival to the microsecond requiring a documented tie rule, one job has duration zero, and a mid-walk priority reassignment happens manually in the trace marked with a flag. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. write down what you did with each defect and why, in the file, not the chat.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
