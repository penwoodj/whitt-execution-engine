# Self-Healing Experiment — Web Research Index

Per-source research docs. Each summarizes: method mechanics, healing trigger, stop condition, metrics, what we borrow for THIS experiment design.

| # | Doc | Source | Role in design |
|---|-----|--------|----------------|
| 01 | [primary-paper-2605.06737](01-primary-paper-2605.06737.md) | arxiv 2605.06737 | **Blueprint**: F1–F4 taxonomy, R=ω₁C+ω₂S+ω₃E (θ=0.65), class→strategy heal, TSR/FDA/RSR |
| 02 | [reflexion](02-reflexion-2303.11366.md) | arxiv 2303.11366 | bounded retries (3), reflection artifact between attempts, escalation |
| 03 | [self-refine](03-self-refine-2303.17651.md) | arxiv 2303.17651 | k=3 knee, structured feedback, early-exit on "looks good" |
| 04 | [critic](04-critic-2305.11738.md) | arxiv 2305.11738 | external verification > self-judge; tool-swap on exec fail (F2 lineage) |
| 05 | [self-debugging](05-self-debugging-2304.05128.md) | arxiv 2304.05128 | feedback richness ladder; explain-before-fix; turn ceiling |
| 06 | [lats](06-lats-2310.04406.md) | arxiv 2310.04406 | strategy-as-branch; budget accounting; future upper-bound comparator |
| 07 | [mast-taxonomy](07-mast-taxonomy-2503.13657.md) | arxiv 2503.13657 | taxonomy coverage validation; termination-as-failure-class; κ bar for live phase |
| 08 | [self-consistency](08-self-consistency-2203.11171.md) | arxiv 2203.11171 | C component (p_SC); p<0.3 uncertainty flag; N=5 hardware compromise |
| 09 | [verbalized-confidence](09-verbalized-confidence-2412.14737.md) | arxiv 2412.14737 + 2510.23458 | confidence degradation signal (F1); τ threshold discipline; small-model simplicity |
| 10 | [au-eu-uncertainty](10-au-eu-uncertainty-2604.17112.md) | arxiv 2604.17112 | confident-errors problem; cross-model EU as v2 signal |
| 11 | [guardrails-schema-validation](11-guardrails-schema-validation.md) | Guardrails AI + NeMo docs | **detector suite**: json/schema/refusal/length/tool-error markers, LLM-free first line |
| 12 | [agent-defect-detection](12-agent-defect-detection-2412.18371.md) | arxiv 2412.18371 + MS taxonomy | static+dynamic detection split; LARD→F2 priority |
| 13 | [2026-landscape](13-2026-self-healing-landscape.md) | multi-source sweep | peer positioning; differentiator = engine-native YAML heal |

## Design Decision Synthesis

1. **Taxonomy**: F1–F4 from primary paper, coverage-checked vs MAST FC1–FC3
2. **Detection**: deterministic first-line suite (doc 11) + spoofed soft signals (confidence, consistency per docs 08–10)
3. **Scoring**: R = 0.35·C + 0.35·S + 0.30·E, θ = 0.65 (primary paper; weights initial, tunable via case data)
4. **Healing**: class-routed strategies, bounded 3 attempts (docs 02–05 convergence)
5. **Evaluation**: TSR / FDA / RSR with injected ground truth (primary paper + MAST trace methodology)
