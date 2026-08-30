# Tier-A Case 086 — Weather Gap Fill (HEAVY)

So build me a full working analysis pipeline, fully self contained in /tmp/opencode/weather-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

the world of this little exercise: a year of daily temperature rows from three stations, then gaps exist: whole days missing, single fields missing, and one station offline for 11 days and interpolation policy must distinguish estimable from not, neighbor stations versus time neighbors, and a freeze-event count depends on the data quality, so gaps change the answer. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

what i need done, in this order:

stage by stage: gap census, per station, duration distribution, total missing percentage. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. fill policy: time-interpolation for short gaps under 3 days, station-correlation for longer, thresholds justified. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
after that filled dataset flagged per row, original, interpolated, cross-station, with method column. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. freeze events computed on raw versus filled, delta reported honestly. edge cases belong in the output, not in your head, list what you hit and what you did with each.
5. weather.md plus filled.csv plus gaps.json. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. with the method visible, not just the result, the how is the deliverable here as much as the what.

the realistic part, the warts: one station reports in celsius and two in fahrenheit, timestamps exist for 366 days in a non-leap year requiring day-of-year care, and one temperature is a physically impossible negative outlier that is not a gap but a sensor lie. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
