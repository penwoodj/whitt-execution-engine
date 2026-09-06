# Tier-A Case 096 — Courier Route Audit (HEAVY)

courier route audit, that's the theme, here's the full spec: build me a full working analysis pipeline, that's the job, workspace is /tmp/opencode/courier-run, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.

what you're working with: a day of courier stops, 60 deliveries with addresses as grid coordinates and time windows, plus the route as driven versus a recomputed better route, and distance metric is manhattan grid distance, state it, after that three deliveries missed their windows, that is the failure to explain. i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

concretely, do these stages, numbered so you don't creative-order them on me:

next as-driven total distance and window compliance, the three misses with their windows and arrival times. and be precise about what counts as done for that one, because vague is where shortcuts hide.
2. recomputed route, nearest-neighbor then 2-opt improve, both distances shown. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
stage by stage: compliance recheck under the new route, misses resolved or not. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
the trade-off analysis, distance saved versus window risk, with a recommendation. if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.
5. courier.md plus routes.json plus compliance.csv. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.

the realistic part, the warts: coordinates include two identical points for different deliveries, one time window is wider than the working day, and the depot appears twice under different names. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. every one of those needs a documented disposition, handled, quarantined, or rejected, with the rule cited.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end. don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.
