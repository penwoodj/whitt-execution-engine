# REA v1 — Design

**Input:** task case YAML = `{prompt, draft_response, auxiliary?, success_criteria}`.
**Output:** `final-answer.txt` + `select-best.json` (ratchet-selected best of
input draft vs enhanced rounds). Never worse than input (hard-fail gate).

## Pipeline (escalation cascade, all sequential)

```
                 ┌──────────────────────────────────────────────┐
                 │ STEP 0: INPUT GATE (zero-LLM)                │
                 │ check-answer.py on draft_response            │
                 │  all pass → early exit: ship draft (0 calls) │
                 └──────────────┬───────────────────────────────┘
                                ↓ fail
                 ┌──────────────────────────────────────────────┐
                 │ STEP 1: DECOMPOSER (1.7B, temp 0.3, ~300 tok)│
                 │ prompt only → 3-5 sub-questions JSON         │  §11, §3
                 │ NO draft visible                             │
                 └──────────────┬───────────────────────────────┘
                                ↓
                 ┌──────────────────────────────────────────────┐
                 │ STEP 2: SUB-SOLVER (4B, temp 0.2, ~500 tok)  │
                 │ answer each sub-question FACTORED —          │  §3, §11
                 │ sees: sub-questions + auxiliary. NO draft.   │
                 └──────────────┬───────────────────────────────┘
                                ↓
                 ┌──────────────────────────────────────────────┐
                 │ STEP 3: VERIFY-PLANNER (4B, temp 0.2, ~300)  │
                 │ draft claims → verification questions        │  §3
                 │ (what MUST be true if draft is right)        │
                 └──────────────┬───────────────────────────────┘
                                ↓
                 ┌──────────────────────────────────────────────┐
                 │ STEP 3b: TOOL-VERIFY (zero-LLM)              │
                 │ check-answer.py: arithmetic/format/counts    │  §5
                 │ on draft + sub-answers → verify.json         │
                 └──────────────┬───────────────────────────────┘
                                ↓
                 ┌──────────────────────────────────────────────┐
                 │ STEP 4: SYNTHESIZER (4B, temp 0.4, ~500 tok) │
                 │ sees ALL: prompt, sub-Q, sub-A, verify.json, │
                 │ open failures, draft (first time)            │
                 │ → enhanced-answer-r1.txt                     │
                 └──────────────┬───────────────────────────────┘
                                ↓
                 ┌──────────────────────────────────────────────┐
                 │ STEP 5: JUDGE GATE (zero-LLM)                │
                 │ check-answer.py on r1. All pass → SHIP.      │  §4, §10
                 └──────────────┬───────────────────────────────┘
                                ↓ fail (round 2 allowed, max 2 total)
                 ┌──────────────────────────────────────────────┐
                 │ STEP 6: ROUND 2 — DIVERSE RE-SYNTH           │
                 │ escalation memory (open failures + hints)    │  §6, §8
                 │ persona lens ≠ round 1, temp 0.7             │
                 │ → enhanced-answer-r2.txt → JUDGE GATE again  │
                 └──────────────┬───────────────────────────────┘
                                ↓ still fail
                 ┌──────────────────────────────────────────────┐
                 │ STEP 7: SELECT-BEST (zero-LLM, always runs)  │
                 │ ratchet: best of {draft, r1, r2} by          │  v8 L6
                 │ subchecks-passed, tie → earlier.             │
                 │ hard-fail classes in winner → no final,      │
                 │ mode=hard_fail_input_fallback               │
                 └──────────────────────────────────────────────┘
```

## Why each choice (evidence → mechanism)

| Mechanism | Evidence | RESEARCH.md |
|---|---|---|
| Step 0 early exit | ESC: cheap gates first | §2 |
| Decomposer separate from solver | DaSLaM, DialCoT: separate modules | §11 |
| 1.7B decomposes, 4B solves | correction-atom v8 cascade won 23/27 | in-house |
| Sub-solver blind to draft | CoVe factored variant | §3 |
| Tool-verify zero-LLM | T1: sLMs can't verify arithmetic | §5 |
| Synthesizer sees all once | CoVe final revise conditions on everything | §3 |
| Deterministic judge | Huang: intrinsic correction fails; Prometheus needs rubric | §4, §9 |
| Round 2 changes context not repetition | s1 "Wait" unreliable | §6 |
| Max 2 rounds | Reflexion Ω=1–3, Kaesberg decay | §8 |
| Ratchet select-best | v8 learning 6, FrugalGPT gate | §10 |
| No voting anywhere | majority vote hurts hard cases | §1 |

## Model roster (correction-atom v8 verified set)

| Role | Model | Sampling |
|---|---|---|
| Decomposer | Qwen3-1.7B-abliterated-q4_k_m | temp 0.3, max 300 |
| Sub-solver | Qwen3-4B-Instruct-2507-Q4_K_M | temp 0.2, max 500 |
| Verify-planner | Qwen3-4B-Instruct-2507-Q4_K_M | temp 0.2, max 300 |
| Synthesizer r1 | Qwen3-4B-Instruct-2507-Q4_K_M | temp 0.4, max 500 |
| Synthesizer r2 (persona) | Qwen3-4B-Instruct-2507-Q4_K_M | temp 0.7, max 500 |

No 9B anywhere (v8 lesson 3: 0/62 wins, ~20s wasted/attempt).

## Budgets (hard)

- ≤8 LLM inferences per case (worst path: 1+1+1+1+1 = 5; round 2 adds 1–2)
- ≤500 output tokens per pass; ≤300 for decomposer/planner
- Context per pass ≤2048 tokens (prompt + embedded state, trimmed by emit-case)
- Wall time ≤180s median, 300s watchdog hard kill
- Max 2 synthesis rounds; zero unbounded loops; sequential inference only

## Escalation memory (inherited from v8 emit-state)

Round 2 prompt injects `OPEN FAILURES (fix ALL)` — per-check latest failures
with `fix_hint` strings from check_lib — plus what round 1 already tried
(persona + temperature recorded in state). Each round targets actual
remaining defects, not generic "try harder".

## Case format (cases/case-00N.yml)

```yaml
case_id: rea-001
difficulty: easy            # easy | medium | hard
domain: arithmetic          # arithmetic | planning | constraint | analysis
prompt: |                   # the user question given to the small model
  ...
draft_response: |           # what the non-thinking model actually returned
  ...
auxiliary: |                # optional source of truth (facts, table)
  ...
success_criteria:
  deterministic_checks:     # run by check-answer.py (zero-LLM)
    contains_required: ["42"]        # substrings that must appear
    forbidden_phrases: ["I cannot"]  # must NOT appear
    min_words: 30
    max_words: 400
    bullet_count_min: 0
    bullet_count_max: 0
    yaml_parsable: false    # if true, output must parse as YAML/JSON
    numbers_must_sum_to:    # optional arithmetic gate: all numbers in
      null                  #   output must sum to value (T1-style check)
  reference_notes: |        # what a big model would say (for human review,
    ...                     #   NOT machine-checked in v1)
```

Difficulty rubric: easy = 1-2 step, single domain; medium = 3-4 step or
multi-constraint; hard = ≥5 step / compositional (LtM: decomposition helps
most here). 6 cases: 2 easy, 2 medium, 2 hard across domains.

## Determinism contract

- check-answer.py = ONLY arbiter of pass/fail. Pure stdlib Python 3, no
  network, no LLM. Same input → same output. Ported+trimmed from
  correction-atom check_angle_lib (attribution in file header).
- select-best.py exit 0 always; outcome recorded in JSON, never in exit code.
- emit-case.py failures are workflow bugs (fail loudly, fail_on_error: true).
- dry-run.py simulates every LLM step with canned fixtures → exercises full
  plumbing with zero models loaded.

## Known v1 limitations (deliberate, revisit after live run)

1. Judge is deterministic-only — rubric-quality dimensions (clarity,
   completeness vs. reference_notes) not machine-scored. v2 candidate:
   Prometheus-style 4B judge with reference. (§9)
2. Sub-solver answers all sub-questions in ONE pass (per-question passes =
   more calls; revisit if sub-answer quality poor — DialCoT says sequential
   per-question is better).
3. No draft-agreement fast-path between r1/r2 (both always run when round 2
   triggered). Revisit with live timing data.
4. Decomposer output validated structurally (JSON list of 3-5 strings), not
   semantically.
