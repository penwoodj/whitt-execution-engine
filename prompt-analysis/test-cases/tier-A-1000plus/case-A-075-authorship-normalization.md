# Tier-A Case 075 — Authorship Normalization (SYNTH)

I want you to assemble a compact analysis-and-audit job for me, the whole thing lives under /tmp/opencode/authors-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

context so you're not guessing: a commit dump where the same humans appear under many identities and aliases differ by email, name spelling, and one person's work laptop identity, after that a mailmap-style mapping must be induced from the data, email domains and name similarity as evidence. one identity is genuinely two different people with the same name. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

here's the task list, in sequence:

1. cluster identities by exact email, then name similarity within domain, evidence per merge. edge cases belong in the output, not in your head, list what you hit and what you did with each. and be precise about what counts as done for that one, because vague is where shortcuts hide.
then the induced mailmap with confidence per mapping, low-confidence ones flagged. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
3. the same-name-different-people case separated correctly, evidence shown. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next per-person commit stats after normalization, before and after counts side by side. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
5. authors.md plus mailmap plus stats.json. and that one gets checked by the verifier too, it's not just a produce-and-hope step. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and bake in the dirt, i've said before i want things tested against reality not the happy path: one email is shared by two people legitimately, a pair of identities merge only via commit-message sign-off trails, and names contain unicode variants that fold differently under different normalizations. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
