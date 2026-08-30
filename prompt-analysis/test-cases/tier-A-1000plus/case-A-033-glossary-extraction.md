# Tier-A Case 033 — Glossary Extraction (SYNTH)

Let's do assemble a compact analysis-and-audit job, entirely inside /tmp/opencode/glossary-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

context so you're not guessing: a corpus of 30 technical docs about one system, after that terms get defined inline in parentheses or with is-when sentences, plus a glossary is needed, term, definition, and where defined and some terms have conflicting definitions across docs. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

concretely, do these stages, numbered so you don't creative-order them on me:

next extract candidate terms, capitalization patterns, repeated multiword phrases, definition sentences. and be precise about what counts as done for that one, because vague is where shortcuts hide. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. deduplicate term variants, api key vs api-key vs API Key. edge cases belong in the output, not in your head, list what you hit and what you did with each.
stage by stage: conflict list where two docs define the same term differently, quote both. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. glossary.md alphabetical with source links, plus conflicts.md. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then glossary.json machine copy with term, definition, sources array. with the method visible, not just the result, the how is the deliverable here as much as the what. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

the realistic part, the warts: one doc defines a term using the term itself, and three definitions are copy-paste identical across docs which is fine, count sources. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
