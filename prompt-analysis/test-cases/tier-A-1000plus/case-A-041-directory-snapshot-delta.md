# Tier-A Case 041 — Directory Snapshot Delta (HEAVY)

Let's do build me a full working analysis pipeline, entirely inside /tmp/opencode/dirsnap-run, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.

here's the situation: two directory snapshots taken a month apart, as manifest files with path, size, mtime, hash and the delta report is what changed: new, gone, grown, shrunk, rewritten, after that mtime alone lies, some files were touched without content change; also hash is truth where present, four files lack hashes. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:

1. five-way delta, added, removed, grown, shrunk, same-size-rewritten, using hash over mtime where available. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
2. mtime-liar list, mtime changed but hash identical. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
3. the four hashless files handled by size-and-mtime heuristic, flagged low-confidence. edge cases belong in the output, not in your head, list what you hit and what you did with each.
4. totals that add up, every file in either snapshot lands in exactly one bucket. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. delta.md plus delta.json, conservation check mandatory. and that one gets checked by the verifier too, it's not just a produce-and-hope step. and be precise about what counts as done for that one, because vague is where shortcuts hide.

now the mess, because a pipeline that only works on clean data is worthless to me: paths moved rather than removed-and-added, detect three renames by hash matching across snapshots, and one path exists in both with same hash but different case. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
