# 02-digest — Research & Reasoning

## Why this atom exists
Local models pay per token twice: latency and attention dilution. Fusion
proved digest slicing sound in unit tests (task digits + contract preserved,
worked example cut); the open question is whether *retry and later stages*
actually solve from digest as well as from full context.

## Sources
- **Lost in the Middle (TACL):** U-shaped position curve — mid-context facts
  retrieved worst. Digest moves all load-bearing facts to a tiny window,
  sidestepping position effects → H3 probes this directly.
- **ICLR 2024 retrieval-vs-long-context:** 4K+retrieval ≈ 16K finetuned
  window — evidence that compact relevant context matches sprawling context.
- **DOS RAG (EMNLP 2025):** simple structure-preserving retrieval matches
  fancy pipelines → digest preserves original document order of kept
  segments (fusion implementation note).
- **ReadAgent:** gist-memory + lookup = 3.5-20× effective context — digest is
  the gist; bookmarks are the lookup.
- **E10 compression hygiene (fusion, proven):** digest offload fixed dilution
  (15/100→100/100 in REA+ live data).
- **Progressive disclosure (2607.17598):** one-level disclosure wins; deeper
  routing never helps → digest is exactly one disclosure level.
- **S44 rigorous benchmarks:** oracle solver discipline — our generation-time
  truth self-pass asserts make the oracle checkable before any run.

## Reasoning chain
1. Full prompts carry orientation material (worked example, expansion) that
   first-solve needs (N8/N9 scaffolds) but retry stages don't.
2. Re-sending them dilutes attention (E10 live evidence) and costs tokens.
3. Digest = core + contract; generation asserts it carries every task digit.
4. If digest retry matches full retry at ≤35% tokens, the atom graduates.

## What would change our mind
- If wait-stage (self-recheck) performance drops on digest vs full, the
   recheck needs to *see its own prior answer*, not the prompt — prior
   embedding already handles this; a failure would indict prior wiring.
- If fact-position buckets diverge, the digit-retention assert has a hole —
  inspect which digit class escapes the regex.

## Relation to opencode parity
opencode compacts context when it grows; digest is the proactive analog —
never let irrelevant material reach the model at all.
