# Tier-A Case 069 — Permissions Matrix (SYNTH)

Construct a self-contained reporting exercise, that's the job, workspace is /tmp/opencode/perms-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

background, and i'm giving you the details the way they actually occur: a role dump and a resource dump from a fake rbac system, and roles have members and permissions, resources have required permissions, plus the matrix: who can access what, directly or via role inheritance, then one permission was granted to a role that no longer exists. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

then resolve role inheritance to effective permissions per user, cycles detected if any. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. the access matrix, user by resource, boolean, plus the via-path for every true cell. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. the orphan permission flagged, granted to a dead role, affects nobody. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. overprovisioning report, users with permissions they have never used per the usage log. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. perms.md plus matrix.csv plus effective.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: role inheritance has a diamond, a inherits b and c which both inherit d, one user has a direct permission that contradicts their role, and the usage log covers only 60 percent of resources. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
