# Tier-A Case 027 — Env Var Drift (SYNTH)

So construct a self-contained reporting exercise, fully self contained in /tmp/opencode/envdrift-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

the shape of the data: four environments, dev, staging, prod, and a forgotten sandbox, plus each has an env file, plus a documented expected matrix, and prod has a variable nobody documented and staging is missing two; also one value differs only by trailing slash and it caused an outage once. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

then matrix of every variable across environments, present, absent, values. with the method visible, not just the result, the how is the deliverable here as much as the what. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
diff against the documented expectation, undocumented, missing, wrong. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
stage by stage: normalize whitespace and quoting before comparing so only real differences show. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. the trailing-slash class of difference, list every instance, it is dangerous. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. envdrift.md plus envdrift.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. with the method visible, not just the result, the how is the deliverable here as much as the what.

the data has problems, deliberately, and handling them is part of the job not an error condition: quotes are inconsistent across files, one file has duplicate definitions of the same var where the last should win, and blank lines vary. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
