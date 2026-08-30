# Tier-A Case 016 — Load Test Ramp (HEAVY)

What i want is build me a full working analysis pipeline, set up inside /tmp/opencode/ramp-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: a load test that ramped from 10 to 1000 virtual users over 30 minutes, then samples of throughput, latency, and error rate every ten seconds, then the system was supposed to degrade gracefully past 600 users, after that it did not, or did it, that is what we are checking. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

what i need done, in this order:

1. plot data as a table, users, throughput, p95, error percent, at each step of the ramp. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. edge cases belong in the output, not in your head, list what you hit and what you did with each.
after that find the knee where latency starts climbing faster than throughput. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. the graceful degradation claim, verdict with numbers, where it broke. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. error taxonomy over the ramp, what kind of errors appear when. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
stage by stage: ramp.md plus ramp.json and a knee.md one-pager explaining the method. edge cases belong in the output, not in your head, list what you hit and what you did with each. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

now the mess, because a pipeline that only works on clean data is worthless to me: some sample rows are missing at the worst moment, two counters reset mid-run, and units are inconsistent across two files. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
