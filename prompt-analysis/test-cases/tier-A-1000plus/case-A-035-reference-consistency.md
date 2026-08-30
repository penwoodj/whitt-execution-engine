# Tier-A Case 035 — Reference Consistency (SYNTH)

What i want is build a small but complete tool with its own verification, set up inside /tmp/opencode/refs-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

background, and i'm giving you the details the way they actually occur: a spec doc with 40 numbered claims, each citing source files by path and line, then the code moved since the spec was written, and some citations are now wrong, pointing at moved or changed lines. a claim with zero citations is a special kind of wrong. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

after that parse citations, path plus line or path alone. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. check each against the current tree, exact hit, moved-hit via search, dead. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: for moved-hits, extract the current line and diff it against what the spec implies. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. claims with no citations listed separately as unverifiable. with the method visible, not just the result, the how is the deliverable here as much as the what.
then refs.md plus refs.json, every claim gets a status of verified, moved, dead, or uncited. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.

the data has problems, deliberately, and handling them is part of the job not an error condition: one cited file was renamed with a .bak twin left behind, line numbers past file end exist, and one claim cites itself. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
