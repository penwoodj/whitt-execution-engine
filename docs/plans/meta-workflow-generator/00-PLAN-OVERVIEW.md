# 00 — Plan Overview

## 1. Objective

Build a meta-workflow generator that, given a single high-complexity agentic prompt
as input, produces a fully-specified, executable agentic workflow YAML file as output
by routing the prompt through **five sequential sub-workflows** (SW1 → SW5), each
producing a single well-formatted Markdown artifact. Every sub-workflow runs
**exclusively** on the local Qwen3.5-9B model via `whitt benchmark --workflow`.
The orchestrator (meta-workflow-generator-v6.yml) chains the five sub-workflows via
shell invocations and accumulates outputs on disk.

## 2. Why Five Sub-Workflows

Direct prompt → YAML generation fails on small local models (≤ 9B params). Quality
collapses when the model must simultaneously plan, evaluate, categorize, structure,
and emit final code in one shot. Decomposing into five scoped stages — each with
internal iteration loops, GWT-based quality gates, and deterministic fix cycles —
compensates for SLM limitations by allowing focused refinement per stage.

## 3. Sub-Workflow Summaries

### SW1 — Task Deconstruction (`task-deconstruction.yml`)
**Output:** `tasks.md` (single MD file, accumulated & iterated)

Pipeline:
1. Receive raw prompt as input.
2. LLM call: break prompt into candidate atomic tasks (group of ~5).
3. LLM call per group: score complexity in story points (1–13 scale) by expanding each
   task into agentic subtasks — count + complexity of subtasks = parent's score.
4. Any task scoring **≥ 5 pts** (≈ 2–3 working days for mid-level engineer) MUST be
   expanded into subtasks. Loop until every task is < 5 pts or already atomic.
5. GWT evaluation: tasks must satisfy (a) atomic, (b) well-named, (c) not overly
   verbose, (d) follow quality engineering principles, (e) GWT criteria fully flushed
   out for each. Route back to fix step if any fail.
6. Final accumulated output: `tasks.md` with hierarchical task tree.

### SW2 — Desired Output State (`desired-output-state.yml`)
**Output:** `outputs.md` (single MD file, accumulated & iterated)

Pipeline:
1. Read `tasks.md` from SW1.
2. Chunk tasks by complexity groups (start with chunks of 3, gradually increase to 5).
3. For each chunk, LLM call: produce "desired end-state" for each task — what observable
   output state proves this task is complete? Include testable criteria (GWT-style).
4. GWT evaluation per chunk: each desired state must be (a) testable, (b) unambiguous,
   (c) tied to a specific task, (d) not over-scoped.
5. Iterate chunks until quality bar passes; merge all chunks into single `outputs.md`.

### SW3 — Agentic Categorization (`agentic-categorization.yml`)
**Output:** `categories.md` (single MD file, accumulated & iterated)

Pipeline:
1. Read `tasks.md` + `outputs.md` from SW1 + SW2.
2. LLM call per task: assign category label (e.g., "file-read", "transform-llm",
   "validate-gate", "loop-iterate", "branch-decision", "shell-execute").
3. GWT: categories must be from the canonical set (defined in plan), mutually exclusive
   per task, and explain WHY the category fits.
4. Iterate until all tasks categorized consistently. Output: `categories.md`.

### SW4 — YAML Substructure Translation (`yaml-substructure-translation.yml`)
**Output:** `structs.md` (single MD file; **YAML code blocks only**, with markdown prose between)

Pipeline:
1. Read `tasks.md` + `outputs.md` + `categories.md`.
2. LLM call per category: emit YAML substructure skeleton for each task in that category
   (using `agentic_workflow.steps.<step_name>:` shape from schema).
3. GWT: each YAML substructure must (a) reference correct model, (b) have correct hook
   wiring for its category, (c) include dependencies on prior tasks, (d) NOT contain
   non-YAML code blocks (no ```rust, ```python, ```bash — only ```yaml).
4. Iterate. Final: `structs.md` documents the YAML substructures + intent prose.

### SW5 — Final Workflow Assembly (`final-workflow-assembly.yml`)
**Output:** `workflow.yml` (executable agentic workflow file)

Pipeline:
1. Read `structs.md` from SW4.
2. LLM call: assemble full workflow YAML (header + providers + models + execution_strategy
   + agentic_workflow.steps merging all substructures).
3. Deterministic post-process (shell hook): run `scripts/fix-generated-yaml.py` then
   `scripts/validate-yaml.py`.
4. Per-task exhaustive review loop: for each task in `tasks.md`, verify the task appears
   and is exhaustively handled in the assembled YAML. If any task is missing or
   under-implemented, route back to fix step.
5. Final pass: emit `workflow.yml`.

## 4. Meta-Workflow (Orchestrator)

**File:** `meta-workflow-generator-v6.yml`

The orchestrator does NOT call `sub_workflow:` step (engine STUB). Instead, it uses
`shell` actions in `before_step_starts` hooks to invoke
`whitt benchmark --workflow <sub-workflow-N.yml> --output-dir <run-dir>`. Each sub-workflow
runs as a standalone `whitt benchmark` invocation; state passes via the filesystem
(each sub-workflow reads predecessor's `.md` output via shell `cat`).

Run ID substitution uses the proven `__RUN_ID__` + sed pattern from v5.

## 5. Acceptance Criteria (User-Defined)

- [ ] All 5 sub-workflows produce single .md/.yml outputs
- [ ] Each sub-workflow passes live `whitt benchmark` run on Docker llama.cpp
- [ ] Each sub-workflow's output **surpasses** the quality of Sisyphus's manual
      single-shot attempt on the same prompt (`artifacts/*/baseline-opencode/`)
- [ ] Only `qwen/qwen3.5-9b` used; configured per spec in §6 below
- [ ] Heavy hooks in BOTH meta and each sub-workflow
- [ ] Iteration loops with GWT evaluation cycles in each sub-workflow
- [ ] Extensive logging in every step of every workflow
- [ ] Unit tests written ONLY after live system validation confirms behavior
- [ ] Dataset = real high-complexity prompts harvested from opencode sessions
- [ ] Final `workflow.yml` is valid, parses, and is executable

## 6. Qwen3.5-9B Configuration (LOCKED)

```yaml
models:
  "qwen35":
    name: "Qwen3.5-9B-UD-Q4_K_XL.gguf"
    host:
      type: llama_cpp_with_vulkan
```

Server CLI (set via Docker config → `LLAMA_ARG_*` env vars in `src/config/mod.rs`):

| Param | Value | LLAMA_ARG env var |
|---|---|---|
| Context length | `262144` | `LLAMA_ARG_CTX_SIZE` |
| GPU offload | `0` (CPU-only) | `LLAMA_ARG_N_GPU_LAYERS` |
| CPU threads | `5` | `LLAMA_ARG_N_THREADS` |
| Concurrent slots | `1` | `LLAMA_ARG_PARALLEL` |
| KV cache K type | `q8_0` | `LLAMA_ARG_CACHE_TYPE_K` |
| KV cache V type | `q8_0` | `LLAMA_ARG_CACHE_TYPE_V` |

**HF repo:** `unsloth/Qwen3.5-9B-GGUF` — file `Qwen3.5-9B-UD-Q4_K_XL.gguf`
**Download command:**
```bash
hf download unsloth/Qwen3.5-9B-GGUF \
    --local-dir /models/qwen3.5-9b \
    --include "*UD-Q4_K_XL*"
```

## 7. Scope Boundaries

### IN SCOPE
- The 5 sub-workflows + meta orchestrator YAMLs (6 files total)
- Docker config for Qwen3.5-9B
- Curated prompt dataset (≥ 5 prompts)
- Baseline opencode attempts per prompt per SW
- Iteration logs with evidence (run logs + outputs)
- Unit tests for confirmed engine behaviors only

### OUT OF SCOPE (DEFERRED)
- Implementing `sub_workflow:` step support in the Rust engine
- Implementing `loop:` validation-based loop support in the Rust engine
- Adding semantic `complexity_score` to engine contexts (token-ratio is current)
- Streaming hooks (`during_step_streaming` — not wired)
- Cross-process state-passing beyond filesystem .md files

## 8. Success Definition

The plan is **complete** when:
1. `meta-workflow-generator-v6.yml` runs end-to-end on Docker llama.cpp.
2. Each of the 5 sub-workflows has a `golden/` artifact matching or beating the
   `baseline-opencode/` artifact for the same input prompt.
3. The final assembled `workflow.yml` is schema-valid and executable by
   `whitt benchmark --workflow workflow.yml`.
4. All iteration gates documented in `07-QA-AND-VERIFICATION.md` pass.
5. Unit tests in `tests/meta_workflow/` cover the confirmed execution paths.

## 9. Non-Goals

- Producing workflows that rival Claude-Opus-grade output. We are compensating for an
  SLM via iteration — the goal is **useful, schema-valid, executable** output, not
  state-of-the-art generation quality.
- Migrating v5's existing pipeline. v6 is a clean rebuild with sub-workflows.
- Replacing the engine's loop/sub-workflow stubs. Document gaps; do not fix engine code
  except where blocked.
