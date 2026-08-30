# Tier-A Case 094 — Office Move Plan (SYNTH)

What i want is build a small but complete tool with its own verification, set up inside /tmp/opencode/officemove-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

background, and i'm giving you the details the way they actually occur: an office inventory: 40 furniture items, 25 boxes of stuff, two floors, elevator booking windows, and the move plan sequences elevator slots, item carries, and box shuttles, and constraints: elevator window 2 hours, two movers, heavy items need both movers, and one desk does not fit the stairwell and must go during the elevator window only. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

what i need done, in this order:

1. model carries and shuttles with time estimates stated per item class. with the method visible, not just the result, the how is the deliverable here as much as the what. edge cases belong in the output, not in your head, list what you hit and what you did with each.
then a feasible schedule respecting elevator windows and mover pairing, verified by simulation walk. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. the critical path, what bounds total time, elevator or carrying. with the method visible, not just the result, the how is the deliverable here as much as the what.
4. what-if: a second elevator slot added, new total, is it worth the cost. and be precise about what counts as done for that one, because vague is where shortcuts hide.
5. move.md plus schedule.csv plus sim.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. edge cases belong in the output, not in your head, list what you hit and what you did with each.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the realistic part, the warts: item dimensions make one doorway tight requiring a documented rotate maneuver assumption, three boxes are labeled fragile and cannot stack, and the inventory has one item listed twice under different names. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
