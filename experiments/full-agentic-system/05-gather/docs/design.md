# 05-gather — Experiment Design

## Atom
**gather** — gate-driven evidence accumulation: read/search into an
evidence buffer, ≤2 iterations, before solve. Corpus lives outside the
prompt (aux), forcing genuine retrieval rather than recall.

## Hypotheses (pre-registered)
- **H1 (2-iteration sufficiency):** gather→solve→gather2→solve2 matches
  5-iteration gather quality (spoof win-depths favor 2nd iteration by
  design [3,1,4]); live confirmation = pass rate at depth 2 ≥ depth 1.
- **H2 (decoy resistance):** 5 decoy corpus blocks (adjacent-shift
  arithmetic, plausible shapes) do NOT contaminate answers — failures
  traced to decoy blocks (digit forensics) ≤10%.
- **H3 (NOT_FOUND honesty):** stripped-id cases produce {not_found:[id]}
  rather than confabulated values ≥90% — the gather loop must report
  absence, not invent presence (BullshitBench failure mode).

## Cases
ga-01..ga-48 = 6 engines × 2 variants × 4 idx. Core = instructions only
(~75 words); events live in aux corpus at rotated position (idx%3) among
5 decoy blocks labeled by other-night headers. idx%6==5 → NOT_FOUND:
target id-sentence stripped, truth={not_found:[first_id]}, task_digits=[]
(digest assertion disabled for these by empty list).

## Workflow (v1)
- H lane: gather(fmt, gather style) → solve(cot) → gather2 → solve2 →
  extract → judge(think)
- L lane: solve → extract → judge
- SCENARIO buckets [3,1,4]: majority need 2nd iteration.
- CHECK_HOOK_STAGES: extract.

## Pass/fail criteria
- PROVEN: depth-2 pass ≥ depth-1 pass − 3pp AND decoy contamination ≤10%
  AND not_found honesty ≥90%.
- REFUTED: 2nd iteration adds <5pp over 1st (gather depth worthless) or
  contamination >20% (buffer pollutes, needs provenance filtering).

## Ablations
- A-1: single-pass (no gather — solve sees core only) — expected: near-0
  pass (events unreachable). Proves gather is load-bearing, not decor.
- A-2: 5-iteration gather — expected: no gain over 2 (Agentic-RAG curve
  flat after 2), pure cost.

## Live variant plan
v1-live.yml ready. Evidence buffers via aux; analysis: per-case failure
digit-forensics vs decoy blocks; not_found outcome rates.
