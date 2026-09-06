# Tier-A Case 033 — Glossary Extraction (SYNTH)

the subject this time is glossary extraction, and i want it done like i mean it. let's assemble a compact analysis-and-audit job, entirely inside /tmp/opencode/glossary-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

context so you're not guessing: a corpus of 30 technical docs about one system, after that terms get defined inline in parentheses or with is-when sentences, plus a glossary is needed, term, definition, and where defined and some terms have conflicting definitions across docs. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

next extract candidate terms, capitalization patterns, repeated multiword phrases, definition sentences. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. deduplicate term variants, api key vs api-key vs API Key. edge cases belong in the output, not in your head, list what you hit and what you did with each.
stage by stage: conflict list where two docs define the same term differently, quote both. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. glossary.md alphabetical with source links, plus conflicts.md. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then glossary.json machine copy with term, definition, sources array. with the method visible, not just the result, the how is the deliverable here as much as the what.

and bake in the dirt, i've said before i want things tested against reality not the happy path: one doc defines a term using the term itself, and three definitions are copy-paste identical across docs which is fine, count sources. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. no silent absorption, each case logged with the rule that resolved it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
