# Tier-A Case 067 — Session Redaction Sweep (SYNTH)

Assemble a compact analysis-and-audit job, that's the job, workspace is /tmp/opencode/redact-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

here's the situation: terminal session logs to share publicly, 15 files, thousands of lines, after that must scrub: ips, paths containing usernames, api-ish tokens, emails, then over-redaction ruins usefulness, under-redaction leaks, calibration is the task; also a redaction map file must allow un-redacting privately later. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

what i need done, in this order:

1. pattern set for ips v4 and v6, emails, token shapes like long base64 or x-prefix headers, and home-dir paths. with the method visible, not just the result, the how is the deliverable here as much as the what. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
next apply with consistent placeholders, ip-1, path-2, stable per unique secret, not per occurrence. and be precise about what counts as done for that one, because vague is where shortcuts hide.
next the redaction map as a separate private file, placeholder to original. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
4. leak audit, rescan output for anything the patterns missed, list suspects for manual eyes. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
5. redacted files plus map.json plus audit.md with false-positive and false-negative notes. and be precise about what counts as done for that one, because vague is where shortcuts hide. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.

now the mess, because a pipeline that only works on clean data is worthless to me: one log contains a pasted json web token split across lines, urls embed credentials in one spot, and the same ip appears in decimal and dotted forms. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.
