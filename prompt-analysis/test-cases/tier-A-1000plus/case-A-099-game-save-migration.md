# Tier-A Case 099 — Game Save Migration (SYNTH)

I want you to build a small but complete tool with its own verification for me, the whole thing lives under /tmp/opencode/gamesave-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the setup is this: a game save format v1 and the v2 spec, plus 50 saves to migrate, plus v2 renames fields, nests previously flat ones, and changes the money units from copper to silver at 100 to 1, then one field in v1 was overloaded with two meanings, disambiguation rule provided. the migration must be lossless with a round-trip proof per save. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

1. write the transform with the field map documented. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
after that money conversion exact with no floating point drift, integer cents arithmetic. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. the overloaded field disambiguated per the rule, counts of each branch. with the method visible, not just the result, the how is the deliverable here as much as the what.
next round-trip proof, v1 to v2 and back equals original byte-wise or value-wise, state which and prove it on all 50. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
migrate.md plus map.json plus roundtrip-results.csv. actual numbers in the output, computed, not eyeballed, and traceable back to input rows. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

now the mess, because a pipeline that only works on clean data is worthless to me: three saves use a v1.5 dialect with one extra field, two saves are truncated but recoverable for the fields present, and one save has the overloaded field with a value satisfying both meanings. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
