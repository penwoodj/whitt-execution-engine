# Tier-A Case 049 — Grade Curve Boundaries (SYNTH)

I need construct a self-contained reporting exercise done in /tmp/opencode/grades-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

what you're working with: a gradebook, 150 students, 8 assignments, weighted to a total; also the curve question: mean is 71 and the department wants distributions reported honestly. boundary cases within 1 point of grade cutoffs get individual scrutiny. one assignment was graded out of 105 not 100 by accident. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

next normalize the misgraded assignment, document the rescale. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
2. distribution stats, mean, median, stddev, histogram in text. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
3. letter-grade assignment under the official cutoffs, plus the boundary list within 1 point. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. what-if: a plus-minus curve shifting cutoffs by 2, who moves, table of changes. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. grades.md plus grades.json plus boundary.csv. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the realistic part, the warts: two students share an id prefix causing join hazards, three submissions are zero with no note distinguishing never-submitted from graded-zero, late penalties are applied inconsistently in the raw. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
