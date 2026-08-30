# Tier-A Case 064 — Dns Zone Sanity (SYNTH)

I want you to build a small but complete tool with its own verification for me, the whole thing lives under /tmp/opencode/dns-run and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.

the world of this little exercise: a zone file dump, 200 records, a, cname, mx, txt, ns. sanity rules: cname cannot coexist with other records on a name, mx must point to a name with an a record, no orphans, then one dangling cname chain of depth 3 exists, plus ttl inconsistency across a record set matters for cache coherence. this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

concretely, do these stages, numbered so you don't creative-order them on me:

1. parse the zone file strictly, every line classified or flagged. and be precise about what counts as done for that one, because vague is where shortcuts hide. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
cname exclusivity check, mx target resolution, orphan and dangling-chain detection with the chain printed. and be precise about what counts as done for that one, because vague is where shortcuts hide.
after that ttl variance report per name, records on one name with wildly different ttls. and be precise about what counts as done for that one, because vague is where shortcuts hide.
the fix list, record, problem, suggested repair, ordered by severity. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
5. dns.md plus dns.json plus fixes.csv. and be precise about what counts as done for that one, because vague is where shortcuts hide. edge cases belong in the output, not in your head, list what you hit and what you did with each.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

now the mess, because a pipeline that only works on clean data is worthless to me: one record has a wildcard label, comments hide two actual records, and a cname points at a name defined only outside this zone which is legal, do not flag it. those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before. the handling rule for each goes in the rules or decisions file, not in your memory.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.
