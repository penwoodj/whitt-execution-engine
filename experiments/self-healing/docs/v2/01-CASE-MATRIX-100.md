# Self-Healing v2 — 100-Case Matrix

Design: 10 domains × 5 archetypes × 2 complexity tiers = 100 cases. Every case prompt 1000–1500 words. Failure classes rotate across cells so each domain×archetype pair sees multiple classes; persist variants (heal-exhaustion → final_fail) included for F1/F2/F4.

## Coverage dimensions

**Domains (10):**
1. `software-eng` — API clients, migrations, code review, build systems
2. `data-analysis` — datasets, metric reports, cohort studies
3. `research-synthesis` — literature matrices, evidence tables
4. `ops-infra` — incidents, capacity, deployments
5. `finance-risk` — exposure reports, reconciliation, audit trails
6. `healthcare-info` — clinical data summaries, terminology mapping
7. `legal-compliance` — policy gap analysis, DPA checklists
8. `marketing-content` — campaign briefs, SEO audits
9. `scientific-compute` — simulation configs, unit conversions, pipelines
10. `product-ux` — usability findings, test scripts, acceptance criteria

**Archetypes (5):**
- `generate-artifact` — produce structured JSON/doc from context+data
- `transform-refine` — fix/migrate given code or config
- `analyze-evaluate` — audit inputs, output verdict report
- `plan-decompose` — multi-step plan w/ dependencies
- `troubleshoot-diagnose` — root-cause from logs/symptoms

**Complexity:** tier A = moderate (2–3): 2–3 constraints, single artifact; tier B = high (4–5): 5+ constraints, nested schema, multi-source data.

**Failure class schedule** (per domain, across its 10 cells): clean×1–2, F1×2, F2×2, F3×1–2, F4×1–2, persist×1. Suite totals: clean 16, F1 20, F2 20, F3 15, F4 15, persist 9 (F1p 3, F2p 3, F4p 3). Expected verdicts: accept 85, final_fail 9, budget-degrade 6 (see 02-WORKFLOW-V2-DESIGN §gates).

## Full matrix (100 rows)

Format: `id | dom | archetype | cx | class(fail-attempts) | expected path (short)`
Path tokens: A=attempt, D=detect, T=triage, HL=heal-light(script), HR=heal-replan(LLM), J=judge(LLM), FA=final_fail, ACC=accept.

| # | case_id | domain | archetype | cx | class (fails@) | verdict | LLM calls |
|---|---------|--------|------------|----|----------------|---------|-----------|
| 1 | se-gen-a-clean | software-eng | generate-artifact | 2 | clean | ACC@1 | 1 |
| 2 | se-trf-a-f1 | software-eng | transform-refine | 2 | F1(@1) | ACC@2 | 2 |
| 3 | se-ana-a-f2 | software-eng | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 4 | se-pld-b-f3 | software-eng | plan-decompose | 4 | F3(@1) | ACC@2 | 3 |
| 5 | se-trb-b-f4 | software-eng | troubleshoot-diagnose | 4 | F4(@1,@2) | ACC@3 | 4 |
| 6 | se-gen-b-f1p | software-eng | generate-artifact | 4 | F1(@1,@2,@3) | FA@3 | 4 |
| 7 | se-ana-b-clean | software-eng | analyze-evaluate | 4 | clean | ACC@1 | 1 |
| 8 | se-pld-a-f2 | software-eng | plan-decompose | 2 | F2(@1) | ACC@2 | 2 |
| 9 | se-trb-a-f3 | software-eng | troubleshoot-diagnose | 3 | F3(@1) | ACC@2 | 3 |
| 10 | se-trf-b-f4p | software-eng | transform-refine | 4 | F4(@1,@2,@3) | FA@3 | 6 |
| 11 | da-gen-a-clean | data-analysis | generate-artifact | 2 | clean | ACC@1 | 1 |
| 12 | da-trf-a-f1 | data-analysis | transform-refine | 2 | F1(@1) | ACC@2 | 2 |
| 13 | da-ana-a-f2 | data-analysis | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 14 | da-ana-b-f3 | data-analysis | analyze-evaluate | 4 | F3(@1) | ACC@2 | 3 |
| 15 | da-trb-b-f4 | data-analysis | troubleshoot-diagnose | 4 | F4(@1,@2) | ACC@3 | 4 |
| 16 | da-gen-b-f2p | data-analysis | generate-artifact | 4 | F2(@1,@2,@3) | FA@3 | 4 |
| 17 | da-pld-b-clean | data-analysis | plan-decompose | 4 | clean | ACC@1 | 1 |
| 18 | da-ana-b-f1 | data-analysis | analyze-evaluate | 4 | F1(@1) | ACC@2 | 2 |
| 19 | da-trf-b-f3 | data-analysis | transform-refine | 4 | F3(@1) | ACC@2 | 3 |
| 20 | da-pld-a-f4 | data-analysis | plan-decompose | 3 | F4(@1,@2) | ACC@3 | 4 |
| 21 | rs-gen-a-clean | research-synthesis | generate-artifact | 2 | clean | ACC@1 | 1 |
| 22 | rs-ana-a-f1 | research-synthesis | analyze-evaluate | 2 | F1(@1) | ACC@2 | 2 |
| 23 | rs-trf-a-f3 | research-synthesis | transform-refine | 2 | F3(@1) | ACC@2 | 3 |
| 24 | rs-pld-b-f2 | research-synthesis | plan-decompose | 4 | F2(@1) | ACC@2 | 2 |
| 25 | rs-ana-b-f4 | research-synthesis | analyze-evaluate | 4 | F4(@1,@2) | ACC@3 | 4 |
| 26 | rs-gen-b-f1p | research-synthesis | generate-artifact | 4 | F1(@1,@2,@3) | FA@3 | 4 |
| 27 | rs-trb-b-clean | research-synthesis | troubleshoot-diagnose | 4 | clean | ACC@1 | 1 |
| 28 | rs-pld-a-f1 | research-synthesis | plan-decompose | 3 | F1(@1) | ACC@2 | 2 |
| 29 | rs-ana-a-f2 | research-synthesis | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 30 | rs-trb-a-f3 | research-synthesis | troubleshoot-diagnose | 3 | F3(@1) | ACC@2 | 3 |
| 31 | oi-gen-a-clean | ops-infra | generate-artifact | 2 | clean | ACC@1 | 1 |
| 32 | oi-trb-a-f4 | ops-infra | troubleshoot-diagnose | 2 | F4(@1,@2) | ACC@3 | 4 |
| 33 | oi-ana-a-f2 | ops-infra | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 34 | oi-pld-b-f3 | ops-infra | plan-decompose | 4 | F3(@1) | ACC@2 | 3 |
| 35 | oi-trb-b-f4 | ops-infra | troubleshoot-diagnose | 4 | F4(@1,@2) | ACC@3 | 4 |
| 36 | oi-gen-b-f3 | ops-infra | generate-artifact | 4 | F3(@1) | ACC@2 | 3 |
| 37 | oi-ana-b-f1 | ops-infra | analyze-evaluate | 4 | F1(@1) | ACC@2 | 2 |
| 38 | oi-trf-b-f2 | ops-infra | transform-refine | 4 | F2(@1) | ACC@2 | 2 |
| 39 | oi-pld-a-clean | ops-infra | plan-decompose | 3 | clean | ACC@1 | 1 |
| 40 | oi-trf-a-f1p | ops-infra | transform-refine | 3 | F1(@1,@2,@3) | FA@3 | 4 |
| 41 | fr-gen-a-clean | finance-risk | generate-artifact | 2 | clean | ACC@1 | 1 |
| 42 | fr-ana-a-f1 | finance-risk | analyze-evaluate | 2 | F1(@1) | ACC@2 | 2 |
| 43 | fr-trf-a-f4 | finance-risk | transform-refine | 3 | F4(@1,@2) | ACC@3 | 4 |
| 44 | fr-ana-b-f2 | finance-risk | analyze-evaluate | 4 | F2(@1) | ACC@2 | 2 |
| 45 | fr-pld-b-f3 | finance-risk | plan-decompose | 4 | F3(@1) | ACC@2 | 3 |
| 46 | fr-gen-b-f4p | finance-risk | generate-artifact | 4 | F4(@1,@2,@3) | FA@3 | 6 |
| 47 | fr-trb-b-clean | finance-risk | troubleshoot-diagnose | 4 | clean | ACC@1 | 1 |
| 48 | fr-ana-b-f1 | finance-risk | analyze-evaluate | 4 | F1(@1) | ACC@2 | 2 |
| 49 | fr-pld-a-f2 | finance-risk | plan-decompose | 3 | F2(@1) | ACC@2 | 2 |
| 50 | fr-trb-a-f3 | finance-risk | troubleshoot-diagnose | 3 | F3(@1) | ACC@2 | 3 |
| 51 | hi-gen-a-clean | healthcare-info | generate-artifact | 2 | clean | ACC@1 | 1 |
| 52 | hi-ana-a-f1 | healthcare-info | analyze-evaluate | 2 | F1(@1) | ACC@2 | 2 |
| 53 | hi-trf-a-f2 | healthcare-info | transform-refine | 2 | F2(@1) | ACC@2 | 2 |
| 54 | hi-pld-b-f3 | healthcare-info | plan-decompose | 4 | F3(@1) | ACC@2 | 3 |
| 55 | hi-trb-b-f4 | healthcare-info | troubleshoot-diagnose | 4 | F4(@1,@2) | ACC@3 | 4 |
| 56 | hi-gen-b-f2 | healthcare-info | generate-artifact | 4 | F2(@1) | ACC@2 | 2 |
| 57 | hi-ana-b-clean | healthcare-info | analyze-evaluate | 4 | clean | ACC@1 | 1 |
| 58 | hi-trf-b-f1 | healthcare-info | transform-refine | 4 | F1(@1) | ACC@2 | 2 |
| 59 | hi-pld-a-f4 | healthcare-info | plan-decompose | 3 | F4(@1,@2) | ACC@3 | 4 |
| 60 | hi-trb-a-f3 | healthcare-info | troubleshoot-diagnose | 3 | F3(@1) | ACC@2 | 3 |
| 61 | lc-gen-a-clean | legal-compliance | generate-artifact | 2 | clean | ACC@1 | 1 |
| 62 | lc-ana-a-f2 | legal-compliance | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 63 | lc-trf-a-f1 | legal-compliance | transform-refine | 2 | F1(@1) | ACC@2 | 2 |
| 64 | lc-pld-b-f3 | legal-compliance | plan-decompose | 4 | F3(@1) | ACC@2 | 3 |
| 65 | lc-ana-b-f4 | legal-compliance | analyze-evaluate | 4 | F4(@1,@2) | ACC@3 | 4 |
| 66 | lc-gen-b-f2p | legal-compliance | generate-artifact | 4 | F2(@1,@2,@3) | FA@3 | 4 |
| 67 | lc-trb-b-clean | legal-compliance | troubleshoot-diagnose | 4 | clean | ACC@1 | 1 |
| 68 | lc-pld-a-f1 | legal-compliance | plan-decompose | 3 | F1(@1) | ACC@2 | 2 |
| 89 | lc-ana-a-f3 | legal-compliance | analyze-evaluate | 2 | F3(@1) | ACC@2 | 3 |
| 70 | lc-trf-b-f4 | legal-compliance | transform-refine | 4 | F4(@1,@2) | ACC@3 | 4 |
| 71 | mc-gen-a-clean | marketing-content | generate-artifact | 2 | clean | ACC@1 | 1 |
| 72 | mc-ana-a-f1 | marketing-content | analyze-evaluate | 2 | F1(@1) | ACC@2 | 2 |
| 73 | mc-trf-a-f4 | marketing-content | transform-refine | 3 | F4(@1,@2) | ACC@3 | 4 |
| 74 | mc-pld-b-f2 | marketing-content | plan-decompose | 4 | F2(@1) | ACC@2 | 2 |
| 75 | mc-ana-b-f3 | marketing-content | analyze-evaluate | 4 | F3(@1) | ACC@2 | 3 |
| 76 | mc-gen-b-f4 | marketing-content | generate-artifact | 4 | F4(@1,@2) | ACC@3 | 4 |
| 77 | mc-trb-b-clean | marketing-content | troubleshoot-diagnose | 4 | clean | ACC@1 | 1 |
| 78 | mc-pld-a-f1 | marketing-content | plan-decompose | 3 | F1(@1) | ACC@2 | 2 |
| 79 | mc-ana-a-f2 | marketing-content | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 80 | mc-trf-b-f1p | marketing-content | transform-refine | 4 | F1(@1,@2,@3) | FA@3 | 4 |
| 81 | sc-gen-a-clean | scientific-compute | generate-artifact | 2 | clean | ACC@1 | 1 |
| 82 | sc-trf-a-f2 | scientific-compute | transform-refine | 2 | F2(@1) | ACC@2 | 2 |
| 83 | sc-ana-a-f1 | scientific-compute | analyze-evaluate | 2 | F1(@1) | ACC@2 | 2 |
| 84 | sc-pld-b-f3 | scientific-compute | plan-decompose | 4 | F3(@1) | ACC@2 | 3 |
| 85 | sc-trf-b-f4 | scientific-compute | transform-refine | 4 | F4(@1,@2) | ACC@3 | 4 |
| 86 | sc-gen-b-f3 | scientific-compute | generate-artifact | 4 | F3(@1) | ACC@2 | 3 |
| 87 | sc-ana-b-clean | scientific-compute | analyze-evaluate | 4 | clean | ACC@1 | 1 |
| 88 | sc-trb-b-f1 | scientific-compute | troubleshoot-diagnose | 4 | F1(@1) | ACC@2 | 2 |
| 89 | sc-pld-a-f2 | scientific-compute | plan-decompose | 3 | F2(@1) | ACC@2 | 2 |
| 90 | sc-trb-a-f4p | scientific-compute | troubleshoot-diagnose | 3 | F4(@1,@2,@3) | FA@3 | 6 |
| 91 | px-gen-a-clean | product-ux | generate-artifact | 2 | clean | ACC@1 | 1 |
| 92 | px-ana-a-f2 | product-ux | analyze-evaluate | 2 | F2(@1) | ACC@2 | 2 |
| 93 | px-trf-a-f1 | product-ux | transform-refine | 2 | F1(@1) | ACC@2 | 2 |
| 94 | px-pld-b-f4 | product-ux | plan-decompose | 4 | F4(@1,@2) | ACC@3 | 4 |
| 95 | px-ana-b-f3 | product-ux | analyze-evaluate | 4 | F3(@1) | ACC@2 | 3 |
| 96 | px-gen-b-f1 | product-ux | generate-artifact | 4 | F1(@1) | ACC@2 | 2 |
| 97 | px-trb-b-clean | product-ux | troubleshoot-diagnose | 4 | clean | ACC@1 | 1 |
| 98 | px-pld-a-f3 | product-ux | plan-decompose | 3 | F3(@1) | ACC@2 | 3 |
| 99 | px-ana-a-f4 | product-ux | analyze-evaluate | 3 | F4(@1,@2) | ACC@3 | 4 |
| 100 | px-trf-b-f4p | product-ux | transform-refine | 4 | F4(@1,@2,@3) | FA@3 | 6 |

(Typo note: row 68 id is `lc-ana-a-f3`; row numbering contiguous 1–100.)

## Suite totals

- Verdicts: accept 88, final_fail 9, (judge gray-zone among accepts where flagged)
- Failure classes: clean 16 · F1 20 · F2 20 · F3 15 · F4 15 · persist 9
- LLM call totals if all routed correctly: 100 worker + 42 replans + ≤10 judge ≈ ≤152 calls ceiling; naive no-healing baseline = 300 (3 attempts × 100). v2 economy ≈ 2× fewer calls.

## Flagship 10 (iteration set — covers all classes, all domains, both persist flavors)

| flagship | matrix # | why |
|----------|----------|-----|
| flg-01 | 1 | clean baseline, JSON SDK artifact |
| flg-02 | 22 | F1 hallucinated citation keys, research domain |
| flg-03 | 13 | F2 schema drift + tool errors, data domain |
| flg-04 | 14 | F3 contradictory metrics, high complexity |
| flg-05 | 35 | F4 truncated incident JSON, ops |
| flg-06 | 6 | F1-persist → final_fail path |
| flg-07 | 82 | F2 scientific code defect, unit conversions |
| flg-08 | 64 | F3 legal plan internal contradiction |
| flg-09 | 76 | F4 high-complexity marketing artifact |
| flg-10 | 60 | F3 healthcare plan (swapped for F4-persist 90: flg-10 = 90 F4p healthcare→scientific swap) — final: flg-10 = #90 sc-trb-a-f4p persistent truncation |

Flagship files: `cases/v2/flagship/case-flg-01..10.yml` — full 1k–1.5k word prompts, hand-written. Matrix 90 via `scripts/gen-cases-v2.py`.
