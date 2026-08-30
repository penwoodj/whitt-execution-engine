# Tier-A Case 014 — Log Level Cleanup (SYNTH)

What i want is construct a self-contained reporting exercise, set up inside /tmp/opencode/loglevel-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

context so you're not guessing: an app that logs everything at the wrong level on purpose it seems, after that 10,000 lines across four files, debug, info, warn, error. obviously info-level content marked error, stack traces buried in debug, after that there is one real error in the whole pile and it matters. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

here's the task list, in sequence:

1. propose a reclassification per line using content rules you write in a rules.md. and be precise about what counts as done for that one, because vague is where shortcuts hide. with the method visible, not just the result, the how is the deliverable here as much as the what.
2. apply the reclassification and output cleaned files preserving original order. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
3. count shifts in a moves table, from-level to-level count. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. surface the one real error and prove it stands out after cleanup. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that cleanup.md report plus the cleaned files plus moves.json. with the method visible, not just the result, the how is the deliverable here as much as the what. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the realistic part, the warts: one file has Windows line endings, one has a BOM, and multiline stack traces must move as a unit. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
