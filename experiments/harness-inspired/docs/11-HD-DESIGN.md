# HD Design — 100 Extra-Hard Cases Targeting v10 Failure Classes

> 2026-08-24. Follows user directive: design 100 extra-hard cases in the areas
> the current reasoning workflow struggles with, then spoof-test the next
> workflow version (v12) end-to-end with ZERO LLM calls, loads, or unloads.

## Evidence Base: Where v10 Actually Failed

v10 live campaign: 72/100. Failure concentration by archetype:

| Archetype | Fails | Class |
|-----------|-------|-------|
| print_quota | 4/4 | boundary counting (>= edges) |
| freeze_windows | 3 | window containment arithmetic |
| payroll_holds | 3 | staged holds/deductions |
| rollout_rings | 3 | ring/rotation state |
| dlq_redrive | 3 | queue retry state machine |
| gym_standby | 3 | capacity + standby accounting |
| fuel_cards | 3 | quota deduction chains |
| specimen_routing | 2 | routing matrix w/ tiers |
| vent_control, quota_grace, table_turns, backoff_budget | 1 each | mixed boundary/decay |

Every failure landed at `end_fail` with a clean leak audit: models produced
well-formed tables whose arithmetic was silently wrong through ALL rescue
stages (fix_1 → fix_2 → fix_3). This matches the REA+ v9 taxonomy:
exact-IO math envelopes, unit conversion, boundary arithmetic.

## Hardness Deltas (hd vs hc)

1. **5-6 interacting entities** (hc used 3-4) — more state to track per case.
2. **Per-entity DERIVED answers** — hd entity cells require unit conversion,
   clamped subtraction, or proration arithmetic, not literal copies. The
   aggregate composition stays deterministic (P2 offload), so difficulty
   concentrates exactly where models fail: per-cell derivation.
3. **Chained rules** — rule N modifies the outcome of rule N-1 (e.g. overrides
   cancel flags, releases requeue holds). Off-by-one boundaries compound.
4. **Variant-varied boundary conventions** — variants flip between
   at-or-over (>=), strictly-past (>), and whole-unit ceil rounding so no
   single lexical heuristic generalizes.
5. **Unit-conversion chains** — GiB↔MiB, ms↔s, cents↔dollars embedded in
   per-entity derivation (the REA+ dead-case class: env-11 disk quota,
   env-13 LRU window).

## The 25 HD Archetypes (× 4 variants = 100 cases)

### Class A — boundary/envelope arithmetic (print_quota, freeze_windows)
| # | Archetype | Domain | Hard mechanism |
|---|-----------|--------|----------------|
| 1 | quota_proration | license seats | pro-rated charge = ceil(days/30 * seat_rate) per tier |
| 2 | tiered_postage | parcel tiers | banded pricing, inclusive lower edge, strict upper |
| 3 | ballot_quorum | board votes | quorum recheck after abstain→recuse flips, 2/3 majority w/ floor |
| 4 | disk_quota_conv | backup storage | GiB→MiB conversion per volume vs quota in MiB (REA+ env-11) |

### Class B — staged holds/deductions (payroll_holds, fuel_cards)
| 5 | escrow_ladder | home escrow | 4-tranche release, each clamped by inspection signoff remainder |
| 6 | per_diem_holds | travel audit | hold released only under both receipt cap AND per-diem cap |
| 7 | multi_wallet | game shop | deduction order fixed by rule, floor-at-zero per wallet |
| 8 | rebate_tiers | reseller program | retroactive tier applies to FULL volume once boundary crossed |

### Class C — rotation/cycle state (rollout_rings)
| 9 | ring_rotation_2cyc | canary rings | two promotion cycles, demote-on-error re-enters prior ring |
| 10 | shift_carousel | on-call rota | N-step advance with skip/pin entries counted modulo |
| 11 | canary_percent | staged rollout | percentage step = ceil(cohort * pct), stuck-at-floor rule |

### Class D — queue/retry state machines (dlq_redrive, backoff_budget)
| 12 | redrive_decay | MQ dead letters | per-attempt success decay, terminal drop after K |
| 13 | backoff_scale | job runner | ms→s backoff, doubling capped, budget gate in seconds |
| 14 | poison_mix | stream proc | classify mixed outcomes: 2-strike poison w/ reset-on-streak |

### Class E — capacity/standby cascades (gym_standby)
| 15 | standby_cascade | class waitlist | no-show cascade promotes in priority order, capped per wave |
| 16 | overbook_bump | flight desk | voluntary before involuntary, checkpoint recount |
| 17 | room_split | conf booking | split/merge leaves remainders, merge respects capacity + tech |

### Class F — routing matrices (specimen_routing)
| 18 | triage_2hop | field triage | 2-hop routing, priority overrides destination per tier |
| 19 | cold_divert | cold chain | excursion minutes per leg → divert decision matrix |
| 20 | customs_tier | imports | duty tier by value band AND origin rule, consolidated shipment |

### Class G — paused-clock/accumulation (REA+ env class, hb-06)
| 21 | sla_pause_clock | support SLA | paused-clock accrual: stop during business hold, resume after (env-class) |
| 22 | window_overlap_dedupe | monitoring | count distinct incident minutes across overlapping windows |
| 23 | accrual_diff | finance close | two-ledger reconciliation: diff-of-sums w/ sign conventions |

### Class H — char-precision / exact-IO envelopes (REA+ log-04, env-09..13)
| 24 | label_surgery | asset tags | char-precision abbreviation rules (drop vowel, suffix, pad) |
| 25 | checksum_weighted | order codes | weighted positional sum mod 97 w/ check letter map |

Difficulty labeling: archetypes 1,4,5,8,9,11,13,16,18,21,23,24,25 heavy
(5-6 hops), rest light (4 hops) → ~52/48 heavy/light split.

## Case Schema

Identical to hc (same keys, same verify stack) so v11 machinery extends
unchanged: prompt 1000-2000 words, task_core, contract formulas + regex
literals, v6 entities + aggregate, json_exact truth. Provenance line carries
the full derivation (lint contract).

## v12 Workflow Deltas (over v11)

1. **fix_3 rescue chain for ALL cases** — v11 gates fix_3 on heavy-only; hd
   light cases need the full chain too (v10 evidence: light fails existed).
2. **Per-entity method hints** — hd_hints.py enriches DERIVED cells with
   method wording ("convert first, then compare") without revealing values.
3. **Exact-token menus + unit-spec wording** — every derived cell's question
   states the unit the answer must carry ("whole MiB", "cents"), preventing
   unit-drift wrong answers (hb-10/11/12 lesson, scaled up).
4. **New spoof scenario wind3** — exercises the fix_3 recovery path (fail,
   fail, fail, fix_3 passes) which v11 scenarios never covered.
5. Digest, METHOD_EXACT prefix, exact-JSON fix forms, leak-audit, two-lane
   det-over-judge scoring: all inherited unchanged from v11.

## Spoof Verification Plan (ZERO LLM)

- Generator emits canned artifacts per scenario (wind0/1/2/3/lose).
- Per-case spoof workflow runs REAL gate scripts (check-table, aggregate,
  check-deterministic, leak-audit, trace) on canned tables — full mechanics.
- Suite: 100 hd + 100 hc v12 twins = 200 sequential whitt invocations,
  isolated output dir (results/v12-spoof-output/), never touching the live
  campaign's directory.
- Zero-load proof: engine zombie count + router slot check before/after.
- Exit: 200/200 spoof mechanics pass → ready for live LLM campaign.
