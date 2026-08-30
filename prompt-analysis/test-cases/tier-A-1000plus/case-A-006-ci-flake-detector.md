# Tier-A Case 006 — Ci Flake Detector (HEAVY)

So put together an end-to-end processing exercise, fully self contained in /tmp/opencode/flake-run, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.

context so you're not guessing: a ci system that ran 500 builds over a month. each build has a pass-fail record per test, 40 distinct tests, and flaky tests pass sometimes and fail sometimes with no code change, plus one test called test_retry_loop is the prime suspect. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the asks, in order, and each one writes its own output so i can see where things broke when they break:

first compute pass rate per test and flag anything between 5 and 95 percent as suspicious. with the method visible, not just the result, the how is the deliverable here as much as the what. edge cases belong in the output, not in your head, list what you hit and what you did with each.
2. for flagged tests, find failure clusters, consecutive runs or same-day bunching. edge cases belong in the output, not in your head, list what you hit and what you did with each.
3. correlation check, do two tests tend to fail together more than chance. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
4. a ranked flake scoreboard with confidence note per row. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
stage by stage: flakes.md report plus flake.json machine copy, numbers must agree. and that one gets checked by the verifier too, it's not just a produce-and-hope step. edge cases belong in the output, not in your head, list what you hit and what you did with each.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

the realistic part, the warts: some build records are duplicated with different build ids, and three builds have missing test fields entirely. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
