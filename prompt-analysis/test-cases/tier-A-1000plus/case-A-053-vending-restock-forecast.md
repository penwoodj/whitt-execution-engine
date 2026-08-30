# Tier-A Case 053 — Vending Restock Forecast (SYNTH)

So build a small but complete tool with its own verification, fully self contained in /tmp/opencode/vending-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

context so you're not guessing: 12 weeks of vending machine sales rows, item, timestamp, price. capacity per slot is 8 items, 30 slots, restock costs a trip. forecast next week per item to decide the restock list, then some items died mid-quarter and their slots are wasted. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. weekly per-item sales matrix with trend and variability columns. and be precise about what counts as done for that one, because vague is where shortcuts hide. edge cases belong in the output, not in your head, list what you hit and what you did with each.
next-week forecast per item, method stated, with a demand-vs-capacity note. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
next the restock list, what to fill which slots with, and dead-item replacement proposals. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. trip-frequency analysis, would weekly versus biweekly restocking starve any item. and be precise about what counts as done for that one, because vague is where shortcuts hide.
stage by stage: vending.md plus vending.json plus restock-list.csv. and be precise about what counts as done for that one, because vague is where shortcuts hide. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

the data has problems, deliberately, and handling them is part of the job not an error condition: timestamps have a batch artifact, sales logged at 09:00 for the whole day, one item has a price change mid-stream, and three slots share one item. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
