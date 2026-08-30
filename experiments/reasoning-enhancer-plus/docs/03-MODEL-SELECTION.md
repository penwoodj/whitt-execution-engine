# REA+ — Stage 1: Model Selection (prerequisite)

**Gate: stage-2 authoring forbidden until results/MODEL-PICKS.md exists.**

## Pool (installed, 4B–11B, general/reasoning-capable)

| # | model file | params | note |
|---|---|---|---|
| 1 | Qwen3-4B-Instruct-2507-Q4_K_M | 4B | current cascade solver |
| 2 | Qwen3-4B-Thinking-2507-Q4_K_M | 4B | reasoning variant |
| 3 | Phi-4-mini-instruct-Q4_K_M | 3.8B | strong instruction-follow rep |
| 4 | Phi-4-mini-reasoning-Q4_K_M | 3.8B | reasoning variant |
| 5 | Yi-6B-200K-Airo-Claude-Puffin-Q4_K_M | 6B | 200k ctx |
| 6 | Mistral-7B-Instruct-v0.3-Q4_K_M | 7B | classic strong IO |
| 7 | Hermes-2-Pro-Mistral-7B.Q4_K_M | 7B | instruction-tuned rep |
| 8 | Falcon-H1-7B-Instruct-Q4_K_M | 7B | hybrid mamba arch |
| 9 | LWM-Text-Chat-1M-Q4_K_M | 7B | 1M ctx |
| 10 | Qwen3-5-9B-Q4_K_M | 9B | prior baseline big model |

Excluded: code-only (starcoder2/granite-code/Refact), <4B, abliterated
variants (unpredictable for benchmarking), duplicates (v0.2 Mistral).

## Probe battery

20 items: 5 categories × 4. `cases/probe/`. Each item:
- prompt + optional auxiliary (source material)
- deterministic checks via check_lib (same as REA)
- reference fix in `fixtures/probe-fixes.yml` (lint-enforced)
- ground truth INDEPENDENTLY derived (hand-computed, shown in fixture
  comment) — not from any model output

Temp 0.0 greedy, max_tokens 250. One load per model, 20 sequential
inference steps. ~6 min/model expected.

## Scoring

```
cat_score(model, cat) = probe_pass_rate(cat)          # 0..1, 4 items
wall(model)          = clean run wall                  # tiebreak
pick(cat)            = argmax cat_score; tie -> lower wall
```

Extra pick constraints:
- model with 0/4 on a category cannot win it
- one model may win ≤2 categories
- picks recorded with per-category scores + walls in MODEL-PICKS.md

## VRAM / safety schedule (HARD, from AGENTS.md)

- ONE model resident at a time (7B+ pairs exceed 8GB RX 580)
- ≤5 consecutive model tests → 60s cooldown + `free -h` check ≥3GB avail
- Between models: unload, confirm /health, swap load
- Any vk::/DeviceLost in docker logs → docker restart + 30s settle
- Preflight per model: RAM≥3GB, swap ACTIVITY (si+so) <25MB/8s
  (swap-used level ignored — B13 lesson)

## Run ladder (smallest portion first)

1. `lint-cases.py` — fixtures pass own checks, ids unique
2. smoke: 1 model × 2 items → verify artifacts + fingerprint.log
3. full sweep: 10 models × 20 items (resume-safe: completed models
   skipped by presence of summary.json)
4. `rank-models.py` → results/model-probe/RANKING.md + MODEL-PICKS.md

## Resume design

`run-probe-model.sh <model>` writes `results/model-probe/<model>/summary.json`
atomically on success. Sweep driver skips dirs with valid summary.json.
Watchdog kill (rc=143) → rerun resumes at model granularity.
