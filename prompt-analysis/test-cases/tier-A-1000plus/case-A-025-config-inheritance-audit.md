# Tier-A Case 025 — Config Inheritance Audit (SYNTH)

Build a small but complete tool with its own verification, that's the job, workspace is /tmp/opencode/nginx-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

here's the situation: an nginx config tree, main file plus four includes, plus per-site files and directives inherit downward and get overridden by more specific blocks, plus three sites claim conflicting timeout values and nobody knows which wins. a typo in one include may be silently ignored. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

concretely, do these stages, numbered so you don't creative-order them on me:

then build the effective config per site, inheritance resolved, actual values that apply. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. the three conflicting timeouts, which value wins per site and why. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
next directives in includes that no block ever consumes, dead config. with the method visible, not just the result, the how is the deliverable here as much as the what.
after that the suspected typo, does nginx actually parse that line or drop it. with the method visible, not just the result, the how is the deliverable here as much as the what.
after that effective.md plus effective.json mapping site to resolved directive set. and be precise about what counts as done for that one, because vague is where shortcuts hide. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

now the mess, because a pipeline that only works on clean data is worthless to me: one site file includes another site file, creating inheritance depth two, and there is a commented-out server block that looks alive. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
