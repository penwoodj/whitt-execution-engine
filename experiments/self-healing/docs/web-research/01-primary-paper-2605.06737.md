# Source: Primary Paper — A Self-Healing Framework for Reliable LLM-Based Autonomous Agents

- **URL:** https://arxiv.org/abs/2605.06737
- **Authors:** Cheonsu Jeong, Younggun Shin
- **Submitted:** 2026-05-07, cs.SE/cs.AI
- **Journal:** Int'l Journal of Software Engineering and Knowledge Engineering 2026, DOI 10.1142/S0218194026500695
- **Retrieved:** 2026-08-24 (abstract page + websearch corroboration; full PDF tables not directly parseable)

## What This Paper Defines (the experiment blueprint)

Four integrated components. This experiment implements all four as deterministic workflow stages.

### 1. Failure Taxonomy (F1–F4)

| Code | Failure | Definition |
|------|---------|-----------|
| F1 | Hallucination errors | Output factually incorrect or unsupported by context |
| F2 | Execution errors | Tool invocation failures, API errors, syntax errors in generated code |
| F3 | Reasoning inconsistency | Logical contradictions across multi-step reasoning path |
| F4 | Workflow propagation errors | Upstream step error cascades into dependent downstream steps |

### 2. Quantitative Reliability Model

```
R = ω₁·C + ω₂·S + ω₃·E
```

- **C** = output consistency (self-consistency; normalized edit distance d_norm across samples)
- **S** = semantic correctness (validated vs external knowledge / RAG)
- **E** = execution success rate (successful tool calls / total attempts)
- **ω₁, ω₂, ω₃** = weighting coefficients, tuned via grid search per task domain
- **Failure trigger:** R < θ, θ = 0.65 (set via preliminary experiments)

### 3. Failure Detection (hybrid)

1. **Execution pattern analysis** — repeated tool failures, abnormal execution sequences (catches F2/F4)
2. **Output consistency checking** — contradictions across reasoning paths (catches F1/F3)
3. **Signals:** R < θ; confidence-score degradation over last 3 outputs (F1); repeated tool failures (F2)

### 4. Self-Healing Mechanism (strategy per class)

| Class | Strategy | Mechanic |
|-------|----------|----------|
| F1 | Corrective prompting | Rewrite prompt to reduce ambiguity; verify confidence recovery |
| F2 | Tool re-selection | Pick alternative tool/API based on evaluation scores |
| F3 | Adaptive replanning | Exclude failed subtasks, decompose objective, priority-based re-execution |
| F4 | Adaptive replanning | Same as F3 — recover from cascading failure |

Re-execution engine loops corrected task until success or max retries.

## Evaluation Metrics (adopt for this experiment)

- **TSR** — Task Success Rate (final correct / total tasks)
- **FDA** — Failure Detection Accuracy (detected + correctly classified / actual failures)
- **RSR** — Recovery Success Rate (recovered / detected failures)
- **Failure propagation** — reduction vs baseline
- **Experimental setup:** artificial failures F1–F4 injected at 30% probability in multi-agent workflow, real-world task scenarios

## What We Borrow Directly

1. F1–F4 taxonomy → classifier output codes 1/2/3/4
2. R = ω₁C + ω₂S + ω₃E with θ=0.65 → classify.py scoring (spoofed components; ω = 0.35/0.35/0.30 initial)
3. Class→strategy map → GWT routing in workflow YAML
4. TSR/FDA/RSR → report.py metrics
5. Failure injection at controlled probability → case YAML `fail_attempts` lists (deterministic sequences instead of random 30%)

## Limitations Noted

- Exact equations for C, S, E components not retrievable from abstract; we approximate with deterministic detectors (see 11-guardrails doc)
- Secondary source claims +27% TSR vs basic retry — unverified, not from paper abstract; do not cite as paper result
