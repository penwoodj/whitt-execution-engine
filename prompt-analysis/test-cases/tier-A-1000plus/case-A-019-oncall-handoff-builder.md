# Tier-A Case 019 — Oncall Handoff Builder (SYNTH)

What i want is construct a self-contained reporting exercise, set up inside /tmp/opencode/handoff-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the world of this little exercise: a week of support tickets, 80 of them, as jsonl and each has subject, body, priority, status, and resolution notes if resolved, plus friday handoff doc needs the still-open items and what happened and recurring themes matter more than individual tickets. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

concretely, do these stages, numbered so you don't creative-order them on me:

1. open tickets grouped by theme with a one-line status each. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
resolved this week, count, and the two biggest stories. with the method visible, not just the result, the how is the deliverable here as much as the what.
follow-ups owed to people, promises with dates. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. a watch-list, things that could blow up over the weekend. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then handoff.md ready to paste plus handoff.json for the record. edge cases belong in the output, not in your head, list what you hit and what you did with each. edge cases belong in the output, not in your head, list what you hit and what you did with each.

the realistic part, the warts: six tickets are marked resolved but have empty resolution notes and two are duplicates of each other across priorities. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
