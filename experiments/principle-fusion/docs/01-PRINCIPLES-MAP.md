# 01 — Principles Map (principle → mechanism → case → check)

Every principle, where the fusion workflow exercises it, which case
stresses it, and what deterministically verifies it. Sources: [S#] =
docs/web-research/FUSION-SOURCES.md. OURS = prior live evidence.

## Matrix

| P# | Principle | Fusion mechanism | Stressed by | Deterministic verification |
|---|---|---|---|---|
| P1 | CoT-first, stable prefix | solve/solve2/sample2 emit the exact v13-proven "Think step by step through every rule, then answer." — never reworded (v14c: prefix flips values at temp 0; [S27]) | all | fingerprint.jsonl hashes prompts; unit test asserts prefix byte-exact |
| P2 | Sample more, reflect less | sample2 = FRESH cross-model sample w/ failure hints; no stage anywhere asks a model to critique its own output ([S1]; OURS wait-retry 0 wins) | fu-01, fu-05 (multi-rule arithmetic) | outcomes ledger shows sample2 wins only after 4B fails; spoof scenario depth ≥ sample2 for H-stress cases |
| P3 | External verification only | every gate = check_lib or scripted artifact; models never self-assess ([S1] Reflexion collapse; [S34]) | all | no `self_check` key exists in any YAML — grep-able invariant, unit test |
| P4 | Plan once, dispatch | H-only `plan` stage (Think, 1000tok) emits ordering scaffold; engine dispatches; plan never re-consulted mid-chain ([S2-S6]) | fu-11, fu-12 (precedence chains) | plan stage UNCHECKED + prior-embedded into solve; unit test prior wiring |
| P5 | Decompose only separable | NO decompose stage (OURS: 0/8 live wins, removed); plan lists steps but does not solve subparts ([S26]: decomposition overhead on monolithic arithmetic) | fu-03, fu-04 | absence in YAML = structural; unit test shape |
| P6 | ADaPT try-first | rescue rungs exist only behind GWT fail gates; nothing runs unless prior check failed ([S16]) | all H | spoof walk: stages before win-depth print SKIP and route onward |
| P7 | Deterministic checks | json_exact / contains_required at every checked stage via check_lib ([S30] det = cheap floor) | all | unit tests run check_lib on computed truths |
| P8 | Retrieve don't recall | digest slices core+spec-tail verbatim into context ([S20-S25]) | fu-01..fu-10 (facts mid-doc) | unit test: every trap number present in digest output |
| P9 | Route by node role | 4B = primaries, Bonsai = deep sample, Think = plan/verify only ([S11]) | all | model per stage fixed in shape; unit test generator model map |
| P10 | Cheap verify, run twice | extract (200tok 4B) precedes every escalation ([S1] cost-matched sampling) | all H | stage order canon in generator: extract < sample2 < verify |
| E1 | Budget forcing bounded | `wait` stage (4B 400tok "Wait. Prior attempt near-miss...") = LAST cheap rung, only on near-miss ([S17,S18]; OURS: 0 standalone wins) | fu-07, fu-08 (off-by-one traps) | wait sits after solve2, before sample2 in canon; unit test order |
| E3/E6 | Variant-level difficulty gating | L cases: solve+extract+judge ONLY — deep stages structurally absent ([S12,S16]; OURS cp02/residency flips) | fu-02,04,06,08,10,12 (L variants) | YAML step count per case differs; unit test asserts no deep stage for L |
| E9 | Verify last, expensive-last | Think 3500 backward-check runs after Bonsai sample (OURS v12.2); harvest before verify | fu-05, fu-06 | canon order unit test |
| E10 | Context hygiene | digest offload ~160 words regardless of 1000+ word case ([OURS v14]; [S20]) | all | digest unit test: expansion stripped, word ceiling |
| N1 | Structure removes responsibility | engine owns gates/routing/ordering/state; model emits answers only ([S7] boundary) | all | no stage prompt contains procedural instructions beyond its own narrow task |
| N2 | Reason free, constrain late | harvester: unconstrained attempts → extract w/ check-before-echo ([S8-S10]) | fu-01..fu-10 | unit test: dirty JSON rejected, clean echoed |
| N3 | Confidence pre-gate | fusion_conf.py: entropy of first-token logprobs routes LIGHT/HEAVY before inference ([S13,S14]) | live-phase hook | unit test on fixture logprob files |
| N4 | Replan on invalidation | `replan` stage reads plan+check-failures, re-emits ordering ([S36] 63→87%) | fu-11, fu-12 | replan prior wiring unit test |
| N5 | Two-lane verdict | det check overrides judge; judge stage = separate blind lane, logged to judge.json, never gates ([S30-S34]; OURS ha-08) | all | collector two_lane field; unit test det-over-judge precedence |
| N6 | Leak-safety | failure hints embed check name + observed + question; expected values never in hints (contract echo = restating the prompt's own JSON form, values still derived) ([S31,S33]) | all | unit test: hint block contains no truth substring |
| N7 | Format-consistent scaffolds | exact-token answer menus in case contracts; quoted routing tokens ("PASS"/"SKIP") — GWT string-match requirement ([S27]) | fu-03, fu-05, fu-09 | check_lib exact match on menus; spoof prints quoted tokens |
| N8 | Worked example, different numbers | each case's expansion embeds a worked micro-example whose numbers differ from the task's ([S26]; OURS agentic-26 technique) | all | unit test: example numbers ∩ task numbers = ∅ |

## Ablation design (live phase, docs/04)

Each principle with a stage-level mechanism gets a removal variant:
no-plan, no-wait, no-sample2, no-verify, no-replan, no-digest,
no-harvester (direct constrain). Prediction: cases with `needs: [X]`
in metadata fail exactly under removal of X. Cases and needs:

| case | engine | diff | needs |
|---|---|---|---|
| fu-01 | quota-banking | H | P2, N2, E10 |
| fu-02 | quota-banking | L | P7, P8 |
| fu-03 | retry-backoff | H | N7, E1, N2 |
| fu-04 | retry-backoff | L | P7, N7 |
| fu-05 | canary-gates | H | E9, P2, P1 |
| fu-06 | canary-gates | L | P7, E6 |
| fu-07 | model-residency | H | E1, N2, P8 |
| fu-08 | model-residency | L | P7, P8 |
| fu-09 | priority-preemption | H | N7, P4, N4 |
| fu-10 | priority-preemption | L | P7, P1 |
| fu-11 | epistemic-precedence | H | P4, N4, N2 |
| fu-12 | epistemic-precedence | L | P7, P8 |

Cross-cutting (every case): P3, P6, P9, P10, N1, N5, N6, N8, E3/E6.
