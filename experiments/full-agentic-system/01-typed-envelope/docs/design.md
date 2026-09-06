# 01-typed-envelope — Experiment Design

## Atom
**typed-envelope** — every inter-stage message is a typed record
{intent, content-ref, confidence, error-class}, never free text.

## Hypotheses (pre-registered)
- **H1 (format reliability):** with the envelope contract stated in the prompt
  (conditions B/C), FORMAT-class failures drop to ~0 vs condition A (plain).
  Rationale: S41 (schema cuts interface misuse), P7 (constrained output),
  N1 (reason free, constrain late — the envelope is declared, the solve stays
  unconstrained, extract packages).
- **H2 (no content tax):** content accuracy (values correct) does not differ
  across A/B/C beyond noise. If B/C lose content accuracy, the envelope is a
  constraint-tax violation (S: constraint-tax) and must be re-scoped to the
  extract stage only.
- **H3 (error-class payoff):** condition C (error-class menu in contract)
  improves recovery speed: near-miss cases (win-depth ≥1) recover at an
  earlier stage under C than B, because failure hints classify cleanly
  (S38 fault taxonomy → recovery mapping).

## Cases
te-01..te-36 = 6 engines × 2 variants × 3 idx. idx%3 selects condition:
- A: plain contract (no envelope language)
- B: envelope-note (typed-record statement added to expansion)
- C: error-class-menu (envelope + enumerated failure classes)

## Truth derivation
Engine simulation (fas_engines.gen) — same truths across conditions; the
manipulation is prompt presentation only. Truths identical → any accuracy
delta is attributable to presentation, not task difficulty.

## Workflow (v1)
- H lane: solve(fmt,cot) → extract(fmt) → wait(fmt) → extract2(fmt) → judge(think, blind)
- L lane: solve(fmt,cot) → extract(fmt) → judge(think, blind)
- CHECK_HOOK_STAGES: extract, extract2 (live)
- SCENARIO buckets [5,3]: ~5/8 win at solve, ~3/8 need wait+extract2

## Pass/fail criteria
- Atom PROVEN if live run shows: FORMAT failures B,C < A by ≥half AND content
  accuracy B,C ≥ A − 3pp.
- Atom REFUTED if envelope conditions lose ≥5pp content accuracy (tax) or
  no format gain.
- H3 evaluated from ledger win-depth by condition.

## Ablations (atom-REMOVED mandatory)
- A-1: envelope removed (all cases condition A) — expected: format failures rise.
- A-2: envelope at solve stage instead of extract — expected: content tax
  (reasoning constrained early = constraint-tax pattern).

## Live variant plan
v1-live.yml ready. Models: fmt for solve/extract, think for judge. Analysis:
group check-*.json failure taxa (FORMAT vs CONTENT) by condition, chi-square.
