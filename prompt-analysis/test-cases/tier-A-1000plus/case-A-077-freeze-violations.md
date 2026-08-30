# Tier-A Case 077 — Freeze Violations (SYNTH)

I want you to assemble a compact analysis-and-audit job for me, the whole thing lives under /tmp/opencode/freeze-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

background, and i'm giving you the details the way they actually occur: a commit dump spanning a code freeze window, dec 15 to jan 5; also violations: non-approved commits merged during the window, then an approvals list exists but uses ticket ids while commits use message refs, matching is fuzzy. emergency commits are legal with an incident tag in the message. think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

concretely, do these stages, numbered so you don't creative-order them on me:

1. freeze window applied, every commit inside classified: incident-tagged, approved, or violation. and be precise about what counts as done for that one, because vague is where shortcuts hide. edge cases belong in the output, not in your head, list what you hit and what you did with each.
stage by stage: the fuzzy matching from commit refs to approval tickets documented, with match confidence. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
3. violation list, author, what, when, and severity by lines touched. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next approval anomalies, approvals granted after the commit, or by the committer themselves. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. freeze.md plus violations.csv plus matching.md. edge cases belong in the output, not in your head, list what you hit and what you did with each. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and bake in the dirt, i've said before i want things tested against reality not the happy path: three commits straddle the boundary by timezone ambiguity, one incident tag is misspelled, and a merge commit parents hide four actual commits inside the window. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
