# Tier-A Case 054 — Menu Cost Margin (SYNTH)

What i want is construct a self-contained reporting exercise, set up inside /tmp/opencode/menu-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

here's the situation: a restaurant menu of 30 dishes with ingredient lists per dish, then an ingredient price sheet, per 100g, that drifted last quarter; also cost per plate, margin at current menu prices, and the loss-leaker list, plus two dishes share an ingredient prep step that halves its cost for the second one. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

concretely, do these stages, numbered so you don't creative-order them on me:

1. cost per dish from ingredients and prices, shared-prep discount applied where it applies, rule stated. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that margin table, cost, price, margin percent, ranked worst to best. edge cases belong in the output, not in your head, list what you hit and what you did with each.
the loss-leakers, negative or under 10 percent margin, with a repricing suggestion each. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
next price-drift impact, margins under last quarter prices side by side. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then menu.md plus margins.csv plus leakers.json. edge cases belong in the output, not in your head, list what you hit and what you did with each. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the data has problems, deliberately, and handling them is part of the job not an error condition: ingredient names are inconsistent, creme fraiche three ways, quantities mix grams and pieces requiring a conversion table, and one price sheet row is per-unit not per-100g. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
