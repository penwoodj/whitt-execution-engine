# Atomic Reasoning Experiments — North Star

## Objective

Build the **smallest unit of reasoning** that, when chained, augments a small local
LLM's capability to produce deliverable-quality outputs at the standard of much
larger models. Each atom is a single YAML workflow that takes one small input
chunk (a paragraph, a YAML block, a code snippet, a single question) and emits
one refined output, using the framework to offload as much cognitive work as
possible from the model.

## Why This Exists

The meta-workflow generator currently uses Qwen3.5-9B as a single generalist.
It over-decomposes simple prompts, hallucinates structure, and produces
templated categories unrelated to the actual input. The hypothesis: a small
model (3-4B) inside a tight, scope-bounded loop with deterministic verification
can outperform the 9B generalist on the same task at lower wall-clock cost.

Research backing this hypothesis:
- Self-Refine (Madaan 2023): +20% avg on GPT-3.5/4 via iterative feedback loop
- Small LMs Need Strong Verifiers (Zhang ACL 2024): pure self-refine fails ≤13B
- Entrospect (Yan ACL 2025): +36% acc, 10x faster than Self-Refine on ≤10B models
- Reflexion (Shinn 2023): verbal RL with episodic memory caps at Ω=1-3 reflections

## Operating Principles (HARD)

1. **Small Model, Small Context, Fast Loop**
   - Target models: 0.5B-4B parameters
   - Target context per pass: ≤2048 tokens
   - Target output per pass: ≤500 tokens
   - Anything bigger = wrong scope for the atom

2. **Model Never Sees Too Much At Once**
   - Each pass: ONE narrow question + ONE small artifact
   - Never: full context + all findings + all drafts simultaneously
   - Compounding happens in the engine (bookmarks), not in the prompt

3. **Different Scope Per Pass = Different Cognitive Mode**
   - GENERATE scope: produce, creative (temp 0.4)
   - CRITIQUE scope: find bugs, analytical (temp 0.2)
   - REFINE scope: apply findings, conservative (temp 0.3)
   - VERIFY scope: deterministic, NO MODEL

4. **External Verification Breaks Self-Confirm Bias**
   - Pure self-refine fails on small models (Zhang 2024)
   - The atom MUST have a Pass 4 (or equivalent) that is deterministic
   - Verification = shell hook + GWT clause + exit code, never model-based

5. **Bounded Iterations**
   - Hard cap at 3 cycles per atom
   - Beyond 3 = entropy collapse, no new info (per Reflexion findings)
   - Stability check (diff-based) auto-emits when changes < threshold

6. **Augmentation Comes From Framework, Not Model Size**
   - Loop control = YAML hooks (GWT, route_to, save_to)
   - Lens library = swappable per task type
   - Iteration cap = schema field, not prompt instruction
   - Model only does narrow text work; engine does state management

## Two Variants Under Test

### Variant A: Single-Model Loop (`single-model/`)
One small model (3-4B) does all passes. Different prompt per pass = different
scope. Cheaper hardware bill, simpler dependency chain. Risk: self-confirm bias
on hard inputs.

### Variant B: Multi-Model Specialist Cascade (`multi-model/`)
Tiny router (0.5B) classifies task type. Specialist model (Phi-4-mini-reasoning,
Gemma-3-1B, Qwen3-4B-Instruct) executes. Fallback generalist (Qwen3-4B) on
hard cases. Risk: routing misclassification, VRAM pressure, model swap overhead.

## Success Criteria (HARD gates)

Each variant must demonstrate, with live system evidence:

| Gate | Criterion | Evidence Required |
| --- | --- | --- |
| G1 | 5 distinct inputs produce 5 successful outputs | Output files committed under `results/` |
| G2 | No single pass takes >30s wall clock | Benchmark log with per-step durations |
| G3 | Quality measurably better than baseline single-shot | Side-by-side baseline comparison |
| G4 | Total atom wall clock <3 min including all loops | Aggregate timing log |
| G5 | Failure mode is bounded (no infinite loops) | Iteration count log shows ≤3 cycles per atom |

## Folder Layout

```
experiments/atomic-reasoning/
├── GUIDELINES.md                 (this file — north star)
├── single-model/                 (Variant A — single-model loop atom)
│   ├── atom-v1.yml               (workflow YAML)
│   ├── atom-v2.yml               (iteration after first test)
│   └── ...                       (one file per iteration)
├── multi-model/                  (Variant B — specialist cascade atom)
│   ├── atom-v1.yml
│   └── ...
├── inputs/                       (5+ example input chunks)
│   ├── input-01-summary.txt
│   ├── input-02-yaml-block.txt
│   └── ...
├── results/                      (live test outputs, per-experiment)
│   ├── exp-001-single-v1/
│   │   ├── output-01.txt
│   │   ├── metrics.json
│   │   └── NOTES.md
│   └── ...
└── benchmarks/                   (aggregate metrics across experiments)
    ├── SUMMARY.md
    └── comparison.csv
```

## 5 Example Inputs (mapping to meta-workflow sub-sectioning)

These represent the kind of chunks a full meta-workflow would feed to atoms:

1. **input-01-summary.txt** — A 200-word technical paragraph to summarize to 3 bullets
2. **input-02-yaml-block.txt** — A partial YAML workflow snippet needing schema fix
3. **input-03-task-decomposition.txt** — A simple user prompt needing 3-5 atomic tasks
4. **input-04-categorization.txt** — A list of 5 tasks needing category assignment
5. **input-05-code-snippet.txt** — A 30-line Python function needing improvement

## Iteration Protocol

For each variant:
1. Write `atom-v{N}.yml` (or update)
2. Run against all 5 inputs → capture under `results/exp-{NNN}-{variant}-v{N}/`
3. Record metrics: per-step duration, total duration, iteration count, quality (manual 1-5)
4. Compare to prior version + baseline
5. Identify weakest step → fix → v{N+1}
6. Repeat until all 5 gates pass

## What "Done" Means

All 5 success gates pass on BOTH variants. Final report under
`benchmarks/SUMMARY.md` with side-by-side comparison + recommendation for
which variant (or hybrid) to integrate into the meta-workflow generator.

## Hard Constraints (from AGENTS.md)

- 7B+ models MUST use `gpu_layers=99` (NEVER 0)
- Top-level repo cleanliness: this folder is the exception; all atom work
  MUST stay bounded here
- Commit + push every meaningful milestone
- No background/parallel agents (sync execution per AGENTS.md directive)
- Caveman style for notes/iteration logs; normal prose for this GUIDELINES file
