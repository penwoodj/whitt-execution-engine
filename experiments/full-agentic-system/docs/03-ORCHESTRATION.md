# Orchestration — Experiment-by-Experiment Iteration Protocol

How to walk the 12 sub-experiments from zero to refined, efficient v1
subworkflows. Follows S48's staged evaluation: offline replay → shadow →
live (user-gated). This phase covers stage 1 only (plus structural parts
of stage 2 via spoof batteries). Live stages gated on user approval per
PRIMARY-OBJECTIVE discipline.

## Execution order (dependency-driven, not numeric)

```
Phase A (independent foundations — any order, recommend this one):
  07-solve        ← control arm; its baselines calibrate every other experiment
  01-typed-envelope
  02-digest
Phase B (routing on top of A):
  04-intent-classify   (uses 02 digests + 07 lanes as its route targets)
Phase C (work acquisition):
  05-gather            (corpora; feeds 10)
  06-plan
Phase D (recovery chains):
  08-replan            (consumes near-miss artifacts from 07/02 runs)
  09-sample-diverse    (consumes stuck-set from 08)
  10-execute-observe   (consumes corpora + fault plans)
Phase E (meta + trust):
  11-verify-blind
  03-cache-addressing
  12-shape-library
```

Rationale: S46 waterfall — routing before retrieval before generation; control
(07) before treatment arms; recovery experiments need failure artifacts that
earlier phases produce.

## Per-experiment loop (max 3 iterations each, per QA cycle rule)

```
1. GENERATE   python3 scripts/gen-cases.py --exp NN-<name>
              (writes cases/, fixtures/; self-asserts word band, truth-vs-check,
               disjointness where applicable)
2. UNIT TEST  python3 scripts/test_hooks.py --exp NN-<name>
              (hook paths: spoof emit, stage emit, check, gates)
3. GENERATE   python3 scripts/gen-workflow.py --exp NN-<name> --spoof
              → workflows/v1-spoof.yml + run-dir scenario.json/shape.json
4. VALIDATE   python3 scripts/meta-v6/validate-workflow.py <spoof.yml>  → 9/9
5. SPOOF RUN  (engine dry run — no LLM; whitt not invoked in this phase;
              instead execute spoof battery via scripts/run-spoof-battery.py
              which walks the YAML gate graph in Python and replays
              spoof-emit at each shell hook)
6. INSPECT    win-depth distribution vs scenario design; failure taxa = none
              expected (spoof passes by construction at win depth)
7. GATE       all green → experiment's v1 declared STRUCTURALLY SOUND;
              record in docs/design.md status block
8. (LATER, user-gated) live run per design.md's live-variant section
```

### Iteration triggers (what makes a v2)
- Generator self-check fails → fix engine, not the assert (truth is law).
- Unit test fails → fix hook script.
- validate-workflow < 9/9 → fix generator emission.
- Spoof battery deviates from scenario design (e.g. skip fires on trap) →
  fix gate logic.
- Token/cost accounting violates design bound → redesign stage shapes.

### Escalation
3 failed iterations on one experiment → stop, write findings to its
docs/design.md "Blocked" section, move to next experiment, surface to user
at report time (S48: acceptance criteria pre-registered; failure is data).

## Refinement to "efficient subworkflow" (post-live, future)

An atom graduates to the shape-library (12) when:
1. Its experiment's H-criteria pass live.
2. Ablation shows atom-REMOVED variant regresses ≥ its claimed effect size.
3. Token cost per case ≤ its design bound.
4. Its workflow YAML is re-emitted as a parameterized template entry in
   `12-shape-library/shapes/` with hash-match metadata.

Graduated atoms compose the meta-workflow-generator replacement per
`00-SYSTEM-DESIGN.md`: SW1-SW5 collapse into shape-match → re-bind →
assemble → validate, with 04 as the intake gate.

## Bookkeeping

- Every run writes ledgers (outcomes.jsonl, fingerprint.jsonl) — S42 traces.
- docs/design.md status blocks updated per loop pass (STRUCTURAL / LIVE-PASS /
  LIVE-FAIL / BLOCKED).
- No cross-experiment file deps except cases-shared/corpora (05,10) and
  shared scripts/.
