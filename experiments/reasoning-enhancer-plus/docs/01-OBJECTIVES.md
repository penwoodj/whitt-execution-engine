# REA+ — Objectives (priority order)

## O0 — Prerequisite: pick 5 specialists (stage 1)

Best model per category from installed 4B–11B pool, live-probed:

| cat | name | failure class it must beat (from STRUGGLE-MAP + critique) |
|---|---|---|
| FMT | exact-IO formatter | char-exact pairs, envelope fields, no decoration |
| DRV | multi-hop deriver | 3+ hop arithmetic, unit conversion, boundary values |
| LOG | constraint logician | negation filters, policy stacks, unique-vs-total |
| PLN | planner | ordered runbooks, dependency chains, budget split |
| AUD | auditor | false-premise rejection, ambiguity flags, cross-checks |

Scoring: category probe accuracy (primary), wall time (tiebreak),
load time (penalty). One model may win ≤2 categories; 5 distinct picks
only if pool supports it — else document fewer.

## O1 — 30-case gap-band suite (stage 2)

Cases MUST sit in gap band: `REA-small fails AND REA+ big passes`.
Realistic = public-bench archetypes (GSM8K multi-hop, DROP reading-arith,
ARC-style elimination, StrategyQA implicit-multi-hop, IFEval complex
instruction stacks, b39-class envelopes with CORRECT math). Full spec:
`04-CASE-SUITE-SPEC.md`. Authoring blocked by stage-1 gate.

## O2 — Multi-dimensional metrics (no single-lens)

8 dimensions (M1–M8), defined in `02-METRICS.md`. Every iteration run
reports ALL dimensions cheaply, TARGETS exactly one.

## O3 — One-metric-at-a-time TDD iteration

Each workflow version vP{n} = one metric objective, red→green:
lint → dry-run → single-case pulse → 6-case slice → 24-case train.
Never jump to full suite from red. Full ladder in `05-TDD-ROADMAP.md`.

## O4 — Fast iteration machinery

- Smallest-portion-first: every live run starts at 1 case / 1 model
- Prompt-input fingerprinting: every step logs case_id+step+sha1(prompt)
  to fingerprint.log — failures traceable to exact input in seconds
- Resume-skip: reruns skip completed items (REA oc-v3.1 pattern)
- Model sweep batches capped at 5 loads → mandatory cooldown+RAM gate

## O5 — Ablations (close critique gap #3)

Three arms on final suite, same 30 cases:
- A: full scaffold (cascade + rubric + retries + repair)
- B: retry-only (single picked model + dumb retry loop, no rubric/cascade)
- C: rubric-only (single model + checks-as-rules, no retry)
Report deltas. Without B/C, scaffold contribution is unmeasured.

## O6 — Held-out generalization

24 train / 6 held-out. Checks NEVER tuned on held-out. Final report
shows both. Held-out authored last, after train suite stabilizes.
