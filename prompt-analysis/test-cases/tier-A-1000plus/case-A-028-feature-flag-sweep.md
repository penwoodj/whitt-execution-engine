# Tier-A Case 028 — Feature Flag Sweep (SYNTH)

What i want is assemble a compact analysis-and-audit job, set up inside /tmp/opencode/flags-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the world of this little exercise: a flag table export, 120 flags, with creation date, last evaluation, and owner, plus flags untouched for 90 days with zero evaluations are dead, then two flags are actually the same flag renamed, owners disagree, plus removing a dead flag still requires a code change reference. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

here's the task list, in sequence:

1. dead-flag list, 90 days and zero evals, sorted oldest first. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. the renamed pair, evidence they are the same, conflicting owners and all. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. orphaned flags, owner left or team dissolved, from a stale-teams list. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. removal plan ordered by risk, low-risk first, each with a one-line reason. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. flags.md plus flags.json. with the method visible, not just the result, the how is the deliverable here as much as the what. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

the realistic part, the warts: four flags have evaluation counts but no creation date, and the export has two rows that are byte-identical duplicates. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
