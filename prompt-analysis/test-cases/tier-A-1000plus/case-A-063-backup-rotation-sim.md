# Tier-A Case 063 — Backup Rotation Sim (HEAVY)

the task is backup rotation sim, and the details are below, all of them. i want you to run a proper multi-stage data job for me, the whole thing lives under /tmp/opencode/backup-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the setup is this: a backup policy simulator: daily backups, retention tiers, 7 daily, 4 weekly, 12 monthly. each simulated backup has a size and a corruption chance; also restore drills happen randomly and need specific snapshots to survive. the question: does the rotation policy survive a year of simulation with acceptable loss. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

after that implement the grandfather-father-son rotation exactly as specified, transitions documented. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. simulate 365 days seeded, deletions and promotions logged daily. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first corruption handling, when a retained snapshot corrupts, what the policy falls back to, count the events. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. restore-drill pass rate, how often a requested snapshot existed and was intact. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. backup.md plus sim-log.csv plus summary.json, rerun must reproduce bit-identical. with the method visible, not just the result, the how is the deliverable here as much as the what.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and bake in the dirt, i've said before i want things tested against reality not the happy path: the seeded corruption rate is 1 percent per snapshot, promotion rules have an ambiguity when a weekly and monthly fall on the same day, and one drill requests a snapshot that was never eligible for retention. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. each mess instance gets its handling logged, rule named, no exceptions.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
