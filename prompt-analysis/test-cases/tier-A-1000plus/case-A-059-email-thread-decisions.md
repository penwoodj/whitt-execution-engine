# Tier-A Case 059 — Email Thread Decisions (SYNTH)

next up, email thread decisions, whole thing specced below. build a small but complete tool with its own verification, that's the job, workspace is /tmp/opencode/thread-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

the world of this little exercise: one long email thread exported as text, 60 messages, three weeks; also a decision got made somewhere in the middle and unmade near the end; also participants drift in and out, quoting styles vary wildly. the extraction target: who decided what when, with message numbers as evidence. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

here's the task list, in sequence:

next message index, number, from, date, one-line gist. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first decision timeline, proposals, counter-proposals, the landing point, with message refs. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. the unmaking, which message reversed it and who objected. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. participants roster with message counts and influence note, who drove the outcome. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. thread.md plus timeline.json plus roster.csv. edge cases belong in the output, not in your head, list what you hit and what you did with each.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the data has problems, deliberately, and handling them is part of the job not an error condition: quoting makes attribution genuinely hard, three messages are forwards containing nested older threads, and one message has no author header at all. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the decisions file gets one line per mess type saying how you dealt with it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
