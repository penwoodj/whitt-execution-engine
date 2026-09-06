# REA — Web Research Synthesis

**Researched:** 2026-08-14. Sources: arXiv, ACL Anthology, peer-reviewed where
available. Focus: strategies that make small models (0.5B–9B) reason above
their weight class, especially iteration-based methods. Every claim below maps
to a design decision in DESIGN.md.

## §1 Self-consistency voting fails on hard problems

**arXiv 2608.11403** — Majority-vote self-consistency *hurts* 56.6%
(Qwen2.5-7B) and 65.7% (Llama-3-8B) of GPQA Diamond problems the model gets
wrong. Confidence does not track correctness; verifier-free gating signals
(answer agreement, entropy) recover nothing. External verification required.

**arXiv 2605.03379** — Qwen2.5-7B produces *highly correlated* stable errors
across samples → voting gains ≈0. Higher temperature decorrelates → votability
rises, but only because errors diversify.

**Design consequence:** No majority voting as correctness signal in REA.
Agreement may be used ONLY as an "easy problem" early-exit signal (see §2).

## §2 Early-stop consensus = easy-signal, not truth-signal

**arXiv 2401.10480 (ESC)** — Stop sampling when answer entropy hits 0 in a
window: GSM8K accuracy identical with 80.1% fewer samples.

**Design consequence:** REA Step 0 uses deterministic checks for the same
purpose — cheap "already good" detection. If input passes all checks → exit
with zero LLM calls. (We use deterministic gates instead of entropy because
§1 showed agreement ≠ correctness; deterministic checks ARE external.)

## §3 Chain-of-Verification (CoVe) — factored is the crucial variant

**arXiv 2309.11495** — Draft → plan verification questions → answer them →
revise. The **factored** variant (verification questions answered WITHOUT the
draft in context) is what works; conditioning verification answers on the
draft repeats its hallucinations. Simple verification questions are answered
more accurately than the original long-form task.

**Design consequence:** REA sub-solvers never see the draft. The
VERIFICATION-PLANNER reads the draft, but its questions are answered from the
task spec / auxiliary source by 4B with draft hidden (Steps 3/3b in DESIGN.md).

## §4 Intrinsic self-correction fails without external feedback

**Huang et al., ICLR 2024 (arXiv 2310.01798)** — LLMs asked to self-correct
*without external feedback* drop in accuracy; they flip correct → incorrect as
often as the reverse. Multi-agent debate ≤ self-consistency at equal response
counts. External feedback (executors, tools, ground truth) is what makes
correction work.

**Design consequence:** Every REA lens transition is gated by an EXTERNAL
artifact — deterministic check JSON, not model self-assessment. No "critique
then revise" free-form loops.

## §5 Small models must offload memorization-heavy verification to tools

**arXiv 2504.04718 (T1)** — Sub-3B models fail verification that requires
memorization (arithmetic, fact recall); routing those verifications to a code
interpreter lets Llama-3.2-1B + test-time compute BEAT Llama-3.1-8B.

**Design consequence:** REA's TOOL-VERIFY step (zero-LLM) executes
arithmetic/format/count checks in Python. Models are never asked to verify
what a script can verify.

## §6 Budget forcing / "think longer" is unreliable

**arXiv 2501.19393 (s1)** — sequential compute scales better than parallel
voting. **BUT arXiv 2507.14419** — appending "Wait" gives inconsistent,
oscillating, mostly-unchanged answers; the real driver of s1 gains is
max-length truncation, not nudging.

**Design consequence:** No nudge-retry loops ("are you sure?"). REA rounds
change CONTEXT (escalation memory + new lens), not just repetition.

## §7 Compute-optimal allocation: match strategy to difficulty

**Snell et al. (arXiv 2408.03314)** — Easy prompts benefit most from
sequential revision; hard prompts from parallel sampling + search; adaptive
allocation gives 2–4× efficiency. Small model + test-time compute beats a 14×
larger model when base success rate is non-trivial.

**Design consequence:** REA escalates: deterministic pass (free) → single
sequential pipeline → one diverse re-synthesis round only if gates fail.
Adaptive, not fixed-cost.

## §8 Debate / diversity of thought

**arXiv 2410.12853** — DIVERSE models of similar capacity are the key: 7B trio
+17% by round 4; 2B trio +10%. **arXiv 2511.07784** — debate bounded by
strongest reasoner; majority pressure suppresses corrections. **ACL 2024
2024.acl-long.331** — single agent + strong prompts ≈ debate when demos
present. **Smit 2024** — MAD hyperparameter-sensitive. **Kaesberg 2025** —
fewer rounds before voting better; voting protocols +13.2% reasoning. NORMAD:
structured debate let 7–9B models match a 27B model.

**Design consequence:** REA gets diversity from LENSES (decomposer /
planner / synthesizer / judge = different context + temperature + model), not
from N-agent chat rounds. Max 2 rounds. No majority pressure — deterministic
selection decides.

## §9 Small judges work only with narrow rubric + reference

**Prometheus (arXiv 2310.08491)** — 13B judge with rubric + reference answer
reaches GPT-4-level agreement (Pearson .897 vs .882). Reference materials
relieve the judge of solving the task — it only compares. **arXiv 2403.02839**
— fine-tuned judges don't generalize; narrow rubric + reference = OK, general
taste = no.

**Design consequence:** REA v1 judge is deterministic (checks JSON). The v2
LLM judge, if added, gets: task spec + reference materials + narrow rubric
only — never free-form quality opinion.

## §10 Cascades: the gate is the crux

**FrugalGPT (arXiv 2305.05176)** — small-model-first + reliability-score gate
→ escalate; 98% cost cut matching GPT-4. Weak judge worse than no cascade.
Escalation threshold is THE hyperparameter.

**Design consequence:** REA's escalation threshold (which check failures
trigger round 2) is an explicit, tunable config in the workflow — first live
tuning target.

## §11 Decomposition is the small-model unlock

**Least-to-Most (arXiv 2205.10625)** — decomposition generalizes beyond
exemplars (SCAN 99% vs 16%); helps most on ≥5-step problems. **DialCoT (ACL
2023 emnlp-main.501)** — CoT is ineffective or detrimental below 10B;
decompose + sequential sub-answers fixes SLMs; step-by-step beats
all-at-once. **DaSLaM** — decomposition and solving must be SEPARATE modules.
Anthropic sub-agent findings: subquestions in separate contexts improve
faithfulness.

**Design consequence:** REA Step 1 (1.7B decomposer) is a separate module
from Step 2 (4B sub-solver). Sub-answers collected per-question, then
synthesized.

## Prior in-house evidence (experiments/atomic-reasoning, correction-atom)

- Zhang ACL 2024: pure self-refine fails ≤13B (cited in GUIDELINES.md there).
- Entrospect (Yan ACL 2025): external introspection +36% at 10× speed.
- Reflexion caps at Ω=1–3 useful cycles.
- correction-atom v8: 9B contributed 0/62 wins; cascade 1.7B→4B optimal;
  escalation memory + hard-fail gate shipped 23/27 with zero leaks.
- atomic-reasoning: multi-cpu 1.2B+0.5B router 3.1× faster than single-cpu;
  small models leak prompt/format without hard output rules.

## Synthesis table → design mapping

| Finding | REA mechanism |
|---|---|
| Voting fails hard problems (§1) | No voting; deterministic selection |
| ESC early stop (§2) | Step 0 zero-LLM gate |
| CoVe factored (§3) | Sub-solvers never see draft |
| Intrinsic correction fails (§4) | External gates between all stages |
| Tool-verify (§5) | TOOL-VERIFY python checks |
| Nudge-retry unreliable (§6) | Rounds change context, not repetition |
| Adaptive allocation (§7) | Escalation cascade w/ early exits |
| Lens diversity > agent count (§8) | 4 lenses, 2 models, max 2 rounds |
| Judge needs rubric+reference (§9) | Deterministic judge v1; narrow rubric v2 |
| Cascade gate = crux (§10) | Tunable escalation threshold |
| Decomposition unlocks SLMs (§11) | Steps 1–2 decomposer + sub-solver |
