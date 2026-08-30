# Principle-Fusion — Unified Validation Experiment

**Created:** 2026-08-24 · **Status:** RESEARCH + ARTIFACTS COMPLETE,
ZERO-LLM VERIFIED · **Live A/B:** NOT RUN (awaits GPU window + user go)

## What this experiment is

Every principle this repo has proven live (5 prior experiment folders)
plus every upgrade the 2026-08-24 deep web research surfaced, fused
into ONE workflow shape (`fusion-v1`) and ONE case suite (12 cases,
1000-1150 words each), designed so each principle is exercised in
combination with the others and validated by a deterministic check
that fails without it.

**This phase is documentation + case authoring + workflow YAML
creation + zero-LLM unit tests only.** No model loads, no inference,
no engine runs. The spoof/live battery and unit tests prove structure;
the live A/B procedure is specified but gated on user approval.

## Hypothesis under test (when run live)

> A deterministic engine that owns ALL procedural responsibility,
> with small models doing only narrow-scope localized inference
> between deterministic boundaries, achieves ≥ the accuracy of any
> single larger model on 1000+ word adversarial agentic cases, at
> bounded cost — and removing ANY single principle from the fusion
> shape measurably hurts a specific case class designed to need it.

Second-order hypothesis (ablation): the 12 cases partition into
principle-dependency classes; ablating principle X fails exactly the
cases whose `needs` metadata names X (see `docs/02-CASE-SUITE-SPEC.md`).

## The fusion pipeline (per case, H difficulty)

```
prompt (1000+ words)
  │
  ▼
[DETERMINISTIC] digest — core+spec-tail slice (~160 words to model)
  │                DOS-RAG structure preservation (E10, P8)
  ▼
[DETERMINISTIC] classify — variant-level difficulty + channel (E3/E6)
  │                (live phase: conf pre-gate from logprobs, N3)
  ▼
 L ──────────────► solve(4B, proven CoT prefix) ─► extract ─► judge-lane
 H: plan(Think) ─► solve(4B) ─► extract(harvester)
        │ fail     │ fail
        ▼          ▼
    replan gate (N4: invalidation-detecting, cheap)
        │
    solve2(4B, hints=check-failures only — leak-safe N6)
        │ fail
    wait(4B, 400tok — E1 budget forcing, LAST cheap rung, bounded)
        │ fail
    sample2(Bonsai-27B — P2 cross-model fresh sample, never self-critique)
        │ fail
    verify(Think 3500, backward-check — E9: most expensive runs last)
        │
    every extract = harvester w/ check-before-echo (N2: reason free,
      constrain late), reads reasoning TAIL 4000, JSON-anywhere,
      digit-garble repair
        │
    two-lane verdict (N5): deterministic check OVERRIDES judge;
      judge = separate-family blind model, logged, never load-bearing
        │
    ledgers (outcomes.jsonl + fingerprint.jsonl) at EVERY gate;
    quoted routing tokens; skip_step terminal (zero-call)
```

Light cases are structurally pruned (E6/AdaptMI): deep stages DO NOT
EXIST in the YAML for L cases — they cannot burn tokens there.

## Principles incorporated (23 total — full map in docs/01)

| # | Principle | Fusion mechanism |
|---|---|---|
| P1 | CoT first, stable prefix | solve/solve2/sample2 use the exact v13-proven "Think step by step..." wording — v14c proved prefix flips values |
| P2 | Sample more, reflect less | sample2 = fresh cross-model sample; NO same-model critique anywhere |
| P3 | External verification only | every gate deterministic; no model judges its own output |
| P4 | Plan once, dispatch | H-only plan stage; execution = deterministic engine steps |
| P5 | Decompose only separable | decomposition removed (0/8 live); plan = ordering scaffold only |
| P6 | ADaPT try-first | rescue chain exists only on deterministic failure |
| P7 | Deterministic checks | check_lib at every checked stage; two json_exact gates |
| P8 | Retrieve don't recall | digest puts facts verbatim in context; no recall demanded |
| P9 | Route by node role | 4B primaries, Bonsai deep sample, Think verify/plan only |
| P10 | Cheap verify, run twice | extract (150tok 4B) before every expensive escalation |
| E1 | Budget forcing (bounded) | wait stage, final cheap rung only, per ICLR-blog bounds |
| E3/E6 | Difficulty gating | variant-level H/L; L stages structurally absent |
| E9 | Verify last | harvest-before-verify; Think 3500 runs last |
| E10 | Context hygiene | digest offload: ~160-word core regardless of case length |
| N1 | Structure removes responsibility | engine owns procedure; model = localized inference only |
| N2 | Reason free, constrain late | harvester: unconstrained solve → deterministic extract |
| N3 | Confidence pre-gate | fusion_conf.py logprob router (live-phase hook, unit tested) |
| N4 | Replan on invalidation | replan gate between extract-fail and solve2 (63→87% external) |
| N5 | Two-lane verdict | det overrides judge; judge logged never load-bearing |
| N6 | Leak-safety | hints = check id + observed + question; contract echo only |
| N7 | Format-consistent scaffolds | exact-token answer menus; quoted routing tokens |
| N8 | Worked examples, diff numbers | per-engine worked micro-example w/ different numbers in expansion |

## Case suite (12 cases, 6 engines x 2 variants)

quota-banking, retry-backoff, canary-gates, model-residency (our own
domain), priority-preemption, epistemic-precedence. Every prompt
1000-1150 words in the user's voice (comma-spliced imperatives,
doubled-back rule restatements, lore provenance, drift warnings,
pre-commit recheck tails). Truths COMPUTED by executing the stated
rule arithmetic in the generator — never hand-typed — and self-verified
against check_lib before anything ships. Each case carries `needs:`
metadata naming the principles it stresses (drives the ablation
matrix). See `docs/02-CASE-SUITE-SPEC.md`.

## Zero-LLM verification status (this phase)

- 12/12 truths self-verified via check_lib (`test_hooks.py`)
- 12/12 prompts word-count enforced ≥1000 programmatically
- digest: core retains every number/trap, expansion stripped
- harvester check-before-echo: clean JSON echoed, dirty rejected
- conf router: entropy threshold routing unit tested on fixtures
- generator: spoof + live YAMLs both validator-PASS
- spoof scenario: win-depths deterministic, exact, per-case asserted
- ledgers: outcomes + fingerprint written at every gate invocation
- Run: `python3 experiments/principle-fusion/scripts/test_hooks.py`

## Layout

```
experiments/principle-fusion/
├── README.md                  ← this file
├── docs/
│   ├── 00-SCOPE.md
│   ├── 01-PRINCIPLES-MAP.md   ← principle → stage → case → check
│   ├── 02-CASE-SUITE-SPEC.md
│   ├── 03-WORKFLOW-DESIGN.md
│   ├── 04-VALIDATION-PLAN.md  ← live A/B procedure (gated on user go)
│   └── web-research/FUSION-SOURCES.md  ← 36 annotated sources
├── cases/       case-fu-01..12.yml (generated, committed)
├── fixtures/    fusion-fixes.yml (computed truths + derivations)
├── scripts/
│   ├── fusion_lib.py          shared: digest, harvester, conf, ledger
│   ├── gen-fusion-cases.py    case/truth generator (computes truths)
│   ├── gen-fusion-workflow.py emits spoof + live workflow YAMLs
│   ├── fusion-stage-emit.py   live-mode gate: skip-if-won, styles, priors
│   ├── fusion-spoof-emit.py   spoof-mode gate: scenario-driven artifacts
│   ├── fusion-conf.py         logprob-entropy confidence router (N3)
│   ├── collect-fusion.py      summary.json from ledgers
│   └── test_hooks.py          20+ zero-LLM unit tests
├── workflows/   fusion-v1-spoof.yml · fusion-v1-live.yml
└── results/     (empty — populated only by future runs)
```

## Live A/B procedure (DO NOT RUN without user approval)

Specified in `docs/04-VALIDATION-PLAN.md`: spoof engine battery →
seeded live-gate run → real live run via `safe-launch.sh` → ablation
runs (principle-removed variants) → comparison vs 9B single-shot
baseline on the same 12 cases. Success = fusion ≥ baseline on all 12
+ ablation failures land on predicted `needs` classes.
