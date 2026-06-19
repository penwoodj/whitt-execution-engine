# 01 — Objectives and Scope

## Executive Summary

This document defines the complete objectives, scope boundaries, acceptance criteria, and quality benchmarks for the Qwen 3.5-9B meta-workflow generator. Every objective traces directly to the user's prompt requirements. Every acceptance criterion is measurable. Every scope boundary is explicit.

---

## 1. Core Objective

**Build a meta-workflow generator that takes a single high-complexity agentic prompt as input and produces a fully-specified, executable agentic workflow YAML file as output.**

The generator achieves this by routing the input prompt through five sequential sub-workflows (SW1 → SW5), each producing a single well-formatted Markdown artifact. The meta-workflow orchestrator (`docs/benchmarks/workflows/meta-workflow-v6.yml`) chains these sub-workflows via shell-based delegation.

**Model**: Qwen 3.5-9B (Q4_K_M quantization) running in llama.cpp with full RX580 GPU offload.

---

## 2. Detailed Objectives

### 2.1 Objective: Task Deconstruction (SW1)

**Statement**: Break down a high-complexity prompt into atomic tasks with story-point complexity scoring and GWT criteria.

**Requirements**:
- Decompose prompt into top-level task list (T1, T2, T3, ...)
- Score each task using story points: 1, 2, 3, 5, 8, 13
- Story point calibration (mid-level engineering team context):
  - 1 point = trivial, <4 hours, single obvious action
  - 2 points = small, 4h-1d, one well-known transform
  - 3 points = medium, 1-2d, multi-step but linear
  - 5 points = large, 2-3d, multi-step with branching (MUST be split)
  - 8 points = too large, 3-5d, multi-component (MUST be split)
  - 13 points = epic, 1+ week (MUST be split)
- For every task scoring ≥5 points: expand into agentic subtasks
- Evaluate complexity by breaking down groups of 5 tasks into subtasks
- Count and complexity of subtasks in total gives the complexity rating
- No single task ≥5 story points may remain unexpanded
- GWT criteria (Given/When/Then) fully flushed out for every task
- Not overly verbose — each task action <100 chars

**Evaluation/Fix Loop**:
- Auto-evaluate against 10 criteria (ATOMIC, COMPLETE, NON_REDUNDANT, GWT_READY, NO_HIGH_COMPLEXITY_LEAVES, CONCISE, ANALYSIS_PHASE_PRESENT, COVERAGE_DEPTH, SUBTASK_EXPANSION, COMPLEXITY_RATING_VALID)
- If VERDICT: FAIL → route to fix step → re-evaluate
- Loop until VERDICT: PASS or max iterations reached

**Output**: Single `.md` file at `outputs/meta-workflow/__RUN_ID__/sw1/tasks.md`

**Acceptance Criteria**:
- [ ] Every task scoring ≥5 has subtasks
- [ ] Every leaf task has GWT criteria (Given + When + Then, each non-empty)
- [ ] No leaf task scores ≥5
- [ ] Total tasks ≥10 (for complex prompts)
- [ ] Every subtask action <100 chars
- [ ] Output is valid markdown with consistent formatting

### 2.2 Objective: Desired Output State Translation (SW2)

**Statement**: Translate task breakdown into desired output state — what the system looks like after each task completes.

**Requirements**:
- For every task (parent AND leaf): produce a desired output state entry
- Desired state describes what is TRUE after task completes (post-condition, not action)
- Artifacts produced: files, data structures, values created
- Acceptance criteria: observable and testable
- Evaluation method: CONCRETE verification (shell command, test name, check)
  - NEVER use vague methods like "manual inspection"
  - Examples: `grep -c 'pattern' file.md`, `test -f path`, `python3 -c 'import yaml; yaml.safe_load(...)'`
- Failure indicators: what failure looks like
- Downstream dependencies: which tasks consume this output

**Chunk Sizing Strategy** (compensating for small model limitations):
- First 2 chunks: 2-3 leaf tasks each (warm-up, validate approach)
- Middle chunks: 3-4 leaf tasks each (steady-state)
- Final chunks: 4-5 leaf tasks each (leverage accumulated context)
- START SMALL, GROW GRADUALLY

**Evaluation/Fix Loop**:
- Evaluate against 7 criteria (COVERAGE, OBSERVABLE, TESTABLE, ARTIFACTS, DEPENDENCIES, FAILURE_MODES, CONCISE)
- If VERDICT: FAIL → route to fix step → re-evaluate

**Output**: Single `.md` file at `outputs/meta-workflow/__RUN_ID__/sw2/outputs.md`

**Acceptance Criteria**:
- [ ] Every task from SW1 has an output-state block
- [ ] Every "Desired state" is observable
- [ ] Every "Acceptance criteria" is testable
- [ ] Every block specifies concrete artifacts
- [ ] Every block <200 words

### 2.3 Objective: Agentic Categorization (SW3)

**Statement**: Translate deconstructed tasks and desired outputs into categories of agentic behavior that correlate to different substructures.

**Requirements**:
- Categorize tasks into agentic behavior types
- Categories correlate to workflow substructures (generation, evaluation, routing, tool-use, etc.)
- Iterative LLM process with evaluation/fix cycles
- NOT translating into the framework yet — just categorizing
- Chunked processing for small model efficiency

**Evaluation/Fix Loop**:
- Evaluate category coverage, consistency, accuracy
- Fix misclassifications

**Output**: Single `.md` file at `outputs/meta-workflow/__RUN_ID__/sw3/categories.md`

**Acceptance Criteria**:
- [ ] Every task from SW1/SW2 has a category assignment
- [ ] Categories are consistent (similar tasks get same category)
- [ ] Categories map to known agentic substructures

### 2.4 Objective: YAML Substructure Translation (SW4)

**Statement**: Translate categorized task list into individual agentic substructures with intent descriptions.

**Requirements**:
- Each task gets a YAML substructure draft
- Substructures include intent description (what they accomplish)
- Substructures describe how they fit within the rest of the task list
- ONLY YAML code blocks allowed in output (no other code block types)
- Markdown descriptions accompany YAML snippets

**Evaluation/Fix Loop**:
- Evaluate YAML validity, completeness, substructure correctness
- Fix malformed or incomplete substructures

**Output**: Single `.md` file at `outputs/meta-workflow/__RUN_ID__/sw4/substructures.md`

**Acceptance Criteria**:
- [ ] Every task has a YAML substructure
- [ ] YAML is syntactically valid
- [ ] No non-YAML code blocks in output
- [ ] Each substructure has intent description

### 2.5 Objective: Final Workflow Assembly (SW5)

**Statement**: Construct a valid, parsable, executable agentic workflow YAML from all prior documents.

**Requirements**:
- Exhaustively include all information from original task breakdown
- Each task evaluated for how it accomplishes its objective in the YAML
- As deterministic as possible
- Output must be valid YAML parseable by `serde_saphyr`
- Output must pass `WorkflowFile::validate()`
- Output must be executable by `whitt benchmark --workflow <file>`

**Evaluation/Fix Loop**:
- Evaluate YAML syntax, schema compliance, task coverage, executability
- Fix any validation failures

**Output**: Single `.yml` file at `outputs/meta-workflow/__RUN_ID__/sw5/workflow.yml`

**Acceptance Criteria**:
- [ ] YAML parses without errors
- [ ] YAML passes schema validation
- [ ] Every task from SW1 is represented
- [ ] Workflow has valid providers/models/steps structure
- [ ] Workflow is executable (passes `whitt benchmark` dry run)

---

## 3. Iterative Quality Standard

### 3.1 Definition: "Surpasses Opencode Baseline Quality"

For each sub-workflow, the output quality must SURPASS what OpenCode (running as an AI agent) can produce in a single-shot attempt given the same input prompt.

**Measurement Method**:
1. Select a test prompt from the dataset (`docs/plans/meta-workflow-qwen35/test-prompts/real/`)
2. OpenCode agent processes the prompt manually (single-shot, minimal iteration)
3. OpenCode output = BASELINE
4. Sub-workflow processes same prompt via `whitt benchmark`
5. Sub-workflow output = CANDIDATE
6. Compare BASELINE vs CANDIDATE across quality dimensions

**Quality Dimensions**:
| Dimension | Weight | Measurement |
|-----------|--------|-------------|
| Task Coverage | 25% | % of prompt objectives addressed |
| GWT Completeness | 20% | % of tasks with full GWT criteria |
| Complexity Compliance | 15% | % of tasks ≥5pts that are expanded |
| Conciseness | 10% | Avg action length (target <100 chars) |
| Format Consistency | 10% | % of entries following output format |
| Testability | 10% | % of criteria that are concrete/verifiable |
| Error Rate | 10% | % of evaluation criteria passing |

**Pass Threshold**: CANDIDATE score ≥ BASELINE score + 10% margin

### 3.2 Iteration Protocol

For each sub-workflow:
1. Create baseline (OpenCode single-shot)
2. Run sub-workflow on same prompt
3. Score both outputs
4. If CANDIDATE < BASELINE + 10%:
   - Analyze failure modes
   - Update sub-workflow YAML (prompts, evaluation criteria, chunk sizes)
   - Re-run
5. Repeat until pass threshold met or max 10 iterations

---

## 4. Scope Boundaries

### 4.1 In Scope

- Meta-workflow generator using Qwen 3.5-9B in llama.cpp (full RX580 GPU offload)
- Five sub-workflows (SW1-SW5) with iterative evaluation/fix loops
- YAML-driven workflow definitions following unified schema
- Hook-driven execution (all behavior from YAML, not hardcoded)
- Single `.md` output per sub-workflow (accumulated, iterated)
- Final `.yml` workflow as ultimate output
- Test prompt dataset from real OpenCode sessions
- Live system testing via Docker + llama.cpp
- Unit tests written AFTER live system validation

### 4.2 Out of Scope

- Full GPU offload (gpu_layers: 99, RX580)
- Non-Qwen models (single model focus)
- Workflow generation for simple prompts (target: complex agentic prompts only)
- Real-time/streaming workflow generation (batch processing)
- Multi-model orchestration within sub-workflows
- External API calls (all inference local)
- Web search integration in workflows
- GUI/visual workflow editor

### 4.3 Deferred

- `during_step_streaming` trigger (requires SSE architecture changes)
- `IterateValues` action (stub implementation)
- Multi-model comparison within sub-workflows
- Workflow versioning and rollback
- Automated prompt complexity pre-screening

---

## 5. Constraints

### 5.1 Model Constraints

| Parameter | Value | Reason |
|-----------|-------|--------|
| Model | Qwen 3.5-9B Q4_K_M | User specification |
| Context window | 262,144 tokens | Full Qwen 3.5 context |
| GPU layers | 99 | Full GPU offload on RX580 |
| CPU threads | 5 | User specification |
| Parallel slots | 1 | Single-shot deterministic |
| KV cache | Q8_0 | Larger context support |

### 5.2 Small Model Limitations

- Limited reasoning depth per inference call
- Tendency to hallucinate or skip steps in complex prompts
- Context window pressure with large task lists
- Lower instruction-following accuracy vs larger models

**Compensation Strategies**:
- Iterative evaluation/fix loops (verify each step, fix failures)
- Chunked processing (small groups, grow gradually)
- Focused prompts (single objective per step)
- Template-driven output (strict format enforcement)
- GWT-based conditional routing (automated quality gates)

### 5.3 Infrastructure Constraints

- Docker compose for llama.cpp server management
- `LoadParams::to_env_vars()` passes config to Docker via env vars
- `LLAMA_ARG_*` environment variables control server startup
- Vulkan backend constraints: `cont_batching: false`, `no_cache_prompt: true`

---

## 6. Success Criteria Summary

| Criterion | Measurement | Target |
|-----------|-------------|--------|
| SW1 produces task breakdown | Output .md exists, well-formatted | ✅ |
| SW1 surpasses baseline | Quality score comparison | CANDIDATE ≥ BASELINE + 10% |
| SW2 produces output states | Output .md exists, covers all tasks | ✅ |
| SW2 surpasses baseline | Quality score comparison | CANDIDATE ≥ BASELINE + 10% |
| SW3 produces categories | Output .md exists, all tasks categorized | ✅ |
| SW3 surpasses baseline | Quality score comparison | CANDIDATE ≥ BASELINE + 10% |
| SW4 produces substructures | Output .md with valid YAML snippets | ✅ |
| SW4 surpasses baseline | Quality score comparison | CANDIDATE ≥ BASELINE + 10% |
| SW5 produces workflow YAML | Output .yml parses + validates | ✅ |
| SW5 surpasses baseline | Quality score comparison | CANDIDATE ≥ BASELINE + 10% |
| Meta-workflow chains SW1-SW5 | Sequential execution completes | ✅ |
| Live system testing passes | Docker run produces valid outputs | ✅ |
| Unit tests cover confirmed behavior | Test suite passes | ✅ |

---

## 7. Quality Engineering Principles

All sub-workflows and their outputs must follow:

1. **Atomic Tasks**: Every leaf task is a single, well-scoped action
2. **GWT Completeness**: Every task has Given/When/Then criteria
3. **Complexity Compliance**: No leaf task ≥5 story points
4. **Coverage**: Every part of the original prompt is addressed
5. **Non-Redundancy**: No two tasks duplicate work
6. **Conciseness**: Action descriptions <100 chars, blocks <200 words
7. **Testability**: All criteria are concrete and verifiable
8. **Analysis Phasing**: Analysis tasks split into read→analyze→execute→verify
9. **Deterministic Output**: Same input produces same output (temperature ≤0.2)
10. **Format Consistency**: All entries follow documented format

---

## 8. References

- Meta-workflow orchestrator: `docs/benchmarks/workflows/meta-workflow-v6.yml`
- Sub-workflow YAMLs: `docs/benchmarks/workflows/sw{1-5}-*.yml`
- Unified schema: `docs/schema/unified-workflow-schema.yml`
- LoadParams struct: `src/model/schema.rs`
- Runner config extraction: `src/benchmark/runner.rs:510-608`
- Docker env var wiring: `src/benchmark/runner.rs:1486-1531`
- Test prompts: `docs/plans/meta-workflow-qwen35/test-prompts/real/`
- Iteration protocol: `docs/plans/meta-workflow-qwen35/04-ITERATION-PROTOCOL.md`
- Quality benchmark methodology: `docs/plans/meta-workflow-qwen35/05-QUALITY-BENCHMARK.md`

---

**Document Status:** Complete
**Last Updated:** 2026-06-17
**Author:** Sisyphus (Whitt Execution Engine Planning)
**Review Status:** Ready for Execution
