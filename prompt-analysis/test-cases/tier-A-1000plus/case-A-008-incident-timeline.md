# Tier-A Case 008 — Incident Timeline (HEAVY)

So put together an end-to-end processing exercise, fully self contained in /tmp/opencode/timeline-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

the setup is this: an incident that lasted 90 minutes, logs from six services; also each service log line has a timestamp, level, and message, plus the timeline has to be reconstructed from evidence, not narrative, plus key moments: detection, mitigation attempt one which failed, mitigation two which worked. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. merge all six logs into one chronological stream with the service name attached to each line. edge cases belong in the output, not in your head, list what you hit and what you did with each. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: detect the detection moment, both mitigation attempts, and resolution, with the exact log lines as proof. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. five-minute bucket activity chart showing where the action was. edge cases belong in the output, not in your head, list what you hit and what you did with each.
stage by stage: a clean timeline table, clock time, event, evidence line reference. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
after that timeline.md plus timeline.json where every entry points at a source line number. with the method visible, not just the result, the how is the deliverable here as much as the what. edge cases belong in the output, not in your head, list what you hit and what you did with each.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the data has problems, deliberately, and handling them is part of the job not an error condition: two services write timestamps in UTC and four in local time offset, and there is a two-minute gap in one log. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
