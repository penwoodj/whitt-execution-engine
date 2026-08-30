# Tier-A Case 015 — Access Log Security Sweep (HEAVY)

What i want is put together an end-to-end processing exercise, set up inside /tmp/opencode/sec-sweep-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: access logs from a public-facing box, two weeks, then patterns of interest: failed logins, path scanning, one ip hammering /admin. legitimate traffic exists and must not be flagged, after that no real ips or secrets, everything is synthetic fixture data. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

here's the task list, in sequence:

1. failed-login frequency by ip, top twenty with counts. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. path scanning detection, many distinct 404 paths from one source. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that the /admin hammer, when it started, stopped, request rate. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. a whitelist pass so obviously-legit crawlers do not pollute the report. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: sweep.md plus sweep.json, with the detection rules documented. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the realistic part, the warts: some ips are ipv6, a few log lines are truncated mid-field, and timestamps drift by seconds across files. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
