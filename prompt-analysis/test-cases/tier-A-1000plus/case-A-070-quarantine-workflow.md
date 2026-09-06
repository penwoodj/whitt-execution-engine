# Tier-A Case 070 — Quarantine Workflow (SYNTH)

today it's quarantine workflow, and no, you don't get to improvise the scope. i need build a small but complete tool with its own verification done in /tmp/opencode/quarantine-run. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.

the world of this little exercise: an uploads folder, 200 files of mixed types, some untrusted, then policy: executables and archives-with-executables quarantine, documents pass to scan, unknown extensions quarantine, and archive inspection must recurse, an archive inside an archive is a real case here, then the quarantine manifest needs a review queue ordered by risk. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

what i need done, in this order:

1. classify every file, type by extension and magic-prefix both, mismatches flagged. with the method visible, not just the result, the how is the deliverable here as much as the what.
first recursive archive inspection with depth guard, contents inventoried. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
3. apply the policy, move nothing, output the disposition plan, pass, quarantine, review. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. the review queue ordered by a stated risk score, executables-in-double-archive on top. with the method visible, not just the result, the how is the deliverable here as much as the what.
then quarantine.md plus disposition.csv plus queue.json. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and bake in the dirt, i've said before i want things tested against reality not the happy path: one archive is self-referential via a symlink trick, three files have double extensions like pdf.exe, and a document type is unknown to the magic table requiring a documented fallback. handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is. the decisions file gets one line per mess type saying how you dealt with it.

on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.
