# Meta-Workflow Generator v6 — Plan Suite

**Status:** ACTIVE — Ralph Loop execution
**Started:** 2026-06-13
**Owner:** Sisyphus (autonomous)
**Model Target:** `qwen/qwen3.5-9b` (Qwen3.5-9B GGUF on llama.cpp/Vulkan, CPU-only)

## What This Is

A 5-sub-workflow **meta-workflow generator** that takes a high-complexity agentic
prompt as input and produces a fully-specified, executable agentic YAML workflow
file as output. The generator itself runs **entirely on the local Qwen3.5-9B
model** to compensate for SLM limitations through heavy iteration, scoped
evaluation, and deterministic fix-loops.

## Pipeline at a Glance

```
                ┌─────────────────────────────────────────┐
                │   META-WORKFLOW-GENERATOR-V6.YML        │
                │   (orchestrator; qwen3.5-9b only)       │
                └─────────────────────────────────────────┘
                                   │
   ┌──────────┬──────────┬─────────┴────────┬──────────┬──────────┐
   ▼          ▼          ▼                  ▼          ▼          ▼
┌─────┐   ┌─────┐   ┌─────┐            ┌─────┐   ┌─────┐
│ SW1 │ → │ SW2 │ → │ SW3 │       →    │ SW4 │ → │ SW5 │
└─────┘   └─────┘   └─────┘            └─────┘   └─────┘
 TASK      DESIRED   AGENTIC            YAML       FINAL
 BREAK     OUTPUT    CATEGORIES         SUB-       WORKFLOW
 DOWN      STATE                         STRUCT    .YML
   │         │          │                  │          │
   ▼         ▼          ▼                  ▼          ▼
tasks.md  outputs.md  categories.md   structs.md  workflow.yml
```

| # | Sub-Workflow | Single Output File | Purpose |
|---|--------------|--------------------|---------|
| SW1 | `task-deconstruction.yml` | `tasks.md` | Break prompt → atomic tasks with story-point complexity, sub-task expansion for any task ≥ 5 pts |
| SW2 | `desired-output-state.yml` | `outputs.md` | For each task → well-defined desired end-state (testable criteria) |
| SW3 | `agentic-categorization.yml` | `categories.md` | Bucket tasks into agentic behavior categories (correlative, not framework) |
| SW4 | `yaml-substructure-translation.yml` | `structs.md` | Translate categories → YAML substructure intent doc (YAML code blocks ONLY) |
| SW5 | `final-workflow-assembly.yml` | `workflow.yml` | Selectively assemble a valid executable agentic workflow YAML |

## Document Index

| File | Audience | Purpose |
|------|----------|---------|
| [00-PLAN-OVERVIEW.md](./00-PLAN-OVERVIEW.md) | ALL READERS | Executive summary, objectives, success criteria, scope boundaries |
| [01-CONTEXT-AND-CONSTRAINTS.md](./01-CONTEXT-AND-CONSTRAINTS.md) | implementer | Qwen3.5-9B spec, engine capabilities/incapabilities, schema source-of-truth map |
| [02-ARCHITECTURE.md](./02-ARCHITECTURE.md) | architect | Pipeline architecture, shell-orchestration pattern (sub_workflow: is STUB), state-passing strategy |
| [03-SUB-WORKFLOW-SPECIFICATIONS/](./03-SUB-WORKFLOW-SPECIFICATIONS/) | implementer | One detailed spec per sub-workflow (5 files + README) |
| [04-META-WORKFLOW-SPECIFICATION.md](./04-META-WORKFLOW-SPECIFICATION.md) | implementer | meta-workflow-generator-v6.yml spec (orchestrator) |
| [05-EXECUTION-PROTOCOL.md](./05-EXECUTION-PROTOCOL.md) | implementer | Ralph Loop iteration protocol: how to validate each SW before advancing |
| [06-TEST-DATASET.md](./06-TEST-DATASET.md) | implementer | Curated high-complexity prompts harvested from real opencode sessions |
| [07-QA-AND-VERIFICATION.md](./07-QA-AND-VERIFICATION.md) | implementer | Per-SW success criteria, baseline gates, iteration log schema |
| [08-ITERATION-LOG.md](./08-ITERATION-LOG.md) | implementer | Live append-only iteration log for each SW (run-by-run) |
| [09-OPEN-CODE-TOOLS-AND-SETUP.md](./09-OPEN-CODE-TOOLS-AND-SETUP.md) | implementer | Tooling research, install steps, environment prep |
| [10-LOGGING-STRATEGY.md](./10-LOGGING-STRATEGY.md) | implementer | Per-step log fields, log analysis scripts, evidence-gathering protocol |

## Artifacts Layout

```
artifacts/
├── sw1-task-deconstruction/
│   ├── baseline-opencode/    # My manual Sisyphus attempts (quality floor)
│   ├── iterations/           # Run-by-run SW outputs (iter-001.md, iter-002.md, ...)
│   └── golden/               # Final approved outputs that passed all gates
├── sw2-desired-output-state/   # same structure
├── sw3-agentic-categorization/ # same structure
├── sw4-yaml-substructure/      # same structure
└── sw5-final-assembly/         # same structure
```

## Operating Constraints (CRITICAL)

1. **Only Qwen3.5-9B** (`unsloth/Qwen3.5-9B-GGUF`, file `Qwen3.5-9B-UD-Q4_K_XL.gguf`) — no other models in any sub-workflow.
2. **CPU-only** (`gpu_layers: 0`) — Vulkan devices still present for KV cache Q8_0 acceleration but no layer offload.
3. **KV cache = Q8_0** — Vulkan supports it as of llama.cpp PR #20797 (2026-04-13).
4. **Context = 262144** — requires ≥ 32 GB RAM; expected 0.5–2 tps inference speed.
5. **No `sub_workflow:` step key** — engine treats it as STUB. Use **shell-based orchestration** instead.
6. **No `loop:` with validation** — only `iterate_values` hook in `before_step_starts` actually loops in the engine.
7. **Live system testing FIRST** — every SW must pass `whitt benchmark --workflow` runs on Docker llama.cpp before unit tests are written.
8. **Single .md output per SW** — each sub-workflow accumulates and iterates on ONE well-formatted markdown file.

## How to Resume

If context is lost, this plan suite is the single source of truth. Start by reading:
1. `00-PLAN-OVERVIEW.md` — understand the goal
2. `08-ITERATION-LOG.md` — find where work paused
3. `05-EXECUTION-PROTOCOL.md` — pick up the protocol at the right phase
