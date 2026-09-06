# Tier-A Case 075 — Authorship Normalization (SYNTH)

today it's authorship normalization, and no, you don't get to improvise the scope. i want you to assemble a compact analysis-and-audit job for me, the whole thing lives under /tmp/opencode/authors-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

here's the situation: a commit dump where the same humans appear under many identities, after that aliases differ by email, name spelling, and one person's work laptop identity. a mailmap-style mapping must be induced from the data, email domains and name similarity as evidence, after that one identity is genuinely two different people with the same name. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

here's the task list, in sequence:

cluster identities by exact email, then name similarity within domain, evidence per merge. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. the induced mailmap with confidence per mapping, low-confidence ones flagged. with the method visible, not just the result, the how is the deliverable here as much as the what.
next the same-name-different-people case separated correctly, evidence shown. with the method visible, not just the result, the how is the deliverable here as much as the what.
next per-person commit stats after normalization, before and after counts side by side. with the method visible, not just the result, the how is the deliverable here as much as the what.
5. authors.md plus mailmap plus stats.json. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and bake in the dirt, i've said before i want things tested against reality not the happy path: one email is shared by two people legitimately, a pair of identities merge only via commit-message sign-off trails, and names contain unicode variants that fold differently under different normalizations. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. every one of those needs a documented disposition, handled, quarantined, or rejected, with the rule cited.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
