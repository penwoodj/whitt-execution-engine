# Source: MAST — Why Do Multi-Agent LLM Systems Fail?

- **URL:** https://arxiv.org/abs/2503.13657 | https://github.com/multi-agent-systems-failure-taxonomy/MAST
- **Retrieved:** 2026-08-24 (websearch)

## Method Mechanics

Failure taxonomy for multi-agent LLM systems, built by expert annotation of execution traces (avg 15,000 lines/trace) across 7 MAS frameworks (incl. GPT4, Claude 3, Qwen2.5, CodeLlama).

**14 failure modes in 3 categories:**

| Category | Failures |
|----------|----------|
| FC1 Specification & System Design | role mismatches, poor conversation management, info gaps |
| FC2 Inter-Agent Misalignment | communication breakdowns, task derailment, format mismatch |
| FC3 Task Verification & Termination | premature termination, insufficient verification, wrong stopping |

## Detection Approach

- Phase 1: human expert annotation (6 experts, Cohen's κ = 0.88 inter-annotator)
- Phase 2: LLM-as-judge with MAST definitions + few-shot (κ = 0.77 vs humans, ≥94% accuracy held-out)

## Key Numbers

- Failure rate 41%–86.7% across 7 SOTA MAS frameworks
- FC3 (verification/termination) = largest bucket in most systems

## What We Borrow

1. **F4 propagation ≈ FC2 inter-agent misalignment** — our F4 case simulates upstream step error corrupting dependent step; MAST confirms cascading communication failure is top-tier real-world failure mode
2. **Termination correctness is a failure class of its own** — our accept path IS a termination decision; report.py validates we neither accept-failed-output (false accept) nor reject-good-output (false reject). FDA metric covers both directions
3. **κ agreement as classification quality bar** — spoofed phase: classification must be 100% deterministic agreement with injected ground truth (κ=1.0 trivially); live phase: target κ ≥ 0.77 (MAST LLM-judge bar)
4. **Trace-first methodology** — MAST annotates traces; our trace.jsonl = machine-readable trace for the same purpose

## Divergence

MAST is diagnostic taxonomy, not healing mechanism. Used here to validate taxonomy coverage, not as competitor.
