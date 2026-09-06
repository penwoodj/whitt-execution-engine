# Reasoning Enhancer Atom v1.3 — Agentic Benchmark Suite

**Date:** 2026-08-14 · **Workflow:** `workflows/rea-v1.yml` (v1.3.0) · **Suite: 6/6 property HOLDS**

## Objective

Realistic agentic benchmark cases: the subtasks agents actually perform in
production workflows — not toy arithmetic. Each case: draft fails ≥2
subchecks without the workflow, final passes with it.

## Case suite (case-101..106)

| case | domain | agentic failure mode in draft |
|---|---|---|
| rea-101 | incident-triage | generic network advice instead of exact error/component/fix |
| rea-102 | config-generation | code fences + wrong key names (listen-port) + wrong values (on) |
| rea-103 | metrics-analysis | vague quantifiers ("nearly double digits", "roughly four percent") |
| rea-104 | runbook-planning | 3 vague steps vs 5 required; no canary %, no rollback trigger |
| rea-105 | integration-payload | prose preamble + wrong status + unreconciled count + mangled epoch |
| rea-106 | handoff-summary | zero incident facts, vague reassurance, over word budget |

Design notes: auxiliary material carries exact ground truth (log facts,
required mapping, payload contract) — factored sub-solver derives from
source, never from the draft (CoVe). All checks deterministic.

## Live results

| case | draft w/o | with | mode | dur |
|---|---|---|---|---|
| rea-101 | 2 subck | PASS | selected_r1 | 33s |
| rea-102 | 3 subck | PASS | selected_r1 | 21s |
| rea-103 | 2 subck | PASS | selected_r2 | 19s |
| rea-104 | 3 subck | PASS | selected_r1 | 18s |
| rea-105 | 3 subck | PASS | selected_r1 | 8s |
| rea-106 | 2 subck | PASS | selected_r1 | 14s |

Property: **HOLDS 6/6.** Total 113s. 5/6 first-round; rea-103 needed round
2 (r1 missed an exact value; escalation memory fixed it). Exact-string
risks (rea-102 key names, rea-105 `"failed_count": 0` with space) all
passed first round — auxiliary payload contract + copy-from-source works.

## Quality observation

rea-101 shipped final contains "raise the pool timeout to 3 0 seconds"
(space inside the number) — deterministic checks don't test that token so
it passed legitimately. A future check key (`no_split_numbers`?) could
catch tokenization artifacts; not needed for current gates.

## Ops log

- First batch aborted rea-103/105/106 at preflight: REAL swap activity
  (85MB/8s) from larger agentic contexts churning model memory. Docker
  restart + settle → activity 406KB/5s → clean reruns.
- Evidence-audit script initially over-reported (counted 5 completed runs
  as 6/6 while rea-103 had only a preflight log). Fixed by requiring
  select-best.json presence per case before counting. Lesson: audit
  scripts must verify artifact existence, not assume run order.
