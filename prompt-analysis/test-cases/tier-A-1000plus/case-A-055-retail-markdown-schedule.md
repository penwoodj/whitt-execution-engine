# Tier-A Case 055 — Retail Markdown Schedule (SYNTH)

Let's do build a small but complete tool with its own verification, entirely inside /tmp/opencode/markdown-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

what you're working with: an inventory of 200 seasonal items with weeks-of-stock and full price, plus markdown policy stages: 25 percent at 8 weeks of stock, 50 at 5, 70 at 2. revenue projection current trajectory versus markdown trajectory, plus one item is a guaranteed sellout that must not be marked down. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

concretely, do these stages, numbered so you don't creative-order them on me:

then apply the policy ladder per item, current price, weeks-of-stock, prescribed markdown. and that one gets checked by the verifier too, it's not just a produce-and-hope step. with the method visible, not just the result, the how is the deliverable here as much as the what.
first project sell-through with and without markdowns, simple decay model, assumptions stated. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then the sellout exception, protected item identified by velocity and stock. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
next revenue comparison table, per category and total, with the honest break-even. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
first markdown.md plus schedule.csv plus projection.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and be precise about what counts as done for that one, because vague is where shortcuts hide.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the realistic part, the warts: weeks-of-stock is negative for three already-underwater items, one item appears in two categories, and velocity data has a holiday spike that distorts the baseline. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
