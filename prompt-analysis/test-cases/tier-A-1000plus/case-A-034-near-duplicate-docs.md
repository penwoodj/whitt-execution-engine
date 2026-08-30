# Tier-A Case 034 — Near Duplicate Docs (SYNTH)

I want you to construct a self-contained reporting exercise for me, the whole thing lives under /tmp/opencode/dupidocs-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

what you're working with: a knowledge base, 200 short articles, then some are near-duplicates, lightly edited versions of each other, after that exact hash will not catch them, similarity threshold needed, after that the canonical version should be the longest, usually. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

stage by stage: token-level similarity matrix, or a smarter bucketing, but justify the method. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. clusters of near-duplicates at a stated threshold, with the threshold swept from 0.7 to 0.95. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. canonical pick per cluster and what gets merged away. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. the oddballs, articles similar to nothing, listed for human eyes. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. dupidocs.md plus clusters.json plus a threshold-sensitivity table. edge cases belong in the output, not in your head, list what you hit and what you did with each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

the realistic part, the warts: ten articles are byte-identical, one is an html version of a markdown one, and two are each half of what should have been one article. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
