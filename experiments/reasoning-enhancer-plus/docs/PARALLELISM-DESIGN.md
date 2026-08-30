# Parallelism Design — Engine-Native DAG Execution

Status: DESIGN ONLY (not implemented). Written 2026-08-21 from live
evidence: native-v6b (case-major, 115.5m), native-v7 (stage-major,
71.6m, 67/73). Ground-truth profile: swaps eliminated by ordering;
remaining wall = token decode (Thinking 43.3m, Bonsai 21.8m) +
sequential step walk.

## 1. What exists today (verified in runner.rs)

| Mechanism | Behavior | Limitation |
|---|---|---|
| `depends_on` | Topological sort of steps | Sort-only: execution stays a linear `while current_index < steps.len()` walk |
| `requires` | Skip step + fire `on_requires_failed` hook when deps missing | Skip, not wait — cannot express join |
| `route_to: [a, b]` | Parallel ONLY when all targets share one model (`try_execute_route_to_parallel`) | No heterogeneous fan-out |
| `WHITT_MAX_CONCURRENT_INFERENCES` | Semaphore permits (VRAM-derived, env override) | Nothing schedules concurrent work onto it |
| Server children | Spawned `--parallel 1` | No prompt-level batching per child |
| `workflow_execution_strategy.timing` | cooldown / load-timeout knobs | No concurrency or admission policy keys |

## 2. Missing capabilities (the five pillars)

1. **Ready-queue DAG scheduler** replacing the linear walk.
2. **Waiting `requires`** — defer + re-enqueue on dependency arrival, with timeout + `on_requires_timeout` hook (parallel to existing `on_requires_failed`).
3. **Heterogeneous fan-out** — multi-target route_to across different models, each a concurrent task.
4. **VRAM admission controller** — residency budget map deciding which eligible tasks may start now (4B+1.7B fits 8GB; Bonsai+anything does not). Extends model_memory metadata.
5. **Join policies + write isolation** — `first_success | all | quorum(n)` convergence semantics; step-keyed bookmarks already isolate outputs; need deterministic `$text_state`/save_to collision rules (namespacing by fan-out arm) and documented hook firing order at joins.

## 3. Vertical-slice architecture

Each slice = schema + parser + scheduler path + hooks + tests, landed
independently (mirrors hooks vertical: context struct → action →
runner wiring → integration test):

```
slice 1: scheduler core (async executor, no semantics change)
  runner.rs: run() loop -> ready-queue executor
  semantics preserved: sequential workflows run identically
  tests: linear workflow byte-identical output vs old walk;
         route_to jumps still honored

slice 2: waiting requires
  schema: requires_timeout_secs (per-step, optional)
  hooks: on_requires_timeout context struct (parallel to
         OnRequiresFailedContext)
  tests: join step waits for both parents; timeout fires hook;
         legacy skip behavior when no timeout configured

slice 3: heterogeneous fan-out
  route_to multiple targets w/ different models -> concurrent tasks
  admission: single-model fast path already exists; multi-model path
  gates on residency map
  tests: two-model fan-out both complete; results land in
         step_outputs keyed per target

slice 4: admission controller
  schema: models.<m>.model_memory already exists — needs GB semantics
  scheduler consults budget map; defers (does not fail) when over
  tests: Bonsai+Thinking fan-out serializes on 8GB; 4B+1.7B runs
         concurrently

slice 5: join policies
  schema: step-level `join: first_success|all|quorum(n)`
  cancel semantics: first_success aborts sibling tasks (cooperative:
    check a shared AtomicBool between decode chunks; server API has
    no cancel — stop consuming + ignore result)
  tests: race produces exactly one winner; quorum waits for 2;
         all-wait barrier
```

Landing order is dependency-ordered: 1 → 2 → (3,4) → 5. Each slice
shippable alone.

## 4. YAML expressiveness target

Existing keys carry most of the weight (route_to, requires, hooks).
Additions needed — all additive, deny_unknown_fields requires schema
+ struct fields at every level:

```yaml
# per-step (slice 2)
requires_timeout_secs: 120

# per-step join (slice 5)
join: first_success        # or: all | quorum: 2

# workflow-level (slice 4)
workflow_execution_strategy:
  concurrency:
    max_parallel_steps: 2          # scheduler breadth cap
    vram_admission: auto           # auto | off
    same_model_prompt_parallel: 1  # child --parallel N
```

Hook surface at joins (reuses existing trigger mechanics):
`before_join_waits`, `after_join_resolves` (context: arms, winner,
cancelled list) — same pattern as before/after_gwt_evaluates.

Expression: today's phase-batched v7/v8 YAML is a manual DAG
linearization. With slices 1-5 the SAME semantics collapse to
per-case fan-out stages and the engine finds the parallelism:

```yaml
rescue_fanout:
  # conceptual target form
  route_to: [think_rescue, bonsai_rescue]
  join: first_success
converge:
  requires: [think_rescue, bonsai_rescue]
  requires_timeout_secs: 180
```

## 5. Expected speed benefit (this workflow, 8GB RX 580)

From v7 measured anatomy (71.6m = swap 10.4 + Thinking 43.3 + Bonsai
21.8 + small-model 4.7 + overhead):
- small-model lane overlap (4B/1.7B concurrent with big decode):
  -15-20m
- rescue racing (first_success kills loser): -5-8m
- Bonsai+Thinking co-residence impossible on 8GB — that overlap
  requires 12GB+ card
- same-model --parallel: decode-bound, ~10-20% on probe battery only
Projected v8-parallel wall: ~45-52m (from ~55-60m sequential v8).
Absolute floor ~28-35m = Thinking decode minus cascade diversion.

## 6. Risks / open questions

- Cancel semantics without server-side abort: cooperative polling
  between chunks; worst case wasted decode until chunk boundary.
  Mitigation: small n_predict chunks for fan-out arms.
- Hook ordering under concurrency: bookmark store is Mutex<HashMap> —
  contention trivial at our scale; log hooks may interleave (already
  timestamped).
- Persistence/checkpoint (agent/persistence.rs) assumes linear step
  order — slice 1 must define checkpoint = completed-step set, not
  index.
- Zombie/health guards assume one child; admission controller must
  own restart policy for concurrent children (extend existing
  preflight, which already handles multi-child zombie detection).
