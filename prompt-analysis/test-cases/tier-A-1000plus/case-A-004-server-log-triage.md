# Tier-A Case 004 — Server Log Triage (HEAVY)

next up, server log triage, whole thing specced below. let's build me a full working analysis pipeline, entirely inside /tmp/opencode/logtriage-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

the setup is this: three web nodes called web-a, web-b, web-c, then nginx access and error logs for one full week, rotated daily, then status codes 200, 301, 429, 500, and 503 all in play, and peak traffic window is 14:00 to 17:00 every day, and one node restarts every night around 03:00 and you can see it in the logs. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

here's the task list, in sequence:

1. parse every log file into one normalized events file with proper ISO timestamps, one event per line. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next error-rate per hour per node as a table, percentages not raw counts. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
then top ten offender endpoints by 5xx count with the count next to each. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. the 03:00 restart blast radius, which in-flight requests died and how you know from the logs alone. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. a markdown report a human reads plus a machine json twin with the same numbers. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the realistic part, the warts: some lines are malformed with mixed quoting, a couple of entries are clock-skewed an hour off, and one rotated file is empty. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. each mess instance gets its handling logged, rule named, no exceptions.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
