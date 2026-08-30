# 08-replan — Research & Reasoning

## Why this atom exists
Near-misses dominate small-model failures (fusion trap analysis: one
wrong rule application, format-clean). Resampling from scratch wastes the
90% that was right; reflexive self-correction silently collapses below
7B (P3). Scoped replan is the middle rung: re-orient the plan, keep the
context, re-derive only what the check flagged.

## Sources
- **TDP (2026):** replanning confined to active sub-task, −82% tokens vs
  global — H2's cost bound is this finding at atom scale.
- **Graph Harness (2604.11378):** strict escalation ladder (retry → local
  patch → replan) prevents the "failure loop" pathology — our chain is
  exactly this ladder with replan before escalation.
- **RandomCommits (fusion S-set):** conditional re-plan 63%→87% beats
  re-sampling — re-planning is cheaper than re-trying when the plan is
  what failed.
- **Leni (2607.17044):** planner re-planned ~38% of GAIA tasks, most
  recovered — replan is where production reliability lives.
- **Self-healing orchestrator (2606.01416):** failure-class-mapped
  recovery 98.8% vs 93.8% full-replanning — targeted recovery beats
  blunt resets. FAULT_PLAN delta = the injected failure class.
- **P3 Reflexion collapse (fusion + Sample-More 2607.28576):** self-
  inspection never fires below 7B → replan is EXTERNAL scaffold text the
  model re-reads, not introspection it must perform.
- **N6 leak-safe hints (fusion, proven):** feedback = check id + observed
  + question; hint never carries the expected value. H3 polices this.

## Reasoning chain
1. Inject deterministic near-miss (delta fault at solve) — recovery is
   then measurable, not sampled.
2. Replan stage sees prior attempt + leak-safe failure hint + replan
   prefix; its output re-orients solve2 (which re-reads digest).
3. Escalation beyond solve2 → sample-diverse (experiment 09's atom)
   composes as the next rung — interfaces must match (typed near-miss
   envelope in, plan delta out).

## What would change our mind
- If recovery concentrates in format-faults only, content near-misses
  need cross-model diversity (09) not re-orientation — narrow replan's
  claim accordingly.
- If cost ratio >0.6, the replan prompt is over-stuffed — trim to rule-
  delta only.

## Relation to opencode parity
opencode iterates on errors within a session cheaply. Scoped replan is
the workflow analog — without it, every failure costs a full re-solve.
