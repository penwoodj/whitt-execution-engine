#!/usr/bin/env python3
"""Tier-A test case generator. Skeleton-driven, seeded, user-voice renderer."""
import random, re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tc_skeletons import S

DROP = {
    "dues-reconciliation": "third reconciliation-mechanics case, invoice+bank+subscription cover it",
    "vocabulary-frequency": "overlaps corpus-readability metrics story",
    "path-normalization": "canonicalization covered by authorship + catalog cases",
    "gradebook-rounding-policy": "second gradebook case, boundaries case covers it",
    "lunch-rotation": "lightweight scheduler, volunteer-shift-solver covers constraints shape",
    "camp-ground-booking": "second booking-constraints case, hotel covers it",
    "purity-checker": "overlaps near-duplicate-docs detection story",
    "uptime-heatmap": "overlaps slo-attainment + golden-signal aggregation",
    "toc-generator": "smallest LIGHT case, linkrot covers docs-repair shape",
    "bibliography-normalize": "fourth normalize/dedupe-text case, glossary covers it",
}
SKELS = [x for x in S if x[0] not in DROP]
assert len(SKELS) == 97, f"expected 97 after drop, got {len(SKELS)}"

OPENINGS = [
 "i want you to {frame} for me, the whole thing lives under /tmp/opencode/{folder} and nothing outside that folder gets touched, i mean it, no repo files, no home directory surprises, i've cleaned up after tools that did that before and i'm not doing it again.",
 "so {frame}, fully self contained in /tmp/opencode/{folder}, no network, no installs, standard library python or plain bash whichever you'd actually reach for, and everything deterministic or seeded so a rerun gives me the same universe bit for bit.",
 "here's the job: {frame}, all under /tmp/opencode/{folder}, and before you ask, yes the data is fake and you'll be creating it yourself, because i want this testable end to end without touching anything real.",
 "i've got {frame} to do and i want it done properly, which means /tmp/opencode/{folder} for everything, real scripts i can rerun, real files on disk, not a description of what would happen.",
 "what i want is {frame}, set up inside /tmp/opencode/{folder}, and i want it treated like production work even though it's synthetic, because the point is the discipline, not the data.",
 "i need {frame} done in /tmp/opencode/{folder}. everything you need is in this prompt, and where it isn't, the gaps are deliberate so you make the call and write down what you picked, i don't want questions about file names.",
 "{frame}, that's the job, workspace is /tmp/opencode/{folder}, and read the whole thing before starting because the constraints at the end apply to every step, not just the last one.",
 "let's do {frame}, entirely inside /tmp/opencode/{folder}, no llm calls, no network, nothing stochastic unless i say it's seeded, and at the end i want to be able to point a skeptical stranger at the outputs and have them check every claim.",
]
FRAMES = {
 "HEAVY": ["build me a full working analysis pipeline", "run a proper multi-stage data job", "put together an end-to-end processing exercise"],
 "SYNTH": ["build a small but complete tool with its own verification", "construct a self-contained reporting exercise", "assemble a compact analysis-and-audit job"],
 "LIGHT": ["do a focused single-purpose task with a clean report at the end", "run one straightforward job and hand me the summary", "do a tidy single-pass piece of work"],
}
CTXLEAD = [
 "the setup is this:", "here's the situation:", "context so you're not guessing:", "the shape of the data:", "background, and i'm giving you the details the way they actually occur:",
 "so the scenario:", "what you're working with:", "the world of this little exercise:",
]
ASKINTRO = [
 "the asks, in order, and each one writes its own output so i can see where things broke when they break:",
 "concretely, do these stages, numbered so you don't creative-order them on me:",
 "what i need done, in this order:",
 "the work, stage by stage, and i want stage logging loud enough that i can follow along after the fact:",
 "here's the task list, in sequence:",
]
VERIF = [
 "and the verification part is not optional, i've been burned by tools grading their own homework. write a separate verifier that recomputes the key numbers from the raw inputs with its own logic, not by importing the pipeline's code, and it exits nonzero with a clear message on any mismatch. run it against the outputs and show me the exit codes. a claim without a number i can check is a vibe and i've had enough vibes.",
 "verification built in, not bolted on. an independent check script, separate logic, recompute from raw, compare field by field, exit nonzero on any drift. counts have to add up too, every row you started with lands somewhere explained, nothing just vanishes into a silent drop. run it, show me the output, don't summarize it for me.",
 "i want a verify pass that would catch me lying. separate script, recomputes independently, strict comparisons, and it checks the invariants too, the things that must be true about the outputs no matter what the data says. if the verifier passes first try i still want its output pasted, not paraphrased.",
 "verification is a first-class deliverable here. independent recomputation from the raw files, its own code path, and a conservation check that everything entering the pipeline is accounted for at the end, kept, rejected, merged, flagged, the books have to balance. nonzero exit on failure, actual output shown in your report.",
 "and write the checker like a hostile reviewer. independent logic, recompute everything checkable from raw inputs, verify the stated invariants mechanically, and flag anything that only holds because the pipeline says so. exit codes visible. i don't trust should-pass and neither should you.",
]
GATES = [
 "don't stop halfway to ask me questions you can answer yourself. names, orderings, column formats, those are yours. only come back at a real fork that changes what i asked for, and there isn't one hiding in here.",
 "style rules since they keep coming up: nothing interactive anywhere, no prompts, nothing that hangs a non-interactive shell, loud failures with nonzero exits, and everything runnable twice without exploding. pick idempotent or self-cleaning, say which, document it.",
 "don't skip steps and don't tell me it's done until the verifier has passed and you've actually looked at the outputs yourself. and keep it dependency free, standard library only, this machine gets cranky.",
 "assume defaults for anything unspecified and note what you picked in a decisions file, i'd rather read three lines of your reasoning than answer three questions. don't stop until every stage above is done and verified.",
 "no network, no installs, no llm calls, deterministic or seeded everywhere, and if any stage is slow relative to its work say so honestly instead of pretending it's fine. don't stop until the whole sequence is verified end to end.",
]
CLOSINGS = [
 "when you're totally done, give me a short summary, what's where, file paths, run times, and the one line on anything that surprised you.",
 "at the end i want the inventory: what lives where, the numbers, and one honest paragraph on where this would break first if i stressed it harder.",
 "finish with the report pasted in chat so i can skim without opening files, plus paths and timings. one line on what surprised you.",
 "when it's done and verified, hand me the map: files, numbers, timings, and anything you'd do differently if this were real.",
 "end state: verified outputs, a short readme pointing at everything, timings, and your one honest surprise.",
]
ASIDES = [
 "and a note on tone in the outputs: plain sentences, tables where tables belong, no buzzword garbage, i want to skim this in two minutes and know exactly what happened.",
 "one more thing, every number you quote me at the end better come from an actual run you can point at, because should-be-roughly-nine is not a result, it's a vibe.",
 "if any rule you write down feels arbitrary, good, that means you noticed, write the threshold and the reason next to it so future-me can argue with it.",
 "logs per stage, one line each is plenty, i want to see where time went and where things broke without spelunking.",
 "and if you find something in the data that contradicts what i said above, the data wins, report the contradiction, don't quietly bend either one.",
 "timings, real ones, timestamps around each stage, a table at the end. not vibes.",
 "keep the rules you invent in a rules.md next to the scripts, so when i come back in a week i know why the machine did what it did.",
 "and when i say documented i mean in a file on disk, not in the chat log where it scrolls away forever.",
 "if two stages could run in either order and it doesn't matter, pick one and move on, i don't need a committee meeting about it.",
 "error messages should say what failed and where, not just that something did, because 'an error occurred' has never helped anybody.",
 "everything gets a run twice test before you call it done, because the first run always works and the second one is where the state bugs live.",
 "and no leaving a to-do in a comment for later-me, later-me is you in ten minutes, finish the job.",
 "i'd rather have an ugly table that's right than a beautiful chart that's approximate, so default to tables unless the data genuinely needs a picture.",
 "if a stage can honestly be a one-liner, let it be a one-liner, padding steps to look thorough is its own kind of lie.",
 "the workspace layout is yours to design but say it in the readme, so the structure is a decision not an accident.",
 "when a rule and a row disagree, both go in the output, the flagged row and the rule that flagged it, side by side.",
 "percentages need denominators next to them always, twelve percent of what, because a percentage alone is half a number.",
 "anything you cache or skip for speed gets a note, because invisible shortcuts are how results stop being reproducible.",
]
FALLBACK = [
 "to say the obvious one more time because it matters: the raw inputs stay untouched, everything you write is a new file, and nothing anywhere in this job needs elevated anything. if a step feels like it wants sudo, that step is wrong, rethink it.",
 "and scope discipline: this prompt is the whole spec, no inferring extra features you think i'd like, no scope creep dressed up as thoroughness. build exactly what's listed, verify it, and stop. extras belong in a future-list file, one line each, not in the build.",
 "one more pass on honesty: if some part of this turns out duller or simpler than it looks, fine, report that. and if some part turns out genuinely hard, say that too, with what made it hard. i read reports for the truth, not for morale.",
]
CTXEXP = [
 "the whole point of this exercise is that the working files and the checkable outputs exist on disk where i can poke at them, so every stage leaves real artifacts, and the artifacts are the deliverable, not the chat narration around them. if a step produces nothing i can open, it didn't happen.",
 "think of this as a miniature version of the real jobs i hand off: messy inputs, stated rules, honest outputs. the rules you invent matter as much as the numbers you compute, because i'm going to reuse this when the data is real and i need to trust the machinery.",
 "i'm deliberately not specifying every little thing, because half the value here is seeing what gets decided when nobody's looking. the decisions file is where you show your work on those, and a wrong call documented beats a right call nobody can find.",
 "the deliverables have two audiences, me skimming on a phone and a verifier grinding line by line, so every report needs the human table up top and the machine copy underneath, same numbers, no drift between them. the day those two disagree is the day i stop trusting the whole setup.",
 "this is the kind of task where the tempting move is to demo the happy path and gesture at the rest. don't. the mess is specified precisely because that's where the value is, and a clean run on clean data tells me nothing about whether the thing works.",
]
OUTSHAPE = [
 "on outputs: every stage writes to its own subfolder under the workspace, naming is yours but be consistent, and the final report goes both human, markdown with actual tables, and machine, json with the same numbers. i'll be checking that the two agree, and i'll be unhappy if they don't.",
 "shape of the deliverables: scripts in a scripts folder, outputs in an outputs folder, the rules and decisions each in their own file at the root of the workspace, and a final report i can read start to finish in two minutes. paths in the report, not just filenames, so i can go look.",
 "the end state on disk should look like something a stranger could pick up: a readme pointing at everything, scripts that run in order, outputs that reproduce, and the report. if a stranger couldn't rerun this from the files alone, it's not done.",
]
ASkelEXP = [
 "and be precise about what counts as done for that one, because vague is where shortcuts hide.",
 "with the method visible, not just the result, the how is the deliverable here as much as the what.",
 "actual numbers in the output, computed, not eyeballed, and traceable back to input rows.",
 "and that one gets checked by the verifier too, it's not just a produce-and-hope step.",
 "if that stage has a natural ordering dependency on the previous one, respect it, the sequence above is not decorative.",
 "edge cases belong in the output, not in your head, list what you hit and what you did with each.",
 "and don't round anything into meaninglessness, i'd rather see the ugly precise number than a tidy lie.",
]
QUIRKSUFFIX = [
 "the handling rule for each goes in the rules or decisions file, not in your memory.",
 "how you handled each one gets written down where i can find it later.",
 "each mess instance gets its handling logged, rule named, no exceptions.",
 "the rule you applied to each wart belongs in the rules file, next to the wart itself.",
 "write down what you did with each defect and why, in the file, not the chat.",
 "every one of those needs a documented disposition, handled, quarantined, or rejected, with the rule cited.",
 "the decisions file gets one line per mess type saying how you dealt with it.",
 "no silent absorption, each case logged with the rule that resolved it.",
]
QUIRKEXP = [
 "those aren't obstacles, they're the actual test, a clean run on clean data is worthless and i've said this before.",
 "handle each one explicitly, log it, and count them, because 'some rows had issues' is not a finding, 'fourteen clock-skewed entries corrected, listed in the log' is.",
 "i want each mess instance visible in the output somewhere, surfaced not absorbed, so i can audit the handling later.",
]
QUIRKLEAD = [
 "now the mess, because a pipeline that only works on clean data is worthless to me:",
 "the data has problems, deliberately, and handling them is part of the job not an error condition:",
 "and bake in the dirt, i've said before i want things tested against reality not the happy path:",
 "the realistic part, the warts:",
 "messiness requirements, on purpose, all of these must be handled explicitly not silently absorbed:",
]

def rng_for(i): return random.Random(24001 + i * 7919)

def flow_join(items, rng):
    out = []
    for k, it in enumerate(items):
        it = it.strip()
        if k == 0:
            out.append(it)
        else:
            c = rng.choice([" and ", ", then ", ", after that ", ", plus ", "; also ", ", and ", ". then "])
            out.append((c if c != ". then " else ". ") + it)
    return "".join(out)

LEADS = [
 "the subject this time is {t}, and i want it done like i mean it.",
 "{t}, that's the theme, here's the full spec:",
 "new job, {t}, same standards as always.",
 "this one is about {t}, read it all before touching anything.",
 "{t}. been meaning to get this done properly for a while.",
 "the task is {t}, and the details are below, all of them.",
 "another one for the pile: {t}, with the usual rules.",
 "{t} is the job, and i want the work to show for it.",
 "today it's {t}, and no, you don't get to improvise the scope.",
 "next up, {t}, whole thing specced below.",
]

def render(idx, sk):
    slug, folder, lane, ctx, asks, quirk = sk
    rng = rng_for(idx)
    words_target = rng.randint(1030, 1250)
    header = f"# Tier-A Case {idx+4:03d} — {title_case(slug)} ({lane})\n\n"
    op = rng.choice(OPENINGS).format(frame=rng.choice(FRAMES[lane]), folder=folder)
    op = __import__("re").sub(r"let's do (do|build|run|put|construct|assemble) ", lambda m: "let's " + m.group(1) + " ", op)
    lead = rng.choice(LEADS).format(t=title_case(slug).lower())
    paras = [header + lead + " " + op]
    ce1, ce2 = rng.sample(CTXEXP, 2)
    paras.append(rng.choice(CTXLEAD) + " " + flow_join(ctx, rng) + ". " + ce1)
    paras.append(ce2)
    body = []
    for j, a in enumerate(asks):
        a = a.strip()
        if rng.random() < 0.5:
            body.append(f"{j+1}. {a}. {rng.choice(ASkelEXP)}")
        else:
            lead = rng.choice(["first ", "next ", "then ", "after that ", "stage by stage: ", ""])
            body.append(lead + a + ". " + rng.choice(ASkelEXP))
    paras.append(rng.choice(ASKINTRO) + "\n\n" + "\n".join(body))
    paras.append(rng.choice(QUIRKLEAD) + " " + quirk.rstrip(".") + ". " + rng.choice(QUIRKEXP) + " " + rng.choice(QUIRKSUFFIX))
    paras.append(rng.choice(OUTSHAPE))
    paras.append(rng.choice(VERIF))
    gates = rng.choice(GATES)
    extra = rng.choice(GATES)
    while extra == gates: extra = rng.choice(GATES)
    paras.append(gates + " " + extra)
    paras.append(rng.choice(CLOSINGS))
    aside_pool = ASIDES[:]
    rng.shuffle(aside_pool)
    fb_pool = FALLBACK[:]
    rng.shuffle(fb_pool)
    text = "\n\n".join(paras)
    while wordcount(text) < words_target and aside_pool:
        a = aside_pool.pop()
        pos = rng.randint(2, len(paras) - 1)
        paras.insert(pos, a)
        text = "\n\n".join(paras)
    while wordcount(text) < 1000 and fb_pool:
        a = fb_pool.pop()
        pos = rng.randint(2, len(paras) - 1)
        paras.insert(pos, a)
        text = "\n\n".join(paras)
    return text

def title_case(slug):
    return " ".join(w.capitalize() for w in slug.split("-"))

def wordcount(t):
    body = "\n".join(l for l in t.splitlines() if not l.startswith("#"))
    return len(re.findall(r"[A-Za-z0-9][A-Za-z0-9'\-.,;:()%]*", body))

def main():
    base = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "test-cases", "tier-A-1000plus")
    os.makedirs(base, exist_ok=True)
    rows = []
    for i, sk in enumerate(SKELS):
        t = render(i, sk)
        w = wordcount(t)
        if w < 1000 or w > 1500:
            print(f"WARN {sk[0]}: {w} words out of range", file=sys.stderr)
        name = f"case-A-{i+4:03d}-{sk[0]}.md"
        with open(os.path.join(base, name), "w") as f:
            f.write(t + "\n")
        rows.append((name, sk[2], w, sk[0]))
    print(f"generated {len(rows)} tier-A cases 004-100")
    lows = sum(1 for r in rows if r[2] < 1000)
    highs = sum(1 for r in rows if r[2] > 1500)
    print(f"out of range: {lows} low, {highs} high")
    print(f"word range: {min(r[2] for r in rows)}-{max(r[2] for r in rows)}")

if __name__ == "__main__":
    main()
