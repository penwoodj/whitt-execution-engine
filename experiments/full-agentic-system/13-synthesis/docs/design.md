# 13-synthesis — Experiment Design

## Atom
**synthesis (orchestrator-workers merge)** — combine independent worker
outputs into one contract; detect part-conflicts; re-derive only the
broken part. Workers never see each other (isolation, S53).

## Hypotheses (pre-registered)
- **H1 (merge correctness):** merged-object pass rate on conflict-free
  cases ≥ single-engine baseline − 3pp (merge is not a tax).
- **H2 (conflict handling):** on worker_b-delta cases (1/3 band), the
  resolve→solve_b2 path recovers ≥50% — synthesis must NOT blind-merge
  a wrong part (AgentPatterns: "synthesis is a reasoning step, not
  aggregation").
- **H3 (isolation value):** worker prompts carry exactly one engine's
  rules (split routes parts; workers get part-digests) — measured via
  fingerprint ledgers; cross-part digit leakage in answers ≤10%.

## Cases
sy-01..sy-30 = 3 engine pairs × 2 variants × 5 idx. Prompt holds BOTH
cores (part A + part B rules), ONE worked example (part A). Contract =
merged object {part_a: {...}, part_b: {...}}; truth computed as pair of
engine truths. Conflict band: FAULT_PLAN worker_b delta (hsh%3==0).

## Workflow (v1)
- H lane: split(unchecked) → worker_a → extract_a → worker_b → extract_b
  → resolve(replan style) → solve_b2 → extract_b2 → merge → extract_s →
  judge
- L lane: solve (whole prompt direct) → extract → judge
- SCENARIO: conflict win = 5 (post resolve path); clean buckets [2,2].
- CHECK_HOOK_STAGES: extract_a, extract_b, extract_b2, extract_s.

## Pass/fail criteria
- PROVEN: H1 AND H2 AND H3 all hold.
- REFUTED: merge < baseline −10pp (two-part prompts exceed 4B context
  shaping) or conflict recovery <25% (blind merge) or leakage >20%.

## Ablations
- A-1: merge removed, direct L-lane solve of full two-part prompt —
  the whole-workflow-vs-orchestrator question (S53: only 1/6 multi-agent
  configs beat single-agent — we must beat it HERE or restrict the atom
  to genuinely independent parts).
- A-2: workers see both parts (no isolation) — expected: cross-part
  contamination (H3's falsifier).

## Live variant plan
v1-live.yml ready (420 steps). Deliverables: merge pass-rate vs 07
baseline, conflict-recovery table, leakage forensics (part_a digits in
part_b answers).
