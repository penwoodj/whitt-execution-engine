# Tier-A Case 036 — Subtitle Repair (HEAVY)

Build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/subs-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

the world of this little exercise: a subtitle file, 800 cues, srt format. problems: overlaps, gaps over 5 seconds, durations under half a second, and numbering gaps, and reading speed matters, too many words per second is a real defect, after that a repaired file must keep every spoken line, nothing dropped. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

here's the task list, in sequence:

1. parse srt strictly, flag every malformed cue instead of dying. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. fix overlaps by trimming, gaps report-only, too-short durations extended, renumber everything. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. reading-speed pass, cues over 21 characters per second get split or extended. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. before-and-after defect counts in a table. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. repaired.srt plus repair-report.md plus defects.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the data has problems, deliberately, and handling them is part of the job not an error condition: two cues are out of order in time, one has a two-line speaker label baked in, and timestamps use comma in one spot and period in another. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
