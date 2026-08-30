# Tier-A Case 093 — License Scan (SYNTH)

What i want is build a small but complete tool with its own verification, set up inside /tmp/opencode/license-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

so the scenario: a dependency dump of 120 packages with license fields, some filled, some empty, and policy: permissive ok, weak-copyleft needs listing, strong-copyleft forbidden, unknown = investigate, then transitive deps matter, a permissive package with a strong-copyleft child is a violation. one package is dual-licensed with a choice clause. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

first classify every license string to a policy bucket, normalization table shown. and that one gets checked by the verifier too, it's not just a produce-and-hope step. with the method visible, not just the result, the how is the deliverable here as much as the what.
then transitive propagation of constraints, packages whose children drag them into violation. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first the violation list and investigate list, separated, with the chain shown for transitive cases. and be precise about what counts as done for that one, because vague is where shortcuts hide.
the dual-license case resolved by the choice clause with the reasoning stated. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. license.md plus buckets.csv plus violations.json. with the method visible, not just the result, the how is the deliverable here as much as the what. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed: license strings include url forms, abbreviations, and typo variants, two packages declare custom licenses requiring human-tier flagging, and one package has different licenses declared in two places. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
