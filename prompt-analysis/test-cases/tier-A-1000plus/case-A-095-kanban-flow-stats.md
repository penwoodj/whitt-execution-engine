# Tier-A Case 095 — Kanban Flow Stats (SYNTH)

I need construct a self-contained reporting exercise done in /tmp/opencode/kanban-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

the setup is this: a board export, 400 cards, with stage timestamps: backlog, doing, review, done and flow stats: per-stage time, total cycle time, wip over time, aging of stuck cards and two cards looped back from review to doing twice, that history matters, plus the aging report flags anything over 14 days in one stage. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

then per-stage dwell computed from timestamp pairs, loop-backs handled with multipass accounting. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and be precise about what counts as done for that one, because vague is where shortcuts hide.
cycle-time distribution, median, p90, histogram in text. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
3. wip-over-time per stage, sampled daily, spikes located. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. the stuck list, cards over 14 days in a stage, with age and stage. and be precise about what counts as done for that one, because vague is where shortcuts hide.
first kanban.md plus flow.json plus stuck.csv. and be precise about what counts as done for that one, because vague is where shortcuts hide. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the data has problems, deliberately, and handling them is part of the job not an error condition: timestamps are timezone-naive except twelve utc-suffixed rows, one card has a done before its doing, and stage names drift with a rename mid-export. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
