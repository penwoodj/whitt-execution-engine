# Tier-A Case 028 — Feature Flag Sweep (SYNTH)

new job, feature flag sweep, same standards as always. what i want is assemble a compact analysis-and-audit job, set up inside /tmp/opencode/flags-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

what you're working with: a flag table export, 120 flags, with creation date, last evaluation, and owner, then flags untouched for 90 days with zero evaluations are dead, plus two flags are actually the same flag renamed, owners disagree, plus removing a dead flag still requires a code change reference. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. dead-flag list, 90 days and zero evals, sorted oldest first. and be precise about what counts as done for that one, because vague is where shortcuts hide.
after that the renamed pair, evidence they are the same, conflicting owners and all. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
then orphaned flags, owner left or team dissolved, from a stale-teams list. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
4. removal plan ordered by risk, low-risk first, each with a one-line reason. with the method visible, not just the result, the how is the deliverable here as much as the what.
stage by stage: flags.md plus flags.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

now the mess, because a pipeline that only works on clean data is worthless to me: four flags have evaluation counts but no creation date, and the export has two rows that are byte-identical duplicates. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. each mess instance gets its handling logged, rule named, no exceptions.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
