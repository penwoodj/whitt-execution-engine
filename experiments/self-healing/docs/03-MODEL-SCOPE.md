# Self-Healing Experiment — Model Scope

**Version:** v1 | Scoped to models recently in use in harness + rea+ experiments (user directive 2026-08-24)

## Model Registry (workflow `models:` block)

| Key | Model | Quant | Origin experiment usage | Self-healing role |
|-----|-------|-------|------------------------|-------------------|
| m_worker | Qwen3-4B-Instruct-2507-Q4_K_M | Q4_K_M | rea+ stage-2 direct pick (top 50-model sweep); harness v9 worker | Task attempts 1–3 (the "agent" being healed) |
| m_ministral | Ministral-3-3B-Instruct-2512-Q4_K_M | Q4_K_M | rea+ stage-2 direct pick (10KB/11-key valid JSON ADR) | F1 heal: corrective prompting |
| m_coder | Qwen2.5-Coder-3B-Instruct-Q8_0 | Q8_0 | rea+ stage-2 direct pick (1.1KB/8-key ADR) | F2 heal: tool re-selection |
| m_heavy | Qwen3-5-9B-Q4_K_M | Q4_K_M | harness v9 heavy (solve/fix/retry cascade); rea+ best PLN | F3/F4 heal: adaptive replanning |
| m_judge | Hermes-2-Pro-Mistral-7B.Q4_K_M | Q4_K_M | harness v9 blind-auditor judge; rea+ best FMT | Accept gate verification |

REA+ stage-1 probe pool context (not cast, on file): Qwen3-4B-Thinking-2507, Phi-4-mini-instruct/reasoning, Yi-6B-200K-Airo, Mistral-7B-Instruct-v0.3, Falcon-H1-7B-Instruct, LWM-Text-Chat-1M — rankings in `experiments/reasoning-enhancer-plus/results/model-probe/RANKING.md`.

## Role Rationale (research-grounded)

- **Worker = Qwen3-4B-Instruct:** rea+ best overall (total 0.55) + harness worker role = current production solver. Failures we inject mirror its known weak categories (AUD 0.25-class errors).
- **F1 heal = Ministral-3-3B:** rea+ stage-2 validated strongest JSON-ADR emitter (10KB/11 keys) → corrective prompting benefits from instruction-precise small model (cheap re-ask, Guardrails reask budget pattern).
- **F2 heal = Qwen2.5-Coder-3B:** tool-call/API errors = code-shaped failures; coder variant selected for structured call repair (LARD dominance, doc 12).
- **F3/F4 heal = Qwen3-5-9B:** replanning = decomposition + prioritization = harness heavy's cascade role; rea+ best PLN (0.75).
- **Judge = Hermes-2-Pro-7B:** accept-gate blind audit; rea+ best FMT (1.00) + harness judge role.

## Hardware Constraints (HARD — AGENTS.md)

- ALL 7B+ models: gpu_layers MUST be 99 if ever loaded (Phase L only)
- NEVER load 2+ models simultaneously (RX 580 8GB)
- Context: default 32768 (engine default; no step needs more in Phase S since prompts are never sent)
- Phase S: zero loads — all steps exit via skip_step/gwt before inference. Workflow references models ONLY as `generative_entity` labels (which model WOULD run each role live).

## Phase L Swap Contract (when user authorizes)

1. Replace spoof agent step with real inference: `generative_entity: ${models.m_worker}`, real prompt, remove skip_step
2. Keep detect/classify/heal/report scripts UNCHANGED (H5)
3. Load/unload discipline: one model resident at a time; judge last; cooldown per AGENTS.md
4. Failure injection moves from case YAML script-level to prompt-level perturbations (same classes)
