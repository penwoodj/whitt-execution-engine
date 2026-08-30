# Tier-A Case 063 — Backup Rotation Sim (HEAVY)

I want you to run a proper multi-stage data job for me, the whole thing lives under /tmp/opencode/backup-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the setup is this: a backup policy simulator: daily backups, retention tiers, 7 daily, 4 weekly, 12 monthly and each simulated backup has a size and a corruption chance. restore drills happen randomly and need specific snapshots to survive; also the question: does the rotation policy survive a year of simulation with acceptable loss. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

what i need done, in this order:

after that implement the grandfather-father-son rotation exactly as specified, transitions documented. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. simulate 365 days seeded, deletions and promotions logged daily. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first corruption handling, when a retained snapshot corrupts, what the policy falls back to, count the events. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. restore-drill pass rate, how often a requested snapshot existed and was intact. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. backup.md plus sim-log.csv plus summary.json, rerun must reproduce bit-identical. with the method visible, not just the result, the how is the deliverable here as much as the what. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the data has problems, deliberately, and handling them is part of the job not an error condition: the seeded corruption rate is 1 percent per snapshot, promotion rules have an ambiguity when a weekly and monthly fall on the same day, and one drill requests a snapshot that was never eligible for retention. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
