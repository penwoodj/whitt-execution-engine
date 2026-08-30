# Tier-A Case 010 — Queue Lag Report (HEAVY)

What i want is put together an end-to-end processing exercise, set up inside /tmp/opencode/queue-run, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.

the setup is this: a message queue with four consumers, topic has partitions, then lag snapshots every minute for two days, consumer offsets vs log end offsets, and one consumer called worker-2 keeps falling behind in bursts, after that rebalance events happened three times and may explain jumps. the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.

and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.

i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.

error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.

timings, real ones, timestamps around each stage, a table at the end. not vibes.

the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.

what i need done, in this order:

1. lag per consumer per partition over time, max, mean, and the burst windows. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.
then correlate lag spikes with the three rebalance timestamps. with the method visible, not just the result, the how is the deliverable here as much as the what.
3. throughput estimate per consumer from offset deltas, messages per minute. actual numbers in the output, computed, not eyeballed, and traceable back to input rows.
first verdict on whether worker-2 is slow or just stuck with a hot partition. and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.
then lag-report.md and lag.json, plus a one-paragraph honest methodology note. with the method visible, not just the result, the how is the deliverable here as much as the what. with the method visible, not just the result, the how is the deliverable here as much as the what.

percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.

if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.

and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.

if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.

the realistic part, the warts: some snapshots are missing entirely for a 20-minute window and offsets occasionally go backwards. i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later. the handling rule for each goes in the rules or decisions file, not in your memory.

shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.

anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.

i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.

one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.

keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.

everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.

when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.

the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.

verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.

logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.

don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here. style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.

and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.

and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.

if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.

when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.
