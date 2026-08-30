# Tier-A Case 030 — Json Sqlite Load (HEAVY)

Let's do build me a full working analysis pipeline, entirely inside /tmp/opencode/jsqlite-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

the shape of the data: a 50,000-record json array of product catalog data, and fields are messy, nested suppliers, tags as comma strings, prices as strings with currency symbols, after that target is a sqlite db with a sane typed schema, and three queries will run against it constantly, so indexes matter. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

what i need done, in this order:

1. design a normalized schema, products, suppliers, tags, with types and a schema.md rationale. with the method visible, not just the result, the how is the deliverable here as much as the what. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. load with type coercion, strip currency, split tags, flatten suppliers. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. load report, rows in, rows rejected and why, per error type. with the method visible, not just the result, the how is the deliverable here as much as the what.
stage by stage: indexes for the three known queries, and a timing before-and-after per query. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
after that catalog.db plus load-report.md plus a verify script that reruns the three queries. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. with the method visible, not just the result, the how is the deliverable here as much as the what.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and bake in the dirt, i've said before i want things tested against reality not the happy path: a few records are missing entire required fields, one record is the array itself nested, and prices include two different currencies that must not be silently summed. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
