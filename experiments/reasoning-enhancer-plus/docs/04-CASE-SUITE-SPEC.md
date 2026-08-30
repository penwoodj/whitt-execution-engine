# REA+ — Stage 2: 30-Case Suite Spec

**BLOCKED until stage-1 gate passes (results/MODEL-PICKS.md exists).**

## Gap-band requirement (the core design constraint)

Every case must satisfy BOTH, live-proven:

1. **old-workflow-fails**: REA v1.4.3 (1.7B+4B cascade) final output
   fails ≥2 subchecks — run artifact archived as evidence
   AMENDMENT 2026-08-17 (empirical): all checks are deterministic
   (same input → same verdict), so a single failed subcheck is a
   stable semantic failure. Exact-IO cases carry one semantic check
   (json_exact); criterion is now "final output fails the case"
   (any failing subcheck). The ≥2 margin remains a preference, not
   a gate, for multi-subcheck reasoning cases.
2. **big-model-passes**: ≥1 picked specialist (direct answer or single
   retry, NO cascade) passes all checks — run artifact archived

If either direction unproven → case re-authored or dropped. No case
ships on assumption. This kills the b39 class of bug: truth comes from
public-bench answer + big-model consensus, and the OLD workflow failing
is verified, not presumed.

## Realism sources (public-bench archetypes, external truth)

| archetype | supplies | count target |
|---|---|---|
| GSM8K-style multi-hop word problems | M3 depth, derivations | 6 |
| DROP-style reading+arithmetic | grounded multi-hop | 5 |
| ARC-style elimination MCQ | multi-constraint logic | 4 |
| StrategyQA-style implicit steps | 2-4 unstated hops | 4 |
| IFEval-style instruction stacks | M2/M5 format+priority | 6 |
| Envelope/config synthesis (b39-class, CORRECT math) | exact-IO | 5 |

Rules:
- Answers/transcriptions adapted, ground truth taken from source bench
  answer — NEVER from our model output. Each case YAML cites archetype
  + provenance note.
- Auxiliary material gives SOURCE FACTS ONLY, never worked derivations
  (critique gap #6: b46/b48 spelled answers out — forbidden here)
- Checks: deterministic (check_lib) wherever possible; semantic-only
  claims need a second grader pass — prefer re-authored deterministic

## Case metadata (required by 02-METRICS)

```yaml
rea_plus:
  hops: 2          # 1..4
  robust: false    # true if negation/distractor/false-premise core
  prio: false      # true if instruction-override core
  format_weight: 1 # count of format-class subchecks
  archetype: gsm8k
  provenance: "adapted from GSM8K sample ####; answer #### independent"
```

## Split

- 24 train / 6 held-out. Held-out authored AFTER train suite green on
  vP-final-candidate. Checks never tuned on held-out.

## Suite shape targets

- hops distribution: 1×4, 2×10, 3×12, 4×4 (depth coverage for M3)
- robust:true ≥ 8 cases (M4 meaningful)
- prio:true ≥ 5 cases (M5 meaningful)
- format_weight ≥1 on ≥20 cases (M2 still earns its keep — it's the
  production use case) but format-ONLY cases ≤6 (reasoning must dominate)

## Dual-validation harness (build with suite)

`validate-gapband.sh`:
1. runs old workflow on all 30 → requires 30/30 FAIL (else case leaks)
2. runs each picked specialist direct on all 30 → per-case pass matrix
3. case valid iff (1) fail + (2) ≥1 pass; emits cases-to-rework list
