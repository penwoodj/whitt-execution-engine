# Tier-A Case 014 — Log Level Cleanup (SYNTH)

log level cleanup. been meaning to get this done properly for a while. what i want is construct a self-contained reporting exercise, set up inside /tmp/opencode/loglevel-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

background, and i'm giving you the details the way they actually occur: an app that logs everything at the wrong level on purpose it seems. 10,000 lines across four files, debug, info, warn, error, after that obviously info-level content marked error, stack traces buried in debug, then there is one real error in the whole pile and it matters. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

here's the task list, in sequence:

1. propose a reclassification per line using content rules you write in a rules.md. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. apply the reclassification and output cleaned files preserving original order. with the method visible, not just the result, the how is the deliverable here as much as the what.
first count shifts in a moves table, from-level to-level count. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
after that surface the one real error and prove it stands out after cleanup. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. cleanup.md report plus the cleaned files plus moves.json. edge cases belong in the output, not in your head, list what you hit and what you did with each.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the realistic part, the warts: one file has Windows line endings, one has a BOM, and multiline stack traces must move as a unit. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. no silent absorption, each case logged with the rule that resolved it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
