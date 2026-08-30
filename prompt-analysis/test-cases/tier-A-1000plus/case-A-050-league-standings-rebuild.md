# Tier-A Case 050 — League Standings Rebuild (SYNTH)

Let's do construct a self-contained reporting exercise, entirely inside /tmp/opencode/league-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

context so you're not guessing: a season of 132 match results as messy rows, after that some rows are corrections superseding earlier results, marked replayed or void; also standings: points, then head-to-head, then goal difference, the rule order matters. two teams tie on everything at the end and the rulebook has one more clause. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

concretely, do these stages, numbered so you don't creative-order them on me:

stage by stage: parse and apply corrections in order, supersession chain preserved in an audit log. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
compute standings under the stated tiebreak order, show work for the tied teams. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. the full-tie pair resolved by the final clause, quoted and applied. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
after that a what-if table with the void matches counted, for the arguments. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. standings.md plus standings.json plus supersession.log. and that one gets checked by the verifier too, it's not just a produce-and-hope step. with the method visible, not just the result, the how is the deliverable here as much as the what.

and bake in the dirt, i've said before i want things tested against reality not the happy path: one correction corrects a correction, two matches have scores as strings, and a team renamed mid-season appears under both names. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
