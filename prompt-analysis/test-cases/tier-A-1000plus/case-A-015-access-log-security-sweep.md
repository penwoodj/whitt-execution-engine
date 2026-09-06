# Tier-A Case 015 — Access Log Security Sweep (HEAVY)

access log security sweep, that's the theme, here's the full spec: what i want is put together an end-to-end processing exercise, set up inside /tmp/opencode/sec-sweep-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the shape of the data: access logs from a public-facing box, two weeks. patterns of interest: failed logins, path scanning, one ip hammering /admin, after that legitimate traffic exists and must not be flagged and no real ips or secrets, everything is synthetic fixture data. the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

concretely, do these stages, numbered so you don't creative-order them on me:

1. failed-login frequency by ip, top twenty with counts. and that one gets checked by the verifier too, it's not just a produce-and-hope step.
2. path scanning detection, many distinct 404 paths from one source. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
3. the /admin hammer, when it started, stopped, request rate. and be precise about what counts as done for that one, because vague is where shortcuts hide.
4. a whitelist pass so obviously-legit crawlers do not pollute the report. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
stage by stage: sweep.md plus sweep.json, with the detection rules documented. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

and bake in the dirt, i've said before i want things tested against reality not the happy path: some ips are ipv6, a few log lines are truncated mid-field, and timestamps drift by seconds across files. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. no silent absorption, each case logged with the rule that resolved it.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it. don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
