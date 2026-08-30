# Tier-A Case 012 — Error Budget Burn (HEAVY)

What i want is build me a full working analysis pipeline, set up inside /tmp/opencode/budget-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: three services with 30 days of uptime checks every minute and slo targets are 99.9, 99.5, and 99.0 percent respectively, after that burn rate is how fast you eat the monthly error budget, and one service already burned its whole budget by day nine. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

concretely, do these stages, numbered so you don't creative-order them on me:

then availability per service per day and per month-to-date. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
2. error budget size in minutes per service, remaining budget timeline. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first burn rate per day, flag any day over 3x sustainable burn. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. the day-nine budget exhaustion, was it one bad hour or a slow grind. edge cases belong in the output, not in your head, list what you hit and what you did with each.
stage by stage: budget.md plus budget.json with every number recomputable from the check files. edge cases belong in the output, not in your head, list what you hit and what you did with each. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

the realistic part, the warts: some checks are missing rather than failed and must not count as downtime silently, decide and document the rule. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
