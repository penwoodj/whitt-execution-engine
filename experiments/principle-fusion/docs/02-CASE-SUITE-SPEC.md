# 02 — Case Suite Spec

12 cases, 6 engines x 2 difficulty variants. Every prompt 1000-1150
words, user's prompt voice. Truths COMPUTED by the generator
(execute the stated rules) — never hand-typed, never from model
output. Each case carries `rea_plus` metadata + `needs:` principle
list (drives ablation predictions).

## Prompt anatomy (every case, same skeleton)

```
role sentence + domain frame
RULES (n stacked, interacting, priority-ordered)
EVENTS (chronological, mid-document traps)
worked micro-example with DIFFERENT numbers (N8)
and to say the rules one more time:   ← digest slice marker (expansion starts)
rule restatement (doubled-back, paraphrased — must NOT add new facts)
lore / provenance paragraph (flavor, zero task info)
discipline paragraph (how to check)
drift warning (what changes if misread)
scope note (what counts, what does not)
before committing, recheck each number:  ← spec-tail marker (digest keeps tail)
checklist rehearsal (3-4 bullets)
OUTPUT CONTRACT: exact JSON keys + value menus (N7)
```

Digest keeps `[0, marker)` + `[tail_marker, end)` — core facts +
recheck tail + contract. Everything between = expansion boilerplate.

## Engines, rules, truths

### e_quota — pool quota banking
Rules: pool starts P; requests grant min(need, pool_left) (partial
grants allowed, deplete pool); arrival with pool_left==0 → DENIED
(list id); expired grant returns floor(grant/2) to pool at its expiry
event; grants never return otherwise.
- **fu-01 (H)** P=50, needs [18,22,16,9,14,6], grant-2 expires after
  r3. Truth computed by simulation.
- **fu-02 (L)** P=40, needs [15,20,10,12], grant-1 expires after r3.

### e_backoff — retry backoff + suppression
Rules: delays before retries double from 2 (2,4,8,16,...); cumulative
delay compared to wait-budget W before the attempt that succeeds; if
cumulative > W the call is suppressed — EXCEPT the budget doubles
exactly once (first breach) and re-compares; suppression only if still
over after the double.
- **fu-03 (H)** W=20, succeeds on retry 4 (delays 2+4+8+16=30 > 20 →
  double → 40 → issued). Keys: attempts, wait_s, budget_after,
  suppressed.
- **fu-04 (L)** W=15, succeeds retry 3 (2+4+8=14 ≤ 15, no breach).

### e_canary — staged rollout gates
Rules: rollout climbs 10% steps; limits: steps 1-5 limit 2.0, steps
6-10 limit 1.0 (error %); breach → pause + rollback to previous step;
FIRST breach resumes with tightened limit 0.8; SECOND breach → STOP
(hold current step, status STOPPED).
- **fu-05 (H)** breach at 30% (1.6 ≥ 1.5? no — engine sets exact
  rates), rollback to 20%, resume, second breach at 50% → STOP at 40%.
  Keys: rollout_pct, rollbacks, status.
- **fu-06 (L)** single breach at 60% (1.9 ≥ 1.0) → PAUSED at 50%.
  Keys: rollout_pct, stages_done, status.

### e_residency — model load/evict accounting (our domain)
Rules: VRAM cap 8; load only if free ≥ size else evict LRU unpinned
until fits or only pinned remain → DEFERRAL (count, list id); pinned
models never evicted; loads/unloads counted per event.
- **fu-07 (H)** start A(3)+B(5) with A pinned; need D(6): evict B,
  free 5 < 6, A pinned → defer D; need E(4): free 5 ≥ 4 load E;
  unload E; need D again → defer again. Keys: loads, deferrals,
  evictions, deferred (ids).
- **fu-08 (L)** start A(3)+B(5) unpinned; need C(4): evict LRU B →
  load C; need B(5): resident A+C=7, free 1 → evict LRU A → load B.
  Keys: loads, load_s, unload_s (30s/10s tariffs).

### e_preempt — priority preemption + checkpoints
Rules: HIGH preempts LOW instantly; LOW checkpoints every 2 completed
minutes (resume from last checkpoint, partial work lost); HIGH runs
to completion; queue drains by priority, resumed jobs outrank equal-
fresh at tie.
- **fu-09 (H)** LOW1 6min starts t0 (ckpt t2), HIGH 3min arrives t2
  (wasted 0), then LOW1(resumed) vs LOW2(fresh) tie → LOW1 first.
  Keys: makespan_min, wasted_min, finish_order.
- **fu-10 (L)** LOW 5min, ckpt t2, HIGH arrives t3 (wasted 1), LOW
  resumes t5 from 2min, runs 3 → t8. Keys: makespan_min, wasted_min,
  resume_points.

### e_epistemic — source precedence audit
Rules: precedence machine-log > signed-handoff > hearsay; dashboard
KNOWN-BUGGY (unusable alone); claim resting solely on hearsay or
solely on dashboard → UNVERIFIED; higher source contradicting lower
→ higher wins, lower's assertion is FALSE.
- **fu-11 (H)** 5 claims incl. handoff contradicted by machine log →
  false. Keys: true, false, unverified (id lists).
- **fu-12 (L)** 4 claims, no contradictions. Keys: true_claims,
  false_claims, unverified.

## Voice rules (authoring constraints)

- lowercase, comma splices, imperatives; "watch the traps"
- numbers as digits (digest preserves digits — [S27] format stability)
- worked example numbers MUST be disjoint from task numbers (N8 test)
- no new facts after restatement marker; lore carries zero info
- contract = exact JSON shape w/ value menus (SHIP/PENDING-style)

## Generation + self-verification

`gen-fusion-cases.py` builds `cases/case-fu-NN.yml` +
`fixtures/fusion-fixes.yml`. Per case it: (1) simulates rules to
compute truth, (2) renders prompt, (3) asserts word count 1000-1150,
(4) asserts example/task number disjointness, (5) runs check_lib
json_exact against truth text, (6) writes derivation comments into
the fixture. Failure of any assert aborts generation.
