# Tier-A Case 082 — Install Order Plan (HEAVY)

So run a proper multi-stage data job, fully self contained in /tmp/opencode/install-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

background, and i'm giving you the details the way they actually occur: a requirements dump, 150 packages with declared dependencies, no versions needed; also produce a valid install order, and a parallel install plan where independence allows, after that post-install scripts exist for some packages and have ordering needs beyond dependencies, then one dependency is satisfied by either of two alternatives, choice must be stated. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

what i need done, in this order:

stage by stage: topological order over the dependency graph, determinism via name-sort among ready nodes. and be precise about what counts as done for that one, because vague is where shortcuts hide. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. parallel batches, level-by-level, everything installable simultaneously grouped. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
stage by stage: script-ordering constraints layered on top, which scripts force serialization and why. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
4. the alternative-choice decision, which of the two picked and the stated reason. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. install.md plus order.txt plus batches.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and that one gets checked by the verifier too, it's not just a produce-and-hope step.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and bake in the dirt, i've said before i want things tested against reality not the happy path: the dump has one package depending on itself, two packages declare mutually exclusive conflicts, and three dependency names reference packages absent from the dump requiring an assumed-external flag. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
