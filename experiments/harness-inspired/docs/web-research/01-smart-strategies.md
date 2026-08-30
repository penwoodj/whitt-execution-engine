# Web Research: Smart/Uncommon Harness Strategies (2024-2026)

> Compiled 2026-08-23. Sources: librarian web research. For harness v3+ design.

## Strategy Summary Table

| # | Strategy | Cost | Needs Fine-Tune? | Fits 4B Local? | Verdict for v3 |
|---|----------|------|------------------|----------------|-----------------|
| 1 | Budget forcing (wait tokens) | Low | Optional | Yes (needs think-token model) | DEFER — Qwen3-4B-2507 Instruct lacks think tokens |
| 2 | Self-Trained Verification | Medium | Yes | No (needs distillation) | SKIP |
| 3 | Weaver weak-verifier ensemble | High | Partial | Partial | SKIP for now (1 GPU) |
| 4 | Block verification (spec decode) | Engine-level | No | Engine-level | N/A |
| 5 | Deterministic trace replay | Low | No | Yes | ADOPT — fingerprint + re-score |
| 6 | Position-bias permutation judge | 10x | No | Partial | SKIP (cost), note in judge doc |
| 7 | Fresh-context verifier quarantine | Low | No | Yes | ADOPT — already in design (H2) |
| 8 | Early-exit cascade + difficulty routing | Low | No | Yes | ADOPT — det-first gating is this |
| 9 | Leak-safe structural feedback | Low | No | Yes | ADOPT — feedback.py implements |

## 1. Budget Forcing (s1)

- Detect end-of-thinking token. Extend: append "Wait", continue. Truncate: force end token over budget.
- s1 (arXiv 2501.19393): AIME24 50%→57% via "Wait" extension. Sequential scaling beats majority voting at same compute.
- Weakness: needs think-token-trained model; >6x extension = repetition loops.
- For us: Qwen3-4B-Instruct-2507 = no think tokens. DEFER until thinking model available.

## 2. Self-Trained Verification (STV)

- Train verifier conditioned on reference solutions, distill to unconditioned. Verify-then-revise only when verifier flags.
- arXiv 2605.30290: AIME24 pass@1 2x lift; SciKnowEval 1.5%→21%.
- Weakness: needs fine-tuning + reference corpus. SKIP (out of scope, no training).

## 3. Weaver — Weak Verifier Ensembling

- Multiple weak verifiers as latent-variable ensemble (Snorkel-style weights), distill to 400M cross-encoder.
- arXiv 2506.18203: +17.9% accuracy for 8B models; Llama-70B+Weaver ≈ o3-mini.
- Weakness: 50K labels for full mode; ensemble inference 10-128x cost. SKIP (single GPU, no training).

## 4. Block Verification (Speculative Decoding)

- Verify draft tokens jointly (sub-blocks), not independently. Optimal, 5-8% speedup.
- arXiv 2403.10444. Engine-level (llama.cpp feature), not harness-level. N/A.

## 5. Deterministic Replay / Trace Fingerprints

- MITM proxy records all LLM/tool calls as serialized traces; replay byte-for-byte offline; noise-aware diff separates semantic drift from timestamps.
- AgentAssay (arXiv 2607.16200): 86% behavioral detection power where binary tests have 0%; 78-100% cost cut via trace-first offline analysis.
- Weakness: same model version required; streaming ordering.
- For us: fingerprint = sha256(case+stage+prompt) per stage (REA+ pattern, AGENTIC-PHASE.md M5). Trace.jsonl = replayable ledger. Collector re-scores without model. ADOPT.

## 6. Position-Bias Permutation (Rubric Judges)

- Balanced permutation (5 forward + 5 reverse cyclic) cancels positional preference.
- arXiv 2602.02219: Spearman +0.089 (Qwen3-32B). JudgeLM swap-augmentation similar.
- Weakness: 10x judge compute. SKIP live; document. If judge disputes arise, revisit.

## 7. Fresh-Context Verifier (Context Quarantine)

- Verifiers unreliable in generator-polluted context. Wipe history; pass only problem + candidate.
- PAG 2025 (verify-then-revise); Jiang et al. 2506.10406 (selective revision). Verifier seeing full history = reward-hacking/collapse risk.
- Weakness: loses inter-step context (fine for our per-case JSON tasks).
- For us: judge step sees ONLY objective + artifact. Already design H2. CONFIRMED by research.

## 8. Early-Exit Cascade + Difficulty Routing

- FrugalGPT-style: route by difficulty/confidence. Small model first, escalate on low confidence.
- MARS (ACL 2026 findings-421): margin-aware speculative verification, 1.4x speedup.
- Weakness: needs difficulty estimator; misrouting risk.
- For us: deterministic check = perfect difficulty signal. det-FAIL → fix cascade; det-PASS → judge only. Win-depth = cascade depth metric. ADOPT (already core).

## 9. Leak-Safe Structural Feedback

- Feedback = location + observed value + admissible alternatives, NOT expected answer. arXiv 2607.14167: +44 pts vs location-only on TextWorld; alternatives = +36 of that.
- FeedbackEval (arXiv 2504.06939): test feedback 61.0% repair success > compiler 49.2% > minimal "code is wrong" 53.1% (minimal surprisingly strong).
- For us: feedback.py gives check-name + category, withheld values. Research says location+observed is BETTER than name-only. TENSION: json_exact expected value IS the answer — observed value (the model's wrong value) is safe to show, expected is not. REVISE feedback.py: include model's own observed value, never expected.

## Extra Findings

- Repair@k plateaus at k=2-3 (FeedbackEval): our FIX x2 (not x3) is research-aligned. Depth>2 = wasted tokens.
- Simple minimal feedback ("attempt failed checks") is 84% as good as detailed feedback — feedback does not need to be clever.
