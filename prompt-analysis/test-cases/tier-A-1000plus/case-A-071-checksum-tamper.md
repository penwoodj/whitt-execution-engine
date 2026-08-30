# Tier-A Case 071 — Checksum Tamper (SYNTH)

Construct a self-contained reporting exercise, that's the job, workspace is /tmp/opencode/tamper-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

context so you're not guessing: a manifest of 500 files with sha256 checksums, taken at snapshot time. the current tree differs from the manifest in specific planted ways, and tamper means hash mismatch, but also: new files, missing files, and case-only renames; also one file legitimately changed and is documented as such in a changelog. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

concretely, do these stages, numbered so you don't creative-order them on me:

next verify every manifest entry, match, mismatch, missing, plus unlisted new files. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
2. the documented legit change reconciled against the changelog, not flagged as tamper. and be precise about what counts as done for that one, because vague is where shortcuts hide.
then case-only renames detected, same hash different path case. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
first tamper report ranked by plausibility of malice versus accident, with reasoning. and be precise about what counts as done for that one, because vague is where shortcuts hide.
after that tamper.md plus tamper.json plus the reconciled changelog cross-ref. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the realistic part, the warts: two files swapped contents but kept names, one manifest line has a truncated hash from a wrapping bug, and the changelog entry is vague about which file it means. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
