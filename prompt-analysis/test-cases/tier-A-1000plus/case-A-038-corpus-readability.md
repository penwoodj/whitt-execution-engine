# Tier-A Case 038 — Corpus Readability (SYNTH)

I want you to construct a self-contained reporting exercise for me, the whole thing lives under /tmp/opencode/readab-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

what you're working with: a corpus of 25 essays, same author, spanning two years, after that readability scores and vocabulary growth over time are the question, and sentence length, rare-word rate, and a standard readability index will do, after that the last three essays were co-written and should stand out. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

what i need done, in this order:

1. per-essay stats, sentences, words, mean and max sentence length, rare word rate against a shared vocabulary. edge cases belong in the output, not in your head, list what you hit and what you did with each. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next a readability index per essay, formula stated and applied consistently. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then trend over time, improving, degrading, or flat, with the numbers. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. the three co-written essays, do they cluster separately on your metrics. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
after that readab.md plus readab.json plus a per-essay table. and that one gets checked by the verifier too, it's not just a produce-and-hope step. edge cases belong in the output, not in your head, list what you hit and what you did with each.

the data has problems, deliberately, and handling them is part of the job not an error condition: one essay is a single 400-word sentence on purpose, two essays have heavy markdown formatting to strip, and footnotes pollute word counts unless excluded. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
