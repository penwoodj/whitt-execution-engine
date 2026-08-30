# HB Live Results — 30/30 Confirmed

> 2026-08-23. v7 harness, 20 new natural-language cases (1500+ words each) + 10 original ha cases.

## Final: 30/30 PASS

| Suite       | Pass      | Notes                                              |
| ----------- | --------- | -------------------------------------------------- |
| ha-01..10   | 10/10     | carried over from v7 run                           |
| hb-01..20   | 20/20     | new natural-language, >1500 words each             |

Timing: hb cases 55-110s each. Mostly depth-0 wins. fix_1 recoveries: hb-14, hb-16 (leak-safe feedback). Judge=fail overridden by det final=pass: hb-14, hb-18 (two-lane scoring).

## Case Inventory (20 new domains, zero overlap with ha)

| Case  | Domain                | Diff  | Hops | Key trap                                        |
| ----- | --------------------- | ----- | ---- | ----------------------------------------------- |
| hb-01 | pager precedence      | light | 3    | stored-state→0-rerun semantics                  |
| hb-02 | budget committee      | light | 4    | signoff≠commit                                  |
| hb-03 | incident dedupe       | light | 3    | 3-source overlap counting                       |
| hb-04 | cloud billing         | heavy | 5    | 5-digit subtraction (offloaded to aggregate)    |
| hb-05 | release train         | heavy | 5    | rollback→waitlist interaction                   |
| hb-06 | SLA pause             | heavy | 5    | paused-clock arithmetic (240+30=270)            |
| hb-07 | pipeline freshness    | light | 4    | stale-partition exclusion                       |
| hb-08 | access grants         | light | 3    | expired/revoked/pending state matrix            |
| hb-09 | warehouse waves       | light | 4    | quarantine→recovery two-step                    |
| hb-10 | kitchen prep          | heavy | 5    | substitution vocabulary                         |
| hb-11 | flight rebooking      | heavy | 6    | standby=0 + volunteer=1 literals                |
| hb-12 | interlibrary loans    | light | 3    | per-tier fill counting                          |
| hb-13 | inspection chain      | light | 4    | reinspection→occupancy delay                    |
| hb-14 | pharmacy validation   | heavy | 6    | refer-vs-substitute-vs-call decision tree       |
| hb-15 | game MMR reset        | light | 3    | conditional reset eligibility                   |
| hb-16 | irrigation zones      | light | 4    | rain-skip→runtime-recalc                        |
| hb-17 | editorial workflow    | heavy | 5    | desk-reject vs review-round counting            |
| hb-18 | freight hazmat        | heavy | 5    | placard→trailer-count constraints               |
| hb-19 | trial enrollment      | heavy | 6    | screen-fail vs consent-withdraw arms            |
| hb-20 | escape-room staffing  | heavy | 5    | role-coverage minimums under no-shows           |

All ≥1500 words (min 1505, max 1604). Natural prose: rule histories, worked procedures, named distractors, closing rituals.

## Iteration History (16/20 → 20/20)

| Failure                             | Root cause                        | Fix                                              |
| ----------------------------------- | --------------------------------- | ------------------------------------------------ |
| hb-04 wrong total twice             | 9B 5-digit subtraction errors     | P2 offload: derived fields → aggregate.py expr   |
| hb-10 wrong vocab                   | menu said 'substituted'≠'usable'  | exact-token answer menus                         |
| hb-11 wrong key name                | menu 'confirmed_flew'≠'flew'      | exact-token menus                                |
| hb-12 wrong tier keys               | menu 'filled_local'≠'local'       | exact-token menus                                |
| hb-14/16 first-attempt fails        | genuine complexity                | fix_1 recovered (cascade working as designed)    |

## Verification Stack

- 150 unit tests passing (incl. 10 hb-case integrity tests)
- 30/30 YAMLs validate from repo root
- Spoof spot-checks: hb-03 wind0, hb-17 wind1 (correct routing + final JSON)
- Aggregate reproducibility: all 30 verified programmatically
- Word count: all hb ≥1500 programmatically enforced
