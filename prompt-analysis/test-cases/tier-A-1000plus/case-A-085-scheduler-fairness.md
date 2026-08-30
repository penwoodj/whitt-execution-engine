# Tier-A Case 085 — Scheduler Fairness (HEAVY)

I want you to run a proper multi-stage data job for me, the whole thing lives under /tmp/opencode/sched-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

here's the situation: a job queue walk: 60 jobs, priorities high medium low, with arrival times and durations; also aging rule: waiting 10 minutes bumps priority one level, plus preemption: higher priority preempts, preempted jobs resume not restart; also the fairness question: do low jobs ever starve, and does aging save them. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. deterministic walk, per-minute state, who runs, who waits, who resumes. edge cases belong in the output, not in your head, list what you hit and what you did with each. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
then wait-time stats per priority class, the starvation evidence or its absence. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
then aging events logged, when bumps happened and what they changed. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. no-aging rerun comparison, starve count with and without. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. sched.md plus walk.csv plus fairness.json. and be precise about what counts as done for that one, because vague is where shortcuts hide. and be precise about what counts as done for that one, because vague is where shortcuts hide.

the data has problems, deliberately, and handling them is part of the job not an error condition: two jobs have identical arrival to the microsecond requiring a documented tie rule, one job has duration zero, and a mid-walk priority reassignment happens manually in the trace marked with a flag. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
