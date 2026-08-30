# Tier-A Case 057 — Kb Staleness Ranking (SYNTH)

I've got build a small but complete tool with its own verification to do and i want it done properly, which means /tmp/opencode/staleness-run for everything, real scripts i can rerun, real files on disk, not a description of what would happen.

context so you're not guessing: a knowledge base, 150 articles, each with updated date, views last 90 days, and product version tags and staleness = version drift plus neglect, not just age; also view counts come from three sources that count differently, and five articles are dead but load-bearing, linked from everywhere. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

concretely, do these stages, numbered so you don't creative-order them on me:

after that normalize view counts across the three sources, method stated. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next staleness score per article, version-tag age, time since update, weighted, formula public. edge cases belong in the output, not in your head, list what you hit and what you did with each.
ranked list, top 20 most stale with the score decomposition shown. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
then the dead-but-load-bearing set, inbound-link count despite staleness. with the method visible, not just the result, the how is the deliverable here as much as the what.
next stale.md plus ranked.csv plus loadbearing.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

the realistic part, the warts: version tags come in 2.3, v2.3, and 2.3.x forms, one article has an updated date in the future, and inbound links must be scraped from article bodies. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the handling rule for each goes in the rules or decisions file, not in your memory.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
