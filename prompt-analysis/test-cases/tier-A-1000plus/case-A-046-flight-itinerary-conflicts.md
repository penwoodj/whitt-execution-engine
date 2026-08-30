# Tier-A Case 046 — Flight Itinerary Conflicts (SYNTH)

What i want is build a small but complete tool with its own verification, set up inside /tmp/opencode/flights-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the world of this little exercise: an itinerary set for a sales team, 30 flights across 6 people, after that each flight: person, origin, destination, depart, arrive, in local times of origin and destination, after that conflicts: overlapping flights for one person, connections under 45 minutes, impossible city sequences, and timezone math is where fakes go to die. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

concretely, do these stages, numbered so you don't creative-order them on me:

1. normalize all times to UTC internally, conversion table documented. and that one gets checked by the verifier too, it's not just a produce-and-hope step. with the method visible, not just the result, the how is the deliverable here as much as the what.
stage by stage: per-person timeline, conflicts marked, overlap or impossible connection. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. city-sequence sanity, departing from a city you never arrived in. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then repair suggestions per conflict, earliest fix, latest fix. with the method visible, not just the result, the how is the deliverable here as much as the what.
flights.md plus conflicts.json plus utc-table.csv. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. edge cases belong in the output, not in your head, list what you hit and what you did with each.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and bake in the dirt, i've said before i want things tested against reality not the happy path: arrival times before departures on red-eyes are legal, two flights share a flight number on different days, and one airport code appears as three letters and as four. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
