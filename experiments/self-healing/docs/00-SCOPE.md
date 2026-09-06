# Self-Healing Experiment — Scope

**Experiment ID:** self-healing | **Version:** v1 (design) | **Created:** 2026-08-24
**Blueprint paper:** arxiv 2605.06737 — A Self-Healing Framework for Reliable LLM-Based Autonomous Agents (Jeong & Shin 2026)
**Research base:** `docs/web-research/` (13 per-source docs + index)

## Objective

Implement + validate the paper's 4-component self-healing framework (failure detection → reliability scoring → classification → class-routed healing) as a whitt workflow, FIRST fully deterministic with spoofed LLM outputs, then (user-gated later) with live models.

## Phase Discipline (HARD — user directive 2026-08-24)

- **Phase S (this phase):** ZERO LLM calls. No model load, unload, or inference. All "model outputs" spoofed by deterministic scripts. Validate: decision logic (classification + routing), hook scripts, GWT expressions, workflow structure, metrics aggregation — everything deterministic in the .yml.
- **Phase L (later):** live models. BLOCKED until user explicitly says so.

## Framework Mapping (paper → workflow)

| Paper component | Workflow implementation |
|-----------------|------------------------|
| Failure taxonomy F1–F4 | case YAML injection + classify.py exit codes 1/2/3/4 |
| Detection (execution patterns + output consistency) | detect.py deterministic suite (json/schema/refusal/length/tool-errors/markers) |
| Reliability R = ω₁C+ω₂S+ω₃E, θ=0.65 | classify.py (ω=0.35/0.35/0.30, θ=0.65; spoofed C/S/E components) |
| Healing: F1 corrective prompt, F2 tool reselect, F3/F4 replan | heal.py + GWT-routed heal steps per class |
| Re-execution loop (max retries) | unrolled attempt 1→2→3 chain (engine forbids cycles) |
| Eval: TSR/FDA/RSR, injected failures | report.py vs case ground truth |

## Model Scope (from harness + rea+ experiments — see 03-MODEL-SCOPE.md)

| Role | Model | Source experiment |
|------|-------|-------------------|
| Worker (task attempts) | Qwen3-4B-Instruct-2507-Q4_K_M | rea+ stage-2 pick + harness worker |
| Corrective prompting (F1 heal) | Ministral-3-3B-Instruct-2512-Q4_K_M | rea+ stage-2 pick |
| Tool re-select (F2 heal) | Qwen2.5-Coder-3B-Instruct-Q8_0 | rea+ stage-2 pick |
| Replan (F3/F4 heal) | Qwen3-5-9B-Q4_K_M | harness heavy |
| Accept gate (final verify) | Hermes-2-Pro-Mistral-7B.Q4_K_M | harness judge |

Phase S: all referenced via `generative_entity` but every step exits via `skip_step: true` or guaranteed GWT route BEFORE inference → zero loads/calls (v9-hc-05-spoof proven pattern).

## In Scope (Phase S)

1. Research folder (done — 13 sources)
2. Experiment docs (this suite)
3. Case suite: 6 probe cases (clean, F1, F2, F3, F4-single, F4-multi/persistent-fail)
4. Spoof scripts: agent_run, detect, classify, heal, outcome, report (+ shared lib)
5. Workflow generator gen-v1.py → per-case workflow YAML (20 steps, unrolled heal chain)
6. Structural validation (scripts/meta-v6/validate-workflow.py) — all pass
7. Live system test: `whitt benchmark --workflow <generated.yml>` per case; assert zero inference events; TSR/FDA/RSR = expected values; iterate until green

## Out of Scope (Phase S)

- Any LLM inference (Phase L)
- Real self-consistency sampling, real verbalized confidence extraction
- Weight tuning via grid search (weights fixed 0.35/0.35/0.30, recorded per run)
- Security failure modes (Microsoft taxonomy — deferred)
- MCTS/LATS-style multi-branch search (noted as future comparator)

## Success Criteria (Phase S exit)

1. `validate-workflow.py` exits 0 on all 6 generated workflows
2. Live runs complete: every case reaches report step, writes report.json + trace.jsonl
3. Log evidence: NO `model_load`, `model_unload`, inference, or HTTP chat-completion events — greppable proof
4. Metrics match ground truth exactly: clean/f1/f2/f3/f4-single → pass paths with correct class routing; f-persist → fail path; suite-level TSR=5/6, FDA=1.0 on injected failures, RSR=4/5
5. Report oracle: report.py exit 0 on all cases
