# Top-Level Workflow — Design (v1)

## What this is
The stitched control flow: one natural-language prompt enters, a
deterministic gate chain routes it through proven atom subworkflows to
a verified deliverable. v1 exists as `top-level/workflows/meta-v1-spoof.yml`
(153 steps, validate 9/9) and `meta-v1-live.yml` (9/9). The dry-run
simulator (`scripts/dry-meta-run.py`) executes the spoof variant
end-to-end with zero LLM calls — 8/8 prompts route correctly and pass
their routelog checks.

## Control flow

```
PROMPT (prompts/meta-prompts.yml — styled on real opencode sessions)
  │
  ▼
[conf gate]  meta-conf.py — probe (entropy/multi-part) → lane token
  ├─ "LIGHT" ──► solve → extract ─────────────────────┐
  ├─ "HEAVY" ──► plan → solve → extract → replan →     │
  │              solve2 → extract2 → verify → extract3 │
  └─ "SYNTH" ──► split → worker_a → extract_a →        │
                 worker_b → extract_b → merge → extract_m
  │                                                    │
  ▼                                                    ▼
[routelog]  spoof-emit --force writes route record; check-runner
            validates json_exact {lane, route_ok, shape}
  │
  ▼
next prompt's conf gate … terminal
```

Every arrow is a GWT clause on a deterministic shell-hook token
(`PASS`/`SKIP`/lane). The model never decides control flow. Lane heads
are reached via `route_to` — the previously-untested E2E pattern, now
proven by the dry runner.

## Composition rules (evidence-indexed)

| Rule | Source |
|------|--------|
| Route before you think (conf gate costs ~60 tok) | GlimpRouter, STEER, E6 |
| Lane heads via GWT route_to, never model choice | RCA boundary |
| Skip-if-won short-circuits every stage boundary | has_pass (proven) |
| Scaffold stages are UNCHECKED and never count as wins | spoof-emit passed=None |
| Escalation is fresh (no failed-context forwarding) | S57 −34.8pp |
| Workers isolated; merge is reasoning not concat | S53 |
| Det checks gate; judge lane records, never gates | two-lane (proven) |
| Route record is force-written per prompt | routelog --force |

## What the spoof dry run proves (and what it doesn't)

PROVEN (deterministically, zero LLM):
- Gate → lane-head routing works for all three lanes (61 hops, no
  fall-throughs)
- Per-lane chains walk stage-by-stage with correct PASS/SKIP semantics
- Skip-if-won fires after first win (case short-circuits to routelog)
- Routelog writes + validates for all 8 prompts (8/8 json_exact)
- All hook scripts (meta-conf, spoof-emit, check-runner) execute for
  real under the simulator

NOT proven by dry run (needs live phase, user-gated):
- Model outputs (spoof-emit stands in for the LLM)
- stage-emit prompt assembly in the top-level path (sub-experiment
  unit tests cover it; top-level live variant wires it)
- Real engine execution (`whitt benchmark`) — YAMLs validate 9/9, and
  the simulator mirrors engine semantics (quoted-token convention,
  PASS-fires-after-hooks, skip_step)

## Prompt inputs
`prompts/meta-prompts.yml` — 8 prompts in the user's actual chat voice
(imperative, multi-clause, tolerance suffixes like "no changes" /
"don't run any llm calls"). Each carries designed meta {lane, shape};
the probe file encodes the lane for spoof runs; live runs replace the
probe with real first-token logprobs + multi-part cue detection.

## File map

```
top-level/
├── workflows/meta-v1-spoof.yml   # 153 steps, 9/9 validate
├── workflows/meta-v1-live.yml    # stage-emit wired, 9/9 validate
├── cases/case-mp-NN.yml          # prompt + route-truth checks
├── fixtures/meta-fixes.yml       # computed route truths
└── runs/spoof-r1/                # probes, scenario, dry artifacts
scripts/
├── gen-meta-system.py            # generator (route_to lanes)
├── meta-conf.py                  # conf gate CLI
├── dry-meta-run.py               # engine simulator (zero LLM)
└── test_meta.py                  # unit tests — ALL PASS
```

## v2 triggers
- Live run shows lane mis-routing >30% → probe cue redesign
- Sub-experiment atoms graduate (per 03-ORCHESTRATION) → their
  validated chains replace v1 lane shapes
- 12-shape-library populated → conf gate + match stage composition
