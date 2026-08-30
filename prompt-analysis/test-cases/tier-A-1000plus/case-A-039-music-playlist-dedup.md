# Tier-A Case 039 — Music Playlist Dedup (SYNTH)

What i want is construct a self-contained reporting exercise, set up inside /tmp/opencode/playlist-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the world of this little exercise: an exported playlist, 900 tracks, with title, artist, duration, and a bpm field where present, and duplicates exist across remasters, live versions, and plain re-entries and true duplicates means same title and artist, near-duration, not just string equal, and the bpm field is missing on 300 tracks. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

concretely, do these stages, numbered so you don't creative-order them on me:

1. exact duplicates first, then fuzzy, title and artist normalized, duration within 3 seconds. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. edge cases belong in the output, not in your head, list what you hit and what you did with each.
next keep rules, studio over live, longest over shortest, stated in rules.md. and be precise about what counts as done for that one, because vague is where shortcuts hide.
3. bpm analysis where present, distribution and the ten most tempo-similar neighbors. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
next dedup report, removed tracks with the reason each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. playlist-clean.csv plus dedup-report.md plus removed.json. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and bake in the dirt, i've said before i want things tested against reality not the happy path: artists are formatted last-first in half the file and first-last in the rest, featuring artists are appended inconsistently, and one track has a negative duration. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
