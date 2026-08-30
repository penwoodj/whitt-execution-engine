# Tier-A Case 044 — Subscription Audit (SYNTH)

I need assemble a compact analysis-and-audit job done in /tmp/opencode/subs-audit-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

background, and i'm giving you the details the way they actually occur: two years of card transactions focused on recurring charges, then subscriptions hide as repeated same-amount same-merchant patterns monthly or yearly; also price hikes mid-stream are the sneaky part, and one service was cancelled but still charged twice after. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

concretely, do these stages, numbered so you don't creative-order them on me:

1. detect recurring patterns, merchant, amount, period, first seen, last seen. and be precise about what counts as done for that one, because vague is where shortcuts hide. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next flag dead ones, no charge in 3x their period, and zombie ones, charges after an apparent cancel gap. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
next price-hike list, same merchant, amount stepped up, when and by how much. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
after that annualized total of everything recurring, the number nobody wants to see. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. subs.md plus subs.json plus a cancel-candidates ranked list. and be precise about what counts as done for that one, because vague is where shortcuts hide. edge cases belong in the output, not in your head, list what you hit and what you did with each.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: one merchant bills on a 28-day cycle which never aligns with calendar months, two share an identical amount and must not be merged. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
