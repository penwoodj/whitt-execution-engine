# Agentic Reasoning — Web Research Findings (2026-08-23)

Sources: Oracle dev blog (16-strategy benchmarks, Mar 2026), Multigrid
SLM guide (Aug 2026), 17-algorithm guide (Feb 2026), EdgeVox
plan-once post (Jun 2026), "Sample More, Reflect Less" (arXiv
2607.28576), ReAct/Plan-Execute/Reflexion comparisons, thought
calibration (EMNLP 2025), s1 budget forcing, early-abort cascade
(arXiv 2607.06503), RouteGoT (arXiv 2603.05818), Google BATS
(arXiv 2511.17006).

## 1. What works on CHEAP/SMALL models (core findings)

P1. **Sequential CoT first.** 88.7% avg vs 81.3% plain — +7pts from
    prompt structure alone, minimal latency. Default entry point.
P2. **Sample more, reflect LESS.** Landmark negative result
    (1.5B–7B, equal token cost): every self-assessment method
    (Self-Refine, forced Reflexion, Best-of-N self-pick) scored
    BELOW cost-matched repeated sampling in all 18 comparisons
    (-3.6 to -14.1pp). Majority-vote > self-critique for small models.
P3. **Reflexion silently collapses on small models.** Qwen2.5-1.5B
    judged itself correct on 100% of questions — retry never fired,
    method became plain CoT while reporting as Reflexion. External
    verification is mandatory; self-assessed retry gates are fiction
    below ~7B.
P4. **Plan once, dispatch deterministically.** Small/quantized models
    in ReAct loops degrade past ~6 hops ("sycophancy on chains":
    one good observation → premature victory lap). Plan-upfront
    executor: N+1 LLM calls → 2. ReWOO: 5× token savings + accuracy
    gain; LLMCompiler: 3.7× latency, 6.7× cost, +9% accuracy.
P5. **Decompose only separable problems.** Decomposed prompting
    scored 38.5% on GSM8K (worse than plain) — overhead without
    benefit on monolithic arithmetic. Wins on trip-planning,
    multi-part tasks where parts deserve focused attention.
P6. **ADaPT: try first, decompose on failure.** Decomposition depth
    naturally matches complexity; only break down what actually fails.
P7. **Constrained decoding / deterministic checks.** Format failures
    (top small-model production problem) stop being POSSIBLE, not
    rarer. Schema/regex/compiler verification beats model judgment.
P8. **Retrieve, don't recall.** Facts in context largely erases the
    capacity disadvantage.
P9. **Route by node role.** RouteGoT: strong models for planning +
    synthesis (globally coupled), light models for leaf subtasks
    (localized). +8.1pp accuracy, -79.1% tokens vs uniform AGoT.
P10. **Cheap verify, run twice.** "Small models fail more often and
     are much cheaper to run twice" — verification belongs to
     deterministic code, retries to sampling.

## 2. Efficiency meta-principles (the smart/less-common ones)

E1. **Budget forcing** (s1): append "Wait" continuation when token
    budget remains — model re-examines and fixes answers. Simple,
    measurable. Our dynamic-budget v8 already exploits the inverse.
E2. **Budget Tracker / BATS**: continuous in-context budget
    awareness changes agent behavior — dig deeper vs PIVOT decisions
    gated on remaining resources; synthesis reserve protected.
E3. **Early-abort cascade**: calibrated probes on hidden states
    predict eventual failure from round 1; abort doomed episodes
    early — 60.2%/54.9% token savings at 90% recall (TextCraft/
    WebShop, 1.7B-7B models). Analog for us: difficulty-classified
    dead-case skip (env-11/13 pattern), generalized.
E4. **Thought calibration**: consistency probes decide when thinking
    can stop — statistical early-stop for reasoning traces.
E5. **Synthesis reserve**: never let exploration spend the tokens the
    final answer needs (RouteGoT reserves max(φ·total, min)).
E6. **Difficulty-budget routing**: 3-class difficulty (cheap/med/
    full) predicted per node; hard nodes get recursion, easy nodes
    forbidden from it — prevents unproductive escalation.
E7. **State-aware re-prompts**: static "not done, continue" strings
    loop forever; re-prompts must read the action log and change.
E8. **Anti-sycophancy markers**: mandatory termination tokens,
    explicit anti-patterns in personas — mitigates but doesn't fix
    small-model early-victory bias.
E9. **Iter caps + convergence exit**: Reflexion 3-5 iters max;
    ToT 5-50× cost — reserve branching for problems where CoT <70%.
E10. **Compression hygiene**: strip old tool responses (BATS
     flag), summarize every N rounds — context discipline.

## 3. Technique taxonomy (pros/cons at small scale)

| technique | gain mechanism | cost | small-model verdict |
|---|---|---|---|
| CoT | structural prompt | ~1× | default, +7pp |
| Self-consistency | ensemble vote | k× | best ROI at equal cost (P2) |
| Self-Refine | self-critique | ~7 calls | NEGATIVE below 7B (P2) |
| Reflexion | memory of failures | 1-10 calls | silent collapse w/o external check (P3) |
| ReAct | tool grounding | N+1 calls | fragile >6 hops on small (P4) |
| Plan-and-Execute | commit upfront | 2 calls | small-model sweet spot (P4) |
| ReWOO | plan w/o observation | ~0.2× tokens | 5× savings, distills to 7B |
| LLMCompiler | DAG parallel | -3.7× latency | best for independent subtasks |
| ToT | branch search | 5-50× | only where CoT fails hard |
| ADaPT | adaptive decomposition | on-demand | prevents over-decomposition (P6) |
| Least-to-Most | easy-first ordering | k× | ordering aid for small models |
| RouteGoT-style routing | node-role model fit | -79% tokens | strong+light mix (P9) |
| Budget forcing | "Wait" continuation | +ε | cheap accuracy lever (E1) |

## 4. Implications for THIS experiment (REA+ v10 design)

I1. Replace repair-style critique chains with SAMPLING chains for
    same-cost retries (P2): second attempt = fresh sample w/ hints,
    not self-critique. Our extract/hint mechanisms already lean this
    way — formalize.
I2. External deterministic gates everywhere (already our pattern —
    P3/P7 validate it; spoof/spoof-emit IS the external verifier).
I3. Stage routing already implements P9 (4B primaries, Bonsai/Think
    rescue); add E6 difficulty classes: light cases forbidden from
    depth-2 escalation (early-exit GWT on case metadata).
I4. Dead-case skip = manual E3; generalize via difficulty ledger.
I5. Budget forcing for stuck thinking: append "Wait" continuation
    stage when a near-miss fails on one check (E1).
I6. Plan-once shapes: our decompose technique = plan+solver (2
    calls); avoid ReAct-shaped deep loops entirely (P4).
I7. Synthesis/extraction reserve: never spend the final extract
    budget on exploration (E5) — our fextract already does this.
