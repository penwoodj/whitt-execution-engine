# Web Research: Weak-Model Harness Meta-Principles (2024-2026)

> Compiled 2026-08-23. 20 principles. WMS = weak-model-specific.

## Tool Integration & Deterministic Offloading

1. **Tool-Integrated Verification** (WMS) — offload memorization-heavy verification (numeric calc, fact-check) to external tools before small model judges. Llama-1B+T1 > Llama-8B on MATH. arxiv.org/html/2504.04718v2. v3: check-deterministic.py IS this.
2. **Deterministic Offloading** (WMS) — convert reasoning to executable/deterministic logic; determinism fraction ↔ scaffold quality r=0.72. GPT-5.4-mini 0.49→0.91 via structure externalization. arxiv.org/html/2608.12307. v3: json_exact gates.
3. **Plan-Tuning for Decomposition** (WMS) — distilled planning trajectories beat brute prompting (+7% GSM8K, +10-12% OOD). Needs training — SKIP, but METHOD line in prompts mimics.

## Routing & Cascading

4. **Calibrated Cascade Thresholds** (WMS) — calibrate uncertainty→error probability (isotonic, ECE 0.03), then threshold; calibration is bottleneck not threshold. arxiv.org/html/2605.18796.
5. **Difficulty-Aware Routing** (WMS) — route on difficulty features (not semantics); 2B analyst suffices. arxiv.org/html/2607.18098v1. v3: hops field in cases = difficulty metadata.
6. **Pre-Generation Routing > Post** (general) — cascades pay cheap-model cost even when escalating; pre-gen routing wins 4/5 datasets. arxiv.org/html/2605.06350. NOTE: our det-check is post-gen but free (shell), so cascade cost argument doesn't bite.
7. **Post-Hoc Quality Estimation** (general) — cascade beats routing only w/ strong post-hoc quality signal. Deterministic check = perfect post-hoc signal → cascade justified. files.sri.inf.ethz.ch dekoninck2024cascaderouting.pdf

## Structured Outputs & Constraints

8. **Reason Free, Constrain Late** (WMS) — hard schema decoding DURING generation drops accuracy 19.7%→11.0%; delayed packaging preserves answers. arxiv.org/html/2605.26128. v3: prompt says reason first, JSON last. No constrained decoding.
9. **Prompt-Based Format Enforcement** (WMS) — iterative prompt constraint beats constrained decoding; 3.6-8.2x lower latency. arxiv.org/html/2605.02363v1. v3: format rules in prompt text only.
10. **Constraint Tax** (WMS) — strict constraints cause structure snowballing (2,850→4,005 tokens on format); track wrong-valid-schema not just parse success. arxiv.org/html/2604.06066. v3: json_exact catches wrong-valid-schema (value mismatch with valid JSON).

## Context & Instructions

11. **Instruction Cap ~80** (WMS) — adherence collapses at 80 simultaneous rules regardless of format. arxiv.org/html/2607.19257v1. v3 prompts: <10 instructions each. SAFE.
12. **Context Rot Mitigation** (WMS) — long trajectories → give-up/uncertain answers; compaction+trimming optimal balance. arxiv.org/html/2606.29718. v3: per-case workflows = short trajectories by construction.
13. **Instruction Drift Countermeasures** (WMS) — system-prompt attention decays exponentially; role isolation/repetition helps (LLaMA-70B drifts within 8 rounds). arxiv.org/html/2402.10962. v3: fresh step = fresh prompt. Each stage re-states role.
14. **Context Isolation Needs Strong Backbone** (general) — sub-agent isolation is model-dependent. arxiv.org/html/2606.29718.

## Verification & Aggregation

15. **Self-Consistency Backfires on Hard** (WMS) — majority vote hurts 56-66% of GPQA for 7-8B; confidence doesn't track correctness; need EXTERNAL verifier. arxiv.org/html/2608.11403v1. v3: no voting; deterministic check = external verifier.
16. **Adaptive Sampling Budgets** (WMS) — confidence-based early stop for self-consistency saves 70% (ReASC, Gemma-3-4B). aclanthology.org 2026.findings-acl.1085. Future: sample-adaptive solving.
17. **Role Drift Prevention** (general) — decomposer plants answers via shortcuts; 86% of RL gain can be role-shortcut. Role Anchor regularizer. arxiv.org/html/2607.21627v1. Analogy: fixer leaking expected answers = role violation → ha-10 case + feedback sanitizer.
18. **Instruction Stability Multi-Turn** (WMS) — multi-turn adherence drops 39% vs single-turn. arxiv.org/html/2606.22528. v3: single-turn steps only. ALIGNED.

## Role & Advanced

19. **Role-Based Error Isolation** (WMS) — same model, 3 isolated roles (summarizer/agent/history-free corrector) breaks repetitive failure loops; correction node = majority of gains (Qwen3-8B 5.4%→8.9% AppWorld). arxiv.org/html/2604.11465. v3: solve → fix_1 (incremental) → fix_2 (history-free from-scratch) = direct application.
20. **Planning Internalization** (WMS) — plan-aware trajectories reach frontier-planner parity w/o inference cost (SWE-bench 55.8% vs 42.8%). Needs training. SKIP.

## v3 Design Consequences (final)

| Principle | v3 Feature |
|-----------|-----------|
| P1/P2/P7 | json_exact deterministic gate = primary verdict; judge secondary |
| P8/P9 | reason-first prompt, JSON last line; no constrained decoding |
| P15 | no self-consistency voting |
| P19 | fix_2 = history-free from-scratch corrector |
| P11/P13 | short prompts, role restated per step |
| F9 (judge doc) | FIX x2 cap |
| P5 | hops field drives analysis (difficulty-aware) |
